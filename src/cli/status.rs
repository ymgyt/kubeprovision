use std::io::Write;

use prettytable::{cell, format, row, Table};

use crate::{
    config::Provider,
    node::{Node, NodeRole, EC2},
    operator::AwsOperator,
    usecase, Config,
};

pub async fn run(config: Config, writer: impl Write) -> anyhow::Result<()> {
    match config.provider {
        Provider::Aws => {
            let operator = AwsOperator::from_config(&config).await?;
            let ec2_nodes = usecase::ec2::collect(&operator).await?;

            write_status(writer, ec2_nodes.nodes())
        }
    }
}

fn write_status<'a>(
    mut w: impl Write,
    instances: impl Iterator<Item = (NodeRole, &'a EC2)>,
) -> anyhow::Result<()> {
    let mut table = Table::new();
    let format = format::FormatBuilder::new().column_separator(' ').build();

    table.set_format(format);
    table.set_titles(row!["Role", "InstanceId", "PublicIp", "State"]);

    for (role, instance) in instances {
        table.add_row(row![
            role,
            instance.id(),
            instance
                .public_ip()
                .map(|ip| ip.to_string())
                .unwrap_or_else(|| "unknown".to_owned()),
            instance.state().as_str(),
        ]);
    }

    table.print(&mut w)?;

    Ok(())
}
