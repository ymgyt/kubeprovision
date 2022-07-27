use std::{
    fmt::{self, Formatter},
    path::Path,
};

pub use aws::*;
use error_stack::{Context, IntoReport, ResultExt};
pub use provider::Provider;
use serde::Deserialize;

use crate::operator::AwsTagSpec;

mod provider;

mod aws;

#[derive(Debug)]
pub struct ParseConfigError {}

impl ParseConfigError {
    pub fn new() -> Self {
        Self {}
    }
}

impl fmt::Display for ParseConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not parse configuration file")
    }
}

impl Context for ParseConfigError {}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(deserialize_with = "provider::deserialize_provider")]
    pub provider: Provider,
    pub aws: Option<AwsConfig>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> error_stack::Result<Self, ParseConfigError> {
        let path = path.as_ref();

        let mut f = std::fs::File::open(path)
            .report()
            .change_context_lazy(ParseConfigError::new)
            .attach_printable_lazy(|| format!("Could not read file {path:?}"))?;

        serde_yaml::from_reader::<_, Config>(&mut f)
            .report()
            .change_context_lazy(ParseConfigError::new)
            .attach_printable_lazy(|| format!("Could not deserialize file {path:?}"))
    }

    pub fn aws_tag_spec(&self) -> Option<AwsTagSpec> {
        self.aws.as_ref().map(|aws| AwsTagSpec {
            node: aws.ec2.node.tag.clone().into(),
            master_node: aws.ec2.node.master.tag.clone().into(),
            worker_node: aws.ec2.node.worker.tag.clone().into(),
        })
    }
}
