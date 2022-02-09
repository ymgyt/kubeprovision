use crate::config::Provider;
use crate::operator::AwsOperator;
use crate::{usecase, Config};

pub async fn run(config: Config) -> anyhow::Result<()> {
    match config.provider {
        Provider::Aws => {
            let operator = AwsOperator::from_config(&config).await?;
            usecase::ec2::provision(&config.aws.unwrap().ec2.node.ssh, &operator).await
        }
    }
}
