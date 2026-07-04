#![feature(nonpoison_rwlock)]
#![feature(sync_nonpoison)]

use std::{
    collections::HashMap,
    sync::{Arc, nonpoison::RwLock},
};

use glam::{Mat4, Vec2};
use serde::{Deserialize, Serialize};
use sonorust_bytecode::{
    compiler::{CompilationResult, Compiler},
    instruction::{InstructionKind, InstructionOperand, UsedStackIndices},
    ir_builder::IRBuilder,
    ir_parser::parse_script,
    vm::{VM, VMState},
};
use sonorust_ir::IRValue;
use sonorust_models::{
    blocks::{
        common::TemporaryMemory,
        play::{
            archetype::{life::PlayArchetypeLife, score::PlayArchetypeScore},
            engine_rom::PlayEngineRom,
            entity::{
                data::PlayEntityDataArray, despawn::PlayEntityDespawn, info::PlayEntityInfoArray,
                input::PlayEntityInput, life::PlayEntityLife, memory::PlayEntityMemoryArray,
                score::PlayEntityScore, shared_memory::PlayEntitySharedMemoryArray,
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
    engine::common::archetype::callbacks::EngineArchetypeCallbackType,
    ids::{ArchetypeId, EntityId},
};
use sonorust_runtime::{
    context::{CurrentEntity, RuntimeContext},
    memory::play::MemoryBlocks,
};
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

#[wasm_bindgen]
#[allow(dead_code)]
struct VMHandle {
    inner: VM,
    memory_blocks: Arc<MemoryBlocks>,
    callback_type: EngineArchetypeCallbackType,
    on_change: Option<js_sys::Function>,
}

#[wasm_bindgen]
#[allow(dead_code)]
impl VMHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(callback_type: EngineArchetypeCallbackType) -> Self {
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
        let level_option = PlayLevelOption::new(&[]);
        let level_bucket = PlayLevelBucket::new(0);
        let level_score = PlayLevelScore::default();
        let level_life = PlayLevelLife::default();

        let engine_rom = PlayEngineRom(Box::new([]));

        let entity_memory = PlayEntityMemoryArray::new(std::iter::empty());
        let entity_shared_memory = PlayEntitySharedMemoryArray::new(std::iter::empty());
        let entity_despawn = PlayEntityDespawn::default();
        let entity_input = PlayEntityInput::new(std::iter::empty());
        let entity_score = PlayEntityScore::new(std::iter::empty());
        let entity_life = PlayEntityLife::new(std::iter::empty());

        let archetype_score = PlayArchetypeScore::new(std::iter::empty());
        let archetype_life = PlayArchetypeLife::new(std::iter::empty());

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
            entity_data: RwLock::new(PlayEntityDataArray::new(std::iter::empty())),
            entity_shared_memory: RwLock::new(entity_shared_memory),
            entity_info: RwLock::new(PlayEntityInfoArray::new(std::iter::empty())),
            entity_despawn: RwLock::new(entity_despawn),
            entity_input: RwLock::new(entity_input),
            entity_score: RwLock::new(entity_score),
            entity_life: RwLock::new(entity_life),
            archetype_score: RwLock::new(archetype_score),
            archetype_life: RwLock::new(archetype_life),
            temporary_memory: RwLock::new(temporary_memory),
        };

        Self {
            inner: VM::new(),
            memory_blocks: Arc::new(memory_blocks),
            on_change: None,
            callback_type,
        }
    }

    pub fn subscribe(&mut self, cb: js_sys::Function) {
        self.on_change = Some(cb);
    }

    pub fn unsubscribe(&mut self) {
        self.on_change = None;
    }

    fn notify_change(&self, mask: u32) {
        if let Some(ref cb) = self.on_change {
            let _ = cb.call1(&JsValue::NULL, &JsValue::from(mask));
        }
    }

    #[wasm_bindgen(js_name = "compileAndLoadScript")]
    pub fn compile_and_load_script(
        &mut self,
        script: &str,
    ) -> Result<Ts<CompilationResult>, JsError> {
        let (_, statements) =
            parse_script(script).map_err(|e| JsError::new(&format!("Parse error: {}", e)))?;
        let mut ir_builder = IRBuilder::new();
        for statement in statements {
            ir_builder.lower_statement(statement);
        }
        let (root_index, ir_nodes) = ir_builder.finish();
        let mut compiler = Compiler::new(&ir_nodes);
        compiler.compile_node(root_index);
        let compilation_result = compiler.finish();
        self.inner.stop();
        self.inner.load_bytecode(&compilation_result.instructions);
        self.notify_change(VMChanges::pc() | VMChanges::stack() | VMChanges::state());
        Ok(compilation_result.into_ts()?)
    }

    pub fn resume(&mut self) {
        self.inner.resume();
        self.notify_change(VMChanges::state());
    }

    pub fn run(&mut self, entity_id: usize, archetype_id: usize, max_steps: usize) {
        let memory_blocks = Arc::clone(&self.memory_blocks);

        memory_blocks.with_memory_for(self.callback_type, |memory| {
            let mut runtime_context = RuntimeContext {
                memory,
                current_entity: CurrentEntity {
                    id: EntityId(entity_id),
                    archetype_id: ArchetypeId(archetype_id),
                },
            };
            self.inner.run(&mut runtime_context, max_steps);
        });
        self.notify_change(VMChanges::pc() | VMChanges::stack() | VMChanges::state());
    }

    pub fn pause(&mut self) {
        self.inner.pause();
        self.notify_change(VMChanges::state());
    }

    pub fn step(&mut self, entity_id: usize, archetype_id: usize) {
        let memory_blocks = Arc::clone(&self.memory_blocks);

        memory_blocks.with_memory_for(self.callback_type, |memory| {
            let mut runtime_context = RuntimeContext {
                memory,
                current_entity: CurrentEntity {
                    id: EntityId(entity_id),
                    archetype_id: ArchetypeId(archetype_id),
                },
            };
            self.inner.step(&mut runtime_context);
        });
        self.notify_change(VMChanges::pc() | VMChanges::stack() | VMChanges::state());
    }

    pub fn stop(&mut self) {
        self.inner.stop();
        self.notify_change(VMChanges::pc() | VMChanges::stack() | VMChanges::state());
    }

    #[wasm_bindgen(js_name = "toggleBreakpoint")]
    pub fn toggle_breakpoint(&mut self, index: usize) {
        self.inner.toggle_breakpoint(index);
        self.notify_change(VMChanges::breakpoints());
    }

    #[wasm_bindgen(js_name = "getInstUsedStackIndices")]
    pub fn get_inst_used_stack_indices(
        &self,
        index: usize,
    ) -> Result<Ts<UsedStackIndices>, JsError> {
        Ok(self
            .inner
            .instructions
            .get(index)
            .map(|inst| inst.used_stack_indices())
            .unwrap_or_else(|| UsedStackIndices(Default::default()))
            .into_ts()?)
    }

    #[wasm_bindgen(js_name = "getInstMnemonic")]
    pub fn get_inst_mnemonic(&self, index: usize) -> String {
        self.inner.instructions[index].mnemonic().to_owned()
    }

    #[wasm_bindgen(js_name = "getInstOperands")]
    pub fn get_inst_operands(&self, index: usize) -> Result<Vec<Ts<InstructionOperand>>, JsError> {
        Ok(self.inner.instructions[index]
            .operands()
            .into_iter()
            .map(|x| x.into_ts())
            .collect::<Result<Vec<_>, _>>()?)
    }

    #[wasm_bindgen(js_name = "getInstKind")]
    pub fn get_inst_kind(&self, index: usize) -> InstructionKind {
        self.inner.instructions[index].kind()
    }

    #[wasm_bindgen(js_name = "getOperandAsString")]
    pub fn get_operand_as_string(
        &self,
        operand: Ts<InstructionOperand>,
    ) -> Result<String, JsError> {
        Ok(operand.to_rust()?.to_string())
    }

    pub fn disassemble(&self, index: usize) -> String {
        format!("{}", self.inner.instructions[index])
    }

    #[wasm_bindgen(getter)]
    pub fn breakpoints(&self) -> Vec<usize> {
        self.inner.breakpoints.iter().copied().collect()
    }

    #[wasm_bindgen(js_name = "readMemoryRange")]
    pub fn read_memory_range(&self, block_id: u64, start: usize, count: usize) -> Vec<IRValue> {
        self.memory_blocks
            .read_range_from_block(block_id, start..start + count)
    }

    #[wasm_bindgen(getter, js_name = "memoryBlockSizes")]
    pub fn memory_block_sizes(&self) -> Result<Ts<MemoryBlockSizes>, JsError> {
        Ok(MemoryBlockSizes(self.memory_blocks.get_memory_block_sizes()).into_ts()?)
    }

    #[wasm_bindgen(getter)]
    pub fn pc(&self) -> usize {
        self.inner.pc
    }

    #[wasm_bindgen(getter)]
    pub fn stack(&self) -> Vec<IRValue> {
        self.inner.stack.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> VMState {
        self.inner.state
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Tsify)]
pub struct MemoryBlockSizes(pub HashMap<u64, usize>);

pub type VMChange = u32;

#[wasm_bindgen]
pub struct VMChanges;

#[wasm_bindgen]
impl VMChanges {
    pub fn breakpoints() -> VMChange {
        1 << 0
    }
    pub fn memory_block_sizes() -> VMChange {
        1 << 1
    }
    pub fn pc() -> VMChange {
        1 << 2
    }
    pub fn stack() -> VMChange {
        1 << 3
    }
    pub fn state() -> VMChange {
        1 << 4
    }
}
