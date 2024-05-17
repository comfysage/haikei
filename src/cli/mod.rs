use haikei_lib::config::Config;
use haikei_lib::data;
use haikei_lib::exec;
use haikei_lib::prelude::*;

use rand::seq::IteratorRandom;

pub mod config;
pub mod env;

pub fn set(path: &str) -> Result<()> {
    data::set_current_state(path)?;
    trace!("set wallpaper {path}");
    exec::set_wallpaper(&path)?;
    Ok(())
}
pub fn random(dir: Option<String>) -> Result<()> {
    let wall_dir = if let Some(dir) = dir {
        dir
    } else {
        let cfg = Config::new()?;
        cfg.main.wallpapers
    };
    let item = std::fs::read_dir(wall_dir)?
        .flatten()
        .filter(|item| item.file_type().map_or(false, |x| x.is_file()))
        .choose(&mut rand::thread_rng())
        .ok_or(make_err!())?;
    let path = item.path();
    let path = path.to_str().ok_or(make_err!())?;
    debug!("select item {path}");
    set(path)?;
    Ok(())
}
pub fn daemon() -> Result<()> {
    todo!()
}
pub fn current() -> Result<()> {
    let current_file = data::get_current_state()?;
    println!("{current_file}");
    Ok(())
}
