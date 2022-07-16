use std::{fmt, fmt::Formatter};

use async_trait::async_trait;
use tracing::info;

use crate::provision::provisioner::ProvisionError;

pub enum Command<'a, 'b> {
    Sudo(&'a [&'b str]),
    Bash(&'a str),
    #[allow(dead_code)]
    Executable(&'a str, &'a [&'b str]),
}

#[derive(Debug)]
pub struct RemoteCommandExecuteError {
    command: String,
    stderr: String,
}

impl fmt::Display for RemoteCommandExecuteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", &self.command, &self.stderr)
    }
}

impl std::error::Error for RemoteCommandExecuteError {}

impl RemoteCommandExecuteError {
    pub fn new(command: impl Into<String>, stderr: impl AsRef<[u8]>) -> Self {
        RemoteCommandExecuteError {
            command: command.into(),
            stderr: String::from_utf8_lossy(stderr.as_ref()).into_owned(),
        }
    }
}

#[async_trait]
pub trait RemoteCommandExecutor {
    async fn execute(&self, command: Command<'_, '_>) -> Result<(), ProvisionError>;
}

#[async_trait]
impl RemoteCommandExecutor for openssh::Session {
    async fn execute(&self, command: Command<'_, '_>) -> Result<(), ProvisionError> {
        let (output, log) = match command {
            Command::Bash(exec) => {
                let log = format!("bash -c {}", &exec);
                let output = self.command("bash").arg("-c").arg(exec).output().await;
                (output, log)
            }
            Command::Sudo(args) => {
                let log = format!("sudo {:20}", args.join(" "));
                let output = self.command("sudo").args(args).output().await;
                (output, log)
            }
            Command::Executable(command, args) => {
                let log = format!("{} {:20}", &command, args.join(" "));
                let output = self.command(command).args(args).output().await;
                (output, log)
            }
        };

        let output = output.map_err(ProvisionError::ssh)?;

        if output.status.success() {
            info!("success {}", log);
            Ok(())
        } else {
            Err(ProvisionError::RemoteCommand(
                RemoteCommandExecuteError::new(log, output.stderr),
            ))
        }
    }
}
