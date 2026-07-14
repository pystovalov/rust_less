pub fn mul(a: &i32, b: &i32) -> i32 {
    a * b
}
pub fn div(a: &i32, b: &i32) -> Option<f64> {
    if *b != 0 {
        Some((*a as f64) / (*b as f64))
    } else {
        None
    }
}
