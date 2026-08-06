use clap::{CommandFactory, Parser};
#[cfg(feature = "ui")]
use wayclick_frontend;

use crate::{
    cli::Cli,
    click::{daemon_start, daemon_stop, toggle_daemon},
};

pub(crate) mod cli;
pub(crate) mod click;

pub fn main() {
    let commands = Cli::parse();
    if let Some(sub) = commands.command {
        match sub {
            cli::SubCommands::Start(autoclicker_args) => daemon_start(autoclicker_args.profile),
            cli::SubCommands::Stop => daemon_stop(),
            cli::SubCommands::Toggle(autoclicker_args) => toggle_daemon(autoclicker_args.profile),
        }
    }

    #[cfg(feature = "ui")]
    wayclick_frontend::main(dirs::config_dir().unwrap().join("wayclick"));

    #[cfg(not(feature = "ui"))]
    Cli::command().print_help().expect("Invalid clap cli setup");
}
