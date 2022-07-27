use crate::{config::Provider, node::NodeRole, operator::AwsOperator, usecase, Config};

pub async fn run(config: Config, command: String, role: Option<NodeRole>) -> anyhow::Result<()> {
    match config.provider {
        Provider::Aws => {
            let operator = AwsOperator::from_config(&config).await?;
            usecase::ec2::exec(config.aws.unwrap().ec2.node.ssh, operator, command, role).await
        }
    }
}
