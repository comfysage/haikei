use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::project;
use crate::util::constants;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigMain {
    pub wallpapers: String,
    pub provider: String,
    pub recurse: bool,
    pub fileformats: Vec<String>,
}

impl Default for ConfigMain {
    fn default() -> Self {
        Self {
            wallpapers: format!("{}/.wallpapers", &*constants::HOME),
            provider: "swww".to_string(),
            recurse: true,
            fileformats: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "gif".to_string(),
                "pnm".to_string(),
                "tga".to_string(),
                "tiff".to_string(),
                "webp".to_string(),
                "bmp".to_string(),
            ],
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
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
        if let Ok(content) = std::fs::read_to_string(path) {
            let config: Self = toml::from_str(&content).unwrap_or(Self::default());
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
    pub fn to_string(&self) -> Result<String> {
        toml::to_string(self).map_err(|_| make_err!(Parse, "couldn't create toml from string"))
    }
}
