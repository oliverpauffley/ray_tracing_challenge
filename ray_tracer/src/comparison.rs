pub const EPSILON: f64 = 0.00001;

#[allow(dead_code)]
pub fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
