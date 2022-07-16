mod ec2;

use std::{fmt, fmt::Formatter, net::IpAddr};

pub use ec2::EC2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeRole {
    Master,
    Worker,
}

impl fmt::Display for NodeRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NodeRole::Master => write!(f, "master"),
            NodeRole::Worker => write!(f, "worker"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeId(String);

impl NodeId {
    pub fn new(id: impl Into<String>) -> Self {
        NodeId(id.into())
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for NodeId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

pub trait Node {
    fn id(&self) -> &NodeId;
    fn public_ip(&self) -> Option<IpAddr>;
}

#[derive(Debug)]
pub struct ClusterNodes<T> {
    pub master: Vec<T>,
    pub worker: Vec<T>,
}

impl<T: Node> ClusterNodes<T> {
    pub fn node_ids(&self) -> impl Iterator<Item = &NodeId> + '_ {
        self.nodes().map(|(_, node)| node.id())
    }

    pub fn nodes(&self) -> impl Iterator<Item = (NodeRole, &T)> {
        let masters = self.master.iter().map(|node| (NodeRole::Master, node));
        let workers = self.worker.iter().map(|node| (NodeRole::Worker, node));

        masters.chain(workers)
    }

    pub fn into_nodes(self) -> impl Iterator<Item = (NodeRole, T)> {
        let masters = self.master.into_iter().map(|node| (NodeRole::Master, node));
        let workers = self.worker.into_iter().map(|node| (NodeRole::Worker, node));

        masters.chain(workers)
    }
}

impl<T> ClusterNodes<T> {
    pub fn len(&self) -> usize {
        self.master.len() + self.worker.len()
    }
}
