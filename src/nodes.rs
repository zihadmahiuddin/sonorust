#[derive(Debug)]
pub enum ResolvedNode {
    Value(f32),
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
}

// Control Flow
#[derive(Debug)]
pub struct Execute {
    pub nodes: Vec<usize>,
}
#[derive(Debug)]
pub struct Execute0 {
    pub nodes: Vec<usize>,
}
#[derive(Debug)]
pub struct Block {
    pub body: usize,
}
#[derive(Debug)]
pub struct Break {
    pub count: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct If {
    pub test: usize,
    pub consequent: usize,
    pub alternate: usize,
}
#[derive(Debug)]
pub struct While {
    pub test: usize,
    pub body: usize,
}
#[derive(Debug)]
pub struct DoWhile {
    pub body: usize,
    pub test: usize,
}
#[derive(Debug)]
pub struct JumpLoop {
    pub first_branch: usize,
    pub mid_branches: Vec<usize>,
    pub last_branch: usize,
}
#[derive(Debug)]
pub struct Switch {
    pub discriminant: usize,
    pub consequents: Vec<usize>,
}
#[derive(Debug)]
pub struct SwitchWithDefault {
    pub discriminant: usize,
    pub consequents: Vec<usize>,
    pub default_consequent: usize,
}
#[derive(Debug)]
pub struct SwitchInteger {
    pub discriminant: usize,
    pub consequents: Vec<usize>,
}
#[derive(Debug)]
pub struct SwitchIntegerWithDefault {
    pub discriminant: usize,
    pub consequents: Vec<usize>,
    pub default_consequent: usize,
}

// Math
#[derive(Debug)]
pub struct Abs {
    pub value: usize,
}
#[derive(Debug)]
pub struct Frac {
    pub value: usize,
}
#[derive(Debug)]
pub struct Trunc {
    pub value: usize,
}
#[derive(Debug)]
pub struct Negate {
    pub value: usize,
}
#[derive(Debug)]
pub struct Add {
    pub inputs: Vec<usize>,
}
#[derive(Debug)]
pub struct Subtract {
    pub inputs: Vec<usize>,
}
#[derive(Debug)]
pub struct Multiply {
    pub inputs: Vec<usize>,
}
#[derive(Debug)]
pub struct Divide {
    pub inputs: Vec<usize>,
}
#[derive(Debug)]
pub struct Mod {
    pub inputs: Vec<usize>,
}
#[derive(Debug)]
pub struct Rem {
    pub inputs: Vec<usize>,
}
#[derive(Debug)]
pub struct Power {
    pub inputs: Vec<usize>,
}
#[derive(Debug)]
pub struct Clamp {
    pub min: usize,
    pub max: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct Lerp {
    pub min: usize,
    pub max: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct LerpClamped {
    pub min: usize,
    pub max: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct Unlerp {
    pub min: usize,
    pub max: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct UnlerpClamped {
    pub min: usize,
    pub max: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct Min {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug)]
pub struct Max {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug)]
pub struct Remap {
    pub from_min: usize,
    pub from_max: usize,
    pub to_min: usize,
    pub to_max: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct RemapClamped {
    pub from_min: usize,
    pub from_max: usize,
    pub to_min: usize,
    pub to_max: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct Round {
    pub value: usize,
}
#[derive(Debug)]
pub struct Floor {
    pub value: usize,
}
#[derive(Debug)]
pub struct Ceil {
    pub value: usize,
}
#[derive(Debug)]
pub struct Sin {
    pub value: usize,
}
#[derive(Debug)]
pub struct Sinh {
    pub value: usize,
}
#[derive(Debug)]
pub struct Cos {
    pub value: usize,
}
#[derive(Debug)]
pub struct Cosh {
    pub value: usize,
}
#[derive(Debug)]
pub struct Tan {
    pub value: usize,
}
#[derive(Debug)]
pub struct Tanh {
    pub value: usize,
}
#[derive(Debug)]
pub struct Arccos {
    pub value: usize,
}
#[derive(Debug)]
pub struct Arcsin {
    pub value: usize,
}
#[derive(Debug)]
pub struct Arctan {
    pub value: usize,
}
#[derive(Debug)]
pub struct Arctan2 {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug)]
pub struct Degree {
    pub value: usize,
}
#[derive(Debug)]
pub struct Radian {
    pub value: usize,
}
#[derive(Debug)]
pub struct Log {
    pub value: usize,
}
#[derive(Debug)]
pub struct Sign {
    pub value: usize,
}
#[derive(Debug)]
pub struct Random {
    pub min: usize,
    pub max: usize,
}
#[derive(Debug)]
pub struct RandomInteger {
    pub min: usize,
    pub max: usize,
}

// Easing
#[derive(Debug)]
pub struct EaseInSine {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutSine {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInOutSine {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutInSine {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInQuad {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutQuad {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInOutQuad {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutInQuad {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInCubic {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutCubic {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInOutCubic {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutInCubic {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInQuart {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutQuart {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInOutQuart {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutInQuart {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInQuint {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutQuint {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInOutQuint {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutInQuint {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInExpo {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutExpo {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInOutExpo {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutInExpo {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInCirc {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutCirc {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInOutCirc {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutInCirc {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInBack {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutBack {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInOutBack {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutInBack {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInElastic {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutElastic {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseInOutElastic {
    pub value: usize,
}
#[derive(Debug)]
pub struct EaseOutInElastic {
    pub value: usize,
}

// Logical
#[derive(Debug)]
pub struct Equal {
    pub lhs: usize,
    pub rhs: usize,
}
#[derive(Debug)]
pub struct NotEqual {
    pub lhs: usize,
    pub rhs: usize,
}
#[derive(Debug)]
pub struct Greater {
    pub lhs: usize,
    pub rhs: usize,
}
#[derive(Debug)]
pub struct GreaterOr {
    pub lhs: usize,
    pub rhs: usize,
}
#[derive(Debug)]
pub struct Less {
    pub lhs: usize,
    pub rhs: usize,
}
#[derive(Debug)]
pub struct LessOr {
    pub lhs: usize,
    pub rhs: usize,
}
#[derive(Debug)]
pub struct And {
    pub inputs: Vec<usize>,
}
#[derive(Debug)]
pub struct Or {
    pub inputs: Vec<usize>,
}
#[derive(Debug)]
pub struct Not {
    pub value: usize,
}

// Memory
#[derive(Debug)]
pub struct Get {
    pub block_id: usize,
    pub index: usize,
}
#[derive(Debug)]
pub struct GetPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
}
#[derive(Debug)]
pub struct GetShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
}
#[derive(Debug)]
pub struct Set {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetAdd {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetAddPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetAddShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetSubtract {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetSubtractPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetSubtractShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetMultiply {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetMultiplyPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetMultiplyShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetDivide {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetDividePointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetDivideShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetMod {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetModPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetModShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetRem {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetRemPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetRemShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetPower {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetPowerPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetPowerShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct Copy {
    pub src_block_id: usize,
    pub src_index: usize,
    pub dst_block_id: usize,
    pub dst_index: usize,
    pub count: usize,
}
#[derive(Debug)]
pub struct DecrementPost {
    pub block_id: usize,
    pub index: usize,
}
#[derive(Debug)]
pub struct DecrementPostPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
}
#[derive(Debug)]
pub struct DecrementPostShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
}
#[derive(Debug)]
pub struct DecrementPre {
    pub block_id: usize,
    pub index: usize,
}
#[derive(Debug)]
pub struct DecrementPrePointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
}
#[derive(Debug)]
pub struct DecrementPreShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
}
#[derive(Debug)]
pub struct IncrementPost {
    pub block_id: usize,
    pub index: usize,
}
#[derive(Debug)]
pub struct IncrementPostPointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
}
#[derive(Debug)]
pub struct IncrementPostShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
}
#[derive(Debug)]
pub struct IncrementPre {
    pub block_id: usize,
    pub index: usize,
}
#[derive(Debug)]
pub struct IncrementPrePointed {
    pub block_id: usize,
    pub index: usize,
    pub offset: usize,
}
#[derive(Debug)]
pub struct IncrementPreShifted {
    pub block_id: usize,
    pub x: usize,
    pub y: usize,
    pub s: usize,
}

// Stack
#[derive(Debug)]
pub struct StackEnter {
    pub size: usize,
}
#[derive(Debug)]
pub struct StackGet {
    pub offset: usize,
}
#[derive(Debug)]
pub struct StackGetFrame {
    pub offset: usize,
}
#[derive(Debug)]
pub struct StackGetFramePointer;
#[derive(Debug)]
pub struct StackGetPointer;
#[derive(Debug)]
pub struct StackGrow {
    pub size: usize,
}
#[derive(Debug)]
pub struct StackInit;
#[derive(Debug)]
pub struct StackLeave;
#[derive(Debug)]
pub struct StackPop;
#[derive(Debug)]
pub struct StackPush {
    pub value: usize,
}
#[derive(Debug)]
pub struct StackSet {
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct StackSetFrame {
    pub offset: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct StackSetFramePointer {
    pub value: usize,
}
#[derive(Debug)]
pub struct StackSetPointer {
    pub value: usize,
}

// Spawning
#[derive(Debug)]
pub struct Spawn {
    pub archetype_id: usize,
    pub data: Vec<usize>,
}

// Drawing
#[derive(Debug)]
pub struct Draw {
    pub sprite_id: usize,
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub x3: usize,
    pub y3: usize,
    pub x4: usize,
    pub y4: usize,
    pub z: usize,
    pub alpha: usize,
}
#[derive(Debug)]
pub struct DrawCurvedB {
    pub sprite_id: usize,
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub x3: usize,
    pub y3: usize,
    pub x4: usize,
    pub y4: usize,
    pub z: usize,
    pub alpha: usize,
    pub n: usize,
    pub p: usize,
    pub q: usize,
}
#[derive(Debug)]
pub struct DrawCurvedBT {
    pub sprite_id: usize,
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub x3: usize,
    pub y3: usize,
    pub x4: usize,
    pub y4: usize,
    pub z: usize,
    pub alpha: usize,
    pub n: usize,
    pub p1: usize,
    pub q1: usize,
    pub p2: usize,
    pub q2: usize,
}
#[derive(Debug)]
pub struct DrawCurvedL {
    pub sprite_id: usize,
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub x3: usize,
    pub y3: usize,
    pub x4: usize,
    pub y4: usize,
    pub z: usize,
    pub alpha: usize,
    pub n: usize,
    pub p: usize,
    pub q: usize,
}
#[derive(Debug)]
pub struct DrawCurvedLR {
    pub sprite_id: usize,
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub x3: usize,
    pub y3: usize,
    pub x4: usize,
    pub y4: usize,
    pub z: usize,
    pub alpha: usize,
    pub n: usize,
    pub p1: usize,
    pub q1: usize,
    pub p2: usize,
    pub q2: usize,
}
#[derive(Debug)]
pub struct DrawCurvedR {
    pub sprite_id: usize,
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub x3: usize,
    pub y3: usize,
    pub x4: usize,
    pub y4: usize,
    pub z: usize,
    pub alpha: usize,
    pub n: usize,
    pub p: usize,
    pub q: usize,
}
#[derive(Debug)]
pub struct DrawCurvedT {
    pub sprite_id: usize,
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub x3: usize,
    pub y3: usize,
    pub x4: usize,
    pub y4: usize,
    pub z: usize,
    pub alpha: usize,
    pub n: usize,
    pub p: usize,
    pub q: usize,
}
#[derive(Debug)]
pub struct Paint {
    pub icon_id: usize,
    pub x: usize,
    pub y: usize,
    pub size: usize,
    pub rotation: usize,
    pub z: usize,
    pub alpha: usize,
}
#[derive(Debug)]
pub struct Print {
    pub value: usize,
    pub format: usize,
    pub decimal_places: usize,
    pub anchor_x: usize,
    pub anchor_y: usize,
    pub pivot_x: usize,
    pub pivot_y: usize,
    pub width: usize,
    pub height: usize,
    pub rotation: usize,
    pub color: usize,
    pub alpha: usize,
    pub horizontal_align: usize,
    pub background: usize,
}

// Timing
#[derive(Debug)]
pub struct BeatToBPM {
    pub beat: usize,
}
#[derive(Debug)]
pub struct BeatToStartingBeat {
    pub beat: usize,
}
#[derive(Debug)]
pub struct BeatToStartingTime {
    pub beat: usize,
}
#[derive(Debug)]
pub struct BeatToTime {
    pub beat: usize,
}
#[derive(Debug)]
pub struct TimeToScaledTime {
    pub time: usize,
}
#[derive(Debug)]
pub struct TimeToStartingScaledTime {
    pub time: usize,
}
#[derive(Debug)]
pub struct TimeToStartingTime {
    pub time: usize,
}
#[derive(Debug)]
pub struct TimeToTimeScale {
    pub time: usize,
}

// Debug
#[derive(Debug)]
pub struct DebugLog {
    pub value: usize,
}
#[derive(Debug)]
pub struct DebugPause {}

// Audio
#[derive(Debug)]
pub struct Play {
    pub id: usize,
    pub distance: usize,
}
#[derive(Debug)]
pub struct PlayLooped {
    pub id: usize,
}
#[derive(Debug)]
pub struct PlayLoopedScheduled {
    pub id: usize,
    pub time: usize,
}
#[derive(Debug)]
pub struct PlayScheduled {
    pub id: usize,
    pub time: usize,
    pub distance: usize,
}
#[derive(Debug)]
pub struct StopLooped {
    pub id: usize,
}
#[derive(Debug)]
pub struct StopLoopedScheduled {
    pub id: usize,
    pub time: usize,
}
#[derive(Debug)]
pub struct HasEffectClip {
    pub id: usize,
}

// Particle
#[derive(Debug)]
pub struct HasParticleEffect {
    pub id: usize,
}
#[derive(Debug)]
pub struct DestroyParticleEffect {
    pub id: usize,
}
#[derive(Debug)]
pub struct MoveParticleEffect {
    pub id: usize,
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub x3: usize,
    pub y3: usize,
    pub x4: usize,
    pub y4: usize,
}
#[derive(Debug)]
pub struct SpawnParticleEffect {
    pub id: usize,
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub x3: usize,
    pub y3: usize,
    pub x4: usize,
    pub y4: usize,
    pub duration: usize,
    pub is_looped: usize,
}

// Skin
#[derive(Debug)]
pub struct HasSkinSprite {
    pub id: usize,
}

// Gameplay
#[derive(Debug)]
pub struct Judge {
    pub source: usize,
    pub target: usize,
    pub min_perfect: usize,
    pub max_perfect: usize,
    pub min_great: usize,
    pub max_great: usize,
    pub min_good: usize,
    pub max_good: usize,
}
#[derive(Debug)]
pub struct JudgeSimple {
    pub source: usize,
    pub target: usize,
    pub max_perfect: usize,
    pub max_great: usize,
    pub max_good: usize,
}

// Replay
#[derive(Debug)]
pub struct ExportValue {
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct StreamGetNextKey {
    pub id: usize,
    pub key: usize,
}
#[derive(Debug)]
pub struct StreamGetPreviousKey {
    pub id: usize,
    pub key: usize,
}
#[derive(Debug)]
pub struct StreamGetValue {
    pub id: usize,
    pub key: usize,
}
#[derive(Debug)]
pub struct StreamHas {
    pub id: usize,
    pub key: usize,
}
#[derive(Debug)]
pub struct StreamSet {
    pub id: usize,
    pub key: usize,
    pub value: usize,
}
