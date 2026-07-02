use std::sync::nonpoison::RwLock;

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
    runtime_environment: &'a RwLock<PlayRuntimeEnvironment>,
    #[memory]
    runtime_update: &'a PlayRuntimeUpdate,
    #[memory]
    runtime_touch_array: &'a PlayRuntimeTouchArray,
    #[memory]
    runtime_skin_transform: &'a RwLock<PlayRuntimeSkinTransform>,
    #[memory]
    runtime_particle_transform: &'a RwLock<PlayRuntimeParticleTransform>,
    #[memory]
    runtime_background: &'a RwLock<PlayRuntimeBackground>,
    #[memory]
    runtime_ui: &'a RwLock<PlayRuntimeUi>,
    #[memory]
    runtime_ui_configuration: &'a RwLock<PlayRuntimeUiConfiguration>,

    #[memory]
    level_memory: &'a RwLock<PlayLevelMemory>,
    #[memory]
    level_data: &'a RwLock<PlayLevelData>,
    #[memory]
    level_option: &'a PlayLevelOption,
    #[memory]
    level_bucket: &'a RwLock<PlayLevelBucket>,
    #[memory]
    level_score: &'a RwLock<PlayLevelScore>,
    #[memory]
    level_life: &'a RwLock<PlayLevelLife>,

    #[memory]
    engine_rom: &'a PlayEngineRom,

    #[memory(index = "*ctx.current_entity.id * PlayEntityMemory::SIZE")]
    entity_memory: &'a RwLock<PlayEntityMemoryArray>,
    #[memory]
    #[memory(block = PlayEntityData, index = "*ctx.current_entity.id * PlayEntityData::SIZE")]
    entity_data: &'a RwLock<PlayEntityDataArray>,
    #[memory]
    #[memory(block = PlayEntitySharedMemory, index = "*ctx.current_entity.id * PlayEntitySharedMemory::SIZE")]
    entity_shared_memory: &'a RwLock<PlayEntitySharedMemoryArray>,
    #[memory]
    #[memory(block = PlayEntityInfo, index = "*ctx.current_entity.id * PlayEntityInfo::SIZE")]
    entity_info: &'a PlayEntityInfoArray,
    #[memory]
    entity_despawn: &'a RwLock<PlayEntityDespawn>,
    #[memory]
    entity_input: &'a RwLock<PlayEntityInput>,
    #[memory]
    entity_score: &'a RwLock<PlayEntityScore>,
    #[memory]
    entity_life: &'a RwLock<PlayEntityLife>,

    #[memory]
    archetype_score: &'a RwLock<PlayArchetypeScore>,
    #[memory]
    archetype_life: &'a RwLock<PlayArchetypeLife>,

    #[memory]
    temporary_memory: &'a RwLock<TemporaryMemory>,
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
    entity_memory: &'a RwLock<PlayEntityMemoryArray>,
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
    entity_despawn: &'a RwLock<PlayEntityDespawn>,
    #[memory]
    entity_input: &'a RwLock<PlayEntityInput>,
    #[memory]
    entity_score: &'a PlayEntityScore,
    #[memory]
    entity_life: &'a PlayEntityLife,

    #[memory]
    archetype_score: &'a PlayArchetypeScore,
    #[memory]
    archetype_life: &'a PlayArchetypeLife,

    #[memory]
    temporary_memory: &'a RwLock<TemporaryMemory>,
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
    runtime_skin_transform: &'a RwLock<PlayRuntimeSkinTransform>,
    #[memory]
    runtime_particle_transform: &'a RwLock<PlayRuntimeParticleTransform>,
    #[memory]
    runtime_background: &'a RwLock<PlayRuntimeBackground>,
    #[memory]
    runtime_ui: &'a PlayRuntimeUi,
    #[memory]
    runtime_ui_configuration: &'a PlayRuntimeUiConfiguration,

    #[memory]
    level_memory: &'a RwLock<PlayLevelMemory>,
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
    entity_memory: &'a RwLock<PlayEntityMemoryArray>,
    #[memory]
    #[memory(block = PlayEntityData, index = "*ctx.current_entity.id * PlayEntityData::SIZE")]
    entity_data: &'a PlayEntityDataArray,
    #[memory]
    #[memory(block = PlayEntitySharedMemory, index = "*ctx.current_entity.id * PlayEntitySharedMemory::SIZE")]
    entity_shared_memory: &'a RwLock<PlayEntitySharedMemoryArray>,
    #[memory]
    #[memory(block = PlayEntityInfo, index = "*ctx.current_entity.id * PlayEntityInfo::SIZE")]
    entity_info: &'a PlayEntityInfoArray,
    #[memory]
    entity_despawn: &'a RwLock<PlayEntityDespawn>,
    #[memory]
    entity_input: &'a RwLock<PlayEntityInput>,
    #[memory]
    entity_score: &'a PlayEntityScore,
    #[memory]
    entity_life: &'a PlayEntityLife,

    #[memory]
    archetype_score: &'a PlayArchetypeScore,
    #[memory]
    archetype_life: &'a PlayArchetypeLife,

    #[memory]
    temporary_memory: &'a RwLock<TemporaryMemory>,
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
    entity_memory: &'a RwLock<PlayEntityMemoryArray>,
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
    entity_despawn: &'a RwLock<PlayEntityDespawn>,
    #[memory]
    entity_input: &'a RwLock<PlayEntityInput>,
    #[memory]
    entity_score: &'a PlayEntityScore,
    #[memory]
    entity_life: &'a PlayEntityLife,

    #[memory]
    archetype_score: &'a PlayArchetypeScore,
    #[memory]
    archetype_life: &'a PlayArchetypeLife,

    #[memory]
    temporary_memory: &'a RwLock<TemporaryMemory>,
}

