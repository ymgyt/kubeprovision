use std::net::IpAddr;

use anyhow::anyhow;
use aws_sdk_ec2::model::InstanceStateName;

use crate::node::{Node, NodeId};

#[derive(Debug)]
pub struct EC2 {
    instance_id: NodeId,
    public_ip_address: Option<IpAddr>,
    state: InstanceStateName,
}

impl Node for EC2 {
    fn id(&self) -> &NodeId {
        &self.instance_id
    }
    // None means ec2 instance is not assigned public ip or not started.
    // should we represent that situation as Result ?
    fn public_ip(&self) -> Option<IpAddr> {
        self.public_ip_address
    }
}

impl EC2 {
    pub fn state(&self) -> InstanceStateName {
        self.state.clone()
    }
}

impl TryFrom<aws_sdk_ec2::model::Instance> for EC2 {
    type Error = anyhow::Error;
    fn try_from(instance: aws_sdk_ec2::model::Instance) -> Result<Self, Self::Error> {
        let instance_id = match instance.instance_id {
            Some(instance_id) => NodeId::new(instance_id),
            None => return Err(anyhow!("instance id required")),
        };

        let public_ip_address = match instance.public_ip_address {
            Some(ip) => Some(ip.parse()?),
            None => None,
        };

        let state = instance
            .state
            .and_then(|s| s.name)
            .unwrap_or_else(|| InstanceStateName::Unknown("unknown".into()));

        Ok(Self {
            instance_id,
            public_ip_address,
            state,
        })
    }
}
