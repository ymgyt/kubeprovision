use crate::node::NodeCategory;
use crate::ssh::Session;
use tracing::{error, info};

pub struct Provisioner {}

impl Provisioner {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn provision(
        &self,
        category: NodeCategory,
        node_id: String,
        session: Session,
    ) -> anyhow::Result<()> {
        // TODO enter span
        let swap = session
            .command("sudo")
            .args(&["swapoff", "-a"])
            .output()
            .await?;
        if swap.status.success() {
            info!(node=%node_id, role=?category, "swapoff success");
        } else {
            error!(node=%node_id, role=?category, "swapoff failed: {}", String::from_utf8_lossy(&swap.stderr));
            return Ok(());
        }

        Ok(())
    }
}
