use crate::{error, Migration, MigrationData, Result};
use serde::Serialize;
use serde_json::Value;
use shlex::Shlex;
use snafu::{OptionExt, ResultExt};
use std::collections::{HashMap, HashSet};

/// We use this migration when we add settings and want to make sure they're removed before we go
/// back to old versions that don't understand them.
pub struct AddSettingsMigration<'a>(pub &'a [&'static str]);

impl Migration for AddSettingsMigration<'_> {
    /// New versions must either have a default for the settings or generate them; we don't need to
    /// do anything.
    fn forward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!(
            "AddSettingsMigration({:?}) has no work to do on upgrade.",
            self.0
        );
        Ok(input)
    }

    /// Older versions don't know about the settings; we remove them so that old versions don't see
    /// them and fail deserialization.  (The settings must be defaulted or generated in new versions,
    /// and safe to remove.)
    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        for setting in self.0 {
            if let Some(data) = input.data.remove(*setting) {
                println!("Removed {}, which was set to '{}'", setting, data);
            } else {
                println!("Found no {} to remove", setting);
            }
        }
        Ok(input)
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// We use this migration when we add a cluster of settings under known prefixes and want to make
/// sure they're removed before we go back to old versions that don't understand them.  Normally
/// you'd use AddSettingsMigration since you know the key names, but this is useful for
/// user-defined keys, for example in a map like settings.kernel.sysctl or
/// settings.host-containers.
pub struct AddPrefixesMigration(pub Vec<&'static str>);

impl Migration for AddPrefixesMigration {
    /// New versions must either have a default for the settings or generate them; we don't need to
    /// do anything.
    fn forward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!(
            "AddPrefixesMigration({:?}) has no work to do on upgrade.",
            self.0
        );
        Ok(input)
    }

    /// Older versions don't know about the settings; we remove them so that old versions don't see
    /// them and fail deserialization.  (The settings must be defaulted or generated in new versions,
    /// and safe to remove.)
    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        let settings = input
            .data
            .keys()
            .filter(|k| self.0.iter().any(|prefix| k.starts_with(prefix)))
            .cloned()
            .collect::<Vec<_>>();
        for setting in settings {
            if let Some(data) = input.data.remove(&setting) {
                println!("Removed {}, which was set to '{}'", setting, data);
            }
        }
        Ok(input)
    }
}

#[cfg(test)]
mod test_add_prefixes_migration {
    use super::AddPrefixesMigration;
    use crate::{Migration, MigrationData};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn single() {
        let data = MigrationData {
            data: hashmap! {
                "keep.me.a".into() => 0.into(),
                "remove.me.b".into() => 0.into(),
                "keep.this.c".into() => 0.into(),
                "remove.me.d.e".into() => 0.into(),
            },
            metadata: HashMap::new(),
        };
        // Run backward, e.g. downgrade, to test that the right keys are removed
        let result = AddPrefixesMigration(vec!["remove.me"])
            .backward(data)
            .unwrap();
        assert_eq!(
            result.data,
            hashmap! {
                "keep.me.a".into() => 0.into(),
                "keep.this.c".into() => 0.into(),
            }
        );
    }

    #[test]
    fn multiple() {
        let data = MigrationData {
            data: hashmap! {
                "keep.me.a".into() => 0.into(),
                "remove.me.b".into() => 0.into(),
                "keep.this.c".into() => 0.into(),
                "remove.this.d.e".into() => 0.into(),
            },
            metadata: HashMap::new(),
        };
        // Run backward, e.g. downgrade, to test that the right keys are removed
        let result = AddPrefixesMigration(vec!["remove.me", "remove.this"])
            .backward(data)
            .unwrap();
        assert_eq!(
            result.data,
            hashmap! {
                "keep.me.a".into() => 0.into(),
                "keep.this.c".into() => 0.into(),
            }
        );
    }

