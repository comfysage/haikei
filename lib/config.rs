use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::project;
use crate::util::constants;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigMain {
    pub wallpapers: String,
    pub provider: String,
}

impl Default for ConfigMain {
    fn default() -> Self {
        Self {
            wallpapers: format!("{}/.wallpapers",&*constants::HOME),
            provider: "swww".to_string(),
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
        return Ok(path);
    }
    pub fn new() -> Result<Self> {
        let path = Self::path()?;
        if let Some(content) = std::fs::read_to_string(path).ok() {
            let config: Self = toml::from_str(&content).unwrap_or(Self::default());
            Ok(config)
        } else {
            return Ok(Self::default());
        }
    }
    pub fn to_string(&self) -> Result<String> {
        toml::to_string(self).map_err(|_| make_err!(Parse, "couldn't create toml from string"))
    }
}
