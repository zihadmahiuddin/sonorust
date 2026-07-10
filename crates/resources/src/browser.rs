use std::io::{Cursor, Read};

use flate2::read::GzDecoder;

use sonorust_models::{
    engine::{configuration::EngineConfiguration, play::data::EnginePlayData},
    level::{data::LevelData, info::LevelInfo},
    skin::data::SkinData,
};
use thiserror::Error;

use crate::{
    error::SonorustResourceError,
    provider::SonorustResourceProvider,
    types::{LevelBgmBytes, SkinTextureBytes},
};

pub struct SonorustResourceBrowser<P: SonorustResourceProvider> {
    resource_provider: P,
}

impl<P> SonorustResourceBrowser<P>
where
    P: SonorustResourceProvider,
    P::Error: std::error::Error + 'static,
{
    pub fn new(resource_provider: P) -> Self {
        SonorustResourceBrowser { resource_provider }
    }

    pub async fn level_info(
        &self,
        level_id: &str,
    ) -> Result<LevelInfo, SonorustResourceBrowserError<P::Error>> {
        let bytes = self
            .resource_provider
            .fetch_bytes(format!("sonolus/levels/{level_id}"))
            .await
            .map_err(SonorustResourceBrowserError::Provider)?;
        let level_info =
            serde_json::from_reader(Cursor::new(bytes)).map_err(SonorustResourceError::from)?;
        Ok(level_info)
    }

    pub async fn level_data(
        &self,
        level_data_url: &str,
    ) -> Result<LevelData, SonorustResourceBrowserError<P::Error>> {
        let bytes = self
            .resource_provider
            .fetch_bytes(level_data_url)
            .await
            .map_err(SonorustResourceBrowserError::Provider)?;
        let decompressed_bytes = Self::decompress(bytes)?;
        let level_info = serde_json::from_reader(Cursor::new(decompressed_bytes))
            .map_err(SonorustResourceError::from)?;
        Ok(level_info)
    }

    pub async fn skin_data(
        &self,
        skin_data_url: &str,
    ) -> Result<SkinData, SonorustResourceBrowserError<P::Error>> {
        let bytes = self
            .resource_provider
            .fetch_bytes(skin_data_url)
            .await
            .map_err(SonorustResourceBrowserError::Provider)?;
        let decompressed_bytes = Self::decompress(bytes)?;
        let level_info = serde_json::from_reader(Cursor::new(decompressed_bytes))
            .map_err(SonorustResourceError::from)?;
        Ok(level_info)
    }

    pub async fn engine_play_data(
        &self,
        play_data_url: &str,
    ) -> Result<EnginePlayData, SonorustResourceBrowserError<P::Error>> {
        let bytes = self
            .resource_provider
            .fetch_bytes(play_data_url)
            .await
            .map_err(SonorustResourceBrowserError::Provider)?;
        let decompressed_bytes = Self::decompress(bytes)?;
        let level_info = serde_json::from_reader(Cursor::new(decompressed_bytes))
            .map_err(SonorustResourceError::from)?;
        Ok(level_info)
    }

    pub async fn engine_configuration(
        &self,
        configuration_url: &str,
    ) -> Result<EngineConfiguration, SonorustResourceBrowserError<P::Error>> {
        let bytes = self
            .resource_provider
            .fetch_bytes(configuration_url)
            .await
            .map_err(SonorustResourceBrowserError::Provider)?;
        let decompressed_bytes = Self::decompress(bytes)?;
        let level_info = serde_json::from_reader(Cursor::new(decompressed_bytes))
            .map_err(SonorustResourceError::from)?;
        Ok(level_info)
    }

    pub async fn bgm_bytes(
        &self,
        bgm_url: &str,
    ) -> Result<LevelBgmBytes, SonorustResourceBrowserError<P::Error>> {
        let bytes = self
            .resource_provider
            .fetch_bytes(bgm_url)
            .await
            .map_err(SonorustResourceBrowserError::Provider)?;
        Ok(LevelBgmBytes(bytes))
    }

    pub async fn skin_texture_bytes(
        &self,
        skin_texture_url: &str,
    ) -> Result<SkinTextureBytes, SonorustResourceBrowserError<P::Error>> {
        let bytes = self
            .resource_provider
            .fetch_bytes(skin_texture_url)
            .await
            .map_err(SonorustResourceBrowserError::Provider)?;
        Ok(SkinTextureBytes(bytes))
    }

    fn decompress(bytes: impl AsRef<[u8]>) -> Result<Vec<u8>, SonorustResourceError> {
        let mut gzip_decoder = GzDecoder::new(Cursor::new(bytes.as_ref()));
        let mut decompressed_bytes = Vec::new();
        let read_bytes = gzip_decoder.read_to_end(&mut decompressed_bytes)?;
        decompressed_bytes.resize(read_bytes, 0);
        Ok(decompressed_bytes)
    }
}

#[derive(Debug, Error)]
pub enum SonorustResourceBrowserError<P>
where
    P: std::error::Error + 'static,
{
    #[error("Resource error: {0}")]
    Resource(#[from] SonorustResourceError),
    #[error("Provider error: {0}")]
    Provider(P),
}
