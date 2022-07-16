mod provider;
pub use provider::Provider;

mod aws;
use std::path::Path;

pub use aws::*;
use serde::Deserialize;

use crate::operator::AwsTagSpec;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(deserialize_with = "provider::deserialize_provider")]
    pub provider: Provider,
    pub aws: Option<AwsConfig>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut f = std::fs::File::open(path)?;
        serde_yaml::from_reader::<_, Config>(&mut f).map_err(anyhow::Error::from)
    }

    pub fn aws_tag_spec(&self) -> Option<AwsTagSpec> {
        self.aws.as_ref().map(|aws| AwsTagSpec {
            node: aws.ec2.node.tag.clone().into(),
            master_node: aws.ec2.node.master.tag.clone().into(),
            worker_node: aws.ec2.node.worker.tag.clone().into(),
        })
    }
}
