use anyhow::anyhow;
use tracing_futures::Instrument;

use crate::{
    config::SshConfig,
    node::{ClusterNodes, Node, NodeRole, EC2},
    operator::AwsOperator,
    provision::{Command, Provisioner, RemoteCommandExecutor},
    ssh,
};

pub async fn collect(operator: &AwsOperator) -> anyhow::Result<ClusterNodes<EC2>> {
    operator.list_nodes().await
}

pub async fn start_instances(operator: &AwsOperator) -> anyhow::Result<ClusterNodes<EC2>> {
    let cluster_nodes = collect(operator).await?;

    operator.start_nodes(&cluster_nodes).await?;

    Ok(cluster_nodes)
}

pub async fn stop_instances(operator: &AwsOperator) -> anyhow::Result<ClusterNodes<EC2>> {
    let cluster_nodes = collect(operator).await?;

    operator.stop_nodes(&cluster_nodes).await?;

    Ok(cluster_nodes)
}

// TODO: Refactor
pub async fn exec(
    ssh_config: SshConfig,
    operator: AwsOperator,
    command: String,
    role: Option<NodeRole>,
) -> anyhow::Result<()> {
    let cluster_nodes = collect(&operator).await?;
    // Create methods
    let nodes = match role {
        Some(role) => match role {
            NodeRole::Master => cluster_nodes.master,
            NodeRole::Worker => cluster_nodes.worker,
        },
        None => cluster_nodes.into_nodes().map(|(_, node)| node).collect(),
    };

    let mut handles = Vec::with_capacity(nodes.len());
    for node in nodes {
        let public_ip = node.public_ip().ok_or_else(|| {
            anyhow!(
                "node {} does not have public ip. maybe not started",
                node.id()
            )
        })?;
        let ssh_user = ssh_config.user.clone();
        let command = command.clone();

        let handle = tokio::spawn(async move {
            let session = ssh::connect(&ssh_user, &public_ip.to_string())
                .await
                .expect("should connect");

            session
                .execute(Command::Bash(&command))
                .await
                .expect("command failed");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}

pub async fn provision(ssh_config: &SshConfig, operator: &AwsOperator) -> anyhow::Result<()> {
    // TODO: make sure all nodes started.
    let cluster_nodes = collect(operator).await?;
    let mut provision_handles = Vec::with_capacity(cluster_nodes.len());

    for (role, node) in cluster_nodes.into_nodes() {
        let handle = tokio::spawn(provision_node(ssh_config.user.clone(), role, node));
        provision_handles.push(handle);
    }

    for provision in provision_handles.into_iter() {
        provision.await??
    }

    Ok(())
}

async fn provision_node(ssh_user: String, role: NodeRole, node: impl Node) -> anyhow::Result<()> {
    let public_ip = node.public_ip().ok_or_else(|| {
        anyhow!(
            "node {} does not have public ip. maybe not started",
            node.id()
        )
    })?;
    let session = ssh::connect(&ssh_user, &public_ip.to_string()).await?;
    let provisioner = Provisioner::new(session);
    provisioner
        .provision()
        .instrument(tracing::info_span!(
            "provision",
            role=%role,
            node_id=%node.id(),
        ))
        .await
        .map_err(anyhow::Error::from)
}