    #[test]
    fn no_match() {
        let data = MigrationData {
            data: hashmap! {
                "keep.me.a".into() => 0.into(),
                "remove.me.b".into() => 0.into(),
                "keep.this.c".into() => 0.into(),
                "remove.this.d.e".into() => 0.into(),
            },
            metadata: HashMap::new(),
        };
        // Run backward, e.g. downgrade, to test that the right keys are removed
        let result = AddPrefixesMigration(vec!["not.found", "nor.this"])
            .backward(data)
            .unwrap();
        assert_eq!(
            result.data,
            hashmap! {
                "keep.me.a".into() => 0.into(),
                "remove.me.b".into() => 0.into(),
                "keep.this.c".into() => 0.into(),
                "remove.this.d.e".into() => 0.into(),
            }
        );
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// We use this migration when we remove settings from the model, so the new version doesn't see
/// them and error.
pub struct RemoveSettingsMigration<'a>(pub &'a [&'static str]);

impl Migration for RemoveSettingsMigration<'_> {
    /// Newer versions don't know about the settings; we remove them so that new versions don't see
    /// them and fail deserialization.  (The settings must be defaulted or generated in old versions,
    /// and safe to remove.)
    fn forward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        for setting in self.0 {
            if let Some(data) = input.data.remove(*setting) {
                println!("Removed {}, which was set to '{}'", setting, data);
            } else {
                println!("Found no {} to remove", setting);
            }
        }
        Ok(input)
    }

    /// Old versions must either have a default for the settings or generate it; we don't need to
    /// do anything.
    fn backward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!(
            "RemoveSettingsMigration({:?}) has no work to do on downgrade.",
            self.0
        );
        Ok(input)
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// We use this migration when we replace a setting's old string value with a new string value.
pub struct ReplaceStringMigration {
    pub setting: &'static str,
    pub old_val: &'static str,
    pub new_val: &'static str,
}

impl Migration for ReplaceStringMigration {
    fn forward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        if let Some(data) = input.data.get_mut(self.setting) {
            match data {
                serde_json::Value::String(data) => {
                    if data == self.old_val {
                        self.new_val.clone_into(data);
                        println!(
                            "Changed value of '{}' from '{}' to '{}' on upgrade",
                            self.setting, self.old_val, self.new_val
                        );
                    } else {
                        println!(
                            "'{}' is not set to '{}', leaving alone",
                            self.setting, self.old_val
                        );
                    }
                }
                _ => {
                    println!(
                        "'{}' is set to non-string value '{}'; ReplaceStringMigration only handles strings",
                        self.setting, data
                    );
                }
            }
        } else {
            println!("Found no '{}' to change on upgrade", self.setting);
        }
        Ok(input)
    }

    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        if let Some(data) = input.data.get_mut(self.setting) {
            match data {
                serde_json::Value::String(data) => {
                    if data == self.new_val {
                        self.old_val.clone_into(data);
                        println!(
                            "Changed value of '{}' from '{}' to '{}' on downgrade",
                            self.setting, self.new_val, self.old_val
                        );
                    } else {
                        println!(
                            "'{}' is not set to '{}', leaving alone",
                            self.setting, self.new_val
                        );
                    }
                }
                _ => {
                    println!(
                        "'{}' is set to non-string value '{}'; ReplaceStringMigration only handles strings",
                        self.setting, data
                    );
                }
            }
        } else {
            println!("Found no '{}' to change on downgrade", self.setting);
        }
        Ok(input)
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// We use this migration when we need to replace settings that contain lists of string values;
/// for example, when a release changes the list of configuration-files associated with a service.
// String is the only type we use today, and handling multiple value types is more complicated than
// we need at the moment.  Allowing &[serde_json::Value] seems nice, but it would allow arbitrary
// data transformations that the API model would then fail to load.
pub struct ListReplacement {
    pub setting: &'static str,
    pub old_vals: &'static [&'static str],
    pub new_vals: &'static [&'static str],
}

pub struct ReplaceListsMigration(pub Vec<ListReplacement>);

impl Migration for ReplaceListsMigration {
    fn forward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        for replacement in &self.0 {
            if let Some(data) = input.data.get_mut(replacement.setting) {
                match data {
                    serde_json::Value::Array(data) => {
                        // We only handle string lists; convert each value to a str we can compare.
                        let list: Vec<&str> = data
                            .iter()
                            .map(|v| v.as_str())
                            .collect::<Option<Vec<&str>>>()
                            .with_context(|| error::ReplaceListContentsSnafu {
                                setting: replacement.setting,
                                data: data.clone(),
                            })?;

                        if list == replacement.old_vals {
                            // Convert back to the original type so we can store it.
                            *data = replacement.new_vals.iter().map(|s| (*s).into()).collect();
                            println!(
                                "Changed value of '{}' from {:?} to {:?} on upgrade",
                                replacement.setting, replacement.old_vals, replacement.new_vals
                            );
                        } else {
                            println!(
                                "'{}' is not set to {:?}, leaving alone",
                                replacement.setting, list
                            );
                        }
                    }
                    _ => {
                        println!(
                            "'{}' is set to non-list value '{}'; ReplaceListsMigration only handles lists",
                            replacement.setting, data
                        );
                    }
                }
            } else {
                println!("Found no '{}' to change on upgrade", replacement.setting);
            }
        }
        Ok(input)
    }

    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        for replacement in &self.0 {
            if let Some(data) = input.data.get_mut(replacement.setting) {
                match data {
                    serde_json::Value::Array(data) => {
                        // We only handle string lists; convert each value to a str we can compare.
                        let list: Vec<&str> = data
                            .iter()
                            .map(|v| v.as_str())
                            .collect::<Option<Vec<&str>>>()
                            .with_context(|| error::ReplaceListContentsSnafu {
                                setting: replacement.setting,
                                data: data.clone(),
                            })?;

                        if list == replacement.new_vals {
                            // Convert back to the original type so we can store it.
                            *data = replacement.old_vals.iter().map(|s| (*s).into()).collect();
                            println!(
                                "Changed value of '{}' from {:?} to {:?} on downgrade",
                                replacement.setting, replacement.new_vals, replacement.old_vals
                            );
                        } else {
                            println!(
                                "'{}' is not set to {:?}, leaving alone",
                                replacement.setting, list
                            );
                        }
                    }
                    _ => {
                        println!(
                        "'{}' is set to non-list value '{}'; ReplaceListsMigration only handles lists",
                        replacement.setting, data
                    );
                    }
                }
            } else {
                println!("Found no '{}' to change on downgrade", replacement.setting);
            }
        }
        Ok(input)
    }
}

