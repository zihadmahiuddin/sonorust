use std::cell::RefCell;

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

pub struct PlayPreprocessMemoryAccess<'a> {
    runtime_environment: &'a RefCell<PlayRuntimeEnvironment>,
    runtime_update: &'a PlayRuntimeUpdate,
    runtime_touch_array: &'a PlayRuntimeTouchArray,
    runtime_skin_transform: &'a RefCell<PlayRuntimeSkinTransform>,
    runtime_particle_transform: &'a RefCell<PlayRuntimeParticleTransform>,
    runtime_background: &'a RefCell<PlayRuntimeBackground>,
    runtime_ui: &'a RefCell<PlayRuntimeUi>,
    runtime_ui_configuration: &'a RefCell<PlayRuntimeUiConfiguration>,

    level_memory: &'a RefCell<PlayLevelMemory>,
    level_data: &'a RefCell<PlayLevelData>,
    level_option: &'a PlayLevelOption,
    level_bucket: &'a RefCell<PlayLevelBucket>,
    level_score: &'a RefCell<PlayLevelScore>,
    level_life: &'a RefCell<PlayLevelLife>,

    engine_rom: &'a PlayEngineRom,

    entity_memory: &'a RefCell<PlayEntityMemoryArray>,
    entity_data: &'a RefCell<PlayEntityDataArray>,
    entity_shared_memory: &'a PlayEntitySharedMemoryArray,
    entity_info: &'a PlayEntityInfoArray,
    entity_despawn: &'a RefCell<PlayEntityDespawn>,
    entity_input: &'a RefCell<PlayEntityInput>,
    entity_score: &'a RefCell<PlayEntityScore>,
    entity_life: &'a RefCell<PlayEntityLife>,

    archetype_score: &'a RefCell<PlayArchetypeScore>,
    archetype_life: &'a RefCell<PlayArchetypeLife>,

    temporary_memory: &'a RefCell<TemporaryMemory>,
}

