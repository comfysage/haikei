use std::ffi::OsStr;
use std::path::Path;

use haikei_lib::config::Config;
use haikei_lib::data;
use haikei_lib::exec;
use haikei_lib::prelude::*;

use rand::seq::IteratorRandom;
use walkdir::WalkDir;

pub mod config;
pub mod env;

pub fn set(path: &str) -> Result<()> {
    let path = Path::new(&path).canonicalize().map_err(|e| make_err!(IO, "could not canonicalize {path}: {e}"))?;
    let path = path.to_str().ok_or(make_err!())?;
    trace!("set wallpaper {path}");
    exec::set_wallpaper(path)?;
    data::set_current_state(path)?;
    Ok(())
}
pub fn random(dir: Option<String>) -> Result<()> {
    let cfg = Config::new()?;
    let wall_dir = if let Some(dir) = dir {
        dir
    } else {
        cfg.main.wallpapers
    };
    let wall_dir = Path::new(&wall_dir).canonicalize().map_err(|e| make_err!(IO, "could not canonicalize {wall_dir}: {e}"))?;
    trace!("using dir {}", wall_dir.to_str().unwrap_or_default());
    let item = WalkDir::new(&wall_dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|entry| {
            if !cfg.main.recurse && entry.depth() > 0 {
                return false;
            }
            entry
                .file_name()
                .to_str()
                .map_or(false, |x| !x.starts_with('.'))
        })
        .filter_map(|entry| entry.ok())
        .filter(|item| {
            cfg.main.fileformats.iter().any(|x| {
                x == Path::new(&item.file_name())
                    .extension()
                    .and_then(OsStr::to_str)
                    .unwrap_or_default()
            })
        })
        .choose(&mut rand::thread_rng())
        .ok_or(make_err!(NotFound, "could not match a wallpaper in dir"))?;
    let path = Path::join(&wall_dir, item.path());
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
