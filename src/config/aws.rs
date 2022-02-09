use crate::operator::AwsTag;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "aws")]
pub struct AwsConfig {
    pub ec2: Ec2Config,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ec2Config {
    pub node: Ec2NodeConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ec2NodeConfig {
    pub distribution: String,
    pub ssh: SshConfig,
    pub tag: TagConfig,
    pub master: Node,
    pub worker: Node,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SshConfig {
    pub user: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TagConfig {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
    pub tag: TagConfig,
}

impl From<TagConfig> for AwsTag {
    fn from(c: TagConfig) -> Self {
        AwsTag::key(c.key).value(c.value)
    }
}
