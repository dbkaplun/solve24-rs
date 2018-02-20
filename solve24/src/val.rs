use std::f64::EPSILON;

pub type Val = f64;

// In lieu of PartialEq trait
pub fn eq(a: Val, b: Val) -> bool {
    (a - b).abs() < EPSILON
}
