use serde::Deserialize;
use sonorust_ir::IRValue;

use crate::engine::play::archetype::data::{EngineArchetypeDataName, EngineArchetypeName};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LevelData {
    pub bgm_offset: IRValue,
    pub entities: Vec<LevelDataEntity>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelDataEntityData {
    pub name: EngineArchetypeDataName,
    #[serde(flatten)]
    pub payload: Option<LevelDataEntityDataPayload>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum LevelDataEntityDataPayload {
    Reference { reference: String },
    Value { value: IRValue },
}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelDataEntity {
    pub name: Option<String>,
    pub archetype: EngineArchetypeName,
    pub data: Vec<LevelDataEntityData>,
}
