use crate::cli::Cli;
#[cfg(not(feature = "ui"))]
use clap::CommandFactory;
use clap::Parser;
use wayclick_click::{daemon_start, daemon_stop, toggle_daemon};
use wayclick_schema::ServerResponse;
#[cfg(feature = "ui")]
use wayclick_schema::dir;

pub(crate) mod cli;
pub(crate) mod network;

#[tokio::main]
pub async fn main() {
    let commands = Cli::parse();

    if let Some(sub) = commands.command {
        match sub {
            cli::SubCommands::Start(autoclicker_args) => daemon_start(autoclicker_args.profile),
            cli::SubCommands::Stop => daemon_stop(),
            cli::SubCommands::Toggle(autoclicker_args) => toggle_daemon(autoclicker_args.profile),
        }
        return;
    }

    #[cfg(not(feature = "ui"))]
    {
        if !commands.version {
            Cli::command().print_help().expect("Invalid clap cli setup");
            return;
        }
    }

    // only check for version update if requested or running the UI
    let (tx, rx) = std::sync::mpsc::channel::<ServerResponse>();
    let join = network::check_update(tx);

    if commands.version {
        println!(
            "Current version: {}. Latest version: {}\nGithub: https://github.com/dragmine149/wayclick/releases/latest",
            env!("CARGO_PKG_VERSION"),
            rx.recv()
                .map_or_else(|_| "Failed to fetch".to_string(), |v| v.version)
        );
        return join.await.unwrap();
    }

    #[cfg(feature = "ui")]
    wayclick_frontend::main(dir(), wayclick_schema::TransferData { rx });
}