#[cfg(test)]
mod test_replace_list {
    use super::{ListReplacement, ReplaceListsMigration};
    use crate::{Migration, MigrationData};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn single() {
        let data = MigrationData {
            data: hashmap! {
                "hi".into() => vec!["there"].into(),
            },
            metadata: HashMap::new(),
        };
        let result = ReplaceListsMigration(vec![ListReplacement {
            setting: "hi",
            old_vals: &["there"],
            new_vals: &["sup"],
        }])
        .forward(data)
        .unwrap();
        assert_eq!(
            result.data,
            hashmap! {
                "hi".into() => vec!["sup"].into(),
            }
        );
    }

    #[test]
    fn backward() {
        let data = MigrationData {
            data: hashmap! {
                "hi".into() => vec!["there"].into(),
            },
            metadata: HashMap::new(),
        };
        let result = ReplaceListsMigration(vec![ListReplacement {
            setting: "hi",
            old_vals: &["sup"],
            new_vals: &["there"],
        }])
        .backward(data)
        .unwrap();
        assert_eq!(
            result.data,
            hashmap! {
                "hi".into() => vec!["sup"].into(),
            }
        );
    }

    #[test]
    fn multiple() {
        let data = MigrationData {
            data: hashmap! {
                "hi".into() => vec!["there", "you"].into(),
                "hi2".into() => vec!["hey", "listen"].into(),
                "ignored".into() => vec!["no", "change"].into(),
            },
            metadata: HashMap::new(),
        };
        let result = ReplaceListsMigration(vec![
            ListReplacement {
                setting: "hi",
                old_vals: &["there", "you"],
                new_vals: &["sup", "hey"],
            },
            ListReplacement {
                setting: "hi2",
                old_vals: &["hey", "listen"],
                new_vals: &["look", "watch out"],
            },
        ])
        .forward(data)
        .unwrap();
        assert_eq!(
            result.data,
            hashmap! {
                "hi".into() => vec!["sup", "hey"].into(),
                "hi2".into() => vec!["look", "watch out"].into(),
                "ignored".into() => vec!["no", "change"].into(),
            }
        );
    }

    #[test]
    fn no_match() {
        let data = MigrationData {
            data: hashmap! {
                "hi".into() => vec!["no", "change"].into(),
                "hi2".into() => vec!["no", "change"].into(),
            },
            metadata: HashMap::new(),
        };
        let result = ReplaceListsMigration(vec![ListReplacement {
            setting: "hi",
            old_vals: &["there"],
            new_vals: &["sup", "hey"],
        }])
        .forward(data)
        .unwrap();
        // No change
        assert_eq!(
            result.data,
            hashmap! {
                "hi".into() => vec!["no", "change"].into(),
                "hi2".into() => vec!["no", "change"].into(),
            }
        );
    }

    #[test]
    fn not_list() {
        let data = MigrationData {
            data: hashmap! {
                "hi".into() => "just a string, not a list".into(),
            },
            metadata: HashMap::new(),
        };
        let result = ReplaceListsMigration(vec![ListReplacement {
            setting: "hi",
            old_vals: &["there"],
            new_vals: &["sup", "hey"],
        }])
        .forward(data)
        .unwrap();
        // No change
        assert_eq!(
            result.data,
            hashmap! {
                "hi".into() => "just a string, not a list".into(),
            }
        );
    }

