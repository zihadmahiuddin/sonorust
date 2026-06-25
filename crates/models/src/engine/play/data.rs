use serde::{Deserialize, Serialize};

use crate::engine::play::archetype::data::EnginePlayDataArchetype;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnginePlayData {
    pub skin: Skin,
    pub archetypes: Vec<EnginePlayDataArchetype>,
    pub nodes: Vec<Node>,
    pub buckets: Vec<Bucket>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Node {
    Literal { value: f64 },
    FunctionCall { func: String, args: Vec<usize> },
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SkinRenderMode {
    #[default]
    Default,
    Standard,
    Lightweight,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    pub unit: Option<String>,
    pub sprites: Vec<BucketSprite>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct BucketSprite {
    id: i64,
    fallback_id: Option<i64>,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    rotation: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct Skin {
    render_mode: Option<SkinRenderMode>,
    pub sprites: Vec<Sprite>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    pub name: String,
    pub id: usize,
}
