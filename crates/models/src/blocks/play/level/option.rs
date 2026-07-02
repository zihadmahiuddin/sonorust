use sonorust_ir::IRValue;
use tracing::warn;

use crate::{blocks::ReadableBlock, engine::configuration::option::EngineOption};

#[derive(Debug)]
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