    #[test]
    fn not_string() {
        let data = MigrationData {
            data: hashmap! {
                "hi".into() => vec![0].into(),
            },
            metadata: HashMap::new(),
        };
        ReplaceListsMigration(vec![ListReplacement {
            setting: "hi",
            old_vals: &["there"],
            new_vals: &["sup", "hey"],
        }])
        .forward(data)
        .unwrap_err();
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// We use this migration when we add metadata and want to make sure they're removed before we go
/// back to old versions that don't understand them.
#[derive(Debug)]
pub struct SettingMetadata {
    pub setting: &'static str,
    pub metadata: &'static [&'static str],
}

pub struct AddMetadataMigration(pub &'static [SettingMetadata]);

impl Migration for AddMetadataMigration {
    /// New versions must have the metadata already defined in defaults.
    fn forward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!(
            "AddMetadataMigration({:?}) has no work to do on upgrade.",
            &self.0
        );
        Ok(input)
    }

    /// Older versions might break with certain settings metadata (such as with setting-generators)
    /// so we need to remove them.
    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        for setting_metadata in self.0 {
            if let Some(found_metadata) = input.metadata.get_mut(setting_metadata.setting) {
                for metadata in setting_metadata.metadata {
                    if let Some(metadata_value) = found_metadata.remove(*metadata) {
                        println!(
                            "Removed {}, which was set to '{}'",
                            metadata, metadata_value
                        );
                    } else {
                        println!(
                            "Found no metadata '{}' to remove on setting '{}'",
                            metadata, setting_metadata.setting
                        );
                    }
                }
            } else {
                println!(
                    "Found no metadata for '{}' setting",
                    setting_metadata.setting
                );
            }
        }
        Ok(input)
    }
}

#[cfg(test)]
mod test_add_metadata {
    use super::{AddMetadataMigration, SettingMetadata};
    use crate::{Migration, MigrationData};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn backward() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hi".into() => hashmap!{"there".into() => "whatever".into()},
            },
        };
        let result = AddMetadataMigration(&[SettingMetadata {
            setting: "hi",
            metadata: &["there"],
        }])
        .backward(data)
        .unwrap();
        assert_eq!(
            result.metadata,
            hashmap! {
                "hi".into() => HashMap::new(),
            }
        );
    }

    #[test]
    fn backward_noop() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hi".into() => hashmap!{"sup".into() => "wassup".into()},
            },
        };
        let result = AddMetadataMigration(&[SettingMetadata {
            setting: "hi",
            metadata: &["there"],
        }])
        .backward(data)
        .unwrap();
        assert_eq!(
            result.metadata,
            hashmap! {
                "hi".into() => hashmap!{"sup".into() => "wassup".into()},
            }
        );
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// We use this migration when we remove metadata
pub struct RemoveMetadataMigration(pub &'static [SettingMetadata]);

impl Migration for RemoveMetadataMigration {
    fn forward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        for setting_metadata in self.0 {
            if let Some(found_metadata) = input.metadata.get_mut(setting_metadata.setting) {
                for metadata in setting_metadata.metadata {
                    if let Some(metadata_value) = found_metadata.remove(*metadata) {
                        println!(
                            "Removed {}, which was set to '{}'",
                            metadata, metadata_value
                        );
                    } else {
                        println!(
                            "Found no metadata '{}' to remove on setting '{}'",
                            metadata, setting_metadata.setting
                        );
                    }
                }
            } else {
                println!(
                    "Found no metadata for '{}' setting",
                    setting_metadata.setting
                );
            }
        }
        Ok(input)
    }

    fn backward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!(
            "RemoveMetadataMigration({:?}) has no work to do on downgrade.",
            &self.0
        );
        Ok(input)
    }
}

