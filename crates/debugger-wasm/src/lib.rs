#![feature(nonpoison_rwlock)]
#![feature(sync_nonpoison)]

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::Cursor,
    sync::{Arc, nonpoison::RwLock},
};

use chrono::Utc;
use glam::{Mat4, Vec2};
use serde::{Deserialize, Serialize};
use sonorust_bytecode::{
    compiler::{CompilationResult, Compiler},
    instruction::{InstructionKind, InstructionOperand, UsedStackIndices},
    vm::{RunResult, VM, VMState},
};
use sonorust_ir::{IRValue, nodes::IRNode};
use sonorust_models::{
    blocks::{
        common::TemporaryMemory,
        play::{
            archetype::{life::PlayArchetypeLife, score::PlayArchetypeScore},
            engine_rom::PlayEngineRom,
            entity::{
                data::{PlayEntityData, PlayEntityDataArray},
                despawn::PlayEntityDespawn,
                info::{EntityState, PlayEntityInfo, PlayEntityInfoArray},
                input::PlayEntityInput,
                life::PlayEntityLife,
                memory::PlayEntityMemoryArray,
                score::PlayEntityScore,
                shared_memory::PlayEntitySharedMemoryArray,
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
    },
    engine::{
        configuration::EngineConfiguration,
        play::{
            archetype::{
                callbacks::PlayEngineArchetypeCallbackType,
                data::{EnginePlayDataArchetype, EnginePlayDataArchetypeCallback},
            },
            data::EnginePlayData,
        },
    },
    ids::{ArchetypeId, EntityId},
    level::data::{LevelDataEntity, LevelDataEntityDataPayload},
    skin::data::{SkinData, SkinSpriteTransformExpression},
};
use sonorust_resources::{
    browser::SonorustResourceBrowser,
    extension::LevelInfoExt,
    types::{LevelBgmBytes, SkinTextureBytes},
    zip::SonorustZipResourceProvider,
};
use sonorust_runtime::{
    access::{SideEffectAccess, TimingAccess},
    context::{CurrentEntity, RuntimeContext},
    memory::play::MemoryBlocks,
    side_effects::{DrawSideEffect, SpawnSideEffect},
};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;
use wasm_tracing::{WasmLayer, WasmLayerConfig};

use crate::logging::{JsTracingLayer, set_log_callback};

pub mod logging;

#[wasm_bindgen(typescript_custom_section)]
const LOG_CALLBACK_TYPE: &'static str = r#"
type LogCallback = (level: string, text: string) => void;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "LogCallback")]
    pub type LogCallback;
}

#[wasm_bindgen(js_name = "initLogging")]
pub fn init_logging(callback: LogCallback) {
    console_error_panic_hook::set_once();

    let callback_js_value: JsValue = callback.into();

    set_log_callback(callback_js_value.into());

    tracing_subscriber::registry()
        .with(WasmLayer::new(WasmLayerConfig::default()))
        .with(JsTracingLayer)
        .init();
}

#[derive(Debug, Clone, Deserialize, Serialize, Tsify)]
pub struct MemoryBlockSizes(pub HashMap<u64, usize>);

pub type PlayerChange = u32;

#[wasm_bindgen]
pub struct PlayerChanges;

#[wasm_bindgen]
impl PlayerChanges {
    pub fn state() -> PlayerChange {
        1 << 0
    }
    pub fn vm_breakpoints() -> PlayerChange {
        1 << 1
    }
    pub fn vm_memory_block_sizes() -> PlayerChange {
        1 << 2
    }
    pub fn vm_pc() -> PlayerChange {
        1 << 3
    }
    pub fn vm_stack() -> PlayerChange {
        1 << 4
    }
    pub fn vm_state() -> PlayerChange {
        1 << 5
    }
}

