use crate::config::Provider;
use crate::operator::AwsOperator;
use crate::{usecase, Config};

enum ChangeState {
    Start,
    Stop,
}
pub async fn start(config: Config) -> anyhow::Result<()> {
    change_node_state(ChangeState::Start, config).await
}

pub async fn stop(config: Config) -> anyhow::Result<()> {
    change_node_state(ChangeState::Stop, config).await
}

async fn change_node_state(change: ChangeState, config: Config) -> anyhow::Result<()> {
    match config.provider {
        Provider::Aws => {
            let operator = AwsOperator::from_config(&config).await?;
            let operation: &str;
            let nodes = match change {
                ChangeState::Start => {
                    operation = "starting";
                    usecase::ec2::start_instances(&operator).await?
                }
                ChangeState::Stop => {
                    operation = "stopping";
                    usecase::ec2::stop_instances(&operator).await?
                }
            };

            tracing::info!("{operation} {:?}", nodes.node_ids().collect::<Vec<_>>());
        }
    }

    Ok(())
}
