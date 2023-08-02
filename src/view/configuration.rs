use std::{env, fs};
use syntect::highlighting::{Theme, ThemeSet};
use toml::Value;
use crate::error::NanoError;


#[derive(Debug, Clone)]
pub struct NanoConfig{
    pub theme: Theme,
    // Add other configuration properties here
}

impl NanoConfig{
    pub fn parse() -> Result<Self, NanoError>{
         //Parse the Nano configuration from a TOML file

        let config_path = env::current_dir().unwrap().join("nano.toml");
        let config_content = fs::read_to_string(&config_path)?;

        // parse the toml config
        let config: Value = config_content.parse().unwrap();
        let appearance = config.get("appearance").and_then(|v| v.as_table());

        let theme_name = appearance
            .and_then(|appearance| appearance.get("theme"))
            .and_then(|v| v.as_str())
            .unwrap_or("base16-ocean.dark");

        let theme_set = ThemeSet::load_defaults();
        
        let theme = theme_set
            .themes
            .get(theme_name)
            .ok_or(NanoError::Generic(format!("Theme not found: {}", theme_name)))?;

        Ok(Self {
            theme: theme.clone(),
        })
    }
}