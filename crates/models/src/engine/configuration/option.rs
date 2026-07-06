use sonorust_ir::IRValue;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase"),
    serde(tag = "type")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
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
