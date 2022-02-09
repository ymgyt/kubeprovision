use crate::config::SshConfig;
use crate::node::{ClusterNodes, Node, NodeCategory, EC2};
use crate::operator::AwsOperator;
use crate::provision::Provisioner;
use crate::ssh::Session;
use anyhow::anyhow;

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

pub async fn provision(ssh_config: &SshConfig, operator: &AwsOperator) -> anyhow::Result<()> {
    // TODO: make sure all nodes started.
    let cluster_nodes = collect(operator).await?;
    let mut provision_handles = Vec::with_capacity(cluster_nodes.len());

    for master in cluster_nodes.master {
        provision_handles.push(tokio::spawn(provision_node(
            ssh_config.user.clone(),
            NodeCategory::Master,
            master,
        )));
    }
    for worker in cluster_nodes.worker {
        provision_handles.push(tokio::spawn(provision_node(
            ssh_config.user.clone(),
            NodeCategory::Worker,
            worker,
        )));
    }

    for provision in provision_handles.into_iter() {
        provision.await??
    }

    Ok(())
}

async fn provision_node(
    ssh_user: String,
    category: NodeCategory,
    instance: EC2,
) -> anyhow::Result<()> {
    let provisioner = Provisioner::new();
    let public_ip = instance.public_ip().ok_or_else(|| {
        anyhow!(
            "ec2 instance {} does not have public ip. maybe not started",
            instance.id()
        )
    })?;
    let session = Session::connect(&ssh_user, &public_ip.to_string()).await?;
    provisioner
        .provision(category, instance.id(), session)
        .await
}
