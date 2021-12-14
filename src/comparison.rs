pub fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}
