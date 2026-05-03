/// Stores volume state
#[derive(Debug)]
pub struct VolumeState {
    pub volume: Volume,
    pub muted: bool,
}
impl Default for VolumeState {
    fn default() -> Self {
        Self {
            volume: Volume(0.0),
            muted: false,
        }
    }
}

/// `f32` with more functions
#[derive(Debug)]
pub struct Volume(pub f32);

impl Volume {
    /// Convert numbers to percents and return self
    pub fn to_percent(&self) -> Self {
        Self(self.0 * 100.0)
    }
    /// Convert percents to numbers and return self
    pub fn from_percent(&self) -> Self {
        Self(self.0 / 100.0)
    }
    /// Return the value of `Volume` as `f32`
    pub fn as_f32(&self) -> f32 {
        self.0
    }
}
impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