pub struct MemoryBlocks {
    pub runtime_environment: RwLock<PlayRuntimeEnvironment>,
    pub runtime_update: RwLock<PlayRuntimeUpdate>,
    pub runtime_touch_array: RwLock<PlayRuntimeTouchArray>,
    pub runtime_skin_transform: RwLock<PlayRuntimeSkinTransform>,
    pub runtime_particle_transform: RwLock<PlayRuntimeParticleTransform>,
    pub runtime_background: RwLock<PlayRuntimeBackground>,
    pub runtime_ui: RwLock<PlayRuntimeUi>,
    pub runtime_ui_configuration: RwLock<PlayRuntimeUiConfiguration>,

    pub level_memory: RwLock<PlayLevelMemory>,
    pub level_data: RwLock<PlayLevelData>,
    pub level_option: RwLock<PlayLevelOption>,
    pub level_bucket: RwLock<PlayLevelBucket>,
    pub level_score: RwLock<PlayLevelScore>,
    pub level_life: RwLock<PlayLevelLife>,

    pub engine_rom: RwLock<PlayEngineRom>,

    pub entity_memory: RwLock<PlayEntityMemoryArray>,
    pub entity_data: RwLock<PlayEntityDataArray>,
    pub entity_shared_memory: RwLock<PlayEntitySharedMemoryArray>,
    pub entity_info: RwLock<PlayEntityInfoArray>,
    pub entity_despawn: RwLock<PlayEntityDespawn>,
    pub entity_input: RwLock<PlayEntityInput>,
    pub entity_score: RwLock<PlayEntityScore>,
    pub entity_life: RwLock<PlayEntityLife>,

    pub archetype_score: RwLock<PlayArchetypeScore>,
    pub archetype_life: RwLock<PlayArchetypeLife>,

    pub temporary_memory: RwLock<TemporaryMemory>,
}

