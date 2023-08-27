//! Provides a helpful "fake" implementation of TemplateImporter to use in test scenarios.
use super::{HelperResolver, SettingsResolver};
use crate::v2::ExtensionRequirement;
use async_trait::async_trait;
use handlebars::Handlebars;

pub struct FakeImporter {
    settings_resolver: FakeSettingsResolver,
    helper_resolver: FakeHelperResolver,
}

impl FakeImporter {
    pub fn new(settings: serde_json::Value, helpers: Vec<(&'static str, StaticHelper)>) -> Self {
        Self {
            settings_resolver: FakeSettingsResolver::new(settings),
            helper_resolver: FakeHelperResolver::new(helpers),
        }
    }
}

crate::impl_template_importer!(FakeImporter, FakeSettingsResolver, FakeHelperResolver);

/// A `SettingsResolver` implementation for test scenarios that use explicit in-memory data.
pub struct FakeSettingsResolver {
    settings: serde_json::Value,
}

impl FakeSettingsResolver {
    pub fn new(settings: serde_json::Value) -> Self {
        Self { settings }
    }

    fn minimize_settings<I>(
        all_settings: &serde_json::Map<String, serde_json::Value>,
        extension_requirements: I,
    ) -> serde_json::Map<String, serde_json::Value>
    where
        I: Iterator<Item = ExtensionRequirement>,
    {
        // TODO: Extension version is disregarded until extensions are implemented on the API side.
        extension_requirements
            // TODO: Disambiguate unset settings vs extension not installed in the API.
            // This isn't possible until extensions are implemented on the API side, so here empty
            // requested settings always silently proceed.
            .filter_map(|extension_requirement| {
                let setting_name = &extension_requirement.name;
                all_settings
                    .get(setting_name)
                    .cloned()
                    .map(|setting_value| (setting_name.to_string(), setting_value))
            })
            // Collect errors from fetching the settings before merging
            .collect()
    }

    fn extract_key_from_api_response(
        key: &str,
        response: &serde_json::Map<String, serde_json::Value>,
    ) -> serde_json::Map<String, serde_json::Value> {
        response.get(key).unwrap().as_object().cloned().unwrap()
    }
}

#[async_trait]
impl SettingsResolver for FakeSettingsResolver {
    async fn fetch_settings<I>(
        &self,
        extension_requirements: I,
    ) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>>
    where
        I: Iterator<Item = ExtensionRequirement> + Send,
    {
        let settings =
            Self::extract_key_from_api_response("settings", &self.settings.as_object().unwrap());
        let minimized = Self::minimize_settings(&settings, extension_requirements);
        Ok(serde_json::json!({"settings": minimized}))
    }
}

/// A handlebars helper defined by a static function.
type StaticHelper = fn(
    &handlebars::Helper,
    &handlebars::Handlebars,
    &handlebars::Context,
    &mut handlebars::RenderContext<'_, '_>,
    &mut dyn handlebars::Output,
) -> handlebars::HelperResult;

pub struct FakeHelperResolver {
    helpers: Vec<(&'static str, StaticHelper)>,
}

impl FakeHelperResolver {
    pub fn new(helpers: Vec<(&'static str, StaticHelper)>) -> Self {
        Self { helpers }
    }
}

/// A `HelperResolver` implementation for test scenarios that use explicit in-memory data.
#[async_trait]
impl HelperResolver for FakeHelperResolver {
    async fn register_template_helpers<'a>(
        &self,
        template_registry: &mut Handlebars<'a>,
        _extension_requirement: &ExtensionRequirement,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        for (name, helper) in &self.helpers {
            template_registry.register_helper(name, Box::new(*helper));
        }
        Ok(())
    }
}
