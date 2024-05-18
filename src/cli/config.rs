use haikei_lib::prelude::*;
use haikei_lib::util::filepath;
use haikei_lib::config::Config;
use haikei_lib::data;

pub fn create() -> Result<()> {
    let config_path = Config::path()?;
    if filepath::exists(&config_path) {
        return Err(make_err!(
            Conflict,
            "config file {config_path} already exists."
        ));
    }
    info!("creating config at {config_path}");
    data::save_config(Config::default())?;

    Ok(())
}
