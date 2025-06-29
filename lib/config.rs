use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::project;
use crate::util::constants;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigMain {
    #[serde(default = "ConfigMain::default_wallpapers")]
    pub wallpapers: String,
    #[serde(default = "ConfigMain::default_provider")]
    pub provider: String,
    #[serde(default = "ConfigMain::default_recurse")]
    pub recurse: bool,
    #[serde(default = "ConfigMain::default_fileformats")]
    pub fileformats: Vec<String>,
}

impl ConfigMain {
    pub fn default_wallpapers() -> String {
        format!("{}/.wallpapers", &*constants::HOME)
    }
    pub fn default_provider() -> String {
        "swww img".to_string()
    }
    pub fn default_recurse() -> bool {
        true
    }
    pub fn default_fileformats() -> Vec<String> {
        vec![
            "jpg".to_string(),
            "jpeg".to_string(),
            "png".to_string(),
            "gif".to_string(),
            "pnm".to_string(),
            "tga".to_string(),
            "tiff".to_string(),
            "webp".to_string(),
            "bmp".to_string(),
        ]
    }
}

impl Default for ConfigMain {
    fn default() -> Self {
        Self {
            wallpapers: Self::default_wallpapers(),
            provider: Self::default_provider(),
            recurse: Self::default_recurse(),
            fileformats: Self::default_fileformats(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub main: ConfigMain,
}

impl Config {
    pub fn path() -> Result<String> {
        let project = project()?;
        let dir = project.config_dir();
        let path = dir
            .join(&*constants::CONFIG_NAME)
            .to_str()
            .ok_or(make_err!())?
            .to_string();
        Ok(path)
    }
    pub fn new() -> Result<Self> {
        let path = Self::path()?;
        if let Ok(content) = std::fs::read_to_string(&path) {
            let config: Self = toml::from_str(&content).unwrap_or_else(|e| {
                debug!("couldn't create config from {path}: {e}");
                debug!("using default config");
                Self::default()
            });
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
    pub fn to_string(&self) -> Result<String> {
        toml::to_string(self).map_err(|_| make_err!(Parse, "couldn't create toml from string"))
    }
}