#[cfg(test)]
mod test_remove_metadata {
    use super::{RemoveMetadataMigration, SettingMetadata};
    use crate::{Migration, MigrationData};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn forward() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hi".into() => hashmap!{"there".into() => "whatever".into() },
            },
        };
        let result = RemoveMetadataMigration(&[SettingMetadata {
            setting: "hi",
            metadata: &["there"],
        }])
        .forward(data)
        .unwrap();
        assert_eq!(result.metadata, hashmap! { "hi".into() => HashMap::new() });
    }

    #[test]
    fn forward_noop() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hi".into() => hashmap!{"there".into() => "whatever".into() },
            },
        };
        let result = RemoveMetadataMigration(&[SettingMetadata {
            setting: "hi",
            metadata: &["which"],
        }])
        .forward(data)
        .unwrap();
        assert_eq!(
            result.metadata,
            hashmap! { "hi".into() => hashmap!{"there".into() => "whatever".into() } }
        );
    }

    #[test]
    fn backward() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hi".into() => hashmap!{"there".into() => "whatever".into()},
            },
        };
        let result = RemoveMetadataMigration(&[SettingMetadata {
            setting: "hi",
            metadata: &["there"],
        }])
        .backward(data)
        .unwrap();
        assert_eq!(
            result.metadata,
            hashmap! {
                "hi".into() => hashmap!{"there".into() => "whatever".into()},
            }
        );
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// We use this migration when we need to replace metadata that contain lists of string values;
/// for example, when a release changes the list of 'affected-services' associated with a setting.
// String is the only type we use today, and handling multiple value types is more complicated than
// we need at the moment.  Allowing &[serde_json::Value] seems nice, but it would allow arbitrary
// data transformations that the API model would then fail to load.
pub struct MetadataListReplacement {
    pub setting: &'static str,
    pub metadata: &'static str,
    pub old_vals: &'static [&'static str],
    pub new_vals: &'static [&'static str],
}

pub struct ReplaceMetadataListsMigration(pub Vec<MetadataListReplacement>);

impl Migration for ReplaceMetadataListsMigration {
    fn forward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        for replacement in &self.0 {
            if let Some(found_metadata) = input.metadata.get_mut(replacement.setting) {
                if let Some(metadata_data) = found_metadata.get_mut(replacement.metadata) {
                    match metadata_data {
                        serde_json::Value::Array(data) => {
                            // We only handle string lists; convert each value to a str we can compare.
                            let list: Vec<&str> = data
                                .iter()
                                .map(|v| v.as_str())
                                .collect::<Option<Vec<&str>>>()
                                .with_context(|| error::ReplaceMetadataListContentsSnafu {
                                    setting: replacement.setting,
                                    metadata: replacement.metadata,
                                    data: data.clone(),
                                })?;

                            if list == replacement.old_vals {
                                // Convert back to the original type so we can store it.
                                *data = replacement.new_vals.iter().map(|s| (*s).into()).collect();
                                println!(
                                    "Changed value of metadata '{}' for setting '{}' from {:?} to {:?} on upgrade",
                                    replacement.metadata,
                                    replacement.setting,
                                    replacement.old_vals,
                                    replacement.new_vals
                                );
                            } else {
                                println!(
                                    "Metadata '{}' for setting '{}' is not set to {:?}, leaving alone",
                                    replacement.metadata, replacement.setting, list
                                );
                            }
                        }
                        _ => {
                            println!(
                                "'Metadata '{}' for setting '{}' is set to non-list value '{}'; ReplaceMetadataListsMigration only handles lists",
                                replacement.metadata, replacement.setting, metadata_data
                            );
                        }
                    }
                } else {
                    println!(
                        "Found no metadata '{}' for setting '{}'",
                        replacement.metadata, replacement.setting
                    );
                }
            } else {
                println!("Found no metadata for '{}' setting", replacement.setting);
            }
        }
        Ok(input)
    }

    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        for replacement in &self.0 {
            if let Some(found_metadata) = input.metadata.get_mut(replacement.setting) {
                if let Some(metadata_data) = found_metadata.get_mut(replacement.metadata) {
                    match metadata_data {
                        serde_json::Value::Array(data) => {
                            // We only handle string lists; convert each value to a str we can compare.
                            let list: Vec<&str> = data
                                .iter()
                                .map(|v| v.as_str())
                                .collect::<Option<Vec<&str>>>()
                                .with_context(|| error::ReplaceMetadataListContentsSnafu {
                                    setting: replacement.setting,
                                    metadata: replacement.metadata,
                                    data: data.clone(),
                                })?;

                            if list == replacement.new_vals {
                                // Convert back to the original type so we can store it.
                                *data = replacement.old_vals.iter().map(|s| (*s).into()).collect();
                                println!(
                                    "Changed value of metadata '{}' for setting '{}' from {:?} to {:?} on downgrade",
                                    replacement.metadata,
                                    replacement.setting,
                                    replacement.new_vals,
                                    replacement.old_vals
                                );
                            } else {
                                println!(
                                    "Metadata '{}' for setting '{}' is not set to {:?}, leaving alone",
                                    replacement.metadata, replacement.setting, list
                                );
                            }
                        }
                        _ => {
                            println!(
                                "'Metadata '{}' for setting '{}' is set to non-list value '{}'; ReplaceMetadataListsMigration only handles lists",
                                replacement.metadata, replacement.setting, metadata_data
                            );
                        }
                    }
                } else {
                    println!(
                        "Found no metadata '{}' for setting '{}'",
                        replacement.metadata, replacement.setting
                    );
                }
            } else {
                println!("Found no metadata for '{}' setting", replacement.setting);
            }
        }
        Ok(input)
    }
}

