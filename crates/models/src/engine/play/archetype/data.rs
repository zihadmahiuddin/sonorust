use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

pub type EngineArchetypeName = Arc<str>;
pub type EngineArchetypeDataName = Arc<str>;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EnginePlayDataArchetypeCallback {
    pub index: usize,
    pub order: Option<i64>,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EnginePlayDataArchetypeImport {
    pub name: EngineArchetypeDataName,
    pub index: usize,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EnginePlayDataArchetype {
    pub name: EngineArchetypeName,
    pub has_input: bool,
    pub preprocess: Option<EnginePlayDataArchetypeCallback>,
    pub spawn_order: Option<EnginePlayDataArchetypeCallback>,
    pub should_spawn: Option<EnginePlayDataArchetypeCallback>,
    pub initialize: Option<EnginePlayDataArchetypeCallback>,
    pub update_sequential: Option<EnginePlayDataArchetypeCallback>,
    pub touch: Option<EnginePlayDataArchetypeCallback>,
    pub update_parallel: Option<EnginePlayDataArchetypeCallback>,
    pub terminate: Option<EnginePlayDataArchetypeCallback>,
    pub imports: Vec<EnginePlayDataArchetypeImport>,
    pub exports: Vec<EngineArchetypeDataName>,
}
