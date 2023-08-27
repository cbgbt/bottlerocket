use anyhow::{ensure, Context, Result};
use argh::FromArgs;
use lazy_static::__Deref;
use rayon::prelude::*;
use schnauzer::import::TemplateImporter;
use std::{
    cell::RefCell,
    fs::create_dir_all,
    path::{Path, PathBuf},
    str::FromStr,
};
use std::{fs::File, process::Command};

#[derive(Debug, FromArgs)]
/// template rendering comparison tests between schnauzer v1 and v2
struct Args {
    /// directory containing models for all desired variants
    #[argh(option)]
    models: PathBuf,

    /// git repo containing v1 templates
    #[argh(option)]
    v1_repo: GitRepo,

    /// git repo containing v2 templates
    #[argh(option)]
    v2_repo: GitRepo,

    /// template paths file -- newline-delimited list of relative paths in the git repos to check.
    #[argh(option)]
    template_paths: PathBuf,
}

#[derive(Debug)]
struct GitRepo {
    repo_url: String,
    repo_branch: String,
}

impl GitRepo {
    fn clone_to<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        let status_code = Command::new("git")
            .args([
                "clone",
                "--branch",
                &self.repo_branch,
                &self.repo_url,
                &dir.as_ref().to_string_lossy(),
            ])
            .status()
            .context(format!("failed to git clone {:?}", self))?;

        ensure!(
            status_code.success(),
            format!("failed to git clone {:?}", self)
        );

        Ok(())
    }
}

impl FromStr for GitRepo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split('@');
        anyhow::ensure!(parts.clone().count() == 2, "invalid git repo");
        let repo_url = parts.next().unwrap().to_string();
        let repo_branch = parts.next().unwrap().to_string();

        Ok(Self {
            repo_url,
            repo_branch,
        })
    }
}

fn read_template_filepaths<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    Ok(std::fs::read_to_string(path.as_ref())
        .context("failed to read template paths file")?
        .trim()
        .split_ascii_whitespace()
        .map(PathBuf::from)
        .collect())
}

