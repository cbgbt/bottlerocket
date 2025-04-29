//! Linter that suggests when settings migrations must be written.
//!
//! Given two Bottlerocket repo paths decides:
//! * Have we added or changed a default value that needs a migration?
//! * Have we changed a settings plugin in a way that requires a migration?
use anyhow::{Context, Result};
use argh::FromArgs;
use settings_migration_lint::BottlerocketRepo;
use settings_migration_lint::{Linter, SettingsPluginLinter};
use std::path::PathBuf;

#[derive(FromArgs)]
/// Linter that suggests when settings migrations must be written.
struct MigrationLintArgs {
    /// path to the previous bottlerocket repo
    #[argh(option)]
    previous_bottlerocket_repo: PathBuf,

    /// path to the proposed bottlerocket repo
    #[argh(option)]
    proposed_bottlerocket_repo: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: MigrationLintArgs = argh::from_env();
    let prev_repo =
        BottlerocketRepo::from_path(std::path::absolute(&args.previous_bottlerocket_repo)?)
            .context(format!(
                "Failed to load bottlerocket repo at '{}'",
                args.previous_bottlerocket_repo.display()
            ))?;

    let next_repo =
        BottlerocketRepo::from_path(std::path::absolute(&args.proposed_bottlerocket_repo)?)
            .context(format!(
                "Failed to load bottlerocket repo at '{}'",
                args.proposed_bottlerocket_repo.display()
            ))?;

    SettingsPluginLinter::new(&prev_repo, &next_repo)
        .await
        .context("Failed to create Settings Plugin linter")?
        .lint()
        .await
        .unwrap();

    Ok(())
}
