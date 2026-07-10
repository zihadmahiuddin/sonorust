use sonorust_models::{
    engine::{configuration::EngineConfiguration, play::data::EnginePlayData},
    level::{data::LevelData, info::LevelInfo},
    skin::data::SkinData,
};

use crate::{
    browser::{SonorustResourceBrowser, SonorustResourceBrowserError},
    provider::SonorustResourceProvider,
    types::{LevelBgmBytes, SkinTextureBytes},
};

pub trait LevelInfoExt<P>
where
    P: SonorustResourceProvider,
    P::Error: std::error::Error + 'static,
{
    fn data(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<LevelData, SonorustResourceBrowserError<P::Error>>>;

    fn skin_data(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<SkinData, SonorustResourceBrowserError<P::Error>>>;

    fn engine_play_data(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<EnginePlayData, SonorustResourceBrowserError<P::Error>>>;

    fn engine_configuration(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<EngineConfiguration, SonorustResourceBrowserError<P::Error>>>;

    fn bgm_bytes(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<LevelBgmBytes, SonorustResourceBrowserError<P::Error>>>;

    fn skin_texture_bytes(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<SkinTextureBytes, SonorustResourceBrowserError<P::Error>>>;
}

impl<P> LevelInfoExt<P> for LevelInfo
where
    P: SonorustResourceProvider,
    P::Error: std::error::Error + 'static,
{
    fn data(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<LevelData, SonorustResourceBrowserError<P::Error>>> {
        browser.level_data(&self.item.data.url)
    }

    fn skin_data(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<SkinData, SonorustResourceBrowserError<P::Error>>> {
        browser.skin_data(&self.item.engine.skin.data.url)
    }

    fn engine_play_data(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<EnginePlayData, SonorustResourceBrowserError<P::Error>>> {
        browser.engine_play_data(&self.item.engine.play_data.url)
    }

    fn engine_configuration(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<EngineConfiguration, SonorustResourceBrowserError<P::Error>>>
    {
        browser.engine_configuration(&self.item.engine.configuration.url)
    }

    fn bgm_bytes(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<LevelBgmBytes, SonorustResourceBrowserError<P::Error>>> {
        browser.bgm_bytes(&self.item.bgm.url)
    }

    fn skin_texture_bytes(
        &self,
        browser: &SonorustResourceBrowser<P>,
    ) -> impl Future<Output = Result<SkinTextureBytes, SonorustResourceBrowserError<P::Error>>>
    {
        browser.skin_texture_bytes(&self.item.engine.skin.texture.url)
    }
}
