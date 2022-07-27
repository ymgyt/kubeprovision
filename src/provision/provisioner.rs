use futures::TryFutureExt;
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
            .and_then(|_| {
                self.install_containerd()
                    .instrument(info_span!("install_containerd"))
            })
            .await
    }

    async fn disable_swap(&self) -> Result<(), ProvisionError> {
        self.executor
            .execute(Command::Sudo(&["swapoff", "-a"]))
            .await
    }

    async fn install_containerd(&self) -> Result<(), ProvisionError> {
        // https://v1-23.docs.kubernetes.io/docs/setup/production-environment/container-runtimes/#containerd

        let put_containerd_conf = "cat <<EOF | sudo tee /etc/modules-load.d/containerd.conf
    overlay
    br_netfilter
    EOF";

        let put_cri_conf = "cat <<EOF | sudo tee /etc/sysctl.d/99-kubernetes-cri.conf
        net.bridge.bridge-nf-call-iptables  = 1
        net.ipv4.ip_forward                 = 1
        net.bridge.bridge-nf-call-ip6tables = 1
        EOF";

        self.executor
            .execute(Command::Bash(put_containerd_conf))
            .and_then(|_| {
                self.executor
                    .execute(Command::Sudo(&["modprobe", "overlay"]))
            })
            .and_then(|_| {
                self.executor
                    .execute(Command::Sudo(&["modprobe", "br_netfilter"]))
            })
            .and_then(|_| self.executor.execute(Command::Bash(put_cri_conf)))
            .and_then(|_| {
                self.executor
                    .execute(Command::Sudo(&["sysctl", "--system"]))
            })
            .and_then(|_| self.executor.execute(Command::Sudo(&["apt-get", "update"])))
            .and_then(|_| {
                self.executor.execute(Command::Sudo(&[
                    "apt-get",
                    "install",
                    "containerd",
                    "--yes",
                ]))
            })
            .and_then(|_| {
                self.executor
                    .execute(Command::Sudo(&["mkdir", "-p", "/etc/containerd"]))
            })
            .and_then(|_| {
                self.executor.execute(Command::Bash(
                    "containerd config default | sudo tee /etc/containerd/config.toml",
                ))
            })
            .and_then(|_| {
                self.executor
                    .execute(Command::Sudo(&["systemctl", "restart", "containerd"]))
            })
            .and_then(|_| {
                self.executor
                    .execute(Command::Executable("service", &["containerd", "status"]))
            })
            .await
    }
}
