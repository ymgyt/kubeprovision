mod ec2;

use std::net::IpAddr;

pub use ec2::EC2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeCategory {
    Master,
    Worker,
}

pub trait Node {
    fn id(&self) -> String;
    fn public_ip(&self) -> Option<IpAddr>;
}

#[derive(Debug)]
pub struct ClusterNodes<T> {
    pub master: Vec<T>,
    pub worker: Vec<T>,
}

impl<T: Node> ClusterNodes<T> {
    pub fn node_ids(&self) -> impl Iterator<Item = String> + '_ {
        self.master
            .iter()
            .map(|node| node.id())
            .chain(self.worker.iter().map(|node| node.id()))
    }
}

impl<T> ClusterNodes<T> {
    pub fn len(&self) -> usize {
        self.master.len() + self.worker.len()
    }
}
