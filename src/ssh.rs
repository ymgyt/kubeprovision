use std::ops::Deref;
pub struct Session {
    session: openssh::Session,
}

impl Session {
    pub async fn connect(user: &str, host: &str) -> anyhow::Result<Self> {
        let session =
            openssh::Session::connect(&format!("{user}@{host}"), openssh::KnownHosts::Accept)
                .await?;

        Ok(Self { session })
    }
}

impl Deref for Session {
    type Target = openssh::Session;
    fn deref(&self) -> &Self::Target {
        &self.session
    }
}
