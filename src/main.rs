#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_init::init_with(tracing_init::Parameter {
        directive_env_key: "LOG",
        ..Default::default()
    });

    let cli = kubeprovision::Cli::parse();

    let config = kubeprovision::Config::from_path(cli.config.as_path())?;

    cli.run(config).await
}
