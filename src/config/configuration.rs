use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use syntect::highlighting::{Theme, ThemeSet};

#[derive(Debug, Deserialize)]
pub struct NanoConfiguration {
    appearance: AppearanceConfig,
    editor: EditorConfiguration,
}
#[derive(Debug, Deserialize)]
pub struct AppearanceConfig {
    pub theme: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct EditorConfiguration {
    pub line_numbers: bool,
}

impl NanoConfiguration {
    pub fn load() -> Self {
        todo!()
    }
    pub fn parse_config() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("nano").required(true))
            .build()?;

        let data: NanoConfiguration = config.try_deserialize()?;
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

impl Default for NanoConfiguration {
    fn default() -> Self {
        Self {
            appearance: AppearanceConfig {
                theme: "Monokai".to_string(),
            },
            editor: EditorConfiguration {
                line_numbers: false,
            },
        }
    }
}

#[cfg(test)]
mod test {

    use super::NanoConfiguration;

    #[test]
    fn parse_config() {
        let nano_config = NanoConfiguration::parse_config().unwrap();
        assert_eq!(nano_config.appearance.theme, "base16-mocha.dark");
    }
    #[test]
    fn load() {
        let config = NanoConfiguration::parse_config().unwrap();
        match config.load_themes() {
            Ok(theme) => {
                assert_eq!(theme.name, Some("Base16 Mocha Dark".to_string()))
            }
            Err(err) => panic!("failed to load theme: {}", err),
        }
    }
    #[test]
    fn test_turn_on_line_numbers() {
        let config = NanoConfiguration::parse_config().unwrap();
        let turn_on = config.editor.line_numbers;
        assert_eq!(turn_on, true);
    }
}
