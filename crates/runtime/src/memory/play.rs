use std::cell::RefCell;

use sonorust_macros::MemoryAccess;
use sonorust_models::blocks::{
    ReadableBlock, WritableBlock,
    common::TemporaryMemory,
    play::{
        archetype::{life::PlayArchetypeLife, score::PlayArchetypeScore},
        engine_rom::PlayEngineRom,
        entity::{
            data::{PlayEntityData, PlayEntityDataArray},
            despawn::PlayEntityDespawn,
            info::{PlayEntityInfo, PlayEntityInfoArray},
            input::PlayEntityInput,
            life::PlayEntityLife,
            memory::{PlayEntityMemory, PlayEntityMemoryArray},
            score::PlayEntityScore,
            shared_memory::{PlayEntitySharedMemory, PlayEntitySharedMemoryArray},
        },
        level::{
            bucket::PlayLevelBucket, data::PlayLevelData, life::PlayLevelLife,
            memory::PlayLevelMemory, option::PlayLevelOption, score::PlayLevelScore,
        },
        runtime::{
            background::PlayRuntimeBackground, environment::PlayRuntimeEnvironment,
            particle_transform::PlayRuntimeParticleTransform,
            skin_transform::PlayRuntimeSkinTransform, touch_array::PlayRuntimeTouchArray,
            ui::PlayRuntimeUi, ui_configuration::PlayRuntimeUiConfiguration,
            update::PlayRuntimeUpdate,
        },
    },
};

use crate::context::{MemoryAccess, RuntimeContext};

/// This is used for the Preprocess callbacks
#[derive(MemoryAccess)]
pub struct PlayPreprocessMemoryAccess<'a> {
    #[memory]
    runtime_environment: &'a RefCell<PlayRuntimeEnvironment>,
    #[memory]
    runtime_update: &'a PlayRuntimeUpdate,
    #[memory]
    runtime_touch_array: &'a PlayRuntimeTouchArray,
    #[memory]
    runtime_skin_transform: &'a RefCell<PlayRuntimeSkinTransform>,
    #[memory]
    runtime_particle_transform: &'a RefCell<PlayRuntimeParticleTransform>,
    #[memory]
    runtime_background: &'a RefCell<PlayRuntimeBackground>,
    #[memory]
    runtime_ui: &'a RefCell<PlayRuntimeUi>,
    #[memory]
    runtime_ui_configuration: &'a RefCell<PlayRuntimeUiConfiguration>,

    #[memory]
    level_memory: &'a RefCell<PlayLevelMemory>,
    #[memory]
    level_data: &'a RefCell<PlayLevelData>,
    #[memory]
    level_option: &'a PlayLevelOption,
    #[memory]
    level_bucket: &'a RefCell<PlayLevelBucket>,
    #[memory]
    level_score: &'a RefCell<PlayLevelScore>,
    #[memory]
    level_life: &'a RefCell<PlayLevelLife>,

    #[memory]
    engine_rom: &'a PlayEngineRom,

    #[memory(index = "*ctx.current_entity.id * PlayEntityMemory::SIZE")]
    entity_memory: &'a RefCell<PlayEntityMemoryArray>,
    #[memory]
    #[memory(block = PlayEntityData, index = "*ctx.current_entity.id * PlayEntityData::SIZE")]
    entity_data: &'a RefCell<PlayEntityDataArray>,
    #[memory]
    #[memory(block = PlayEntitySharedMemory, index = "*ctx.current_entity.id * PlayEntitySharedMemory::SIZE")]
    entity_shared_memory: &'a PlayEntitySharedMemoryArray,
    #[memory]
    #[memory(block = PlayEntityInfo, index = "*ctx.current_entity.id * PlayEntityInfo::SIZE")]
    entity_info: &'a PlayEntityInfoArray,
    #[memory]
    entity_despawn: &'a RefCell<PlayEntityDespawn>,
    #[memory]
    entity_input: &'a RefCell<PlayEntityInput>,
    #[memory]
    entity_score: &'a RefCell<PlayEntityScore>,
    #[memory]
    entity_life: &'a RefCell<PlayEntityLife>,

    #[memory]
    archetype_score: &'a RefCell<PlayArchetypeScore>,
    #[memory]
    archetype_life: &'a RefCell<PlayArchetypeLife>,

    #[memory]
    temporary_memory: &'a RefCell<TemporaryMemory>,
}

