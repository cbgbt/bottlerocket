use super::Linter;
use crate::rustanalyzer::{RustAnalyzer, RustAnalyzerError, ServerStatus};
use crate::util::file_uri;
use crate::{BottlerocketRepo, SettingsPlugin};
use anyhow::{Context, Result};
use lsp_types::{self as lsp, notification};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use syn::spanned::Spanned;

const _MAX_RUST_ANALYZER_WAIT: std::time::Duration = std::time::Duration::from_secs(120);

pub struct SettingsPluginLinter<'a> {
    prev_repo: &'a BottlerocketRepo,
    next_repo: &'a BottlerocketRepo,
    rust_analyzer: RustAnalyzer,
}

impl SettingsPluginLinter<'_> {
    pub async fn new<'a>(
        prev_repo: &'a BottlerocketRepo,
        next_repo: &'a BottlerocketRepo,
    ) -> Result<SettingsPluginLinter<'a>> {
        let rust_analyzer = Self::initialize_rust_analyzer(prev_repo, next_repo).await?;

        Ok(SettingsPluginLinter {
            prev_repo,
            next_repo,
            rust_analyzer,
        })
    }

    async fn initialize_rust_analyzer(
        prev_repo: &BottlerocketRepo,
        next_repo: &BottlerocketRepo,
    ) -> Result<RustAnalyzer> {
        let ra = RustAnalyzer::start(std::env::current_dir().unwrap()).await?;
        let mut ra_notifications = ra.notifications();

        ra.send_request::<lsp::request::Initialize>(lsp::InitializeParams {
            process_id: Some(std::process::id()),
            workspace_folders: Some(vec![
                lsp::WorkspaceFolder {
                    uri: file_uri(prev_repo.repo_path.join("sources"))?,
                    name: "previous-repo".to_string(),
                },
                lsp::WorkspaceFolder {
                    uri: file_uri(next_repo.repo_path.join("sources"))?,
                    name: "proposed-repo".to_string(),
                },
            ]),
            client_info: Some(lsp::ClientInfo {
                name: "settings-migration-lint".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: lsp::ClientCapabilities {
                experimental: Some(serde_json::json!({
                    "serverStatusNotification": true
                })),
                ..Default::default()
            },
            ..Default::default()
        })
        .await?;

        ra.send_notification::<lsp::notification::Initialized>(lsp::InitializedParams {})
            .await?;

        // Wait for the analyzer to become "quiescent"
        println!("Waiting for rust-analyzer to become quiescent");
        while let Ok((method, notification)) = ra_notifications.recv().await {
            if method == "experimental/serverStatus" {
                let status: ServerStatus = serde_json::from_value(notification.clone()).context(
                    format!("Failed to parse rust-analyzer server status from '{notification}'"),
                )?;
                anyhow::ensure!(
                    status.health == "ok",
                    format!("rust-analyzer is not healthy: {:?}", status.message)
                );
                if status.quiescent {
                    println!("rust-analyzer is ready.");
                    break;
                } else {
                    println!("rust-analyzer is not yet quiescent.");
                    if let Some(msg) = status.message {
                        println!("rust-analyzer status message: {msg}");
                    }
                }
            }
        }

        Ok(ra)
    }

    /// Returns settings plugin crates that are seemingly in-common between both bottlerocket repos
    fn possibly_changed_plugins(&self) -> Vec<(SettingsPlugin, SettingsPlugin)> {
        let prev_plugins: HashMap<&String, &SettingsPlugin> = self
            .prev_repo
            .settings_plugins
            .iter()
            .map(|plugin| (&plugin.crate_name, plugin))
            .collect();

        let next_plugins: HashMap<&String, &SettingsPlugin> = self
            .next_repo
            .settings_plugins
            .iter()
            .map(|plugin| (&plugin.crate_name, plugin))
            .collect();

        prev_plugins
            .into_iter()
            .filter_map(|(crate_name, prev_plugin)| {
                next_plugins
                    .get(&crate_name)
                    .map(|next_plugin| (prev_plugin.clone(), (*next_plugin).clone()))
            })
            .collect()
    }

    /// Tries to find the SettingsPlugin struct within a given SettingsPlugin
    ///
    /// These are usually located in `lib.rs` and have `#[derive(SettingsPlugin)]`
    async fn find_plugin_definition(&self, settings_plugin: &SettingsPlugin) -> Result<()> {
        let source_file = settings_plugin.path.join("src/lib.rs");
        anyhow::ensure!(
            source_file.exists(),
            "No lib.rs found in '{}'",
            source_file.display()
        );

        let plugin_struct = Self::find_settings_plugin_struct(&source_file).await?;
        self.expand_rust_type(&source_file, &plugin_struct).await?;

        Ok(())
    }

    async fn find_settings_plugin_struct(source_file: impl AsRef<Path>) -> Result<syn::ItemStruct> {
        let source_file = source_file.as_ref();
        let source_code = tokio::fs::read_to_string(&source_file).await?;
        let syntax = syn::parse_file(&source_code)
            .context(format!("Failed to parse '{}'", source_file.display()))?;

        let plugin_struct = syntax
            .items
            .iter()
            // Find structs that `#[derive(SettingsPlugin)]`
            .find(|item| {
                if let syn::Item::Struct(item_struct) = item {
                    item_struct.attrs.iter().any(|attr| {
                        if attr.path().is_ident("derive") {
                            let meta = match attr.meta.require_list() {
                                Ok(meta) => meta,
                                Err(_) => return false,
                            };
                            meta.clone()
                                .tokens
                                .into_iter()
                                .any(|token| token.to_string() == "SettingsPlugin")
                        } else {
                            false
                        }
                    })
                } else {
                    false
                }
            })
            .context(format!(
                "Failed to find SettingsPlugin struct in '{}'",
                source_file.display()
            ))?
            .clone();
        let plugin_struct = match plugin_struct {
            syn::Item::Struct(item_struct) => item_struct,
            _ => unreachable!(),
        };
        Ok(plugin_struct)
    }

    async fn expand_rust_type(
        &self,
        source_file: impl AsRef<Path>,
        expand: ToExpand<'_>,
    ) -> Result<ExpandedType> {
        let source_file = source_file.as_ref();
        let source_uri = file_uri(source_file)?;
        let type_loc = str.ident.span().start();

        match expand {
            ToExpand::Struct(str) => {
                let str = syn::ItemStruct {
                    attrs: str.attrs.clone(),
                    vis: str.vis.clone(),
                    struct_token: str.struct_token.clone(),
                    ident: str.ident.clone(),
                    generics: str.generics.clone(),
                    fields: str.fields.clone(),
                    semi_token: str.semi_token.clone(),
                };
            }
            ToExpand::Enum(_) => todo!(),
        }

        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
enum ToExpand<'a> {
    Struct(&'a syn::ItemStruct),
    Enum(&'a syn::ItemEnum),
}

impl Linter for SettingsPluginLinter<'_> {
    async fn lint(&self) -> Result<()> {
        for (prev_plugin, next_plugin) in self.possibly_changed_plugins() {
            let (prev_def, next_def) = tokio::join!(
                self.find_plugin_definition(&prev_plugin),
                self.find_plugin_definition(&next_plugin)
            );
            let _prev_def = prev_def.context(format!(
                "Failed to find settings plugin definition for '{}'",
                prev_plugin.path.display()
            ))?;

            let _next_def = next_def.context(format!(
                "Failed to find settings plugin definition for '{}'",
                next_plugin.path.display()
            ))?;
            break;
        }

        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpandedType {
    /// Fully qualified path, e.g. "crate::foo::Bar"
    pub path: String,

    pub generic_args: Vec<ExpandedType>,

    /// All named fields for struct.
    pub fields: Vec<Field>,

    /// Enum variants.
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    /// The field identifier.
    ///
    /// None for tuple-structs.
    pub name: Option<String>,
    pub ty: Box<ExpandedType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    /// Variant name, e.g. "Rectangle"
    pub name: String,
    /// For tuple- or struct-like variants, this is the fields
    pub fields: Vec<Field>,
}
