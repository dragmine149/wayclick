use clap::{CommandFactory, Parser};
#[cfg(feature = "ui")]
use wayclick_frontend;

use crate::cli::Cli;

pub(crate) mod cli;

pub fn main() {
    let commands = Cli::parse();
    if let Some(sub) = commands.command {
        match sub {
            cli::SubCommands::Start(autoclicker_args) => todo!(),
            cli::SubCommands::Stop => todo!(),
            cli::SubCommands::Toggle(autoclicker_args) => todo!(),
        }
    }

    #[cfg(feature = "ui")]
    wayclick_frontend::main(dirs::config_dir().unwrap().join("wayclick"));

    #[cfg(not(feature = "ui"))]
    Cli::command().print_help().expect("Invalid clap cli setup");
}
