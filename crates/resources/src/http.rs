use reqwest::Client;
use thiserror::Error;
use url::Url;

use crate::provider::SonorustResourceProvider;

pub struct SonorustHttpResourceProvider {
    client: Client,
    base_url: Url,
}

impl SonorustHttpResourceProvider {
    pub fn new(base_url: &str) -> Result<Self, SonorustHttpResourceError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Self { client, base_url })
    }
}

impl SonorustResourceProvider for SonorustHttpResourceProvider {
    type Error = SonorustHttpResourceError;

    async fn fetch_bytes(&self, path: impl AsRef<str>) -> Result<Vec<u8>, Self::Error> {
        let url = self.base_url.join(path.as_ref())?;
        let bytes = self.client.get(url).send().await?.bytes().await?;
        Ok(bytes.into())
    }
}

#[derive(Debug, Error)]
pub enum SonorustHttpResourceError {
    #[error("Reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Url parsing error occurred: {0}")]
    Url(#[from] url::ParseError),
}
