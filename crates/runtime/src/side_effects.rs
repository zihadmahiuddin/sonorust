use sonorust_ir::IRValue;
use sonorust_models::ids::ArchetypeId;

#[derive(Debug, Clone)]
pub struct SpawnSideEffect {
    pub archetype_id: ArchetypeId,
    pub data: Vec<IRValue>,
}

#[derive(Debug, Clone)]
pub struct DrawSideEffect {
    pub sprite_id: usize,
    pub x1: IRValue,
    pub y1: IRValue,
    pub x2: IRValue,
    pub y2: IRValue,
    pub x3: IRValue,
    pub y3: IRValue,
    pub x4: IRValue,
    pub y4: IRValue,
    pub alpha: IRValue,
    pub z1: IRValue,
    pub z2: Option<IRValue>,
    pub z3: Option<IRValue>,
    pub z4: Option<IRValue>,
}
