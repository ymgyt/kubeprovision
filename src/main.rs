#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("LOG"))
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .init();

    let cli = kubeprovision::Cli::parse();

    let config = kubeprovision::Config::from_path(cli.config.as_path())?;

    cli.run(config).await
}