impl MemoryBlocks {
    pub fn with_preprocess_access<F, R>(&self, f: F) -> R
    where
        F: FnOnce(PlayPreprocessMemoryAccess<'_>) -> R,
    {
        let access = PlayPreprocessMemoryAccess {
            runtime_environment: &self.runtime_environment,
            runtime_update: &self.runtime_update.read(),
            runtime_touch_array: &self.runtime_touch_array.read(),
            runtime_skin_transform: &self.runtime_skin_transform,
            runtime_particle_transform: &self.runtime_particle_transform,
            runtime_background: &self.runtime_background,
            runtime_ui: &self.runtime_ui,
            runtime_ui_configuration: &self.runtime_ui_configuration,

            level_memory: &self.level_memory,
            level_data: &self.level_data,
            level_option: &self.level_option.read(),
            level_bucket: &self.level_bucket,
            level_score: &self.level_score,
            level_life: &self.level_life,

            engine_rom: &self.engine_rom.read(),

            entity_memory: &self.entity_memory,
            entity_data: &self.entity_data,
            entity_shared_memory: &self.entity_shared_memory,
            entity_info: &self.entity_info.read(),
            entity_despawn: &self.entity_despawn,
            entity_input: &self.entity_input,
            entity_score: &self.entity_score,
            entity_life: &self.entity_life,

            archetype_score: &self.archetype_score,
            archetype_life: &self.archetype_life,

            temporary_memory: &self.temporary_memory,
        };

        f(access)
    }

    pub fn with_initialization_access<F, R>(&self, f: F) -> R
    where
        F: FnOnce(PlayInitializationMemoryAccess<'_>) -> R,
    {
        let access = PlayInitializationMemoryAccess {
            runtime_environment: &self.runtime_environment.read(),
            runtime_update: &self.runtime_update.read(),
            runtime_touch_array: &self.runtime_touch_array.read(),
            runtime_skin_transform: &self.runtime_skin_transform.read(),
            runtime_particle_transform: &self.runtime_particle_transform.read(),
            runtime_background: &self.runtime_background.read(),
            runtime_ui: &self.runtime_ui.read(),
            runtime_ui_configuration: &self.runtime_ui_configuration.read(),

            level_memory: &self.level_memory.read(),
            level_data: &self.level_data.read(),
            level_option: &self.level_option.read(),
            level_bucket: &self.level_bucket.read(),
            level_score: &self.level_score.read(),
            level_life: &self.level_life.read(),

            engine_rom: &self.engine_rom.read(),

            entity_memory: &self.entity_memory,
            entity_data: &self.entity_data.read(),
            entity_shared_memory: &self.entity_shared_memory.read(),
            entity_info: &self.entity_info.read(),
            entity_despawn: &self.entity_despawn,
            entity_input: &self.entity_input,
            entity_score: &self.entity_score.read(),
            entity_life: &self.entity_life.read(),

            archetype_score: &self.archetype_score.read(),
            archetype_life: &self.archetype_life.read(),

            temporary_memory: &self.temporary_memory,
        };

        f(access)
    }

    pub fn with_sequential_access<F, R>(&self, f: F) -> R
    where
        F: FnOnce(PlaySequentialMemoryAccess<'_>) -> R,
    {
        let access = PlaySequentialMemoryAccess {
            runtime_environment: &self.runtime_environment.read(),
            runtime_update: &self.runtime_update.read(),
            runtime_touch_array: &self.runtime_touch_array.read(),
            runtime_skin_transform: &self.runtime_skin_transform,
            runtime_particle_transform: &self.runtime_particle_transform,
            runtime_background: &self.runtime_background,
            runtime_ui: &self.runtime_ui.read(),
            runtime_ui_configuration: &self.runtime_ui_configuration.read(),

            level_memory: &self.level_memory,
            level_data: &self.level_data.read(),
            level_option: &self.level_option.read(),
            level_bucket: &self.level_bucket.read(),
            level_score: &self.level_score.read(),
            level_life: &self.level_life.read(),

            engine_rom: &self.engine_rom.read(),

            entity_memory: &self.entity_memory,
            entity_data: &self.entity_data.read(),
            entity_shared_memory: &self.entity_shared_memory,
            entity_info: &self.entity_info.read(),
            entity_despawn: &self.entity_despawn,
            entity_input: &self.entity_input,
            entity_score: &self.entity_score.read(),
            entity_life: &self.entity_life.read(),

            archetype_score: &self.archetype_score.read(),
            archetype_life: &self.archetype_life.read(),

            temporary_memory: &self.temporary_memory,
        };

        f(access)
    }

    pub fn with_parallel_access<F, R>(&self, f: F) -> R
    where
        F: FnOnce(PlayParallelMemoryAccess<'_>) -> R,
    {
        let access = PlayParallelMemoryAccess {
            runtime_environment: &self.runtime_environment.read(),
            runtime_update: &self.runtime_update.read(),
            runtime_touch_array: &self.runtime_touch_array.read(),
            runtime_skin_transform: &self.runtime_skin_transform.read(),
            runtime_particle_transform: &self.runtime_particle_transform.read(),
            runtime_background: &self.runtime_background.read(),
            runtime_ui: &self.runtime_ui.read(),
            runtime_ui_configuration: &self.runtime_ui_configuration.read(),

            level_memory: &self.level_memory.read(),
            level_data: &self.level_data.read(),
            level_option: &self.level_option.read(),
            level_bucket: &self.level_bucket.read(),
            level_score: &self.level_score.read(),
            level_life: &self.level_life.read(),

            engine_rom: &self.engine_rom.read(),

            entity_memory: &self.entity_memory,
            entity_data: &self.entity_data.read(),
            entity_shared_memory: &self.entity_shared_memory.read(),
            entity_info: &self.entity_info.read(),
            entity_despawn: &self.entity_despawn,
            entity_input: &self.entity_input,
            entity_score: &self.entity_score.read(),
            entity_life: &self.entity_life.read(),

            archetype_score: &self.archetype_score.read(),
            archetype_life: &self.archetype_life.read(),

            temporary_memory: &self.temporary_memory,
        };

        f(access)
    }
}
