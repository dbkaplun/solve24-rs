use std;

pub type Val = f64;

pub const EPSILON: Val = std::f32::EPSILON as Val;

// In lieu of PartialEq trait
pub fn eq(a: Val, b: Val) -> bool {
    (a - b).abs() < EPSILON
}