fn all_models<P: AsRef<Path>>(
    models: P,
) -> Result<impl Iterator<Item = Result<(String, String, serde_json::Value)>>> {
    let models = models.as_ref();

    let models_dir = std::fs::read_dir(models).context("failed to read models directory")?;

    let model_dirs = models_dir
        .map(|file_entry| {
            let file_entry = file_entry.context("failed to read models directory")?;
            let variant_name = file_entry.file_name().to_string_lossy().to_string();
            Ok(
                if file_entry
                    .file_type()
                    .context("failed to read models directory")?
                    .is_dir()
                {
                    Some((variant_name, file_entry.path()))
                } else {
                    None
                },
            )
        })
        .collect::<Result<Vec<Option<(String, PathBuf)>>>>()?
        .into_iter()
        .filter_map(std::convert::identity);

    // This is kind of illegible, but it's hard to make it straightforward while handling all error cases.
    Ok(model_dirs
        // For every model dir, we're creating an iterator over the models inside to flatten together later.
        .map(|(variant_name, model_dir)| {
            let model_dir = std::fs::read_dir(model_dir).context(format!(
                "failed to read model directory for {}",
                variant_name
            ))?;

            // Iterate through everything that could be a model file for this variant
            Ok(model_dir
                .map(|model_file_entry| {
                    let model_file_entry = model_file_entry.context(format!(
                        "failed to read model directory for {}",
                        variant_name
                    ))?;
                    let model_filepath = model_file_entry.path();
                    let model_name = model_file_entry.file_name().to_string_lossy().to_string();
                    let file_type = model_file_entry.file_type().context(format!(
                        "Failed to stat file type of model file {}",
                        model_filepath.to_string_lossy()
                    ))?;
                    Ok((variant_name.clone(), model_name, model_filepath, file_type))
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .filter_map(|(variant_name, model_name, model_filepath, file_type)| {
                    file_type.is_file().then(|| {
                        File::open(&model_filepath)
                            .context(format!(
                                "Failed to open model file {}",
                                model_filepath.to_string_lossy()
                            ))
                            .and_then(|model_file| {
                                serde_json::from_reader(&model_file).context(format!(
                                    "Failed to parse model file {}",
                                    model_filepath.to_string_lossy()
                                ))
                            })
                            .map(|model| (variant_name.clone(), model_name.clone(), model))
                    })
                }))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten())
}

thread_local! {
    pub static RUNTIME: RefCell<tokio::runtime::Runtime> = RefCell::new(tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap());
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let operating_dir = tempfile::TempDir::new().context("Failed to create temp dir")?;
    create_dir_all(operating_dir.path())?;

    let v1_dir = operating_dir.path().join("v1");
    let v2_dir = operating_dir.path().join("v2");
    args.v1_repo.clone_to(&v1_dir)?;
    args.v2_repo.clone_to(&v2_dir)?;

    let template_filepaths = read_template_filepaths(&args.template_paths)
        .context("failed to read template paths file")?;

    let templates = template_filepaths
        .iter()
        .map(|filepath| {
            let v1_filepath = v1_dir.join(filepath);
            let v2_filepath = v2_dir.join(filepath);

            let filepath_str = filepath.to_string_lossy();

            let v1_template = std::fs::read_to_string(v1_filepath)
                .context(format!("Failed to read template file {}", filepath_str))?;
            let v2_template = std::fs::read_to_string(v2_filepath)
                .context(format!("Failed to read template file {}", filepath_str))?;
            Ok((filepath_str, v1_template, v2_template))
        })
        .collect::<Result<Vec<_>>>()?;

    all_models(&args.models)?
        .par_bridge()
        .filter_map(|maybe_model| {
            if let Err(e) = &maybe_model {
                eprintln!("{}", e);
            }
            maybe_model.ok()
        })
        .for_each(|(variant, model_name, model)| {
            RUNTIME.with(|rt| {
                let rt = rt.borrow();
                for (filepath, v1_template, v2_template) in &templates {
                    let v1 = render_v1(&v1_template, &model);
                    let v2 = rt.block_on(render_v2(&v2_template, &model));

                    if let Err(e) = v2 {
                        eprintln!(
                            "Error rendering template {} on variant {}",
                            filepath, variant
                        );
                    }

                    // match (&v1, &v2) {
                    //     (Err(_), Err(_)) => {
                    //         // Both failed, good.
                    //         continue;
                    //     }
                    //     (Ok(r1), Ok(r2)) if r1 == r2 => {
                    //         // Perfect case, they match
                    //         continue;
                    //     }
                    //     (Ok(r1), Ok(r2)) if r1 != r2 => {
                    //         eprintln!("Template {} was not the same!!!!", filepath);
                    //         eprintln!("Rendered for variant/model {}/{}", variant, model_name);
                    //         eprintln!("v1:\n{}\n", r1);
                    //         eprintln!("v2:\n{}\n", r2);
                    //     }
                    //     _ => {
                    //         eprintln!("Template {} was not the same!!!!", filepath);
                    //         eprintln!("Rendered for variant/model {}/{}", variant, model_name);
                    //         eprintln!("v1:\n{:?}\n", v1);
                    //         eprintln!("v2:\n{:?}\n", v2);
                    //     }
                    // }
                }
            })
        });

    Ok(())
}

fn render_v1(template: &str, model: &serde_json::Value) -> Result<String> {
    Ok("asplod".to_string())
}

struct HalfFakeImporter {
    settings: schnauzer::FakeSettingsResolver,
    helpers: schnauzer::import::StaticHelperResolver,
}

impl TemplateImporter for HalfFakeImporter {
    type SettingsResolver = schnauzer::FakeSettingsResolver;
    type HelperResolver = schnauzer::import::StaticHelperResolver;

    fn settings_resolver(&self) -> &Self::SettingsResolver {
        &self.settings
    }

    fn helper_resolver(&self) -> &Self::HelperResolver {
        &self.helpers
    }
}

async fn render_v2(template: &str, model: &serde_json::Value) -> Result<String> {
    let importer = HalfFakeImporter {
        settings: schnauzer::FakeSettingsResolver::new(serde_json::json!({"settings": model})),
        helpers: Default::default(),
    };

    schnauzer::v2::render_template_str(&importer, template)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}
