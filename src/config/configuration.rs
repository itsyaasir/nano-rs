use crate::error::NanoError;
use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use syntect::highlighting::{Theme, ThemeSet};

#[derive(Debug, Clone,Deserialize)]
pub struct NanoConfig {
    appearance: AppearanceConfig,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AppearanceConfig {
    pub theme: String,
    // Add other configuration properties here
    // font: String,
    // font_size: i32,
    // line_numbers: bool,
}

impl NanoConfig {
    pub fn parse() -> Result<Self, NanoError> {
        //Parse the Nano configuration from a TOML file
        let config = Config::builder()
            .add_source(File::with_name("nano"))
            .set_default("theme", "Monokai")
            .expect("failed to set default values")
            .build().expect("failed to parse file");

        let data: NanoConfig = config.try_deserialize().unwrap();
        Ok(data)
    }

    pub fn load_themes(&self) -> Result<Theme, ConfigError> {
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get(&self.appearance.theme)
            .ok_or(ConfigError::Message(format!(
                "theme not found: {}",
                self.appearance.theme
            )))?
            .clone();
        Ok(theme)
    }
}

#[cfg(test)]
mod test {

    use super::NanoConfig;

    #[test]
    fn parse_config() {
        let nano_config = NanoConfig::parse().unwrap();
        assert_eq!(nano_config.appearance.theme, "base16-mocha.dark");
    }
    #[test]
    fn load() {
        let config = NanoConfig::parse().unwrap();
        match config.load_themes() {
            Ok(theme) => {
                assert_eq!(theme.name, Some("Base16 Mocha Dark".to_string()))
            }
            Err(err) => panic!("failed to load theme: {}", err),
        }
    }
}
