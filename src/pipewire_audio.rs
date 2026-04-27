use crate::AudioTarget;
use crate::VolumeState;
use crate::error::AudioError;
use crate::lexer;
use std::process::Command;

const WPCTL_BIN: &'static str = "wpctl";

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
        let output = Command::new(WPCTL_BIN)
            .args(["get-volume", &self.target.as_wpctl()])
            .output()
            .ok()?;
        let utf8_lossy = String::from_utf8_lossy(&output.stdout);
        let tokens = lexer::lex(&utf8_lossy);
        let volume: f32 = tokens.get(1)?.parse().ok()?;
        let muted = tokens.get(2).is_some();

        Some(VolumeState { volume, muted })
    }
    pub fn set_volume(&self, volume: f32) -> Result<(), AudioError> {
        let output = Command::new(WPCTL_BIN)
            .args(["set-volume", &self.target.as_wpctl(), &volume.to_string()])
            .output()
            .map_err(|err| AudioError::Execute {
                program: WPCTL_BIN,
                err,
            })?;
        if !output.status.success() {
            let ec = output.status.code();
            return Err(AudioError::Exit {
                program: WPCTL_BIN,
                ec,
            });
        }
        Ok(())
    }

    pub fn set_mute(&self, mute: bool) -> Result<(), AudioError> {
        let output = Command::new(WPCTL_BIN)
            .args([
                "set-mute",
                &self.target.as_wpctl(),
                if mute { "1" } else { "0" },
            ])
            .output()
            .map_err(|err| AudioError::Execute {
                program: WPCTL_BIN,
                err,
            })?;
        if !output.status.success() {
            let ec = output.status.code();
            return Err(AudioError::Exit {
                program: WPCTL_BIN,
                ec,
            });
        }
        Ok(())
    }
}
