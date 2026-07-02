use serde::{Deserialize, Serialize};
use sonorust_ir::IRValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum EngineOption {
    Slider {
        name: String,
        description: Option<String>,
        standard: Option<bool>,
        advanced: Option<bool>,
        scope: Option<String>,
        def: IRValue,
        min: IRValue,
        max: IRValue,
        step: IRValue,
        unit: Option<String>,
    },
    Toggle {
        name: String,
        description: Option<String>,
        standard: Option<bool>,
        advanced: Option<bool>,
        scope: Option<String>,
        def: IRValue,
    },
    Select {
        name: String,
        description: Option<String>,
        standard: Option<bool>,
        advanced: Option<bool>,
        scope: Option<String>,
        def: IRValue,
        values: Vec<String>,
    },
}
