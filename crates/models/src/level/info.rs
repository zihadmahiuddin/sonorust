use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelInfo {
    pub item: LevelItem,
    pub description: String,
    pub has_community: bool,
    pub leaderboards: Vec<Leaderboard>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelItem {
    pub name: String,
    pub source: Option<String>,
    pub version: i64,
    pub rating: i64,
    pub title: String,
    pub artists: String,
    pub author: String,
    pub tags: Vec<Tag>,
    pub engine: Engine,
    pub use_skin: UseSkin,
    pub use_background: UseBackground,
    pub use_effect: UseEffect,
    pub use_particle: UseParticle,
    pub cover: Srl,
    pub bgm: Srl,
    pub data: Srl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub title: String,
    pub icon: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Engine {
    pub name: String,
    pub source: Option<String>,
    pub version: i64,
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub tags: Vec<Tag>,
    pub skin: Skin,
    pub background: Background,
    pub effect: Effect,
    pub particle: Particle,
    pub thumbnail: Srl,
    pub play_data: Srl,
    pub watch_data: Srl,
    pub preview_data: Srl,
    pub tutorial_data: Srl,
    pub configuration: Srl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub name: String,
    pub source: Option<String>,
    pub version: i64,
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub tags: Vec<Tag>,
    pub thumbnail: Srl,
    pub data: Srl,
    pub texture: Srl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    pub name: String,
    pub source: Option<String>,
    pub version: i64,
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub tags: Vec<Tag>,
    pub thumbnail: Srl,
    pub image: Srl,
    pub data: Srl,
    pub configuration: Srl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub name: String,
    pub source: Option<String>,
    pub version: i64,
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub tags: Vec<Tag>,
    pub thumbnail: Srl,
    pub data: Srl,
    pub audio: Srl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Particle {
    pub name: String,
    pub source: Option<String>,
    pub version: i64,
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub tags: Vec<Tag>,
    pub thumbnail: Srl,
    pub data: Srl,
    pub texture: Srl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UseSkin {
    pub use_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UseBackground {
    pub use_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UseEffect {
    pub use_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UseParticle {
    pub use_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Srl {
    pub url: String,
    pub hash: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub name: String,
    pub title: String,
}
