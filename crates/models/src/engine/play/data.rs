use sonorust_ir::IRValue;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::engine::play::archetype::data::EnginePlayDataArchetype;

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EnginePlayData {
    pub skin: Skin,
    pub archetypes: Vec<EnginePlayDataArchetype>,
    pub nodes: Vec<Node>,
    pub buckets: Vec<Bucket>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase"),
    serde(untagged)
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum Node {
    Literal { value: IRValue },
    FunctionCall { func: String, args: Vec<usize> },
}

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum SkinRenderMode {
    #[default]
    Default,
    Standard,
    Lightweight,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Bucket {
    pub unit: Option<String>,
    pub sprites: Vec<BucketSprite>,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[allow(unused)]
pub struct BucketSprite {
    id: i64,
    fallback_id: Option<i64>,
    x: IRValue,
    y: IRValue,
    w: IRValue,
    h: IRValue,
    rotation: IRValue,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[allow(unused)]
pub struct Skin {
    render_mode: Option<SkinRenderMode>,
    pub sprites: Vec<Sprite>,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Sprite {
    pub name: String,
    pub id: usize,
}