#[wasm_bindgen(js_name = "initSonorustPlayer")]
pub async fn init_sonorust_player(
    zip_bytes: &[u8],
    level_id: &str,
) -> Result<SonorustPlayerHandle, JsError> {
    let sonorust_resource_browser = SonorustResourceBrowser::new(
        SonorustZipResourceProvider::new(Cursor::new(zip_bytes)).unwrap(),
    );
    let level_info = sonorust_resource_browser
        .level_info(level_id)
        .await
        .unwrap();
    let engine_play_data = level_info
        .engine_play_data(&sonorust_resource_browser)
        .await
        .unwrap();
    let engine_configuration = level_info
        .engine_configuration(&sonorust_resource_browser)
        .await
        .unwrap();
    let level_data = level_info.data(&sonorust_resource_browser).await.unwrap();
    let skin_data = level_info
        .skin_data(&sonorust_resource_browser)
        .await
        .unwrap();
    let skin_texture_bytes = level_info
        .skin_texture_bytes(&sonorust_resource_browser)
        .await
        .unwrap();
    let level_bgm_bytes = level_info
        .bgm_bytes(&sonorust_resource_browser)
        .await
        .unwrap();
    let archetypes: BTreeMap<_, _> = engine_play_data
        .archetypes
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, archetype)| (ArchetypeId(index), Arc::new(archetype)))
        .collect();

    let archetype_name_map_with_id: BTreeMap<_, _> = archetypes
        .iter()
        .map(|(archetype_id, archetype)| {
            (
                Arc::clone(&archetype.name),
                (*archetype_id, Arc::clone(&archetype)),
            )
        })
        .collect();

    let (entity_info_map, entity_data_map, mut bpm_changes, mut time_scale_changes) =
        level_data.entities.clone().into_iter().enumerate().fold(
            (PlayEntityInfoArray {items: BTreeMap::new()}, PlayEntityDataArray { items: BTreeMap::new()}, Vec::new(), Vec::new()),
            |(mut entity_info_map, mut entity_data_map, mut bpm_changes, mut timescale_changes), (entity_index, entity)| {
                if let Some((archetype_id, archetype)) = archetype_name_map_with_id.get(&entity.archetype) {
                    let mut entity_data = [0.0; PlayEntityData::SIZE];
                    let level_data_entity_map = entity
                        .data
                        .iter()
                        .map(|a| (&a.name, &a.payload))
                        .collect::<HashMap<_, _>>();

                    for import in &archetype.imports {
                        if let Some(Some(payload)) = level_data_entity_map.get(&import.name) {
                            match payload {
                                LevelDataEntityDataPayload::Reference { reference } => {
                                    warn!(
                                        "TODO: Level Data Entity Data Payload by reference {reference}"
                                    );
                                }
                                LevelDataEntityDataPayload::Value { value } => {
                                    entity_data[import.index] = *value;
                                }
                            }
                        }
                    }

                    let entity_id = EntityId(entity_index);
                    entity_info_map.items.insert(
                        entity_id,
                        PlayEntityInfo {
                            index: entity_index,
                            archetype_id: *archetype_id,
                            state: EntityState::Waiting,
                        },
                    );
                    entity_data_map.items.insert(entity_id, PlayEntityData::new(entity_data));
                } else if let Ok(special_archetype) = SpecialArchetype::try_from(entity) {
                    match special_archetype {
                        SpecialArchetype::BpmChange { beat, bpm } => {
                            bpm_changes.push((beat, bpm));
                        }
                        SpecialArchetype::TimescaleChange { beat, timescale } => {
                            timescale_changes.push((beat, timescale));
                        }
                    }
                } else {
                    unreachable!()
                }

                (entity_info_map, entity_data_map, bpm_changes, timescale_changes)
            },
        );

    bpm_changes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let bpm_changes = bpm_changes
        .into_iter()
        .fold(Vec::new(), |mut bpm_changes, (beat, bpm)| {
            let Some(last_change) = bpm_changes.last() else {
                bpm_changes.push(BpmChange {
                    bpm,
                    starting_beat: beat,
                    starting_time: 0.0,
                });
                return bpm_changes;
            };

            let last_change_beat_count = beat - last_change.starting_beat;
            let last_change_bps = last_change.bpm / 60.0;
            let last_change_duration_secs = last_change_beat_count / last_change_bps;

            let change = BpmChange {
                bpm,
                starting_beat: beat,
                starting_time: last_change.starting_time + last_change_duration_secs,
            };
            bpm_changes.push(change);
            bpm_changes
        });
    let bpm_changes = BpmChanges {
        changes: bpm_changes,
    };

    time_scale_changes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // set default time scale from the beginning of the level to 1
    if time_scale_changes.is_empty() || time_scale_changes[0].0 != 0.0 {
        time_scale_changes.insert(0, (0.0, 1.0));
    }

    let time_scale_changes = time_scale_changes.into_iter().fold(
        Vec::new(),
        |mut time_scale_changes, (beat, time_scale)| {
            let Some(last_change) = time_scale_changes.last() else {
                time_scale_changes.push(TimeScaleChange {
                    time_scale,
                    starting_time: bpm_changes.beat_to_time(beat),
                    starting_scaled_time: bpm_changes.beat_to_time(beat) * time_scale,
                });
                return time_scale_changes;
            };

            let starting_time = bpm_changes.beat_to_time(beat);
            let duration = starting_time - last_change.starting_time;
            let scaled_duration = duration * last_change.time_scale;
            let starting_scaled_time = last_change.starting_scaled_time + scaled_duration;

            let change = TimeScaleChange {
                time_scale,
                starting_time,
                starting_scaled_time,
            };
            time_scale_changes.push(change);
            time_scale_changes
        },
    );
    let time_scale_changes = TimeScaleChanges {
        changes: time_scale_changes,
    };

    let runtime_environment = PlayRuntimeEnvironment {
        debug_mode: false,
        screen_aspect_ratio: 16.0 / 9.0,
        audio_offset: 0.0,
        input_offset: 0.0,
        multiplayer: false,
        safe_area_min: Vec2::ZERO,
        safe_area_max: Vec2::ZERO,
    };
    let runtime_update = PlayRuntimeUpdate {
        time: 0.0,
        delta_time: 0.0,
        scaled_time: 0.0,
        touch_count: 0,
    };
    let runtime_touch_array = PlayRuntimeTouchArray(Vec::new());
    let runtime_skin_transform = PlayRuntimeSkinTransform(Mat4::IDENTITY);
    let runtime_particle_transform = PlayRuntimeParticleTransform(Mat4::IDENTITY);
    let runtime_background = PlayRuntimeBackground {
        bottom_left: Vec2::ZERO,
        top_left: Vec2::ZERO,
        top_right: Vec2::ZERO,
        bottom_right: Vec2::ZERO,
    };
    let runtime_ui = PlayRuntimeUi::default();
    let runtime_ui_configuration = PlayRuntimeUiConfiguration::default();

    let level_memory = PlayLevelMemory::default();
    let play_level_data = PlayLevelData::default();
    let level_option = PlayLevelOption::new(&engine_configuration.options);
    let level_bucket = PlayLevelBucket::new(engine_play_data.buckets.len());
    let level_score = PlayLevelScore::default();
    let level_life = PlayLevelLife::default();

    let engine_rom = PlayEngineRom(Box::new([]));

    let entity_memory = PlayEntityMemoryArray::new(entity_info_map.items.keys());
    let entity_shared_memory = PlayEntitySharedMemoryArray::new(entity_info_map.items.keys());
    let entity_despawn = PlayEntityDespawn::default();
    let entity_input = PlayEntityInput::new(entity_info_map.items.keys());
    let entity_score = PlayEntityScore::new(entity_info_map.items.keys());
    let entity_life = PlayEntityLife::new(entity_info_map.items.keys());

    let archetype_score = PlayArchetypeScore::new(archetypes.keys());
    let archetype_life = PlayArchetypeLife::new(archetypes.keys());

    let temporary_memory = TemporaryMemory::default();

    let memory_blocks = MemoryBlocks {
        runtime_environment: RwLock::new(runtime_environment),
        runtime_update: RwLock::new(runtime_update),
        runtime_touch_array: RwLock::new(runtime_touch_array),
        runtime_skin_transform: RwLock::new(runtime_skin_transform),
        runtime_particle_transform: RwLock::new(runtime_particle_transform),
        runtime_background: RwLock::new(runtime_background),
        runtime_ui: RwLock::new(runtime_ui),
        runtime_ui_configuration: RwLock::new(runtime_ui_configuration),
        level_memory: RwLock::new(level_memory),
        level_data: RwLock::new(play_level_data),
        level_option: RwLock::new(level_option),
        level_bucket: RwLock::new(level_bucket),
        level_score: RwLock::new(level_score),
        level_life: RwLock::new(level_life),
        engine_rom: RwLock::new(engine_rom),
        entity_memory: RwLock::new(entity_memory),
        entity_data: RwLock::new(entity_data_map),
        entity_shared_memory: RwLock::new(entity_shared_memory),
        entity_info: RwLock::new(entity_info_map),
        entity_despawn: RwLock::new(entity_despawn),
        entity_input: RwLock::new(entity_input),
        entity_score: RwLock::new(entity_score),
        entity_life: RwLock::new(entity_life),
        archetype_score: RwLock::new(archetype_score),
        archetype_life: RwLock::new(archetype_life),
        temporary_memory: RwLock::new(temporary_memory),
    };

    let (oks, errs) = engine_play_data
        .nodes
        .iter()
        .map(IRNode::try_from)
        .partition::<Vec<_>, _>(|r| r.is_ok());

    let ir_nodes: Vec<_> = oks.into_iter().map(Result::unwrap).collect();
    let ir_node_errs: HashSet<_> = errs.into_iter().map(Result::unwrap_err).collect();

    if !ir_node_errs.is_empty() {
        panic!("Failed to convert JSON nodes to IR nodes: {ir_node_errs:#?}",);
    };

    let cached_vms = engine_play_data
        .archetypes
        .iter()
        .enumerate()
        .map(|(archetype_id, archetype)| {
            let compilation_results = CALLBACK_ACCESSORS
                .iter()
                .filter_map(|(callback_type, accessor)| {
                    accessor(archetype).as_ref().map(|cb| {
                        let mut compiler = Compiler::new(&ir_nodes);
                        compiler.compile_node(cb.index.into());
                        let compilation_result = compiler.finish();
                        let mut vm = VM::new();
                        vm.load_instructions(&compilation_result.instructions);
                        (*callback_type, (vm, compilation_result))
                    })
                })
                .collect::<HashMap<_, _>>();

            (ArchetypeId(archetype_id), compilation_results)
        })
        .collect::<HashMap<_, _>>();

    let player = SonorustPlayer {
        cached_vms,
        engine_play_data,
        archetypes,
        spawn_queue: Vec::new(),
        initialize_queue: Vec::new(),
        memory_blocks,
        bpm_changes,
        time_scale_changes,
        side_effects: SideEffects::default(),
        engine_configuration,
        skin_data,
        skin_texture_bytes,
        level_bgm_bytes,
        bgm_offset: level_data.bgm_offset,
        state: SonorustPlayerState::Waiting,
        current_stage: GameStage::AwaitingStart,
        current_queue_index: 0,
        preprocess_queue: Vec::new(),
        spawn_order_queue: Vec::new(),
        evaluated_spawn_queue: Vec::new(),
        sequential_update_queue: Vec::new(),
        parallel_update_queue: Vec::new(),
        despawn_queue: Vec::new(),
    };

    Ok(SonorustPlayerHandle {
        inner: player,
        on_change: None,
    })
}

