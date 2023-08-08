
use config::{Config, File};
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
        let config = Config::builder()
            .add_source(File::with_name("nano"))
            .build()
            .unwrap();

        let data : Data = config.try_deserialize().unwrap();
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

#[cfg(test)]
mod test{

    use super::NanoConfig;
    #[test]
    fn parse_config(){
    let nano_config = NanoConfig::parse().unwrap();
    assert_eq!(nano_config.theme, "base16-mocha.dark");

    }
    #[test]
    fn load(){
        let config = NanoConfig::parse().unwrap();
        match config.load_themes() {
            Ok(theme) => {
                assert_eq!(theme.name, Some("Base16 Mocha Dark".to_string()))
            }
            Err(err) => panic!("failed to load theme: {}", err)
        }
    }
}