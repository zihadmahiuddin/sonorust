use crate::{IRIndex, IRValue};

#[derive(Debug)]
pub enum IRNode {
    Value(IRValue),
    OpCode(OpCode),
}

#[derive(Debug)]
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
}

// Control Flow
#[derive(Debug)]
pub struct Execute {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Execute0 {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Block {
    pub body: IRIndex,
}
#[derive(Debug)]
pub struct Break {
    pub count: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct If {
    pub test: IRIndex,
    pub consequent: IRIndex,
    pub alternate: IRIndex,
}
#[derive(Debug)]
pub struct While {
    pub test: IRIndex,
    pub body: IRIndex,
}
#[derive(Debug)]
pub struct DoWhile {
    pub body: IRIndex,
    pub test: IRIndex,
}
#[derive(Debug)]
pub struct JumpLoop {
    pub first_branch: IRIndex,
    pub mid_branches: Vec<IRIndex>,
    pub last_branch: IRIndex,
}
#[derive(Debug)]
pub struct Switch {
    pub discriminant: IRIndex,
    pub tests_and_consequents: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct SwitchWithDefault {
    pub discriminant: IRIndex,
    pub tests_and_consequents: Vec<IRIndex>,
    pub default_consequent: IRIndex,
}
#[derive(Debug)]
pub struct SwitchInteger {
    pub discriminant: IRIndex,
    pub consequents: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct SwitchIntegerWithDefault {
    pub discriminant: IRIndex,
    pub consequents: Vec<IRIndex>,
    pub default_consequent: IRIndex,
}

// Math
#[derive(Debug)]
pub struct Abs {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Frac {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Trunc {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Negate {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Add {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Subtract {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Multiply {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Divide {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Mod {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Rem {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Power {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Clamp {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Lerp {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct LerpClamped {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Unlerp {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct UnlerpClamped {
    pub min: IRIndex,
    pub max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Min {
    pub x: IRIndex,
    pub y: IRIndex,
}
#[derive(Debug)]
pub struct Max {
    pub x: IRIndex,
    pub y: IRIndex,
}
#[derive(Debug)]
pub struct Remap {
    pub from_min: IRIndex,
    pub from_max: IRIndex,
    pub to_min: IRIndex,
    pub to_max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct RemapClamped {
    pub from_min: IRIndex,
    pub from_max: IRIndex,
    pub to_min: IRIndex,
    pub to_max: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Round {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Floor {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Ceil {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Sin {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Sinh {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Cos {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Cosh {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Tan {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Tanh {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Arccos {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Arcsin {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Arctan {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Arctan2 {
    pub x: IRIndex,
    pub y: IRIndex,
}
#[derive(Debug)]
pub struct Degree {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Radian {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Log {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Sign {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Random {
    pub min: IRIndex,
    pub max: IRIndex,
}
#[derive(Debug)]
pub struct RandomInteger {
    pub min: IRIndex,
    pub max: IRIndex,
}

// Easing
#[derive(Debug)]
pub struct EaseInSine {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutSine {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInOutSine {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutInSine {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInQuad {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutQuad {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInOutQuad {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutInQuad {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInCubic {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutCubic {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInOutCubic {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutInCubic {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInQuart {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutQuart {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInOutQuart {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutInQuart {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInQuint {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutQuint {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInOutQuint {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutInQuint {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInExpo {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutExpo {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInOutExpo {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutInExpo {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInCirc {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutCirc {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInOutCirc {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutInCirc {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInBack {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutBack {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInOutBack {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutInBack {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInElastic {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutElastic {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseInOutElastic {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct EaseOutInElastic {
    pub value: IRIndex,
}

// Logical
#[derive(Debug)]
pub struct Equal {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
pub struct NotEqual {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
pub struct Greater {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
pub struct GreaterOr {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
pub struct Less {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
pub struct LessOr {
    pub lhs: IRIndex,
    pub rhs: IRIndex,
}
#[derive(Debug)]
pub struct And {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Or {
    pub args: Vec<IRIndex>,
}
#[derive(Debug)]
pub struct Not {
    pub value: IRIndex,
}

// Memory
#[derive(Debug)]
pub struct Get {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
pub struct GetPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
pub struct GetShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}
#[derive(Debug)]
pub struct Set {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetAdd {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetAddPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetAddShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetSubtract {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetSubtractPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetSubtractShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetMultiply {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetMultiplyPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetMultiplyShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetDivide {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetDividePointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetDivideShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetMod {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetModPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetModShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetRem {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetRemPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetRemShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetPower {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetPowerPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct SetPowerShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct Copy {
    pub src_block_id: IRIndex,
    pub src_index: IRIndex,
    pub dst_block_id: IRIndex,
    pub dst_index: IRIndex,
    pub count: IRIndex,
}
#[derive(Debug)]
pub struct DecrementPost {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
pub struct DecrementPostPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
pub struct DecrementPostShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}
#[derive(Debug)]
pub struct DecrementPre {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
pub struct DecrementPrePointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
pub struct DecrementPreShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}
#[derive(Debug)]
pub struct IncrementPost {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
pub struct IncrementPostPointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
pub struct IncrementPostShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}
#[derive(Debug)]
pub struct IncrementPre {
    pub block_id: IRIndex,
    pub index: IRIndex,
}
#[derive(Debug)]
pub struct IncrementPrePointed {
    pub block_id: IRIndex,
    pub index: IRIndex,
    pub offset: IRIndex,
}
#[derive(Debug)]
pub struct IncrementPreShifted {
    pub block_id: IRIndex,
    pub x: IRIndex,
    pub y: IRIndex,
    pub s: IRIndex,
}

// Stack
#[derive(Debug)]
pub struct StackEnter {
    pub size: IRIndex,
}
#[derive(Debug)]
pub struct StackGet {
    pub offset: IRIndex,
}
#[derive(Debug)]
pub struct StackGetFrame {
    pub offset: IRIndex,
}
#[derive(Debug)]
pub struct StackGetFramePointer;
#[derive(Debug)]
pub struct StackGetPointer;
#[derive(Debug)]
pub struct StackGrow {
    pub size: IRIndex,
}
#[derive(Debug)]
pub struct StackInit;
#[derive(Debug)]
pub struct StackLeave;
#[derive(Debug)]
pub struct StackPop;
#[derive(Debug)]
pub struct StackPush {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct StackSet {
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct StackSetFrame {
    pub offset: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct StackSetFramePointer {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct StackSetPointer {
    pub value: IRIndex,
}

// Spawning
#[derive(Debug)]
pub struct Spawn {
    pub archetype_id: IRIndex,
    pub data: Vec<IRIndex>,
}

// Drawing
#[derive(Debug)]
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
    pub z: IRIndex,
    pub alpha: IRIndex,
}
#[derive(Debug)]
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
pub struct BeatToBPM {
    pub beat: IRIndex,
}
#[derive(Debug)]
pub struct BeatToStartingBeat {
    pub beat: IRIndex,
}
#[derive(Debug)]
pub struct BeatToStartingTime {
    pub beat: IRIndex,
}
#[derive(Debug)]
pub struct BeatToTime {
    pub beat: IRIndex,
}
#[derive(Debug)]
pub struct TimeToScaledTime {
    pub time: IRIndex,
}
#[derive(Debug)]
pub struct TimeToStartingScaledTime {
    pub time: IRIndex,
}
#[derive(Debug)]
pub struct TimeToStartingTime {
    pub time: IRIndex,
}
#[derive(Debug)]
pub struct TimeToTimeScale {
    pub time: IRIndex,
}

// Debug
#[derive(Debug)]
pub struct DebugLog {
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct DebugPause {}

// Audio
#[derive(Debug)]
pub struct Play {
    pub id: IRIndex,
    pub distance: IRIndex,
}
#[derive(Debug)]
pub struct PlayLooped {
    pub id: IRIndex,
}
#[derive(Debug)]
pub struct PlayLoopedScheduled {
    pub id: IRIndex,
    pub time: IRIndex,
}
#[derive(Debug)]
pub struct PlayScheduled {
    pub id: IRIndex,
    pub time: IRIndex,
    pub distance: IRIndex,
}
#[derive(Debug)]
pub struct StopLooped {
    pub id: IRIndex,
}
#[derive(Debug)]
pub struct StopLoopedScheduled {
    pub id: IRIndex,
    pub time: IRIndex,
}
#[derive(Debug)]
pub struct HasEffectClip {
    pub id: IRIndex,
}

// Particle
#[derive(Debug)]
pub struct HasParticleEffect {
    pub id: IRIndex,
}
#[derive(Debug)]
pub struct DestroyParticleEffect {
    pub id: IRIndex,
}
#[derive(Debug)]
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
pub struct HasSkinSprite {
    pub id: IRIndex,
}

// Gameplay
#[derive(Debug)]
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
pub struct JudgeSimple {
    pub source: IRIndex,
    pub target: IRIndex,
    pub max_perfect: IRIndex,
    pub max_great: IRIndex,
    pub max_good: IRIndex,
}

// Replay
#[derive(Debug)]
pub struct ExportValue {
    pub index: IRIndex,
    pub value: IRIndex,
}
#[derive(Debug)]
pub struct StreamGetNextKey {
    pub id: IRIndex,
    pub key: IRIndex,
}
#[derive(Debug)]
pub struct StreamGetPreviousKey {
    pub id: IRIndex,
    pub key: IRIndex,
}
#[derive(Debug)]
pub struct StreamGetValue {
    pub id: IRIndex,
    pub key: IRIndex,
}
#[derive(Debug)]
pub struct StreamHas {
    pub id: IRIndex,
    pub key: IRIndex,
}
#[derive(Debug)]
pub struct StreamSet {
    pub id: IRIndex,
    pub key: IRIndex,
    pub value: IRIndex,
}
