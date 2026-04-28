/// Converts a percent value to a number (0.0–1.0).
pub fn from_percent(percent: f32) -> f32 {
    percent / 100.0
}
/// Converts a number (0.0–1.0) to a percent value
pub fn to_percent(number: f32) -> f32 {
    number * 100.0
}
