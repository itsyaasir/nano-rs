use std::fs;
use serde_derive::Deserialize;
use syntect::highlighting::{Theme, ThemeSet};
use crate::error::NanoError;


#[derive(Deserialize)]
struct Data{
    appearance: NanoConfig,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NanoConfig{
    pub theme: String,
    // Add other configuration properties here
    // font: String,
    // font_size: i32,
    // line_numbers: bool,
}

impl NanoConfig{
    pub fn parse() -> Result<Self, NanoError>{
         //Parse the Nano configuration from a TOML file
        let filename = "nano.toml";
        let contents = fs::read_to_string(filename).expect("could not read file");
        let data: Data = toml::from_str(&contents).unwrap();
        Ok(data.appearance)
    }

    pub fn load_themes(&self) -> Result<Theme, NanoError>{
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get(&self.theme)
            .ok_or(NanoError::Generic(format!("Theme not found: {}", self.theme)))?
            .clone();
        Ok(theme)
    }
}