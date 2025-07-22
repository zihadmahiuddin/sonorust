#[derive(Debug)]
pub enum ResolvedNode {
    Value(f64),
    OpCode(OpCode),
}

#[derive(Debug)]
pub enum OpCode {
    Execute(Execute),
    If(If),
    Block(Block),
    Break(Break),
    While(While),
    SwitchInteger(SwitchInteger),
    SwitchIntegerWithDefault(SwitchIntegerWithDefault),
    Add(Add),
    Subtract(Subtract),
    Multiply(Multiply),
    Divide(Divide),
    Equal(Equal),
    NotEqual(NotEqual),
    Greater(Greater),
    GreaterOr(GreaterOr),
    Less(Less),
    LessOr(LessOr),
    And(And),
    Or(Or),
    Not(Not),
    Get(Get),
    GetShifted(GetShifted),
    Set(Set),
    SetAdd(SetAdd),
    SetSubtract(SetSubtract),
    SetMultiply(SetMultiply),
    SetShifted(SetShifted),
    SetRem(SetRem),
}

// Control Flow
#[derive(Debug)]
pub struct Execute {
    pub nodes: Vec<usize>,
}
#[derive(Debug)]
pub struct If {
    pub test: usize,
    pub consequent: usize,
    pub alternate: usize,
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
pub struct While {
    pub test: usize,
    pub body: usize,
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
// Abs { value },
// Frac { value },
// Negate { value },
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
// Mod { ..inputs },
// Rem { ..inputs },
// Power { ..inputs },
// Clamp { min, max, value },
// Lerp { min, max, value },
// Unlerp { min, max, value },
// UnlerpClamped { min, max, value },
// Min { x, y },
// Max { x, y },
// Remap { from_min, from_max, to_min, to_max, value },
// Round { value },
// Floor { value },
// Ceil { value },
// Sin { value },
// Cos { value },
// Arctan2 { x, y },

// Random { min, max },

// Easing
// EaseInCubic { value },
// EaseInQuad { value },
// EaseOutCubic { value },
// EaseOutQuad { value },

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
pub struct SetAdd {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetSubtract {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}
#[derive(Debug)]
pub struct SetMultiply {
    pub block_id: usize,
    pub index: usize,
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
pub struct SetRem {
    pub block_id: usize,
    pub index: usize,
    pub value: usize,
}

// Side Effects
// Spawn { archetype_id },
// Draw {
//     sprite_id,
//     x1,
//     y1,
//     x2,
//     y2,
//     x3,
//     y3,
//     x4,
//     y4,
//     z,
//     alpha,
// },

// Timing
// BeatToTime { beat },
// BeatToBPM { beat },
// TimeToScaledTime { time },

// Debug
#[derive(Debug)]
pub struct DebugLog {
    pub value: usize,
}

// Audio
// Play { id, distance },
// PlayLooped { id },
// PlayLoopedScheduled { id, time },
// PlayScheduled { id, time, distance },
// StopLooped { id },
// StopLoopedScheduled { id, time },
// HasEffectClip { id },

// Particle
// HasParticleEffect { id },
// DestroyParticleEffect { id },
// MoveParticleEffect { id, x1, y1, x2, y2, x3, y3, x4, y4 },
// SpawnParticleEffect { id, x1, y1, x2, y2, x3, y3, x4, y4, duration, is_looped },

// Skin
// HasSkinSprite { id },

// Gameplay
// Judge { source, target, min_perfect, max_perfect, min_great, max_great, min_good, max_good },

// Replay
// ExportValue { index, value },
// StreamSet { id, key, value },
