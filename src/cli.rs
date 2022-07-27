mod exec;
mod node_state;
mod provision;
mod status;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{cli, node::NodeRole, Config};

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
            Command::Status { .. } => cli::status::run(config, std::io::stdout()).await,
            Command::Start { .. } => cli::node_state::start(config).await,
            Command::Stop { .. } => cli::node_state::stop(config).await,
            Command::Exec { command, role } => cli::exec::run(config, command, role).await,
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
    #[clap(about = "Execute given command in nodes")]
    Exec {
        #[clap(long, short = 'c', help = "execute command in bash -c ")]
        command: String,
        #[clap(long, help = "target node role")]
        role: Option<NodeRole>,
    },
}
