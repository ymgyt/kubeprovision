mod node_state;
mod provision;
mod status;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{cli, Config};

#[derive(Parser, Debug)]
#[clap(
    name = "kubeprovision",
    version,
    about = "provision kubernetes cluster nodes"
)]
pub struct Cli {
    // global and required cannot be specified at the same time in clap.
    #[clap(help = "configuration file path", short = 'c', long, env = "CONFIG")]
    pub config: PathBuf,

    #[clap(subcommand)]
    command: Command,
}

impl Cli {
    pub fn parse() -> Cli {
        clap::Parser::parse()
    }

    pub async fn run(self, config: Config) -> anyhow::Result<()> {
        match self.command {
            Command::Provision { .. } => cli::provision::run(config).await,
            Command::Status { .. } => cli::status::run(config).await,
            Command::Start { .. } => cli::node_state::start(config).await,
            Command::Stop { .. } => cli::node_state::stop(config).await,
        }
    }
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Provision kubernetes nodes")]
    Provision {},
    #[clap(about = "Print current nodes status")]
    Status,
    #[clap(about = "Start kubernetes nodes")]
    Start,
    #[clap(about = "Stop kubernetes nodes")]
    Stop,
}
