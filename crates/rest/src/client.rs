use std::io::{Cursor, Read};

use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use sonorust_models::{
    engine::{configuration::EngineConfiguration, play::data::EnginePlayData},
    level::{data::LevelData, info::LevelInfo},
    skin::data::SkinData,
};
use url::Url;

use crate::{
    error::SonorustRestError,
    types::{LevelBgmBytes, SkinTextureBytes},
};

pub struct SonorustRestClient {
    client: Client,
    base_url: Url,
}

impl SonorustRestClient {
    pub fn new(base_url: &str) -> Result<Self, SonorustRestError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(SonorustRestClient { client, base_url })
    }

    pub fn level_info(&self, level_id: &str) -> Result<LevelInfo, SonorustRestError> {
        let url = self.base_url.join(&format!("sonolus/levels/{level_id}"))?;
        let bytes = self.client.get(url).send()?.bytes()?;
        let level_info = serde_json::from_reader(Cursor::new(bytes))?;
        Ok(level_info)
    }

    pub fn level_data(&self, level_data_url: &str) -> Result<LevelData, SonorustRestError> {
        let url = self.base_url.join(level_data_url)?;
        let bytes = self.client.get(url).send()?.bytes()?;
        let decompressed_bytes = Self::decompress(bytes)?;
        let level_info = serde_json::from_reader(Cursor::new(decompressed_bytes))?;
        Ok(level_info)
    }

    pub fn skin_data(&self, skin_data_url: &str) -> Result<SkinData, SonorustRestError> {
        let url = self.base_url.join(skin_data_url)?;
        let bytes = self.client.get(url).send()?.bytes()?;
        let decompressed_bytes = Self::decompress(bytes)?;
        let level_info = serde_json::from_reader(Cursor::new(decompressed_bytes))?;
        Ok(level_info)
    }

    pub fn engine_play_data(
        &self,
        play_data_url: &str,
    ) -> Result<EnginePlayData, SonorustRestError> {
        let url = self.base_url.join(play_data_url)?;
        let bytes = self.client.get(url).send()?.bytes()?;
        let decompressed_bytes = Self::decompress(bytes)?;
        let level_info = serde_json::from_reader(Cursor::new(decompressed_bytes))?;
        Ok(level_info)
    }

    pub fn engine_configuration(
        &self,
        configuration_url: &str,
    ) -> Result<EngineConfiguration, SonorustRestError> {
        let url = self.base_url.join(configuration_url)?;
        let bytes = self.client.get(url).send()?.bytes()?;
        let decompressed_bytes = Self::decompress(bytes)?;
        let level_info = serde_json::from_reader(Cursor::new(decompressed_bytes))?;
        Ok(level_info)
    }

    pub fn bgm_bytes(&self, bgm_url: &str) -> Result<LevelBgmBytes, SonorustRestError> {
        let url = self.base_url.join(bgm_url)?;
        let bytes = self.client.get(url).send()?.bytes()?.to_vec();
        Ok(LevelBgmBytes(bytes))
    }

    pub fn skin_texture_bytes(
        &self,
        skin_texture_url: &str,
    ) -> Result<SkinTextureBytes, SonorustRestError> {
        let url = self.base_url.join(skin_texture_url)?;
        let bytes = self.client.get(url).send()?.bytes()?.to_vec();
        Ok(SkinTextureBytes(bytes))
    }

    fn decompress(bytes: impl AsRef<[u8]>) -> Result<Vec<u8>, SonorustRestError> {
        let mut gzip_decoder = GzDecoder::new(Cursor::new(bytes.as_ref()));
        let mut decompressed_bytes = Vec::new();
        let read_bytes = gzip_decoder.read_to_end(&mut decompressed_bytes)?;
        decompressed_bytes.resize(read_bytes, 0);
        Ok(decompressed_bytes)
    }
}
