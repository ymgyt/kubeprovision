use thiserror::Error;
use tracing::info_span;
use tracing_futures::Instrument;

use crate::provision::{
    remote_command::{Command, RemoteCommandExecuteError},
    RemoteCommandExecutor,
};

#[derive(Error, Debug)]
pub enum ProvisionError {
    #[error("command: {0}")]
    RemoteCommand(RemoteCommandExecuteError),
    #[error("ssh error")]
    Ssh { impl_err: anyhow::Error },
}

impl ProvisionError {
    pub(super) fn ssh<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        let impl_err = anyhow::Error::from(err);
        ProvisionError::Ssh { impl_err }
    }
}

pub struct Provisioner<Executor> {
    executor: Executor,
}

impl<Executor> Provisioner<Executor> {
    pub fn new(executor: Executor) -> Self {
        Self { executor }
    }
}

impl<Executor> Provisioner<Executor>
where
    Executor: RemoteCommandExecutor,
{
    pub async fn provision(&self) -> Result<(), ProvisionError> {
        self.disable_swap()
            .instrument(info_span!("disable_swap"))
            .await?;

        self.install_containerd()
            .instrument(info_span!("install_containerd"))
            .await?;

        Ok(())
    }

    async fn disable_swap(&self) -> Result<(), ProvisionError> {
        self.executor
            .execute(Command::Sudo(&["swapoff", "-a"]))
            .await
    }

    async fn install_containerd(&self) -> Result<(), ProvisionError> {
        let command = "cat <<EOF | sudo tee /etc/modules-load.d/containerd.conf
    overlay
    br_netfilter
    EOF";
        self.executor.execute(Command::Bash(command)).await?;

        self.executor
            .execute(Command::Sudo(&["modprobe", "overlay"]))
            .await?;
        self.executor
            .execute(Command::Sudo(&["modprobe", "br_netfilter"]))
            .await
    }
}
