use directories::ProjectDirs;

pub mod error;
pub mod prelude;

use prelude::*;

pub fn project() -> Result<ProjectDirs> {
    ProjectDirs::from("dev", "comfysage", "haikei").ok_or(make_err!())
}

pub mod util;
pub mod exec;
pub mod config;
pub mod data;

extern crate log;
extern crate directories;
extern crate lazy_static;
extern crate regex;
extern crate serde;
extern crate toml;
extern crate nom;
