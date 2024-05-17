use crate::prelude::*;
use crate::util::constants;

mod provider;
mod run;

pub fn set_wallpaper(path: &str) -> Result<()> {
    run::run_one(
        format!("{} {path}", provider::get_provider_cmd()?),
        &*constants::CWD,
    )
}
