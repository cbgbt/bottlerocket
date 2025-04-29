use anyhow::{Context, Result};
use lsp_types as lsp;
use std::path::{Path, PathBuf};
use util::{crate_name, file_uri};

pub mod lints;
pub mod rustanalyzer;
pub mod util;

pub use lints::{Linter, plugin::SettingsPluginLinter};

#[derive(Debug)]
pub struct BottlerocketRepo {
    pub repo_path: PathBuf,
    pub settings_profiles: Vec<SettingsProfile>,
    pub settings_plugins: Vec<SettingsPlugin>,
}

impl BottlerocketRepo {
    pub fn path_uri(&self) -> Result<lsp::Uri> {
        file_uri(&self.repo_path)
    }

    pub fn from_path(repo_path: impl Into<PathBuf>) -> Result<Self> {
        let repo_path = repo_path.into();
        let settings_profiles = Self::settings_profiles(&repo_path)?;
        let settings_plugins = Self::settings_plugins(&repo_path)?;

        Ok(Self {
            repo_path,
            settings_profiles,
            settings_plugins,
        })
    }

    /// Returns the list of "settings profiles" under sources/settings-defaults
    fn settings_profiles(repo_path: impl AsRef<Path>) -> Result<Vec<SettingsProfile>> {
        let repo_path = repo_path.as_ref();
        let settings_path = repo_path.join("sources/settings-defaults");

        Ok(std::fs::read_dir(&settings_path)
            .context(format!(
                "Failed to read settings directory '{}'",
                settings_path.display()
            ))?
            .map(|entry| {
                let entry = entry.context("Failed to read directory entry")?;
                Ok(entry.path())
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter_map(|path| path.is_dir().then_some(SettingsProfile::from_path(path)))
            .collect())
    }

    /// Returns the list of settings plugin crates under sources/settings-plugins
    fn settings_plugins(repo_path: impl AsRef<Path>) -> Result<Vec<SettingsPlugin>> {
        let repo_path = repo_path.as_ref();
        let settings_path = repo_path.join("sources/settings-plugins");

        std::fs::read_dir(&settings_path)
            .context(format!(
                "Failed to read settings directory '{}'",
                settings_path.display()
            ))?
            .map(|entry| {
                let entry = entry.context("Failed to read directory entry")?;
                Ok(entry.path())
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter_map(|path| path.is_dir().then_some(SettingsPlugin::from_path(path)))
            .collect::<Result<_, _>>()
            .context("Failed to load settings plugin crates")
    }
}

#[derive(Debug, Clone)]
pub struct SettingsProfile {
    pub path: PathBuf,
}

impl SettingsProfile {
    fn from_path(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsPlugin {
    pub path: PathBuf,
    pub crate_name: String,
}

impl SettingsPlugin {
    fn from_path(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let crate_name = crate_name(&path)?;
        Ok(Self {
            path: path.into(),
            crate_name,
        })
    }
}
