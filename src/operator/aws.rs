use crate::node::{ClusterNodes, NodeCategory, EC2};
use crate::Config;
use anyhow::anyhow;
use aws_sdk_ec2::{
    model::{Filter, Instance},
    Client as EC2Client,
};
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct AwsTag {
    key: String,
    value: Option<String>,
}

impl From<AwsTag> for Filter {
    fn from(tag: AwsTag) -> Self {
        let builder = match tag.value {
            Some(value) => Filter::builder()
                .name(format!("tag:{}", tag.key))
                .values(value),
            None => Filter::builder().name("tag-key").values(tag.key),
        };
        builder.build()
    }
}

impl AwsTag {
    pub fn key(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl PartialEq<aws_sdk_ec2::model::Tag> for AwsTag {
    fn eq(&self, other: &aws_sdk_ec2::model::Tag) -> bool {
        other.key() == Some(self.key.as_str()) && other.value() == self.value.as_deref()
    }
}

#[derive(Debug)]
pub struct AwsTagSpec {
    pub node: AwsTag,
    pub master_node: AwsTag,
    pub worker_node: AwsTag,
}

pub struct AwsOperator {
    client: EC2Client,
    tag_spec: AwsTagSpec,
}

impl AwsOperator {
    pub async fn from_config(config: &Config) -> anyhow::Result<Self> {
        let tag_spec = config
            .aws_tag_spec()
            .ok_or(anyhow!("aws config not found"))?;
        AwsOperator::new(tag_spec).await
    }
    pub async fn new(tag_spec: AwsTagSpec) -> anyhow::Result<Self> {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_ec2::Client::new(&config);

        Ok(AwsOperator::with(client, tag_spec))
    }

    pub fn with(client: EC2Client, tag_spec: AwsTagSpec) -> Self {
        Self { client, tag_spec }
    }
}

impl AwsOperator {
    pub async fn list_nodes(&self) -> anyhow::Result<ClusterNodes<EC2>> {
        let mut master_nodes = Vec::new();
        let mut worker_nodes = Vec::new();
        let mut next_token = None;
        loop {
            let output = self
                .client
                .describe_instances()
                .filters(self.tag_spec.node.clone().into())
                .set_max_results(Some(1000))
                .set_next_token(next_token.clone())
                .send()
                .await?;

            for reservation in output.reservations.unwrap_or_default() {
                for instance in reservation.instances.unwrap_or_default() {
                    if let Some(category) = self.categorize(&instance) {
                        let node = EC2::try_from(instance)?;
                        match category {
                            NodeCategory::Master => master_nodes.push(node),
                            NodeCategory::Worker => worker_nodes.push(node),
                        }
                    }
                }
            }

            next_token = output.next_token;
            if next_token.is_none() {
                break;
            }
        }

        Ok(ClusterNodes {
            master: master_nodes,
            worker: worker_nodes,
        })
    }

    fn categorize(&self, instance: &Instance) -> Option<NodeCategory> {
        if let Some(tags) = instance.tags() {
            for tag in tags {
                if self.tag_spec.master_node.eq(tag) {
                    return Some(NodeCategory::Master);
                } else if self.tag_spec.worker_node.eq(tag) {
                    return Some(NodeCategory::Worker);
                }
            }
        }
        None
    }
}

enum ChangeInstanceState {
    Start,
    Stop,
}

impl AwsOperator {
    pub async fn start_nodes(&self, nodes: &ClusterNodes<EC2>) -> anyhow::Result<()> {
        self.change_instances_state(nodes, ChangeInstanceState::Start)
            .await
    }
    pub async fn stop_nodes(&self, nodes: &ClusterNodes<EC2>) -> anyhow::Result<()> {
        self.change_instances_state(nodes, ChangeInstanceState::Stop)
            .await
    }

    async fn change_instances_state(
        &self,
        nodes: &ClusterNodes<EC2>,
        state: ChangeInstanceState,
    ) -> anyhow::Result<()> {
        let ids = nodes.node_ids().collect::<Vec<_>>();
        if !ids.is_empty() {
            match state {
                ChangeInstanceState::Start => {
                    self.client
                        .start_instances()
                        .set_instance_ids(Some(ids))
                        .send()
                        .await?;
                }

                ChangeInstanceState::Stop => {
                    self.client
                        .stop_instances()
                        .set_instance_ids(Some(ids))
                        .send()
                        .await?;
                }
            }
        }
        Ok(())
    }
}