impl<'a> MemoryAccess for PlayPreprocessMemoryAccess<'a> {
    fn read(
        &self,
        ctx: &RuntimeContext,
        block_id: u64,
        index: usize,
    ) -> Option<sonorust_ir::IRValue> {
        match block_id {
            PlayRuntimeEnvironment::BLOCK_ID => self.runtime_environment.borrow().read(index),
            PlayRuntimeUpdate::BLOCK_ID => self.runtime_update.read(index),
            PlayRuntimeTouchArray::BLOCK_ID => self.runtime_touch_array.read(index),
            PlayRuntimeSkinTransform::BLOCK_ID => self.runtime_skin_transform.borrow().read(index),
            PlayRuntimeParticleTransform::BLOCK_ID => {
                self.runtime_particle_transform.borrow().read(index)
            }
            PlayRuntimeBackground::BLOCK_ID => self.runtime_background.borrow().read(index),
            PlayRuntimeUi::BLOCK_ID => self.runtime_ui.borrow().read(index),
            PlayRuntimeUiConfiguration::BLOCK_ID => {
                self.runtime_ui_configuration.borrow().read(index)
            }
            PlayLevelMemory::BLOCK_ID => self.level_memory.borrow().read(index),
            PlayLevelData::BLOCK_ID => self.level_data.borrow().read(index),
            PlayLevelOption::BLOCK_ID => self.level_option.read(index),
            PlayLevelBucket::BLOCK_ID => self.level_bucket.borrow().read(index),
            PlayLevelScore::BLOCK_ID => self.level_score.borrow().read(index),
            PlayLevelLife::BLOCK_ID => self.level_life.borrow().read(index),
            PlayEngineRom::BLOCK_ID => self.engine_rom.read(index),
            PlayEntityMemoryArray::BLOCK_ID => {
                let index_in_array = *ctx.current_entity.id * PlayEntityMemory::SIZE;
                self.entity_memory.borrow().read(index_in_array)
            }
            PlayEntityDataArray::BLOCK_ID => self.entity_data.borrow().read(index),
            PlayEntityData::BLOCK_ID => {
                let index_in_array = *ctx.current_entity.id * PlayEntityData::SIZE;
                self.entity_data.borrow().read(index_in_array)
            }
            PlayEntitySharedMemoryArray::BLOCK_ID => self.entity_shared_memory.read(index),
            PlayEntitySharedMemory::BLOCK_ID => {
                let index_in_array = *ctx.current_entity.id * PlayEntitySharedMemory::SIZE;
                self.entity_shared_memory.read(index_in_array)
            }
            PlayEntityInfoArray::BLOCK_ID => self.entity_info.read(index),
            PlayEntityInfo::BLOCK_ID => {
                let index_in_array = *ctx.current_entity.id * PlayEntityInfo::SIZE;
                self.entity_info.read(index_in_array)
            }
            PlayEntityDespawn::BLOCK_ID => self.entity_despawn.borrow().read(index),
            PlayEntityInput::BLOCK_ID => self.entity_input.borrow().read(index),
            PlayEntityScore::BLOCK_ID => self.entity_score.borrow().read(index),
            PlayEntityLife::BLOCK_ID => self.entity_life.borrow().read(index),
            PlayArchetypeScore::BLOCK_ID => self.archetype_score.borrow().read(index),
            PlayArchetypeLife::BLOCK_ID => self.archetype_life.borrow().read(index),
            TemporaryMemory::BLOCK_ID => self.temporary_memory.borrow().read(index),
            other => {
                tracing::warn!(
                    "Attempted to read from unknown block ID {}, index {}",
                    other,
                    index
                );
                None
            }
        }
    }
    fn write(
        &self,
        ctx: &RuntimeContext,
        block_id: u64,
        index: usize,
        value: sonorust_ir::IRValue,
    ) -> Option<sonorust_ir::IRValue> {
        match block_id {
            PlayRuntimeEnvironment::BLOCK_ID => self
                .runtime_environment
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayRuntimeSkinTransform::BLOCK_ID => self
                .runtime_skin_transform
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayRuntimeParticleTransform::BLOCK_ID => self
                .runtime_particle_transform
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayRuntimeBackground::BLOCK_ID => self
                .runtime_background
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayRuntimeUi::BLOCK_ID => self
                .runtime_ui
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayRuntimeUiConfiguration::BLOCK_ID => self
                .runtime_ui_configuration
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayLevelMemory::BLOCK_ID => self
                .level_memory
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayLevelData::BLOCK_ID => self
                .level_data
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayLevelBucket::BLOCK_ID => self
                .level_bucket
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayLevelScore::BLOCK_ID => self
                .level_score
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayLevelLife::BLOCK_ID => self
                .level_life
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayEntityMemoryArray::BLOCK_ID => {
                let index_in_array = *ctx.current_entity.id * PlayEntityMemory::SIZE;
                self.entity_memory
                    .borrow_mut()
                    .write(index_in_array, value)
                    .then_some(value)
            }
            PlayEntityDataArray::BLOCK_ID => self
                .entity_data
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayEntityData::BLOCK_ID => {
                let index_in_array = *ctx.current_entity.id * PlayEntityData::SIZE;
                self.entity_data
                    .borrow_mut()
                    .write(index_in_array, value)
                    .then_some(value)
            }
            PlayEntityDespawn::BLOCK_ID => self
                .entity_despawn
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayEntityInput::BLOCK_ID => self
                .entity_input
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayEntityScore::BLOCK_ID => self
                .entity_score
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayEntityLife::BLOCK_ID => self
                .entity_life
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayArchetypeScore::BLOCK_ID => self
                .archetype_score
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            PlayArchetypeLife::BLOCK_ID => self
                .archetype_life
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            TemporaryMemory::BLOCK_ID => self
                .temporary_memory
                .borrow_mut()
                .write(index, value)
                .then_some(value),
            other => {
                tracing::warn!(
                    "Attempted to write to unknown block ID {}, index {}",
                    other,
                    index
                );
                None
            }
        }
    }
}
