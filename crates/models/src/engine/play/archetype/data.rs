use std::sync::Arc;

use serde::Deserialize;

pub type EngineArchetypeName = Arc<str>;
pub type EngineArchetypeDataName = Arc<str>;

#[derive(Debug, Deserialize, Clone)]
pub struct EnginePlayDataArchetypeCallback {
    pub index: usize,
    pub order: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnginePlayDataArchetypeImport {
    pub name: EngineArchetypeDataName,
    pub index: usize,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
