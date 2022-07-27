use error_stack::ResultExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_init::init_with(tracing_init::Parameter {
        directive_env_key: "LOG",
        ..Default::default()
    });

    let cli = kubeprovision::Cli::parse();

    let config = match kubeprovision::Config::from_path(cli.config.as_path())
        .attach_printable(format!("Loading configuration file {cli:?}"))
    {
        Ok(config) => config,
        Err(report) => {
            tracing::error!("{report:?}");
            return Ok(());
        }
    };

    cli.run(config).await
}