/// This is used for the SpawnOrder, ShouldSpawn, and Initialize callbacks
#[derive(MemoryAccess)]
pub struct PlayInitializationMemoryAccess<'a> {
    #[memory]
    runtime_environment: &'a PlayRuntimeEnvironment,
    #[memory]
    runtime_update: &'a PlayRuntimeUpdate,
    #[memory]
    runtime_touch_array: &'a PlayRuntimeTouchArray,
    #[memory]
    runtime_skin_transform: &'a PlayRuntimeSkinTransform,
    #[memory]
    runtime_particle_transform: &'a PlayRuntimeParticleTransform,
    #[memory]
    runtime_background: &'a PlayRuntimeBackground,
    #[memory]
    runtime_ui: &'a PlayRuntimeUi,
    #[memory]
    runtime_ui_configuration: &'a PlayRuntimeUiConfiguration,

    #[memory]
    level_memory: &'a PlayLevelMemory,
    #[memory]
    level_data: &'a PlayLevelData,
    #[memory]
    level_option: &'a PlayLevelOption,
    #[memory]
    level_bucket: &'a PlayLevelBucket,
    #[memory]
    level_score: &'a PlayLevelScore,
    #[memory]
    level_life: &'a PlayLevelLife,

    #[memory]
    engine_rom: &'a PlayEngineRom,

    #[memory(index = "*ctx.current_entity.id * PlayEntityMemory::SIZE")]
    entity_memory: &'a RefCell<PlayEntityMemoryArray>,
    #[memory]
    #[memory(block = PlayEntityData, index = "*ctx.current_entity.id * PlayEntityData::SIZE")]
    entity_data: &'a PlayEntityDataArray,
    #[memory]
    #[memory(block = PlayEntitySharedMemory, index = "*ctx.current_entity.id * PlayEntitySharedMemory::SIZE")]
    entity_shared_memory: &'a PlayEntitySharedMemoryArray,
    #[memory]
    #[memory(block = PlayEntityInfo, index = "*ctx.current_entity.id * PlayEntityInfo::SIZE")]
    entity_info: &'a PlayEntityInfoArray,
    #[memory]
    entity_despawn: &'a RefCell<PlayEntityDespawn>,
    #[memory]
    entity_input: &'a RefCell<PlayEntityInput>,
    #[memory]
    entity_score: &'a PlayEntityScore,
    #[memory]
    entity_life: &'a PlayEntityLife,

    #[memory]
    archetype_score: &'a PlayArchetypeScore,
    #[memory]
    archetype_life: &'a PlayArchetypeLife,

    #[memory]
    temporary_memory: &'a RefCell<TemporaryMemory>,
}

