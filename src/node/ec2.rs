use crate::node::Node;
use anyhow::anyhow;
use std::net::IpAddr;

#[derive(Debug)]
pub struct EC2 {
    instance_id: String,
    public_ip_address: Option<IpAddr>,
}

impl Node for EC2 {
    fn id(&self) -> String {
        // TODO: return &str
        self.instance_id.clone()
    }
    fn public_ip(&self) -> Option<IpAddr> {
        self.public_ip_address
    }
}

impl TryFrom<aws_sdk_ec2::model::Instance> for EC2 {
    type Error = anyhow::Error;
    fn try_from(instance: aws_sdk_ec2::model::Instance) -> Result<Self, Self::Error> {
        let instance_id = match instance.instance_id {
            Some(instance_id) => instance_id,
            None => return Err(anyhow!("instance id required")),
        };

        let public_ip_address = match instance.public_ip_address {
            Some(ip) => Some(ip.parse()?),
            None => None,
        };

        Ok(Self {
            instance_id,
            public_ip_address,
        })
    }
}
