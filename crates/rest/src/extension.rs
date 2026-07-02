use sonorust_models::{
    engine::{configuration::EngineConfiguration, play::data::EnginePlayData},
    level::{data::LevelData, info::LevelInfo},
    skin::data::SkinData,
};

use crate::{
    client::SonorustRestClient,
    error::SonorustRestError,
    types::{LevelBgmBytes, SkinTextureBytes},
};

pub trait LevelInfoExt {
    fn data(&self, client: &SonorustRestClient) -> Result<LevelData, SonorustRestError>;

    fn skin_data(&self, client: &SonorustRestClient) -> Result<SkinData, SonorustRestError>;

    fn engine_play_data(
        &self,
        client: &SonorustRestClient,
    ) -> Result<EnginePlayData, SonorustRestError>;

    fn engine_configuration(
        &self,
        client: &SonorustRestClient,
    ) -> Result<EngineConfiguration, SonorustRestError>;

    fn bgm_bytes(&self, client: &SonorustRestClient) -> Result<LevelBgmBytes, SonorustRestError>;

    fn skin_texture_bytes(
        &self,
        client: &SonorustRestClient,
    ) -> Result<SkinTextureBytes, SonorustRestError>;
}

impl LevelInfoExt for LevelInfo {
    fn data(&self, client: &SonorustRestClient) -> Result<LevelData, SonorustRestError> {
        client.level_data(&self.item.data.url)
    }

    fn skin_data(&self, client: &SonorustRestClient) -> Result<SkinData, SonorustRestError> {
        client.skin_data(&self.item.engine.skin.data.url)
    }

    fn engine_play_data(
        &self,
        client: &SonorustRestClient,
    ) -> Result<EnginePlayData, SonorustRestError> {
        client.engine_play_data(&self.item.engine.play_data.url)
    }

    fn engine_configuration(
        &self,
        client: &SonorustRestClient,
    ) -> Result<EngineConfiguration, SonorustRestError> {
        client.engine_configuration(&self.item.engine.configuration.url)
    }

    fn bgm_bytes(&self, client: &SonorustRestClient) -> Result<LevelBgmBytes, SonorustRestError> {
        client.bgm_bytes(&self.item.bgm.url)
    }

    fn skin_texture_bytes(
        &self,
        client: &SonorustRestClient,
    ) -> Result<SkinTextureBytes, SonorustRestError> {
        client.skin_texture_bytes(&self.item.engine.skin.texture.url)
    }
}
