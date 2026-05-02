use crate::error::AudioError;
use crate::lexer::{Token, lex};
use crate::{
    audio_target::AudioTarget,
    volume::{Volume, VolumeState},
};
use std::borrow::Cow;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct PipewireAudio {
    target: AudioTarget,
    bin: Cow<'static, str>,
}
impl PipewireAudio {
    pub fn new() -> Self {
        Self {
            target: AudioTarget::Default,
            bin: "wpctl".into(),
        }
    }
    pub fn with_target(mut self, target: AudioTarget) -> Self {
        self.target = target;
        self
    }
    pub fn with_bin(mut self, bin: impl Into<Cow<'static, str>>) -> Self {
        self.bin = bin.into();
        self
    }

    pub fn access_state(&self) -> Result<VolumeState, AudioError> {
        let output = Command::new(&*self.bin)
            .args(["get-volume", &self.target.as_wpctl()])
            .output()
            .map_err(|err| AudioError::Execute {
                program: self.bin.to_string(),
                err,
            })?;
        let utf8 = String::from_utf8(output.stdout).map_err(AudioError::InvalidUtf8)?;
        let tokens = lex(&utf8);
        let volume: Volume = match tokens.get(0) {
            Some(Token::FloatValue(v)) => Volume(*v),
            _ => Volume(0.0),
        };
        let muted = match tokens.get(1) {
            Some(Token::Muted) => true,
            _ => false,
        };

        Ok(VolumeState { volume, muted })
    }
    pub fn fetch_state(&self) -> Option<VolumeState> {
        self.access_state().ok()
    }
    pub fn state(&self) -> VolumeState {
        self.fetch_state().unwrap_or_default()
    }

    pub fn set_volume(&self, volume: Volume) -> Result<(), AudioError> {
        let output = Command::new(&*self.bin)
            .args(["set-volume", &self.target.as_wpctl(), &volume.to_string()])
            .output()
            .map_err(|err| AudioError::Execute {
                program: self.bin.to_string(),
                err,
            })?;
        if !output.status.success() {
            return Err(AudioError::Exit {
                program: self.bin.to_string(),
                ec: output.status,
            });
        }
        Ok(())
    }

    pub fn set_mute(&self, mute: bool) -> Result<(), AudioError> {
        let output = Command::new(&*self.bin)
            .args([
                "set-mute",
                &self.target.as_wpctl(),
                if mute { "1" } else { "0" },
            ])
            .output()
            .map_err(|err| AudioError::Execute {
                program: self.bin.to_string(),
                err,
            })?;
        if !output.status.success() {
            return Err(AudioError::Exit {
                program: self.bin.to_string(),
                ec: output.status,
            });
        }
        Ok(())
    }
}
