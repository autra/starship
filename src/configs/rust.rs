use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct RustConfig<'a> {
    pub format: &'a str,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for RustConfig<'a> {
    fn default() -> Self {
        RustConfig {
            format: "[$prefix$symbol($version)$suffix]($style)",
            prefix: "$default_module_prefix",
            suffix: "$default_module_suffix",
            version_format: "v${raw}",
            symbol: "🦀 ",
            style: "bold red",
            disabled: false,
            detect_extensions: vec!["rs"],
            detect_files: vec!["Cargo.toml"],
            detect_folders: vec![],
        }
    }
}