type CallbackAccessor = fn(&EnginePlayDataArchetype) -> &Option<EnginePlayDataArchetypeCallback>;
const CALLBACK_ACCESSORS: [(PlayEngineArchetypeCallbackType, CallbackAccessor); 7] = [
    (PlayEngineArchetypeCallbackType::Preprocess, |a| {
        &a.preprocess
    }),
    (PlayEngineArchetypeCallbackType::SpawnOrder, |a| {
        &a.spawn_order
    }),
    (PlayEngineArchetypeCallbackType::ShouldSpawn, |a| {
        &a.should_spawn
    }),
    (PlayEngineArchetypeCallbackType::Initialize, |a| {
        &a.initialize
    }),
    (PlayEngineArchetypeCallbackType::UpdateSequential, |a| {
        &a.update_sequential
    }),
    (PlayEngineArchetypeCallbackType::UpdateParallel, |a| {
        &a.update_parallel
    }),
    (PlayEngineArchetypeCallbackType::Terminate, |a| &a.terminate),
];

#[derive(Debug, Tsify, Deserialize, Serialize)]
pub struct ArchetypeNameMap(HashMap<ArchetypeId, String>);

#[wasm_bindgen]
pub struct SonorustPlayerHandle {
    inner: SonorustPlayer,
    on_change: Option<js_sys::Function>,
}

#[wasm_bindgen]
impl SonorustPlayerHandle {
    #[wasm_bindgen(js_name = "runUntilBreak")]
    pub fn run_until_break(
        &mut self,
        max_steps: usize,
    ) -> Result<Ts<SonorustPlayerState>, JsError> {
        self.inner.run_until_break(max_steps);
        self.notify_change(
            PlayerChanges::state()
                | PlayerChanges::vm_pc()
                | PlayerChanges::vm_stack()
                | PlayerChanges::vm_state(),
        );
        Ok(self.inner.state.into_ts()?)
    }

    #[wasm_bindgen(js_name = "toggleBreakpoint")]
    pub fn toggle_breakpoint(
        &mut self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
        pc: usize,
    ) -> bool {
        let Some(vm) = self.get_vm_mut(archetype_id, callback_type) else {
            warn!(
                "Could not toggle breakpoint because cached VM was not found for archetype {archetype_id} {callback_type:?}"
            );
            return false;
        };
        let added = vm.toggle_breakpoint(pc);
        info!(
            "Breakpoint {} at pc {} for archetype {}, callback type {:?}",
            if added { "added" } else { "removed" },
            pc,
            archetype_id,
            callback_type
        );
        self.notify_change(PlayerChanges::vm_state() | PlayerChanges::vm_breakpoints());
        added
    }

    #[wasm_bindgen(js_name = "getCurrentExecutionTarget")]
    pub fn get_current_execution_target(&self) -> Result<Option<Ts<ExecutionTarget>>, JsError> {
        match self.inner.current_execution_target() {
            Some(x) => Ok(Some(x.into_ts()?)),
            None => Ok(None),
        }
    }

    #[wasm_bindgen(js_name = "getArchetypes")]
    pub fn get_archetypes(&self) -> Result<Ts<ArchetypeNameMap>, JsError> {
        Ok(ArchetypeNameMap(
            self.inner
                .archetypes
                .iter()
                .map(|(k, v)| (*k, v.name.to_string()))
                .collect(),
        )
        .into_ts()?)
    }

