use clap::builder::styling;
use haikei::cli;
use haikei_lib::prelude::*;

use clap::{arg, Command};

fn get_commands() -> Command {
    let effects = (styling::Effects::BOLD | styling::Effects::UNDERLINE).clear();
    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::White.on_default() | effects)
        .usage(styling::AnsiColor::White.on_default() | effects)
        .literal(styling::AnsiColor::BrightWhite.on_default() | effects)
        .placeholder(styling::AnsiColor::BrightWhite.on_default() | effects);

    Command::new("haikei")
        .about("a tiny wallpaper helper.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .styles(styles)
        .subcommand(Command::new("env").about("show environment script"))
        .subcommand(
            Command::new("config")
                .about("manage local configuration")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("create").about("create default config.toml")),
        )
        .subcommand(Command::new("current").about("get path to current wallpaper"))
        .subcommand(
            Command::new("set")
                .about("set wallpaper")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> "path to wallpaper")),
        )
        .subcommand(
            Command::new("random")
                .alias("r")
                .about("set random wallpaper")
                .arg_required_else_help(false)
                .arg(arg!([DIR] "use wallpaper dir")),
        )
        .subcommand(
            Command::new("daemon")
                .about("start daemon"),
        )
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let matches = get_commands().get_matches();

    match matches.subcommand() {
        Some(("env", _)) => {
            cli::env::env()?;
            Ok(())
        }
        Some(("config", sub_matches)) => {
            let subcommand = sub_matches.subcommand().ok_or(make_err!())?;
            match subcommand {
                ("create", _) => cli::config::create(),
                (&_, _) => Err(Error::Unexpected),
            }
        }
        Some(("current", _)) => {
            cli::current()?;
            Ok(())
        }
        Some(("set", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("PATH")
                .ok_or(make_err!(Missing, "no path specified."))?;
            cli::set(path)?;
            Ok(())
        }
        Some(("random", sub_matches)) => {
            let dir = sub_matches
                .get_one::<String>("DIR");
            cli::random(dir.cloned())?;
            Ok(())
        }
        Some(("daemon", _)) => {
            cli::daemon()?;
            Ok(())
        }
        // if all subcommands are defined above, anything else is unreachable!()
        _ => Err(make_err!()),
    }
}
