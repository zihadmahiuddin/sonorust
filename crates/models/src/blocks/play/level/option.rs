use sonorust_ir::IRValue;
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::{blocks::ReadableBlock, engine::configuration::option::EngineOption};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayLevelOption(pub Vec<IRValue>);

impl PlayLevelOption {
    pub const BLOCK_ID: u64 = 2002;

    pub fn new(options: &[EngineOption]) -> Self {
        Self(
            options
                .iter()
                .map(|option| match option {
                    EngineOption::Slider { def, name, .. } => {
                        if name == "#NOTE_SPEED" {
                            10.5
                        } else {
                            *def
                        }
                    }
                    EngineOption::Toggle { def, .. } => *def,
                    EngineOption::Select { def, .. } => *def,
                })
                .collect(),
        )
    }
}

impl ReadableBlock for PlayLevelOption {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.0.get(index) {
            Some(value) => Some(*value),
            None => {
                warn!("Attempted to read from out of bounds index {index} on PlayLevelOption");
                None
            }
        }
    }
}
