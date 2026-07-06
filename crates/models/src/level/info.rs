#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct LevelInfo {
    pub item: LevelItem,
    pub description: String,
    pub has_community: bool,
    pub leaderboards: Vec<Leaderboard>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
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

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Tag {
    pub title: String,
    pub icon: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
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

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
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

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
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

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
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

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
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

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct UseSkin {
    pub use_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct UseBackground {
    pub use_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct UseEffect {
    pub use_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct UseParticle {
    pub use_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Srl {
    pub url: String,
    pub hash: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Leaderboard {
    pub name: String,
    pub title: String,
}
