use crate::{config::Provider, operator::AwsOperator, usecase, Config};

pub async fn run(config: Config) -> anyhow::Result<()> {
    match config.provider {
        Provider::Aws => {
            let operator = AwsOperator::from_config(&config).await?;
            let ec2_nodes = usecase::ec2::collect(&operator).await?;

            tracing::info!("{:#?}", ec2_nodes);
        }
    }

    Ok(())
}
