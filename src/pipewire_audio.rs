use crate::error::AudioError;
use crate::lexer::{Token, lex};
use crate::{
    audio_target::AudioTarget,
    volume::{Volume, VolumeState},
};
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
        let tokens = lex(&utf8_lossy);
        let volume: Volume = match tokens.get(0) {
            Some(Token::FloatValue(v)) => Volume(*v),
            _ => Volume(0.0),
        };
        let muted = match tokens.get(1) {
            Some(Token::Muted) => true,
            _ => false,
        };

        Some(VolumeState { volume, muted })
    }
    pub fn set_volume(&self, volume: Volume) -> Result<(), AudioError> {
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