#[cfg(test)]
mod test_replace_metadata_list {
    use super::{MetadataListReplacement, ReplaceMetadataListsMigration};
    use crate::{Migration, MigrationData};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn single_forward() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "sunny".into() => hashmap!{"affected-service".into() => vec!["ice", "cube"].into()},
            },
        };
        let result = ReplaceMetadataListsMigration(vec![MetadataListReplacement {
            setting: "sunny",
            metadata: "affected-service",
            old_vals: &["ice", "cube"],
            new_vals: &["warm", "water"],
        }])
        .forward(data)
        .unwrap();
        assert_eq!(
            result.metadata,
            hashmap! {
                "sunny".into() =>hashmap!{"affected-service".into() => vec!["warm", "water"].into()},
            }
        );
    }

    #[test]
    fn single_backward() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "freezing".into() => hashmap!{"affected-service".into() => vec!["warm", "water"].into()},
            },
        };
        let result = ReplaceMetadataListsMigration(vec![MetadataListReplacement {
            setting: "freezing",
            metadata: "affected-service",
            old_vals: &["ice", "cube"],
            new_vals: &["warm", "water"],
        }])
        .backward(data)
        .unwrap();
        assert_eq!(
            result.metadata,
            hashmap! {
                "freezing".into() =>hashmap!{"affected-service".into() => vec!["ice", "cube"].into()},
            }
        );
    }

    #[test]
    fn multiple_forward() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "greeting".into() => hashmap!{"hi".into() => vec!["konichiwa", "privet"].into()},
                "goodbye".into() => hashmap!{"bye".into() => vec!["spokoynoy nochi", "do svidaniya"].into()},
                "ignored".into() => hashmap!{"sad".into() => vec!["no", "change"].into()},
            },
        };
        let result = ReplaceMetadataListsMigration(vec![
            MetadataListReplacement {
                setting: "greeting",
                metadata: "hi",
                old_vals: &["konichiwa", "privet"],
                new_vals: &["aloha", "annyeong"],
            },
            MetadataListReplacement {
                setting: "goodbye",
                metadata: "bye",
                old_vals: &["spokoynoy nochi", "do svidaniya"],
                new_vals: &["annyeong", "aloha"],
            },
        ])
        .forward(data)
        .unwrap();
        assert_eq!(
            result.metadata,
            hashmap! {
                "greeting".into() => hashmap!{"hi".into() => vec!["aloha", "annyeong"].into()},
                "goodbye".into() => hashmap!{"bye".into() => vec!["annyeong", "aloha"].into()},
                "ignored".into() => hashmap!{"sad".into() => vec!["no", "change"].into()},
            }
        );
    }

    #[test]
    fn no_match() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hi1".into() => hashmap!{"hello?".into() => vec!["konichiwa", "privet"].into()},
                "hi2".into() => hashmap!{"goodbye?".into() => vec!["spokoynoy nochi", "do svidaniya"].into()},
            },
        };
        let result = ReplaceMetadataListsMigration(vec![
            MetadataListReplacement {
                setting: "hi1",
                metadata: "not hello",
                old_vals: &["hey?"],
                new_vals: &["whats", "up"],
            },
            MetadataListReplacement {
                setting: "hi1",
                metadata: "hello?",
                old_vals: &["goodbye", "not match"],
                new_vals: &["whats", "up"],
            },
            MetadataListReplacement {
                setting: "hi3",
                metadata: "no",
                old_vals: &["goodbye", "not match"],
                new_vals: &["whats", "up"],
            },
        ])
        .forward(data)
        .unwrap();
        // No change
        assert_eq!(
            result.metadata,
            hashmap! {
                "hi1".into() => hashmap!{"hello?".into() => vec!["konichiwa", "privet"].into()},
                "hi2".into() => hashmap!{"goodbye?".into() => vec!["spokoynoy nochi", "do svidaniya"].into()},
            }
        );
    }

    #[test]
    fn not_list() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hi".into() => hashmap!{"whats going on".into() => "just a string, not a list".into()},
            },
        };
        let result = ReplaceMetadataListsMigration(vec![MetadataListReplacement {
            setting: "hi",
            metadata: "whats going on",
            old_vals: &["there"],
            new_vals: &["sup", "hey"],
        }])
        .forward(data)
        .unwrap();
        // No change
        assert_eq!(
            result.metadata,
            hashmap! {
                "hi".into() => hashmap!{"whats going on".into() => "just a string, not a list".into()},
            }
        );
    }

    #[test]
    fn not_string() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hi".into() => hashmap!{"whats going on".into() => vec![0].into()},
            },
        };
        ReplaceMetadataListsMigration(vec![MetadataListReplacement {
            setting: "hi",
            metadata: "whats going on",
            old_vals: &["why"],
            new_vals: &["sup", "hey"],
        }])
        .forward(data)
        .unwrap_err();
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// We use this migration when we need to replace a single metadata value;
/// for example, what a release changes the `setting-generator` associated with a setting.
// String is the only type we use today, and handling multiple value types is more complicated than
// we need at the moment.  Allowing &[serde_json::Value] seems nice, but it would allow arbitrary
// data transformations that the API model would then fail to load.