    #[wasm_bindgen(js_name = "getEntitiesForArchetype")]
    pub fn get_entities_for_archetype(&self, archetype_id: usize) -> Vec<usize> {
        self.inner
            .memory_blocks
            .entity_info
            .read()
            .items
            .iter()
            .filter_map(|(entity_id, entity_info)| {
                (*entity_info.archetype_id == archetype_id).then_some(**entity_id)
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "getInstUsedStackIndices")]
    pub fn get_inst_used_stack_indices(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
        index: usize,
    ) -> Result<Option<Ts<UsedStackIndices>>, JsError> {
        let Some(vm) = self.get_vm(archetype_id, callback_type) else {
            return Ok(None);
        };
        Ok(Some(
            vm.instructions
                .get(index)
                .map(|inst| inst.used_stack_indices(&vm.stack))
                .unwrap_or_else(|| UsedStackIndices(Default::default()))
                .into_ts()?,
        ))
    }

    #[wasm_bindgen(js_name = "getInstMnemonic")]
    pub fn get_inst_mnemonic(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
        index: usize,
    ) -> Option<String> {
        let Some(vm) = self.get_vm(archetype_id, callback_type) else {
            return None;
        };
        Some(vm.instructions[index].mnemonic().to_owned())
    }

    #[wasm_bindgen(js_name = "getInstOperands")]
    pub fn get_inst_operands(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
        index: usize,
    ) -> Result<Option<Vec<Ts<InstructionOperand>>>, JsError> {
        let Some(vm) = self.get_vm(archetype_id, callback_type) else {
            return Ok(None);
        };
        Ok(Some(
            vm.instructions[index]
                .operands()
                .into_iter()
                .map(|x| x.into_ts())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    #[wasm_bindgen(js_name = "getInstKind")]
    pub fn get_inst_kind(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
        index: usize,
    ) -> Option<InstructionKind> {
        self.get_vm(archetype_id, callback_type)
            .map(|vm| vm.instructions[index].kind())
    }

    #[wasm_bindgen(js_name = "getOperandAsString")]
    pub fn get_operand_as_string(
        &self,
        operand: Ts<InstructionOperand>,
    ) -> Result<String, JsError> {
        Ok(operand.to_rust()?.to_string())
    }

    #[wasm_bindgen(js_name = "getCompilationResult")]
    pub fn get_compilation_result_wasm(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
    ) -> Result<Option<Ts<CompilationResult>>, JsError> {
        let Some(c) = self.get_compilation_result(archetype_id, callback_type) else {
            return Ok(None);
        };
        Ok(Some(c.into_ts()?))
    }

    #[wasm_bindgen(js_name = "getVmBreakpoints")]
    pub fn get_vm_breakpoints(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
    ) -> Option<Vec<usize>> {
        self.get_vm(archetype_id, callback_type)
            .map(|vm| vm.breakpoints.iter().copied().collect())
    }

    #[wasm_bindgen(js_name = "getVmPc")]
    pub fn get_vm_pc(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
    ) -> Option<usize> {
        self.get_vm(archetype_id, callback_type).map(|vm| vm.pc)
    }

    #[wasm_bindgen(js_name = "getVmStack")]
    pub fn get_vm_stack(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
    ) -> Option<Vec<IRValue>> {
        self.get_vm(archetype_id, callback_type)
            .map(|vm| vm.stack.clone())
    }

    #[wasm_bindgen(js_name = "getVmState")]
    pub fn get_vm_state(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
    ) -> Option<VMState> {
        self.get_vm(archetype_id, callback_type).map(|vm| vm.state)
    }

    #[wasm_bindgen(js_name = "readMemoryRange")]
    pub fn read_memory_range(&self, block_id: u64, start: usize, count: usize) -> Vec<IRValue> {
        self.inner
            .memory_blocks
            .read_range_from_block(block_id, start..start + count)
    }

    #[wasm_bindgen(getter, js_name = "memoryBlockSizes")]
    pub fn memory_block_sizes(&self) -> Result<Ts<MemoryBlockSizes>, JsError> {
        Ok(MemoryBlockSizes(self.inner.memory_blocks.get_memory_block_sizes()).into_ts()?)
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> Result<Ts<SonorustPlayerState>, JsError> {
        Ok(self.inner.state.into_ts()?)
    }

    pub fn subscribe(&mut self, cb: js_sys::Function) {
        self.on_change = Some(cb);
    }

    pub fn unsubscribe(&mut self) {
        self.on_change = None;
    }

    fn notify_change(&self, mask: PlayerChange) {
        if let Some(ref cb) = self.on_change {
            let _ = cb.call1(&JsValue::NULL, &JsValue::from(mask));
        }
    }

    fn get_vm(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
    ) -> Option<&VM> {
        self.inner
            .cached_vms
            .get(&ArchetypeId(archetype_id))
            .and_then(|vms| vms.get(&callback_type))
            .map(|x| &x.0)
    }

    fn get_vm_mut(
        &mut self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
    ) -> Option<&mut VM> {
        self.inner
            .cached_vms
            .get_mut(&ArchetypeId(archetype_id))
            .and_then(|vms| vms.get_mut(&callback_type))
            .map(|x| &mut x.0)
    }

    fn get_compilation_result(
        &self,
        archetype_id: usize,
        callback_type: PlayEngineArchetypeCallbackType,
    ) -> Option<&CompilationResult> {
        self.inner
            .cached_vms
            .get(&ArchetypeId(archetype_id))
            .and_then(|vms| vms.get(&callback_type))
            .map(|x| &x.1)
    }
}

#[derive(Debug, Clone, Copy, Tsify, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionTarget {
    entity_id: EntityId,
    archetype_id: ArchetypeId,
    callback_type: PlayEngineArchetypeCallbackType,
}

#[derive(Debug, Clone, Copy, Tsify, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SonorustPlayerState {
    Waiting,
    Running,
    #[serde(rename_all = "camelCase")]
    Paused {
        entity_id: EntityId,
        archetype_id: ArchetypeId,
        callback_type: PlayEngineArchetypeCallbackType,
        pc: usize,
    },
    FinishedFrame,
}

#[derive(Debug, PartialEq)]
pub enum GameStage {
    AwaitingStart,
    PreprocessInit,
    PreprocessLoop,
    SpawnOrderInit,
    SpawnOrderLoop,
    AwaitingUpdate,
    SpawningInit,
    SpawningLoop,
    InitializeInit,
    InitializeLoop,
    UpdateSequentialInit,
    UpdateSequentialLoop,
    Input,
    UpdateParallelInit,
    UpdateParallelLoop,
    DespawningInit,
    DespawningLoop,
    Presentation,
    FinishedFrame,
}

struct SonorustPlayer {
    current_stage: GameStage,
    current_queue_index: usize,

    state: SonorustPlayerState,
    cached_vms:
        HashMap<ArchetypeId, HashMap<PlayEngineArchetypeCallbackType, (VM, CompilationResult)>>,
    engine_play_data: EnginePlayData,
    archetypes: BTreeMap<ArchetypeId, Arc<EnginePlayDataArchetype>>,
    memory_blocks: MemoryBlocks,
    bpm_changes: BpmChanges,
    time_scale_changes: TimeScaleChanges,
    side_effects: SideEffects,
    engine_configuration: EngineConfiguration,
    skin_data: SkinData,
    skin_texture_bytes: SkinTextureBytes,
    level_bgm_bytes: LevelBgmBytes,
    bgm_offset: IRValue,

    spawn_queue: Vec<(EntityId, ArchetypeId)>,
    initialize_queue: Vec<(EntityId, ArchetypeId)>,
    preprocess_queue: Vec<(EntityId, ArchetypeId, i64)>,
    spawn_order_queue: Vec<(EntityId, ArchetypeId, i64)>,
    evaluated_spawn_queue: Vec<(EntityId, ArchetypeId, IRValue)>,
    sequential_update_queue: Vec<(EntityId, ArchetypeId, i64)>,
    parallel_update_queue: Vec<(EntityId, ArchetypeId)>,
    despawn_queue: Vec<(EntityId, ArchetypeId)>,
}

impl SonorustPlayer {
    pub fn run_until_break(&mut self, max_steps: usize) -> SonorustPlayerState {
        match self.state {
            SonorustPlayerState::Waiting => {
                self.current_stage = GameStage::AwaitingStart;
            }
            SonorustPlayerState::FinishedFrame => {
                self.current_stage = GameStage::AwaitingUpdate;
            }
            SonorustPlayerState::Running | SonorustPlayerState::Paused { .. } => {}
        }

        self.state = SonorustPlayerState::Running;

        self.state = if self.is_in_start_phase() {
            self.start(max_steps)
        } else {
            self.update(max_steps)
        };

        self.state
    }

    pub fn current_execution_target(&self) -> Option<ExecutionTarget> {
        use GameStage::*;

        match self.current_stage {
            AwaitingStart | PreprocessInit | SpawnOrderInit | AwaitingUpdate | SpawningInit
            | InitializeInit | UpdateSequentialInit | Input | UpdateParallelInit
            | DespawningInit | Presentation | FinishedFrame => None,
            PreprocessLoop => self.preprocess_queue.get(self.current_queue_index).map(
                |&(entity_id, archetype_id, _)| ExecutionTarget {
                    archetype_id,
                    entity_id,
                    callback_type: PlayEngineArchetypeCallbackType::Preprocess,
                },
            ),
            SpawnOrderLoop => self.spawn_order_queue.get(self.current_queue_index).map(
                |&(entity_id, archetype_id, _)| ExecutionTarget {
                    archetype_id,
                    entity_id,
                    callback_type: PlayEngineArchetypeCallbackType::SpawnOrder,
                },
            ),
            SpawningLoop => {
                self.spawn_queue
                    .get(self.current_queue_index)
                    .map(|&(entity_id, archetype_id)| ExecutionTarget {
                        entity_id,
                        archetype_id,
                        callback_type: PlayEngineArchetypeCallbackType::ShouldSpawn,
                    })
            }
            InitializeLoop => self.initialize_queue.get(self.current_queue_index).map(
                |&(entity_id, archetype_id)| ExecutionTarget {
                    entity_id,
                    archetype_id,
                    callback_type: PlayEngineArchetypeCallbackType::Initialize,
                },
            ),
            UpdateSequentialLoop => self
                .sequential_update_queue
                .get(self.current_queue_index)
                .map(|&(entity_id, archetype_id, _)| ExecutionTarget {
                    entity_id,
                    archetype_id,
                    callback_type: PlayEngineArchetypeCallbackType::UpdateSequential,
                }),
            UpdateParallelLoop => self
                .parallel_update_queue
                .get(self.current_queue_index)
                .map(|&(entity_id, archetype_id)| ExecutionTarget {
                    entity_id,
                    archetype_id,
                    callback_type: PlayEngineArchetypeCallbackType::UpdateParallel,
                }),
            DespawningLoop => self.despawn_queue.get(self.current_queue_index).map(
                |&(entity_id, archetype_id)| ExecutionTarget {
                    entity_id,
                    archetype_id,
                    callback_type: PlayEngineArchetypeCallbackType::Terminate,
                },
            ),
        }
    }

    fn start(&mut self, max_steps: usize) -> SonorustPlayerState {
        loop {
            match self.current_stage {
                GameStage::AwaitingStart => {
                    self.current_stage = GameStage::PreprocessInit;
                }
                GameStage::PreprocessInit => {
                    let mut queue: Vec<_> = {
                        let entity_info = self.memory_blocks.entity_info.read();
                        entity_info
                            .items
                            .iter()
                            .filter_map(|(&id, info)| {
                                self.archetypes
                                    .get(&info.archetype_id)
                                    .and_then(|a| a.preprocess.as_ref())
                                    .map(|p| (id, info.archetype_id, p.order.unwrap_or_default()))
                            })
                            .collect()
                    };
                    queue.sort_by_key(|&(_, _, order)| order);
                    self.preprocess_queue = queue;
                    self.current_queue_index = 0;
                    self.current_stage = GameStage::PreprocessLoop;
                }
                GameStage::PreprocessLoop => {
                    let memory_blocks = &self.memory_blocks;
                    let bpm_changes = &self.bpm_changes;
                    let time_scale_changes = &self.time_scale_changes;
                    let side_effects = &self.side_effects;

                    while self.current_queue_index < self.preprocess_queue.len() {
                        let (entity_id, archetype_id, _) =
                            self.preprocess_queue[self.current_queue_index];

                        let Some((vm, _)) =
                            self.cached_vms.get_mut(&archetype_id).and_then(|items| {
                                items.get_mut(&PlayEngineArchetypeCallbackType::Preprocess)
                            })
                        else {
                            warn!(
                                "Could not find cached VM for archetype {}, callback type {:?}",
                                *archetype_id,
                                PlayEngineArchetypeCallbackType::Preprocess
                            );
                            self.current_queue_index += 1;
                            continue;
                        };
                        let run_result = memory_blocks.with_preprocess_access(|memory| {
                            let context = RuntimeContext {
                                current_entity: CurrentEntity {
                                    id: entity_id,
                                    archetype_id,
                                },
                                memory: &memory,
                                timing: &TimingInfo {
                                    bpm_changes,
                                    time_scale_changes,
                                },
                                side_effects,
                            };
                            vm.run(&context, max_steps)
                        });

                        match run_result {
                            RunResult::StepLimitReached => return SonorustPlayerState::Running,
                            RunResult::Paused => {
                                return SonorustPlayerState::Paused {
                                    entity_id,
                                    archetype_id,
                                    callback_type: PlayEngineArchetypeCallbackType::Preprocess,
                                    pc: vm.pc,
                                };
                            }
                            RunResult::Finished => {
                                self.current_queue_index += 1;
                            }
                        }
                    }

                    self.current_stage = GameStage::SpawnOrderInit;
                    self.current_queue_index = 0;
                }
                GameStage::SpawnOrderInit => {
                    let mut queue: Vec<_> = {
                        let entity_info = self.memory_blocks.entity_info.read();
                        entity_info
                            .items
                            .iter()
                            .map(|(&id, info)| {
                                let order = self
                                    .archetypes
                                    .get(&info.archetype_id)
                                    .and_then(|a| a.spawn_order.as_ref())
                                    .and_then(|s| s.order)
                                    .unwrap_or_default();
                                (id, info.archetype_id, order)
                            })
                            .collect()
                    };
                    queue.sort_by_key(|&(_, _, cb_order)| cb_order);
                    self.spawn_order_queue = queue;
                    self.current_queue_index = 0;
                    self.evaluated_spawn_queue.clear();
                    self.current_stage = GameStage::SpawnOrderLoop;
                }
                GameStage::SpawnOrderLoop => {
                    self.evaluated_spawn_queue = Vec::with_capacity(self.spawn_order_queue.len());

                    let memory_blocks = &self.memory_blocks;
                    let bpm_changes = &self.bpm_changes;
                    let time_scale_changes = &self.time_scale_changes;
                    let side_effects = &self.side_effects;

                    while self.current_queue_index < self.spawn_order_queue.len() {
                        let (entity_id, archetype_id, _) =
                            self.spawn_order_queue[self.current_queue_index];

                        let Some((vm, _)) =
                            self.cached_vms.get_mut(&archetype_id).and_then(|items| {
                                items.get_mut(&PlayEngineArchetypeCallbackType::SpawnOrder)
                            })
                        else {
                            warn!(
                                "Could not find cached VM for archetype {}, callback type {:?}",
                                *archetype_id,
                                PlayEngineArchetypeCallbackType::SpawnOrder
                            );
                            self.evaluated_spawn_queue
                                .push((entity_id, archetype_id, 0.0));
                            self.current_queue_index += 1;
                            continue;
                        };
                        let run_result = memory_blocks.with_initialization_access(|memory| {
                            let context = RuntimeContext {
                                current_entity: CurrentEntity {
                                    id: entity_id,
                                    archetype_id,
                                },
                                memory: &memory,
                                timing: &TimingInfo {
                                    bpm_changes,
                                    time_scale_changes,
                                },
                                side_effects,
                            };

                            vm.run(&context, max_steps)
                        });

                        match run_result {
                            RunResult::StepLimitReached => return SonorustPlayerState::Running,
                            RunResult::Paused => {
                                return SonorustPlayerState::Paused {
                                    entity_id,
                                    archetype_id,
                                    callback_type: PlayEngineArchetypeCallbackType::SpawnOrder,
                                    pc: vm.pc,
                                };
                            }
                            RunResult::Finished => {
                                let computed_order = vm.stack.pop().unwrap_or_else(|| {
                                    warn!("Stack was empty after executing spawnOrder for entity {}, archetype {}, run result: {:?}", *entity_id, *archetype_id, run_result);
                                    0.0
                                });

                                self.evaluated_spawn_queue.push((
                                    entity_id,
                                    archetype_id,
                                    computed_order,
                                ));

                                self.current_queue_index += 1;
                            }
                        }
                    }

                    self.evaluated_spawn_queue
                        .sort_by(|a, b| a.2.total_cmp(&b.2));

                    self.spawn_queue = self
                        .evaluated_spawn_queue
                        .drain(..)
                        .map(|(entity_id, archetype_id, _)| (entity_id, archetype_id))
                        .collect();

                    self.current_queue_index = 0;
                    self.current_stage = GameStage::AwaitingUpdate;
                }
                _ => break,
            }
        }

        SonorustPlayerState::FinishedFrame
    }

    fn update(&mut self, max_steps: usize) -> SonorustPlayerState {
        loop {
            match self.current_stage {
                GameStage::AwaitingStart
                | GameStage::PreprocessInit
                | GameStage::PreprocessLoop
                | GameStage::SpawnOrderInit
                | GameStage::SpawnOrderLoop => {
                    panic!("Called update while in start stages. This should not happen.");
                }
                GameStage::AwaitingUpdate => {
                    // Keep RuntimeUpdate memory block up-to-date
                    {
                        let mut runtime_update = self.memory_blocks.runtime_update.write();
                        // TODO: time
                        // runtime_update.time = time.elapsed_secs();
                        // runtime_update.delta_time = time.delta_secs();
                        // runtime_update.scaled_time = time.elapsed_secs();
                        runtime_update.touch_count = 0;
                        info!("time: {}", Utc::now());
                    }

                    self.current_stage = GameStage::SpawningInit;
                }
                GameStage::SpawningInit => {
                    info!("spawning init");
                    self.current_queue_index = 0;
                    self.current_stage = GameStage::SpawningLoop;
                }
                GameStage::SpawningLoop => {
                    info!("spawning loop");
                    let memory_blocks = &self.memory_blocks;
                    let bpm_changes = &self.bpm_changes;
                    let time_scale_changes = &self.time_scale_changes;
                    let side_effects = &self.side_effects;

                    info!("spawn queue len {}", self.spawn_queue.len());
                    while self.current_queue_index < self.spawn_queue.len() {
                        let (entity_id, archetype_id) = self.spawn_queue[self.current_queue_index];

                        let Some((vm, _)) =
                            self.cached_vms.get_mut(&archetype_id).and_then(|items| {
                                items.get_mut(&PlayEngineArchetypeCallbackType::ShouldSpawn)
                            })
                        else {
                            warn!(
                                "Could not find cached VM for {} {:?}, skipping shouldSpawn callback",
                                *archetype_id,
                                PlayEngineArchetypeCallbackType::ShouldSpawn
                            );
                            self.current_queue_index += 1;
                            continue;
                        };

                        let run_result = memory_blocks.with_initialization_access(|memory| {
                            let context = RuntimeContext {
                                current_entity: CurrentEntity {
                                    id: entity_id,
                                    archetype_id: archetype_id,
                                },
                                memory: &memory,
                                timing: &TimingInfo {
                                    bpm_changes,
                                    time_scale_changes,
                                },
                                side_effects,
                            };

                            vm.run(&context, max_steps)
                        });

                        match run_result {
                            RunResult::StepLimitReached => return SonorustPlayerState::Running,
                            RunResult::Paused => {
                                return SonorustPlayerState::Paused {
                                    entity_id,
                                    archetype_id,
                                    callback_type: PlayEngineArchetypeCallbackType::ShouldSpawn,
                                    pc: vm.pc,
                                };
                            }
                            RunResult::Finished => {
                                let should_spawn = vm.stack.pop().unwrap_or_default();
                                let should_spawn = should_spawn != 0.0 && !should_spawn.is_nan();
                                if should_spawn {
                                    self.current_queue_index += 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    let ready_to_spawn: Vec<_> = self
                        .spawn_queue
                        .drain(0..self.current_queue_index)
                        .collect();

                    info!("ready to spawn {}", ready_to_spawn.len());

                    for (to_be_spawned_entity_id, to_be_spawned_archetype_id) in ready_to_spawn {
                        self.spawn_entity(to_be_spawned_entity_id, to_be_spawned_archetype_id);
                    }
                    self.current_queue_index = 0;
                    self.current_stage = GameStage::InitializeInit;
                }
                GameStage::InitializeInit => {
                    self.current_queue_index = 0;
                    self.current_stage = GameStage::InitializeLoop;
                }
                GameStage::InitializeLoop => {
                    let bpm_changes = &self.bpm_changes;
                    let time_scale_changes = &self.time_scale_changes;
                    let side_effects = &self.side_effects;

                    while self.current_queue_index < self.initialize_queue.len() {
                        let (entity_id, archetype_id) =
                            self.initialize_queue[self.current_queue_index];
                        let Some((vm, _)) =
                            self.cached_vms.get_mut(&archetype_id).and_then(|items| {
                                items.get_mut(&PlayEngineArchetypeCallbackType::Initialize)
                            })
                        else {
                            warn!(
                                "Could not find cached VM for {} {:?}, skipping initialize callback",
                                *archetype_id,
                                PlayEngineArchetypeCallbackType::Initialize
                            );
                            self.current_queue_index += 1;
                            continue;
                        };

                        let run_result = self.memory_blocks.with_initialization_access(|memory| {
                            let context = RuntimeContext {
                                current_entity: CurrentEntity {
                                    id: entity_id,
                                    archetype_id,
                                },
                                memory: &memory,
                                timing: &TimingInfo {
                                    bpm_changes,
                                    time_scale_changes,
                                },
                                side_effects,
                            };

                            vm.run(&context, max_steps)
                        });

                        match run_result {
                            RunResult::StepLimitReached => return SonorustPlayerState::Running,
                            RunResult::Paused => {
                                return SonorustPlayerState::Paused {
                                    entity_id,
                                    archetype_id,
                                    callback_type: PlayEngineArchetypeCallbackType::Initialize,
                                    pc: vm.pc,
                                };
                            }
                            RunResult::Finished => {
                                self.current_queue_index += 1;
                            }
                        }
                    }

                    self.initialize_queue.clear();
                    self.current_queue_index = 0;
                    self.current_stage = GameStage::UpdateSequentialInit;
                }
                GameStage::UpdateSequentialInit => {
                    let mut queue: Vec<_> = {
                        let entity_info = self.memory_blocks.entity_info.read();
                        entity_info
                            .items
                            .iter()
                            .filter_map(|(&id, info)| {
                                if info.state != EntityState::Active {
                                    return None;
                                }

                                self.archetypes
                                    .get(&info.archetype_id)
                                    .and_then(|a| a.update_sequential.as_ref())
                                    .map(|s| (id, info.archetype_id, s.order.unwrap_or_default()))
                            })
                            .collect()
                    };

                    // Sort the queue by the order parameter
                    queue.sort_by_key(|&(_, _, order)| order);

                    self.sequential_update_queue = queue;
                    self.current_queue_index = 0;
                    self.current_stage = GameStage::UpdateSequentialLoop;
                }
                GameStage::UpdateSequentialLoop => {
                    let bpm_changes = &self.bpm_changes;
                    let time_scale_changes = &self.time_scale_changes;
                    let side_effects = &self.side_effects;

                    while self.current_queue_index < self.sequential_update_queue.len() {
                        let (entity_id, archetype_id, _) =
                            self.sequential_update_queue[self.current_queue_index];

                        let Some((vm, _)) =
                            self.cached_vms.get_mut(&archetype_id).and_then(|items| {
                                items.get_mut(&PlayEngineArchetypeCallbackType::UpdateSequential)
                            })
                        else {
                            warn!(
                                "Could not find cached VM for {} {:?}, skipping updateSequential callback",
                                *archetype_id,
                                PlayEngineArchetypeCallbackType::UpdateSequential
                            );
                            self.current_queue_index += 1;
                            continue;
                        };

                        let run_result = self.memory_blocks.with_sequential_access(|memory| {
                            let context = RuntimeContext {
                                current_entity: CurrentEntity {
                                    id: entity_id,
                                    archetype_id,
                                },
                                memory: &memory,
                                timing: &TimingInfo {
                                    bpm_changes,
                                    time_scale_changes,
                                },
                                side_effects,
                            };

                            vm.run(&context, max_steps)
                        });

                        match run_result {
                            RunResult::StepLimitReached => return SonorustPlayerState::Running,
                            RunResult::Paused => {
                                return SonorustPlayerState::Paused {
                                    entity_id,
                                    archetype_id,
                                    callback_type:
                                        PlayEngineArchetypeCallbackType::UpdateSequential,
                                    pc: vm.pc,
                                };
                            }
                            RunResult::Finished => {
                                self.current_queue_index += 1;
                            }
                        }
                    }

                    // Clean up and move to the next stage
                    self.sequential_update_queue.clear();
                    self.current_queue_index = 0;

                    // Replace with your next stage
                    self.current_stage = GameStage::Input;
                }
                GameStage::Input => {
                    // TODO
                    self.current_stage = GameStage::UpdateParallelInit;
                }
                GameStage::UpdateParallelInit => {
                    let queue: Vec<_> = {
                        let entity_info = self.memory_blocks.entity_info.read();
                        entity_info
                            .items
                            .iter()
                            .filter_map(|(&id, info)| {
                                (info.state == EntityState::Active)
                                    .then_some((id, info.archetype_id))
                            })
                            .collect()
                    };

                    self.parallel_update_queue = queue;
                    self.current_queue_index = 0;
                    self.current_stage = GameStage::UpdateParallelLoop;
                }
                GameStage::UpdateParallelLoop => {
                    let bpm_changes = &self.bpm_changes;
                    let time_scale_changes = &self.time_scale_changes;
                    let side_effects = &self.side_effects;

                    while self.current_queue_index < self.parallel_update_queue.len() {
                        let (entity_id, archetype_id) =
                            self.parallel_update_queue[self.current_queue_index];

                        let Some((vm, _)) =
                            self.cached_vms.get_mut(&archetype_id).and_then(|items| {
                                items.get_mut(&PlayEngineArchetypeCallbackType::UpdateParallel)
                            })
                        else {
                            self.current_queue_index += 1;
                            continue;
                        };

                        let run_result = self.memory_blocks.with_parallel_access(|memory| {
                            let context = RuntimeContext {
                                current_entity: CurrentEntity {
                                    id: entity_id,
                                    archetype_id,
                                },
                                memory: &memory,
                                timing: &TimingInfo {
                                    bpm_changes,
                                    time_scale_changes,
                                },
                                side_effects,
                            };

                            vm.run(&context, max_steps)
                        });

                        match run_result {
                            RunResult::StepLimitReached => return SonorustPlayerState::Running,
                            RunResult::Paused => {
                                return SonorustPlayerState::Paused {
                                    entity_id,
                                    archetype_id,
                                    callback_type: PlayEngineArchetypeCallbackType::UpdateParallel,
                                    pc: vm.pc,
                                };
                            }
                            RunResult::Finished => {
                                self.current_queue_index += 1;
                            }
                        }
                    }

                    self.parallel_update_queue.clear();
                    self.current_queue_index = 0;

                    self.current_stage = GameStage::DespawningInit;
                }
                GameStage::DespawningInit => {
                    let queue: Vec<_> = {
                        let entity_info = self.memory_blocks.entity_info.read();
                        let entity_despawn_items = &self.memory_blocks.entity_despawn.read().items;
                        entity_info
                            .items
                            .iter()
                            .filter_map(|(&id, info)| {
                                (info.state == EntityState::Active
                                    && entity_despawn_items.contains(&id))
                                .then_some((id, info.archetype_id))
                            })
                            .collect()
                    };

                    self.despawn_queue = queue;
                    self.current_queue_index = 0;
                    self.current_stage = GameStage::DespawningLoop;
                }
                GameStage::DespawningLoop => {
                    let bpm_changes = &self.bpm_changes;
                    let time_scale_changes = &self.time_scale_changes;
                    let side_effects = &self.side_effects;

                    while self.current_queue_index < self.despawn_queue.len() {
                        let (entity_id, archetype_id) =
                            self.despawn_queue[self.current_queue_index];

                        let Some((vm, _)) =
                            self.cached_vms.get_mut(&archetype_id).and_then(|items| {
                                items.get_mut(&PlayEngineArchetypeCallbackType::Terminate)
                            })
                        else {
                            self.current_queue_index += 1;
                            continue;
                        };

                        let run_result = self.memory_blocks.with_parallel_access(|memory| {
                            let context = RuntimeContext {
                                current_entity: CurrentEntity {
                                    id: entity_id,
                                    archetype_id,
                                },
                                memory: &memory,
                                timing: &TimingInfo {
                                    bpm_changes,
                                    time_scale_changes,
                                },
                                side_effects,
                            };

                            vm.run(&context, max_steps)
                        });

                        match run_result {
                            RunResult::StepLimitReached => return SonorustPlayerState::Running,
                            RunResult::Paused => {
                                return SonorustPlayerState::Paused {
                                    entity_id,
                                    archetype_id,
                                    callback_type: PlayEngineArchetypeCallbackType::Terminate,
                                    pc: vm.pc,
                                };
                            }
                            RunResult::Finished => {
                                let mut entity_info = self.memory_blocks.entity_info.write();
                                if let Some(ei) = entity_info.entry_mut(&entity_id) {
                                    ei.state = EntityState::Despawned;
                                }
                                self.memory_blocks.entity_despawn.write().remove(&entity_id);
                                self.current_queue_index += 1;
                            }
                        }
                    }

                    self.despawn_queue.clear();

                    self.current_queue_index = 0;
                    self.current_stage = GameStage::Presentation;
                }
                GameStage::Presentation => {
                    // TODO
                    self.current_stage = GameStage::FinishedFrame;
                }
                GameStage::FinishedFrame => break,
            }
        }

        SonorustPlayerState::FinishedFrame
    }

    fn spawn_entity(&mut self, entity_id: EntityId, archetype_id: ArchetypeId) {
        self.initialize_queue.push((entity_id, archetype_id));
        let mut a = self.memory_blocks.entity_info.write();
        let b = a
            .items
            .entry(entity_id)
            .and_modify(|entry| entry.state = EntityState::Active);
        info!("bruh {:?}", b);
    }

    fn is_in_start_phase(&self) -> bool {
        matches!(
            self.current_stage,
            GameStage::AwaitingStart
                | GameStage::PreprocessInit
                | GameStage::PreprocessLoop
                | GameStage::SpawnOrderInit
                | GameStage::SpawnOrderLoop
        )
    }
}

#[derive(Debug)]
struct BpmChange {
    bpm: IRValue,
    starting_beat: IRValue,
    starting_time: IRValue,
}

#[derive(Debug)]
struct TimeScaleChange {
    time_scale: IRValue,
    starting_scaled_time: IRValue,
    starting_time: IRValue,
}

enum SpecialArchetype {
    BpmChange { beat: IRValue, bpm: IRValue },
    TimescaleChange { beat: IRValue, timescale: IRValue },
}

impl TryFrom<LevelDataEntity> for SpecialArchetype {
    type Error = ();

    fn try_from(value: LevelDataEntity) -> Result<Self, Self::Error> {
        let mut data = value
            .data
            .into_iter()
            .map(|data| (data.name, data.payload))
            .collect::<HashMap<_, _>>();
        let beat = match data.remove("#BEAT") {
            Some(Some(LevelDataEntityDataPayload::Value { value })) => value,
            _ => return Err(()),
        };
        match &*value.archetype {
            "#BPM_CHANGE" => {
                let bpm = match data.remove("#BPM") {
                    Some(Some(LevelDataEntityDataPayload::Value { value })) => value,
                    _ => return Err(()),
                };
                Ok(SpecialArchetype::BpmChange { beat, bpm })
            }
            "#TIMESCALE_CHANGE" => {
                let timescale = match data.remove("#TIMESCALE") {
                    Some(Some(LevelDataEntityDataPayload::Value { value })) => value,
                    _ => return Err(()),
                };
                Ok(SpecialArchetype::TimescaleChange { beat, timescale })
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct BpmChanges {
    changes: Vec<BpmChange>,
}

impl BpmChanges {
    fn beat_to_last_change(&self, beat: IRValue) -> Option<&BpmChange> {
        self.changes
            .iter()
            .rev()
            .find(|change| change.starting_beat < beat)
    }

    pub fn beat_to_time(&self, beat: IRValue) -> IRValue {
        let Some(bpm_change) = self.beat_to_last_change(beat) else {
            return 0.0;
        };

        let mut time = bpm_change.starting_time;

        let remaining_beats = beat - bpm_change.starting_beat;
        let bps = bpm_change.bpm / 60.0;
        let remaining_seconds = remaining_beats / bps;
        time += remaining_seconds;

        time
    }

    pub fn beat_to_bpm(&self, beat: IRValue) -> IRValue {
        let Some(bpm_change) = self.beat_to_last_change(beat) else {
            return self
                .changes
                .first()
                .map(|change| change.bpm)
                .unwrap_or_default();
        };
        bpm_change.bpm
    }
}

#[derive(Debug)]
struct TimeScaleChanges {
    changes: Vec<TimeScaleChange>,
}

impl TimeScaleChanges {
    pub fn time_to_scaled_time(&self, time: IRValue) -> IRValue {
        let Some(last_change) = self
            .changes
            .iter()
            .rev()
            .find(|change| change.starting_time <= time)
        else {
            return 0.0;
        };

        let delta = time - last_change.starting_time;
        last_change.starting_scaled_time + delta * last_change.time_scale
    }
}

#[derive(Debug)]
struct TimingInfo<'a> {
    bpm_changes: &'a BpmChanges,
    time_scale_changes: &'a TimeScaleChanges,
}

impl<'a> TimingAccess for TimingInfo<'a> {
    fn beat_to_time(&self, beat: IRValue) -> IRValue {
        self.bpm_changes.beat_to_time(beat)
    }

    fn beat_to_bpm(&self, beat: IRValue) -> IRValue {
        self.bpm_changes.beat_to_bpm(beat)
    }

    fn time_to_scaled_time(&self, time: IRValue) -> IRValue {
        self.time_scale_changes.time_to_scaled_time(time)
    }
}

#[derive(Default)]
struct SideEffects {
    pub spawns: RwLock<Vec<SpawnSideEffect>>,
    pub draws: RwLock<Vec<DrawSideEffect>>,
}

impl SideEffectAccess for SideEffects {
    fn spawn(&self, spawn_side_effect: SpawnSideEffect) {
        self.spawns.write().push(spawn_side_effect);
    }

    fn draw(&self, draw_side_effect: DrawSideEffect) {
        self.draws.write().push(draw_side_effect)
    }
}

fn apply_sonolus_transform_expression(
    expression: &SkinSpriteTransformExpression,
    (input_x1, input_y1): (f32, f32),
    (input_x2, input_y2): (f32, f32),
    (input_x3, input_y3): (f32, f32),
    (input_x4, input_y4): (f32, f32),
) -> f32 {
    let mut output = 0.0;
    if let Some(x1_multiplier) = expression.x1 {
        output += input_x1 * x1_multiplier;
    }
    if let Some(y1_multiplier) = expression.y1 {
        output += input_y1 * y1_multiplier;
    }
    if let Some(x2_multiplier) = expression.x2 {
        output += input_x2 * x2_multiplier;
    }
    if let Some(y2_multiplier) = expression.y2 {
        output += input_y2 * y2_multiplier;
    }
    if let Some(x3_multiplier) = expression.x3 {
        output += input_x3 * x3_multiplier;
    }
    if let Some(y3_multiplier) = expression.y3 {
        output += input_y3 * y3_multiplier;
    }
    if let Some(x4_multiplier) = expression.x4 {
        output += input_x4 * x4_multiplier;
    }
    if let Some(y4_multiplier) = expression.y4 {
        output += input_y4 * y4_multiplier;
    }

    output
}