/// This is used for the UpdateSequential and Touch callbacks
#[derive(MemoryAccess)]
pub struct PlaySequentialMemoryAccess<'a> {
    #[memory]
    runtime_environment: &'a PlayRuntimeEnvironment,
    #[memory]
    runtime_update: &'a PlayRuntimeUpdate,
    #[memory]
    runtime_touch_array: &'a PlayRuntimeTouchArray,
    #[memory]
    runtime_skin_transform: &'a RefCell<PlayRuntimeSkinTransform>,
    #[memory]
    runtime_particle_transform: &'a RefCell<PlayRuntimeParticleTransform>,
    #[memory]
    runtime_background: &'a RefCell<PlayRuntimeBackground>,
    #[memory]
    runtime_ui: &'a PlayRuntimeUi,
    #[memory]
    runtime_ui_configuration: &'a PlayRuntimeUiConfiguration,

    #[memory]
    level_memory: &'a RefCell<PlayLevelMemory>,
    #[memory]
    level_data: &'a PlayLevelData,
    #[memory]
    level_option: &'a PlayLevelOption,
    #[memory]
    level_bucket: &'a PlayLevelBucket,
    #[memory]
    level_score: &'a PlayLevelScore,
    #[memory]
    level_life: &'a PlayLevelLife,

    #[memory]
    engine_rom: &'a PlayEngineRom,

    #[memory(index = "*ctx.current_entity.id * PlayEntityMemory::SIZE")]
    entity_memory: &'a RefCell<PlayEntityMemoryArray>,
    #[memory]
    #[memory(block = PlayEntityData, index = "*ctx.current_entity.id * PlayEntityData::SIZE")]
    entity_data: &'a PlayEntityDataArray,
    #[memory]
    #[memory(block = PlayEntitySharedMemory, index = "*ctx.current_entity.id * PlayEntitySharedMemory::SIZE")]
    entity_shared_memory: &'a RefCell<PlayEntitySharedMemoryArray>,
    #[memory]
    #[memory(block = PlayEntityInfo, index = "*ctx.current_entity.id * PlayEntityInfo::SIZE")]
    entity_info: &'a PlayEntityInfoArray,
    #[memory]
    entity_despawn: &'a RefCell<PlayEntityDespawn>,
    #[memory]
    entity_input: &'a RefCell<PlayEntityInput>,
    #[memory]
    entity_score: &'a PlayEntityScore,
    #[memory]
    entity_life: &'a PlayEntityLife,

    #[memory]
    archetype_score: &'a PlayArchetypeScore,
    #[memory]
    archetype_life: &'a PlayArchetypeLife,

    #[memory]
    temporary_memory: &'a RefCell<TemporaryMemory>,
}

/// This is used for the UpdateParallel and Terminate callbacks
#[derive(MemoryAccess)]
pub struct PlayParallelMemoryAccess<'a> {
    #[memory]
    runtime_environment: &'a PlayRuntimeEnvironment,
    #[memory]
    runtime_update: &'a PlayRuntimeUpdate,
    #[memory]
    runtime_touch_array: &'a PlayRuntimeTouchArray,
    #[memory]
    runtime_skin_transform: &'a PlayRuntimeSkinTransform,
    #[memory]
    runtime_particle_transform: &'a PlayRuntimeParticleTransform,
    #[memory]
    runtime_background: &'a PlayRuntimeBackground,
    #[memory]
    runtime_ui: &'a PlayRuntimeUi,
    #[memory]
    runtime_ui_configuration: &'a PlayRuntimeUiConfiguration,

    #[memory]
    level_memory: &'a PlayLevelMemory,
    #[memory]
    level_data: &'a PlayLevelData,
    #[memory]
    level_option: &'a PlayLevelOption,
    #[memory]
    level_bucket: &'a PlayLevelBucket,
    #[memory]
    level_score: &'a PlayLevelScore,
    #[memory]
    level_life: &'a PlayLevelLife,

    #[memory]
    engine_rom: &'a PlayEngineRom,

    #[memory(index = "*ctx.current_entity.id * PlayEntityMemory::SIZE")]
    entity_memory: &'a RefCell<PlayEntityMemoryArray>,
    #[memory]
    #[memory(block = PlayEntityData, index = "*ctx.current_entity.id * PlayEntityData::SIZE")]
    entity_data: &'a PlayEntityDataArray,
    #[memory]
    #[memory(block = PlayEntitySharedMemory, index = "*ctx.current_entity.id * PlayEntitySharedMemory::SIZE")]
    entity_shared_memory: &'a PlayEntitySharedMemoryArray,
    #[memory]
    #[memory(block = PlayEntityInfo, index = "*ctx.current_entity.id * PlayEntityInfo::SIZE")]
    entity_info: &'a PlayEntityInfoArray,
    #[memory]
    entity_despawn: &'a RefCell<PlayEntityDespawn>,
    #[memory]
    entity_input: &'a RefCell<PlayEntityInput>,
    #[memory]
    entity_score: &'a PlayEntityScore,
    #[memory]
    entity_life: &'a PlayEntityLife,

    #[memory]
    archetype_score: &'a PlayArchetypeScore,
    #[memory]
    archetype_life: &'a PlayArchetypeLife,

    #[memory]
    temporary_memory: &'a RefCell<TemporaryMemory>,
}