#[derive(Debug, Clone)]
pub struct MetadataReplacement {
    pub setting: &'static str,
    pub metadata: &'static str,
    pub old_val: &'static str,
    pub new_val: &'static str,
}

impl MetadataReplacement {
    /// Executes the metadata replacement on given datastore.
    ///
    /// State which prevents the replacement from being performed results in messages to stdout.
    /// Returns whether or not the migration performed changes.
    fn perform_replacement(&self, input: &mut MigrationData) -> bool {
        input
            .metadata
            .get_mut(self.setting)
            .or_else(|| {
                println!("Found no setting '{}'", self.setting);
                None
            })
            .and_then(|found_metadata| {
                let metadata_value = found_metadata.get_mut(self.metadata);
                if metadata_value.is_none() {
                    println!(
                        "Found no metadata '{}' for setting '{}'",
                        self.metadata, self.setting
                    );
                }
                metadata_value
            })
            .and_then(|metadata| {
                // If we have a matching string, replace it with our new value
                match metadata {
                    serde_json::Value::String(data) => {
                        Some(data)
                    },
                    _ => {
                        println!(
                            "Metadata '{}' for setting '{}' is set to non-string value {}; ReplaceMetadataMigration only handles strings.",
                            self.metadata, self.setting, metadata
                        );
                        None
                    }
                }
            })
            .and_then(|data| {
                if data == self.old_val {
                    self.new_val.clone_into(data);
                    println!(
                        "Changed value of metadata '{}' for setting '{}' from '{}' to '{}'.",
                        self.metadata,
                        self.setting,
                        self.old_val,
                        self.new_val
                    );
                    Some(data)
                } else {
                    println!(
                        "Metadata '{}' for setting '{}' is not set to {}, leaving alone",
                        self.metadata, self.setting, self.old_val
                    );
                    None
                }
            })
            .is_some()
    }
}

pub struct ReplaceMetadataMigration(pub Vec<MetadataReplacement>);

impl Migration for ReplaceMetadataMigration {
    fn forward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        self.0.iter().for_each(|replacement| {
            replacement.perform_replacement(&mut input);
        });
        Ok(input)
    }

    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        self.0.iter().for_each(|replacement| {
            // Invert our forward migrations, then run them against the data store.
            let mut backwards_replacement = replacement.clone();
            backwards_replacement.old_val = replacement.new_val;
            backwards_replacement.new_val = replacement.old_val;

            backwards_replacement.perform_replacement(&mut input);
        });
        Ok(input)
    }
}

