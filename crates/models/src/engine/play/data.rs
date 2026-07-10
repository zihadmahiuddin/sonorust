use std::{error::Error, fmt::Display};

use sonorust_ir::{IRIndex, IRValue, nodes::*};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::engine::play::archetype::data::EnginePlayDataArchetype;

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EnginePlayData {
    pub skin: Skin,
    pub archetypes: Vec<EnginePlayDataArchetype>,
    pub nodes: Vec<Node>,
    pub buckets: Vec<Bucket>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase"),
    serde(untagged)
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum Node {
    Literal { value: IRValue },
    FunctionCall { func: String, args: Vec<usize> },
}

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum SkinRenderMode {
    #[default]
    Default,
    Standard,
    Lightweight,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Bucket {
    pub unit: Option<String>,
    pub sprites: Vec<BucketSprite>,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[allow(unused)]
pub struct BucketSprite {
    id: i64,
    fallback_id: Option<i64>,
    x: IRValue,
    y: IRValue,
    w: IRValue,
    h: IRValue,
    rotation: IRValue,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[allow(unused)]
pub struct Skin {
    render_mode: Option<SkinRenderMode>,
    pub sprites: Vec<Sprite>,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Sprite {
    pub name: String,
    pub id: usize,
}

macro_rules! match_opcodes {
    (
        $func:expr, $args:expr;
        unit: [$( $unit_op:ident ),* $(,)?],
        vec_args: [$( $va_op:ident ),* $(,)?],
        first_mid_last: [$( $fml_op:ident { $first:ident, $( $middle:ident ),+; $last:ident $(,)? } ),* $(,)?],
        fields: [$( $field_op:ident { $( $field:ident ),* $(,)? } ),* $(,)?],
        custom: { $( $custom_op:ident => $block:expr ),* $(,)? }
    ) => {
        match $func {
            $( stringify!($unit_op) => OpCode::$unit_op($unit_op), )*
            $( stringify!($va_op) => OpCode::$va_op($va_op { args: $args.iter().map(|x| IRIndex::from(*x)).collect() }), )*
            $(
                stringify!($fml_op) => {
                    let items: Vec<IRIndex> = $args.iter().map(|x| IRIndex::from(*x)).collect();
                    let (first, rest) = items.split_first().ok_or_else(|| {
                        JsonNodeToIRNodeError::MissingArgument { fn_name: String::from(stringify!($fml_op)) }
                    })?;
                    let (last, middle) = rest.split_last().ok_or_else(|| {
                        JsonNodeToIRNodeError::MissingArgument { fn_name: String::from(stringify!($fml_op)) }
                    })?;
                    OpCode::$fml_op($fml_op {
                        first: first.clone(),
                        middle: middle.to_vec(),
                        last: last.clone(),
                    })
                }
            )*
            $(
                stringify!($field_op) => {
                    // for structs with 0 fields (e.g. DebugPause)
                    #[allow(unused_mut, unused_variables)]
                    let mut it = $args.into_iter();
                    OpCode::$field_op($field_op {
                        $(
                            $field: it.next().copied().map(|x| IRIndex::from(x)).ok_or_else(|| {
                                JsonNodeToIRNodeError::MissingArgument { fn_name: String::from(stringify!($field_op)) }
                            })?,
                        )*
                    })
                }
            )*
            $(
                stringify!($custom_op) => {
                    let args = $args;
                    OpCode::$custom_op($block(args)?)
                }
            )*
            _ => return Err(JsonNodeToIRNodeError::UnknownFunction { fn_name: String::from($func) }),
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum JsonNodeToIRNodeError {
    MissingArgument { fn_name: String },
    UnknownFunction { fn_name: String },
}

impl Display for JsonNodeToIRNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonNodeToIRNodeError::MissingArgument { fn_name } => {
                write!(f, "Missing argument for {fn_name}")
            }
            JsonNodeToIRNodeError::UnknownFunction { fn_name } => {
                write!(f, "Unknown function {fn_name}")
            }
        }
    }
}

impl Error for JsonNodeToIRNodeError {}

impl TryFrom<&Node> for IRNode {
    type Error = JsonNodeToIRNodeError;

    fn try_from(value: &Node) -> Result<Self, Self::Error> {
        match value {
            Node::Literal { value } => Ok(IRNode::Value(IRValue::from(*value))),
            Node::FunctionCall { func, args } => {
                let op = match_opcodes!(
                    func.as_str(), args;

                    unit: [
                        // StackGetFramePointer, StackGetPointer, StackInit, StackLeave, StackPop, DebugPause,
                    ],

                    vec_args: [
                        Add, Subtract, Multiply, Divide, Mod, Rem, Power, And, Or, Execute, Execute0,
                    ],

                    first_mid_last: [JumpLoop { first, middle; last }],

                    fields: [
                        // Control Flow
                        Block { body },
                        Break { count, value },
                        If { test, consequent, alternate },
                        While { test, body },
                        // DoWhile { body, test },

                        // Math Basic & Trigonometry
                        Abs { value }, Frac { value }, Trunc { value }, Negate { value },
                        Clamp { min, max, value }, Lerp { min, max, value }, LerpClamped { min, max, value },
                        Unlerp { min, max, value }, UnlerpClamped { min, max, value }, Min { x, y }, Max { x, y },
                        Remap { from_min, from_max, to_min, to_max, value },
                        RemapClamped { from_min, from_max, to_min, to_max, value },
                        Round { value }, Floor { value }, Ceil { value },
                        Sin { value }, Sinh { value }, Cos { value }, Cosh { value }, Tan { value }, Tanh { value },
                        Arccos { value }, Arcsin { value }, Arctan { value }, Arctan2 { x, y },
                        Degree { value }, Radian { value }, Log { value }, Sign { value },
                        Random { min, max }, RandomInteger { min, max },

                        // Easing Functions
                        // EaseInSine { value }, EaseOutSine { value }, EaseInOutSine { value }, EaseOutInSine { value },
                        // EaseInQuad { value }, EaseOutQuad { value }, EaseInOutQuad { value }, EaseOutInQuad { value },
                        // EaseInCubic { value }, EaseOutCubic { value }, // EaseInOutCubic { value }, EaseOutInCubic { value },
                        // EaseInQuart { value }, EaseOutQuart { value }, EaseInOutQuart { value }, EaseOutInQuart { value },
                        // EaseInQuint { value }, EaseOutQuint { value }, EaseInOutQuint { value }, EaseOutInQuint { value },
                        // EaseInExpo { value }, EaseOutExpo { value }, EaseInOutExpo { value }, EaseOutInExpo { value },
                        // EaseInCirc { value }, EaseOutCirc { value }, EaseInOutCirc { value }, EaseOutInCirc { value },
                        // EaseInBack { value }, EaseOutBack { value }, EaseInOutBack { value }, EaseOutInBack { value },
                        // EaseInElastic { value }, EaseOutElastic { value }, EaseInOutElastic { value }, EaseOutInElastic { value },

                        // Logical
                        Equal { lhs, rhs }, NotEqual { lhs, rhs }, Greater { lhs, rhs }, GreaterOr { lhs, rhs },
                        Less { lhs, rhs }, LessOr { lhs, rhs }, Not { value },

                        // Memory Operations
                        Get { block_id, index },
                        GetPointed { block_id, index, offset },
                        GetShifted { block_id, x, y, s },
                        Set { block_id, index, value },
                        SetPointed { block_id, index, offset, value },
                        SetShifted { block_id, x, y, s, value },
                        SetAdd { block_id, index, value },
                        SetAddPointed { block_id, index, offset, value },
                        SetAddShifted { block_id, x, y, s, value },
                        SetSubtract { block_id, index, value },
                        SetSubtractPointed { block_id, index, offset, value },
                        SetSubtractShifted { block_id, x, y, s, value },
                        SetMultiply { block_id, index, value },
                        SetMultiplyPointed { block_id, index, offset, value },
                        SetMultiplyShifted { block_id, x, y, s, value },
                        SetDivide { block_id, index, value },
                        SetDividePointed { block_id, index, offset, value },
                        SetDivideShifted { block_id, x, y, s, value },
                        SetMod { block_id, index, value },
                        SetModPointed { block_id, index, offset, value },
                        SetModShifted { block_id, x, y, s, value },
                        SetRem { block_id, index, value },
                        SetRemPointed { block_id, index, offset, value },
                        SetRemShifted { block_id, x, y, s, value },
                        SetPower { block_id, index, value },
                        SetPowerPointed { block_id, index, offset, value },
                        SetPowerShifted { block_id, x, y, s, value },
                        Copy { src_block_id, src_index, dst_block_id, dst_index, count },
                        // DecrementPost { block_id, index },
                        // DecrementPostPointed { block_id, index, offset },
                        // DecrementPostShifted { block_id, x, y, s },
                        // DecrementPre { block_id, index },
                        // DecrementPrePointed { block_id, index, offset },
                        // DecrementPreShifted { block_id, x, y, s },
                        // IncrementPost { block_id, index },
                        // IncrementPostPointed { block_id, index, offset },
                        // IncrementPostShifted { block_id, x, y, s },
                        // IncrementPre { block_id, index },
                        // IncrementPrePointed { block_id, index, offset },
                        // IncrementPreShifted { block_id, x, y, s },

                        // Stack
                        // StackEnter { size }, StackGet { offset }, StackGetFrame { offset }, StackGrow { size },
                        // StackPush { value }, StackSet { offset, value }, StackSetFrame { offset, value },
                        // StackSetFramePointer { value }, StackSetPointer { value },

                        // Drawing
                        // Draw { sprite_id, x1, y1, x2, y2, x3, y3, x4, y4, z, alpha },
                        // DrawCurvedB { sprite_id, x1, y1, x2, y2, x3, y3, x4, y4, z, alpha, n, p, q },
                        // DrawCurvedBT { sprite_id, x1, y1, x2, y2, x3, y3, x4, y4, z, alpha, n, p1, q1, p2, q2 },
                        // DrawCurvedL { sprite_id, x1, y1, x2, y2, x3, y3, x4, y4, z, alpha, n, p, q },
                        // DrawCurvedLR { sprite_id, x1, y1, x2, y2, x3, y3, x4, y4, z, alpha, n, p1, q1, p2, q2 },
                        // DrawCurvedR { sprite_id, x1, y1, x2, y2, x3, y3, x4, y4, z, alpha, n, p, q },
                        // DrawCurvedT { sprite_id, x1, y1, x2, y2, x3, y3, x4, y4, z, alpha, n, p, q },
                        // Paint { icon_id, x, y, size, rotation, z, alpha },
                        // Print { value, format, decimal_places, anchor_x, anchor_y, pivot_x, pivot_y, width, height, rotation, color, alpha, horizontal_align, background },

                        // Timing
                        // BeatToStartingBeat { beat }, BeatToStartingTime { beat },
                        BeatToTime { beat },
                        // BeatToBPM { beat },
                        // TimeToScaledTime { time }, // TimeToStartingScaledTime { time }, TimeToStartingTime { time }, TimeToTimeScale { time },

                        // Debug & Audio
                        DebugLog { value }, DebugPause { },
                        HasEffectClip { id },
                        Play { id, distance }, PlayLooped { id }, PlayLoopedScheduled { id, time }, PlayScheduled { id, time, distance },
                        StopLooped { id }, StopLoopedScheduled { id, time },

                        // Particle & Skin
                        HasParticleEffect { id },
                        DestroyParticleEffect { id },
                        // MoveParticleEffect { id, x1, y1, x2, y2, x3, y3, x4, y4 },
                        SpawnParticleEffect { id, x1, y1, x2, y2, x3, y3, x4, y4, duration, is_looped },
                        // HasSkinSprite { id },

                        // Gameplay & Replay
                        Judge { source, target, min_perfect, max_perfect, min_great, max_great, min_good, max_good },
                        // JudgeSimple { source, target, max_perfect, max_great, max_good },
                        ExportValue { index, value },
                        // StreamGetNextKey { id, key }, StreamGetPreviousKey { id, key },
                        // StreamGetValue { id, key }, StreamHas { id, key }
                        StreamSet { id, key, value },
                    ],

                    custom: {
                        Draw => |args: &[usize]| -> Result<Draw, Self::Error> {
                            // Draw { sprite_id, x1, y1, x2, y2, x3, y3, x4, y4, z, alpha }
                            let mut it = args.iter().map(|x| IRIndex::from(*x));
                            Ok(Draw {
                                sprite_id: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                x1: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                y1: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                x2: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                y2: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                x3: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                y3: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                x4: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                y4: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                z1: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                alpha: it.next().ok_or_else(|| JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Draw") })?,
                                z2: it.next(),
                                z3: it.next(),
                                z4: it.next(),
                            })
                        },
                        Spawn => |args: &[usize]| -> Result<Spawn, Self::Error> {
                            Ok(Spawn {
                                archetype_id: args[0].into(),
                                data: args[1..].iter().map(|x| IRIndex::from(*x)).collect(),
                            })
                        },
                        Switch => |args: &[usize]| -> Result<Switch, Self::Error> {
                            if args.is_empty() { return Err(JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("Switch") }); };
                            Ok(Switch {
                                discriminant: args[0].into(),
                                tests_and_consequents: args[1..].iter().map(|x| IRIndex::from(*x)).collect(),
                            })
                        },
                        SwitchWithDefault => |args: &[usize]| -> Result<SwitchWithDefault, Self::Error> {
                            if args.len() < 2 { return Err(JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("SwitchWithDefault") }); };
                            Ok(SwitchWithDefault {
                                discriminant: args[0].into(),
                                default_consequent: IRIndex::from(*args.last().unwrap()),
                                tests_and_consequents: args[1..args.len() - 1].iter().map(|x| IRIndex::from(*x)).collect()
                            })
                        },
                        SwitchInteger => |args: &[usize]| -> Result<SwitchInteger, Self::Error> {
                            if args.is_empty() { return Err(JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("SwitchInteger") }); };
                            Ok(SwitchInteger {
                                discriminant: args[0].into(),
                                consequents: args[1..].iter().map(|x| IRIndex::from(*x)).collect(),
                            })
                        },
                        SwitchIntegerWithDefault => |args: &[usize]| -> Result<SwitchIntegerWithDefault, Self::Error> {
                            if args.len() < 2 { return Err(JsonNodeToIRNodeError::MissingArgument { fn_name: String::from("SwitchIntegerWithDefault") }); };
                            Ok(SwitchIntegerWithDefault {
                                discriminant: args[0].into(),
                                default_consequent: IRIndex::from(*args.last().unwrap()),
                                consequents: args[1..args.len() - 1].iter().map(|x| IRIndex::from(*x)).collect()
                            })
                        },
                    }
                );

                Ok(IRNode::OpCode(op))
            }
        }
    }
}
