pub async fn connect(user: &str, host: &str) -> anyhow::Result<openssh::Session> {
    let session =
        openssh::Session::connect(&format!("{user}@{host}"), openssh::KnownHosts::Accept).await?;

    Ok(session)
}