#[cfg(test)]
mod test_replace_metadata {
    use super::{MetadataReplacement, ReplaceMetadataMigration};
    use crate::{Migration, MigrationData};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn test_forward() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hiking".into() => hashmap!{"setting-generator".into() => "weather-is-sunny".into()}
            },
        };
        let result = ReplaceMetadataMigration(vec![MetadataReplacement {
            setting: "hiking",
            metadata: "setting-generator",
            old_val: "weather-is-sunny",
            new_val: "/bin/true",
        }])
        .forward(data)
        .unwrap();

        assert_eq!(
            result.metadata,
            hashmap! {
                "hiking".into() => hashmap!{"setting-generator".into() => "/bin/true".into()}
            }
        );
    }

    #[test]
    fn test_backward() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "favorite-dog-park".into() => hashmap!{"setting-generator".into() => "closest-lake".into()}
            },
        };
        let result = ReplaceMetadataMigration(vec![MetadataReplacement {
            setting: "favorite-dog-park",
            metadata: "setting-generator",
            old_val: "closest-beach",
            new_val: "closest-lake",
        }])
        .backward(data)
        .unwrap();

        assert_eq!(
            result.metadata,
            hashmap! {
                "favorite-dog-park".into() => hashmap!{"setting-generator".into() => "closest-beach".into()}
            }
        );
    }

    #[test]
    fn no_match() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "hi1".into() => hashmap!{"hello?".into() => "konichiwa".into()},
                "hi2".into() => hashmap!{"goodbye?".into() => "spokoynoy nochi".into()},
            },
        };
        let result = ReplaceMetadataMigration(vec![
            MetadataReplacement {
                setting: "hi1",
                metadata: "not hello",
                old_val: "hey?",
                new_val: "whats up",
            },
            MetadataReplacement {
                setting: "hi1",
                metadata: "hello?",
                old_val: "goodbye",
                new_val: "whats up",
            },
            MetadataReplacement {
                setting: "hi3",
                metadata: "no",
                old_val: "goodbye",
                new_val: "whats up",
            },
        ])
        .forward(data)
        .unwrap();
        // No change
        assert_eq!(
            result.metadata,
            hashmap! {
                "hi1".into() => hashmap!{"hello?".into() => "konichiwa".into()},
                "hi2".into() => hashmap!{"goodbye?".into() => "spokoynoy nochi".into()},
            }
        );
    }

    #[test]
    fn not_string() {
        let data = MigrationData {
            data: HashMap::new(),
            metadata: hashmap! {
                "dirtywave".into() => hashmap!{"qualities".into() => vec!["synthesizer", "sequencer"].into()}
            },
        };
        let result = ReplaceMetadataMigration(vec![MetadataReplacement {
            setting: "dirtywave",
            metadata: "qualities",
            old_val: "sequencer",
            new_val: "tracker",
        }])
        .forward(data)
        .unwrap();
        // No change
        assert_eq!(
            result.metadata,
            hashmap! {
                "dirtywave".into() => hashmap!{"qualities".into() => vec!["synthesizer", "sequencer"].into()}
            }
        );
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// When we add conditional migrations that can only run for specific variants, we need to run this
/// migration helper for cases where the migration does NOT apply so migrator will still create a valid
/// intermediary datastore that the host can transition to.
#[derive(Debug)]
pub struct NoOpMigration;

impl Migration for NoOpMigration {
    /// No work to do on forward migrations, copy the same datastore
    fn forward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!("NoOpMigration has no work to do on upgrade.",);
        Ok(input)
    }

    /// No work to do on backward migrations, copy the same datastore
    fn backward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!("NoOpMigration has no work to do on downgrade.",);
        Ok(input)
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// We use this migration to remove a setting string if it matches the old value.
/// We will need this migration once to adapt the concept of Strength on settings.
pub struct RemoveMatchingString {
    pub setting: &'static str,
    pub old_val: &'static str,
}

impl Migration for RemoveMatchingString {
    fn forward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        if let Some(data) = input.data.get_mut(self.setting) {
            match data {
                serde_json::Value::String(data) => {
                    if data == self.old_val {
                        input.data.remove(self.setting);
                    } else {
                        println!(
                            "'{}' is not set to '{}', leaving alone",
                            self.setting, self.old_val
                        );
                    }
                }
                _ => {
                    println!(
                        "'{}' is set to non-string value '{}'; RemoveOldData expects a string setting value",
                        self.setting, data
                    );
                }
            }
        } else {
            println!("Found no '{}' to change on upgrade", self.setting);
        }
        Ok(input)
    }

    fn backward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!("RemoveOldData has no work to do on downgrade.",);
        Ok(input)
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

// When we downgrade multiple version to a version where migrator is not aware of deleting the
// setting-generator as struct or the strength file.
// This migration will remove the setting-generator as struct and strength metadata.
// Also We will delete the metadata on downgrade and
// depend on storewolf to populate the metadata from defaults.
#[derive(Debug)]
pub struct RemoveMetadataAndWeakSettingsMigration;

impl Migration for RemoveMetadataAndWeakSettingsMigration {
    /// No work to do on forward migrations, copy the same datastore
    fn forward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!("RemoveMetadataAndWeakSettingsMigration has no work to do on upgrade.",);
        Ok(input)
    }

    /// Delete all the weak settings on backward migrations
    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        let mut keys_to_remove = HashSet::new();
        // Collect keys where the inner HashMap contains the key "strength"
        for (key, inner_map) in &input.metadata {
            if let Some(strength) = inner_map.get("strength") {
                if strength == &Value::String("weak".to_string()) {
                    keys_to_remove.insert(key.clone());
                }
            }
        }
        // Remove weak settings
        for key in keys_to_remove {
            input.data.remove(&key);
        }

        // Remove all the metadata
        input.metadata = HashMap::new();
        Ok(input)
    }
}
