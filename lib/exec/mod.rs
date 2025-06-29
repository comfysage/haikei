use crate::config::Config;
use crate::prelude::*;
use crate::util::constants;

mod run;

/// create cmd to set wallpaper
pub fn get_provider_cmd() -> Result<String> {
    let cfg = Config::new()?;
    let mut provider = vec![cfg.provider.name.to_string()];
    provider.extend(cfg.provider.args.iter().map(|s| s.to_string()));
    let cmd = provider.join(" ");
    debug!("provider: {cmd}");
    Ok(cmd)
}

pub fn set_wallpaper(path: &str) -> Result<()> {
    run::run_one(format!("{} {path}", get_provider_cmd()?), &*constants::CWD)
}
