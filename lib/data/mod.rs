use crate::config::Config;
use crate::prelude::*;
use crate::project;
use crate::util::constants;

use std::path::PathBuf;

/// Write a *Config* to the config file
///
/// - *config*: config to save
pub fn save_config(config: Config) -> Result<()> {
    let str = config.to_string()?;
    let path = Config::path()?;
    let path_as_buf = PathBuf::from(path.clone());
    std::fs::create_dir_all(path_as_buf.parent().ok_or(make_err!())?)?;
    std::fs::write(path, str)?;
    Ok(())
}

/// Set current state
///
/// - *path*: path to current wallpaper file
pub fn set_current_state(path: &str) -> Result<()> {
    trace!("set current state {path}");
    let state_file = project()?
        .state_dir()
        .ok_or(make_err!())?
        .join(&*constants::STATE_FILE);
    trace!("using {}", state_file.display());
    std::fs::create_dir_all(state_file.parent().ok_or(make_err!())?)?;
    std::fs::write(state_file, path)?;
    Ok(())
}

/// Get current state
/// get file of current wallpaper
pub fn get_current_state() -> Result<String> {
    let state_file = project()?
        .state_dir()
        .ok_or(make_err!())?
        .join(&*constants::STATE_FILE);
    debug!("using {}", state_file.display());
    if !state_file.is_file() {
        return Err(make_err!(NotFound, "no current state"));
    }
    let contents = std::fs::read_to_string(state_file)?;
    let current = PathBuf::from(contents);
    let current_file = current.display();
    if !current.is_file() {
        return Err(make_err!(
            NotFound,
            "current state is not a valid file: {current_file}"
        ));
    }
    return Ok(current_file.to_string());
}
