mod cli;
mod ui;
mod ui_main;
use clap::Parser;

use crate::{
    cli::{Cli, daemon_start, daemon_stop},
    ui_main::ui_main,
};

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Some(sub_commands) => match sub_commands {
            cli::Subcommands::Start => daemon_start(),
            cli::Subcommands::Stop => daemon_stop(),
            cli::Subcommands::Ui => ui_main(),
        },
        None => ui_main(),
    }
}
