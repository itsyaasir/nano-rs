use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use syntect::highlighting::{Theme, ThemeSet};

#[derive(Debug, Clone, Deserialize)]
pub struct NanoConfiguration {
    appearance: AppearanceConfig,
    editor: EditorConfiguration,
}
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AppearanceConfig {
    pub theme: String,
    // Add other configuration properties here
    // line_numbers: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct EditorConfiguration {
    pub line_numbers: bool,
}

impl Default for NanoConfiguration {
    fn default() -> Self {
        Self {
            appearance: Default::default(),
            editor: Default::default(),
        }
    }
}

impl NanoConfiguration {
    pub fn parse() -> Result<Self, ConfigError> {
        //Parse the Nano configuration from a TOML file
        let config = Config::builder()
            .add_source(File::with_name("nano"))
            .set_default("theme", "Monokai")?
            .build()
            .expect("failed to config file");

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
    pub fn turn_on_line_numbers(&self) -> Result<bool, ConfigError>{
        let turn_on_off = &self.editor.line_numbers;
        Ok(turn_on_off.clone())
    }
}

#[cfg(test)]
mod test {

    use super::NanoConfiguration;

    #[test]
    fn parse_config() {
        let nano_config = NanoConfiguration::parse().unwrap();
        assert_eq!(nano_config.appearance.theme, "base16-mocha.dark");
    }
    #[test]
    fn load() {
        let config = NanoConfiguration::parse().unwrap();
        match config.load_themes() {
            Ok(theme) => {
                assert_eq!(theme.name, Some("Base16 Mocha Dark".to_string()))
            }
            Err(err) => panic!("failed to load theme: {}", err),
        }
    }
    #[test]
    fn test_turn_on_line_numbers(){
        let config = NanoConfiguration::parse().unwrap();
        let turn_on = config.editor.line_numbers;
        assert_eq!(turn_on, true);
    }
}
