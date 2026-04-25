use crate::AudioTarget;
use crate::VolumeState;
use crate::lexer;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipewireError {
    #[error("failed to execute wpctl: {0}")]
    Io(#[from] std::io::Error),

    #[error("wpctl failed with status code: {0:?}")]
    Exit(Option<i32>),

    #[error("invalid output: {0}")]
    Utf8(String),
}

#[derive(Debug, Clone, Copy)]
pub struct PipewireAudio {
    pub target: AudioTarget,
}
impl PipewireAudio {
    pub fn new() -> Self {
        Self {
            target: AudioTarget::Default,
        }
    }
    pub fn with_target(mut self, target: AudioTarget) -> Self {
        self.target = target;
        self
    }
    pub fn get_volume(&self) -> Option<VolumeState> {
        let output = Command::new("wpctl")
            .args(["get-volume", &self.target.as_wpctl()])
            .output()
            .ok()?;
        let utf8_lossy = String::from_utf8_lossy(&output.stdout);
        let tokens = lexer::lex(&utf8_lossy);
        let volume: f32 = tokens.get(1)?.parse().ok()?;
        let muted = if tokens.get(2).is_some() { true } else { false };

        Some(VolumeState { volume, muted })
    }
    pub fn set_volume(&self, volume: f32) -> Result<(), PipewireError> {
        let output = Command::new("wpctl")
            .args(["set-volume", &self.target.as_wpctl(), &volume.to_string()])
            .output()?;
        if !output.status.success() {
            return Err(PipewireError::Exit(output.status.code()));
        }
        Ok(())
    }
}
