#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum PlayEngineArchetypeCallbackType {
    Preprocess,
    SpawnOrder,
    ShouldSpawn,
    Initialize,
    UpdateSequential,
    Touch,
    UpdateParallel,
    Terminate,
}
