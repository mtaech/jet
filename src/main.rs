mod config;
mod logger;
mod package;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "win.rs")]
mod os;

use crate::os::{app_install, app_uninstall, font, repo_add, repo_delete};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "jet")]
#[command(author = "jet aechjet@outlook.com")]
#[command(version = "0.1")]
#[command(about = "jet's env tool")]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Debug, Subcommand)]
enum Commands {
    /// manage app,install uninstall app
    App {
        mode: Mode,
    },
    Font {
        mode: Mode,
    },
    Repo {
        mode: Mode,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    Install,
    Uninstall,
}

fn main() {
    logger::init_log();
    let config = config::init();
    let cli = Cli::parse();
    if cli.command.is_some() {
        let command = cli.command.unwrap();
        match command {
            Commands::App { mode } => match mode {
                Mode::Install => {
                    app_install(config);
                }
                Mode::Uninstall => {
                    app_uninstall(config);
                }
            },
            Commands::Repo { mode } => match mode {
                Mode::Install => repo_add(config),
                Mode::Uninstall => repo_delete(config),
            },
            _ => {}
        }
    }
}
