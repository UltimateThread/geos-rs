pub struct NumberUtil {}

impl NumberUtil {
    pub fn equals_with_tolerance(x1: f64, x2: f64, tolerance: f64) -> bool {
        f64::abs(x1 - x2) <= tolerance
    }
}
