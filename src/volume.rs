#[derive(Debug)]
pub struct VolumeState {
    pub volume: Volume,
    pub muted: bool,
}

#[derive(Debug)]
pub struct Volume(pub f32);

impl Volume {
    pub fn to_percent(&self) -> f32 {
        self.0 * 100.0
    }
    pub fn from_percent(&self) -> Self {
        let value = self.0;
        Self(value / 100.0)
    }
}
impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
