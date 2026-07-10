use crate::{IRIndex, IRValue};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum IRNode {
    Value(IRValue),
    OpCode(OpCode),
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum OpCode {
    // Control Flow
    Execute(Execute),
    Execute0(Execute0),
    Block(Block),
    Break(Break),
    If(If),
    While(While),
    Switch(Switch),
    SwitchWithDefault(SwitchWithDefault),
    SwitchInteger(SwitchInteger),
    SwitchIntegerWithDefault(SwitchIntegerWithDefault),
    JumpLoop(JumpLoop),

    // Math
    Abs(Abs),
    Frac(Frac),
    Trunc(Trunc),
    Negate(Negate),
    Add(Add),
    Subtract(Subtract),
    Multiply(Multiply),
    Divide(Divide),
    Mod(Mod),
    Rem(Rem),
    Power(Power),
    Clamp(Clamp),
    Lerp(Lerp),
    LerpClamped(LerpClamped),
    Unlerp(Unlerp),
    UnlerpClamped(UnlerpClamped),
    Min(Min),
    Max(Max),
    Remap(Remap),
    RemapClamped(RemapClamped),
    Round(Round),
    Floor(Floor),
    Ceil(Ceil),
    Sin(Sin),
    Sinh(Sinh),
    Cos(Cos),
    Cosh(Cosh),
    Tan(Tan),
    Tanh(Tanh),
    Arcsin(Arcsin),
    Arccos(Arccos),
    Arctan(Arctan),
    Arctan2(Arctan2),
    Degree(Degree),
    Radian(Radian),
    Log(Log),
    Sign(Sign),
    Random(Random),
    RandomInteger(RandomInteger),

    // Logical
    Equal(Equal),
    NotEqual(NotEqual),
    Greater(Greater),
    GreaterOr(GreaterOr),
    Less(Less),
    LessOr(LessOr),
    And(And),
    Or(Or),
    Not(Not),

    // Memory
    Get(Get),
    GetPointed(GetPointed),
    GetShifted(GetShifted),
    Set(Set),
    SetPointed(SetPointed),
    SetShifted(SetShifted),
    SetAdd(SetAdd),
    SetAddPointed(SetAddPointed),
    SetAddShifted(SetAddShifted),
    SetSubtract(SetSubtract),
    SetSubtractPointed(SetSubtractPointed),
    SetSubtractShifted(SetSubtractShifted),
    SetMultiply(SetMultiply),
    SetMultiplyPointed(SetMultiplyPointed),
    SetMultiplyShifted(SetMultiplyShifted),
    SetDivide(SetDivide),
    SetDividePointed(SetDividePointed),
    SetDivideShifted(SetDivideShifted),
    SetMod(SetMod),
    SetModPointed(SetModPointed),
    SetModShifted(SetModShifted),
    SetPower(SetPower),
    SetPowerPointed(SetPowerPointed),
    SetPowerShifted(SetPowerShifted),
    SetRem(SetRem),
    SetRemPointed(SetRemPointed),
    SetRemShifted(SetRemShifted),
    Copy(Copy),

    // Spawning
    Spawn(Spawn),

    // Drawing
    Draw(Draw),

    // Timing
    BeatToTime(BeatToTime),

    // Debug
    DebugLog(DebugLog),
    DebugPause(DebugPause),

    // Audio
    HasEffectClip(HasEffectClip),
    Play(Play),
    PlayLooped(PlayLooped),
    PlayLoopedScheduled(PlayLoopedScheduled),
    PlayScheduled(PlayScheduled),
    StopLooped(StopLooped),
    StopLoopedScheduled(StopLoopedScheduled),

    // Particle
    DestroyParticleEffect(DestroyParticleEffect),
    HasParticleEffect(HasParticleEffect),
    SpawnParticleEffect(SpawnParticleEffect),

    // Gameplay
    Judge(Judge),

    // Replay
    ExportValue(ExportValue),
    StreamSet(StreamSet),
}

// Control Flow
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Execute {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Execute0 {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Block {
    pub body: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Break {
    pub count: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct If {
    pub test: IRIndex,
    pub consequent: IRIndex,
    pub alternate: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct While {
    pub test: IRIndex,
    pub body: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DoWhile {
    pub body: IRIndex,
    pub test: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct JumpLoop {
    pub first: IRIndex,
    pub middle: Vec<IRIndex>,
    pub last: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Switch {
    pub discriminant: IRIndex,
    pub tests_and_consequents: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SwitchWithDefault {
    pub discriminant: IRIndex,
    pub tests_and_consequents: Vec<IRIndex>,
    pub default_consequent: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SwitchInteger {
    pub discriminant: IRIndex,
    pub consequents: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SwitchIntegerWithDefault {
    pub discriminant: IRIndex,
    pub consequents: Vec<IRIndex>,
    pub default_consequent: IRIndex,
}

// Math
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Abs {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Frac {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Trunc {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Negate {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Add {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Subtract {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Multiply {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Divide {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Mod {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Rem {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Power {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Clamp {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Lerp {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct LerpClamped {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Unlerp {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct UnlerpClamped {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Min {
    pub x: IRIndex,
    pub y: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Max {
    pub x: IRIndex,
    pub y: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Remap {
    pub from_min: IRIndex,
    pub from_max: IRIndex,
    pub to_min: IRIndex,
    pub to_max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct RemapClamped {
    pub from_min: IRIndex,
    pub from_max: IRIndex,
    pub to_min: IRIndex,
    pub to_max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Round {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Floor {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Ceil {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Sin {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Sinh {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Cos {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Cosh {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Tan {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Tanh {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Arccos {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Arcsin {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Arctan {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Arctan2 {
    pub x: IRIndex,
    pub y: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Degree {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Radian {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Log {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Sign {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Random {
    pub min: IRIndex,
    pub max: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct RandomInteger {
    pub min: IRIndex,
    pub max: IRIndex,
}

// Easing
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInSine {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutSine {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInOutSine {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutInSine {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInQuad {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutQuad {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInOutQuad {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutInQuad {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInCubic {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutCubic {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInOutCubic {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutInCubic {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInQuart {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutQuart {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInOutQuart {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutInQuart {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInQuint {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutQuint {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInOutQuint {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutInQuint {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInExpo {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutExpo {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInOutExpo {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutInExpo {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInCirc {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutCirc {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInOutCirc {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutInCirc {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInBack {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutBack {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInOutBack {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutInBack {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInElastic {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutElastic {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseInOutElastic {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct EaseOutInElastic {
    pub value: IRIndex,
}

// Logical
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Equal {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct NotEqual {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Greater {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct GreaterOr {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Less {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct LessOr {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct And {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Or {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Not {
    pub value: IRIndex,
}

// Memory
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Get {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct GetPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct GetShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Set {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetAdd {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetAddPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetAddShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetSubtract {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetSubtractPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetSubtractShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetMultiply {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetMultiplyPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetMultiplyShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetDivide {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetDividePointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetDivideShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetMod {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetModPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetModShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetRem {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetRemPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetRemShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetPower {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetPowerPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SetPowerShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Copy {
    pub src_block_id: IRIndex,
    pub src_index: IRIndex,
    pub dst_block_id: IRIndex,
    pub dst_index: IRIndex,
    pub count: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DecrementPost {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DecrementPostPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DecrementPostShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DecrementPre {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DecrementPrePointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DecrementPreShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IncrementPost {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IncrementPostPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IncrementPostShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IncrementPre {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IncrementPrePointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IncrementPreShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}

// Stack
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackEnter {
    pub size: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackGet {
    pub offset: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackGetFrame {
    pub offset: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackGetFramePointer;
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackGetPointer;
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackGrow {
    pub size: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackInit;
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackLeave;
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackPop;
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackPush {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackSet {
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackSetFrame {
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackSetFramePointer {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StackSetPointer {
    pub value: IRIndex,
}

// Spawning
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Spawn {
    pub archetype_id: IRIndex,
    pub data: Vec<IRIndex>,
}

// Drawing
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Draw {
    pub sprite_id: IRIndex,
    pub x1: IRIndex,
    pub y1: IRIndex,
    pub x2: IRIndex,
    pub y2: IRIndex,
    pub x3: IRIndex,
    pub y3: IRIndex,
    pub x4: IRIndex,
    pub y4: IRIndex,
    pub alpha: IRIndex,
    pub z1: IRIndex,
    pub z2: Option<IRIndex>,
    pub z3: Option<IRIndex>,
    pub z4: Option<IRIndex>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DrawCurvedB {
    pub sprite_id: IRIndex,
    pub x1: IRIndex,
    pub y1: IRIndex,
    pub x2: IRIndex,
    pub y2: IRIndex,
    pub x3: IRIndex,
    pub y3: IRIndex,
    pub x4: IRIndex,
    pub y4: IRIndex,
    pub z: IRIndex,
    pub alpha: IRIndex,
    pub n: IRIndex,
    pub p: IRIndex,
    pub q: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DrawCurvedBT {
    pub sprite_id: IRIndex,
    pub x1: IRIndex,
    pub y1: IRIndex,
    pub x2: IRIndex,
    pub y2: IRIndex,
    pub x3: IRIndex,
    pub y3: IRIndex,
    pub x4: IRIndex,
    pub y4: IRIndex,
    pub z: IRIndex,
    pub alpha: IRIndex,
    pub n: IRIndex,
    pub p1: IRIndex,
    pub q1: IRIndex,
    pub p2: IRIndex,
    pub q2: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DrawCurvedL {
    pub sprite_id: IRIndex,
    pub x1: IRIndex,
    pub y1: IRIndex,
    pub x2: IRIndex,
    pub y2: IRIndex,
    pub x3: IRIndex,
    pub y3: IRIndex,
    pub x4: IRIndex,
    pub y4: IRIndex,
    pub z: IRIndex,
    pub alpha: IRIndex,
    pub n: IRIndex,
    pub p: IRIndex,
    pub q: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DrawCurvedLR {
    pub sprite_id: IRIndex,
    pub x1: IRIndex,
    pub y1: IRIndex,
    pub x2: IRIndex,
    pub y2: IRIndex,
    pub x3: IRIndex,
    pub y3: IRIndex,
    pub x4: IRIndex,
    pub y4: IRIndex,
    pub z: IRIndex,
    pub alpha: IRIndex,
    pub n: IRIndex,
    pub p1: IRIndex,
    pub q1: IRIndex,
    pub p2: IRIndex,
    pub q2: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DrawCurvedR {
    pub sprite_id: IRIndex,
    pub x1: IRIndex,
    pub y1: IRIndex,
    pub x2: IRIndex,
    pub y2: IRIndex,
    pub x3: IRIndex,
    pub y3: IRIndex,
    pub x4: IRIndex,
    pub y4: IRIndex,
    pub z: IRIndex,
    pub alpha: IRIndex,
    pub n: IRIndex,
    pub p: IRIndex,
    pub q: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DrawCurvedT {
    pub sprite_id: IRIndex,
    pub x1: IRIndex,
    pub y1: IRIndex,
    pub x2: IRIndex,
    pub y2: IRIndex,
    pub x3: IRIndex,
    pub y3: IRIndex,
    pub x4: IRIndex,
    pub y4: IRIndex,
    pub z: IRIndex,
    pub alpha: IRIndex,
    pub n: IRIndex,
    pub p: IRIndex,
    pub q: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Paint {
    pub icon_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub size: IRIndex,
    pub rotation: IRIndex,
    pub z: IRIndex,
    pub alpha: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Print {
    pub value: IRIndex,
    pub format: IRIndex,
    pub decimal_places: IRIndex,
    pub anchor_x: IRIndex,
    pub anchor_y: IRIndex,
    pub pivot_x: IRIndex,
    pub pivot_y: IRIndex,
    pub width: IRIndex,
    pub height: IRIndex,
    pub rotation: IRIndex,
    pub color: IRIndex,
    pub alpha: IRIndex,
    pub horizontal_align: IRIndex,
    pub background: IRIndex,
}

// Timing
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct BeatToBPM {
    pub beat: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct BeatToStartingBeat {
    pub beat: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct BeatToStartingTime {
    pub beat: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct BeatToTime {
    pub beat: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct TimeToScaledTime {
    pub time: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct TimeToStartingScaledTime {
    pub time: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct TimeToStartingTime {
    pub time: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct TimeToTimeScale {
    pub time: IRIndex,
}

// Debug
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DebugLog {
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DebugPause {}

// Audio
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Play {
    pub id: IRIndex,
    pub distance: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayLooped {
    pub id: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayLoopedScheduled {
    pub id: IRIndex,
    pub time: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayScheduled {
    pub id: IRIndex,
    pub time: IRIndex,
    pub distance: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StopLooped {
    pub id: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StopLoopedScheduled {
    pub id: IRIndex,
    pub time: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct HasEffectClip {
    pub id: IRIndex,
}

// Particle
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct HasParticleEffect {
    pub id: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct DestroyParticleEffect {
    pub id: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct MoveParticleEffect {
    pub id: IRIndex,
    pub x1: IRIndex,
    pub y1: IRIndex,
    pub x2: IRIndex,
    pub y2: IRIndex,
    pub x3: IRIndex,
    pub y3: IRIndex,
    pub x4: IRIndex,
    pub y4: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct SpawnParticleEffect {
    pub id: IRIndex,
    pub x1: IRIndex,
    pub y1: IRIndex,
    pub x2: IRIndex,
    pub y2: IRIndex,
    pub x3: IRIndex,
    pub y3: IRIndex,
    pub x4: IRIndex,
    pub y4: IRIndex,
    pub duration: IRIndex,
    pub is_looped: IRIndex,
}

// Skin
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct HasSkinSprite {
    pub id: IRIndex,
}

// Gameplay
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Judge {
    pub source: IRIndex,
    pub target: IRIndex,
    pub min_perfect: IRIndex,
    pub max_perfect: IRIndex,
    pub min_great: IRIndex,
    pub max_great: IRIndex,
    pub min_good: IRIndex,
    pub max_good: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct JudgeSimple {
    pub source: IRIndex,
    pub target: IRIndex,
    pub max_perfect: IRIndex,
    pub max_great: IRIndex,
    pub max_good: IRIndex,
}

// Replay
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct ExportValue {
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StreamGetNextKey {
    pub id: IRIndex,
    pub key: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StreamGetPreviousKey {
    pub id: IRIndex,
    pub key: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StreamGetValue {
    pub id: IRIndex,
    pub key: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StreamHas {
    pub id: IRIndex,
    pub key: IRIndex,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct StreamSet {
    pub id: IRIndex,
    pub key: IRIndex,
    pub value: IRIndex,
}
