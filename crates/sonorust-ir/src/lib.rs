pub mod nodes;

pub type IRValue = f32;

pub fn modulo(a: IRValue, n: IRValue) -> IRValue {
    ((a % n) + n) % n
}
