use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    version,
    about = "An autoclicker designed for use with wayland compositors",
    long_about = "Adding yet another autoclicker to the massive collection of autoclickers. Wayland linux is hard to find an autoclicker for though, hence this."
)]
pub struct Cli {
    /// What to get the autoclick to do.
    #[cfg_attr(feature = "ui", doc = "Will open the UI if not provided")]
    #[cfg_attr(
        not(feature = "ui"),
        doc = "Will provide the help information if not provided"
    )]
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Starts the autoclicker
    Start(AutoclickerArgs),
    /// Stops the autoclicker
    Stop,
    /// Toggle the state of the autoclicker, preferred over [SubCommands::Start] and [SubCommands::Stop]
    Toggle(AutoclickerArgs),
}

#[derive(Debug, Args)]
pub struct AutoclickerArgs {
    /// Override the profile to use as specified by the default profile.
    pub profile: Option<String>,
}
