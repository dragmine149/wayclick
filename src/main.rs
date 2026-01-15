mod cli;
mod duration_input;
mod macros;
mod storage;
mod ui;
mod ui_main;

use crate::{
    cli::{Cli, daemon_start, daemon_stop, toggle_daemon},
    ui_main::ui_main,
};
use clap::Parser;
use notify_rust::Notification;

fn main() {
    let cli = Cli::parse();
    Notification::new()
        .summary("Wayclick")
        .body(&format!("{:?}", cli))
        .show()
        .unwrap();

    // switch between modes.
    match cli.commands {
        Some(sub_commands) => match sub_commands {
            cli::Subcommands::Start => daemon_start(),
            cli::Subcommands::Stop => daemon_stop(),
            cli::Subcommands::Toggle => toggle_daemon(),
            cli::Subcommands::Ui => ui_main(),
        },
        None => ui_main(),
    }
}
