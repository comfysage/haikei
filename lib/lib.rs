use directories::ProjectDirs;

pub mod error;
pub mod prelude;

use prelude::*;

pub fn project() -> Result<ProjectDirs> {
    ProjectDirs::from("dev", "comfysage", "haikei").ok_or(make_err!())
}

pub mod config;
pub mod data;
pub mod exec;
pub mod util;

extern crate directories;
extern crate lazy_static;
extern crate log;
extern crate nom;
extern crate regex;
extern crate serde;
extern crate toml;
