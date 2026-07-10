use sonorust_models::{
    engine::{configuration::EngineConfiguration, play::data::EnginePlayData},
    level::{data::LevelData, info::LevelInfo},
    skin::data::SkinData,
};

use crate::{
    client::SonorustResourceManager,
    error::SonorustResourceError,
    types::{LevelBgmBytes, SkinTextureBytes},
};

pub trait LevelInfoExt {
    fn data(&self, client: &SonorustResourceManager) -> Result<LevelData, SonorustResourceError>;

    fn skin_data(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<SkinData, SonorustResourceError>;

    fn engine_play_data(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<EnginePlayData, SonorustResourceError>;

    fn engine_configuration(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<EngineConfiguration, SonorustResourceError>;

    fn bgm_bytes(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<LevelBgmBytes, SonorustResourceError>;

    fn skin_texture_bytes(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<SkinTextureBytes, SonorustResourceError>;
}

impl LevelInfoExt for LevelInfo {
    fn data(&self, client: &SonorustResourceManager) -> Result<LevelData, SonorustResourceError> {
        client.level_data(&self.item.data.url)
    }

    fn skin_data(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<SkinData, SonorustResourceError> {
        client.skin_data(&self.item.engine.skin.data.url)
    }

    fn engine_play_data(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<EnginePlayData, SonorustResourceError> {
        client.engine_play_data(&self.item.engine.play_data.url)
    }

    fn engine_configuration(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<EngineConfiguration, SonorustResourceError> {
        client.engine_configuration(&self.item.engine.configuration.url)
    }

    fn bgm_bytes(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<LevelBgmBytes, SonorustResourceError> {
        client.bgm_bytes(&self.item.bgm.url)
    }

    fn skin_texture_bytes(
        &self,
        client: &SonorustResourceManager,
    ) -> Result<SkinTextureBytes, SonorustResourceError> {
        client.skin_texture_bytes(&self.item.engine.skin.texture.url)
    }
}
