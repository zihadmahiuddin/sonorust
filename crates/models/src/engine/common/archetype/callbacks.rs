#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::engine::play::archetype::callbacks::PlayEngineArchetypeCallbackType;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum EngineArchetypeCallbackType {
    Play(PlayEngineArchetypeCallbackType),
    Watch(PlaceholderWatchCallbackType), // TODO
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum PlaceholderWatchCallbackType {
    Preprocess,
    SpawnOrder,
}
