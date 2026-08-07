use clap::Parser;
#[cfg(feature = "ui")]
use wayclick_frontend;
#[cfg(feature = "ui")]
use wayclick_schema::dir;

use crate::cli::Cli;
use wayclick_click::{daemon_start, daemon_stop, toggle_daemon};

pub(crate) mod cli;

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
    wayclick_frontend::main(dir());

    #[cfg(not(feature = "ui"))]
    Cli::command().print_help().expect("Invalid clap cli setup");
}
