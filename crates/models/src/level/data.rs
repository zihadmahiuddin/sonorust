use sonorust_ir::IRValue;

use crate::engine::play::archetype::data::{EngineArchetypeDataName, EngineArchetypeName};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct LevelData {
    pub bgm_offset: IRValue,
    pub entities: Vec<LevelDataEntity>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct LevelDataEntityData {
    pub name: EngineArchetypeDataName,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub payload: Option<LevelDataEntityDataPayload>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum LevelDataEntityDataPayload {
    Reference { reference: String },
    Value { value: IRValue },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct LevelDataEntity {
    pub name: Option<String>,
    pub archetype: EngineArchetypeName,
    pub data: Vec<LevelDataEntityData>,
}
