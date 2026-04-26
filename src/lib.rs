mod lexer;
mod pipewire_audio;
mod pulse_audio;

use std::borrow::Cow;

pub struct VolumeState {
    pub volume: f32,
    pub muted: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum AudioTarget {
    Default,
    Sink(u32),
}
impl AudioTarget {
    pub fn as_wpctl(&self) -> Cow<'static, str> {
        match self {
            AudioTarget::Default => "@DEFAULT_AUDIO_SINK@".into(),
            AudioTarget::Sink(id) => id.to_string().into(),
        }
    }
    pub fn as_pactl(&self) -> Cow<'static, str> {
        match self {
            AudioTarget::Default => "@DEFAULT_SINK@".into(),
            AudioTarget::Sink(id) => id.to_string().into(),
        }
    }
}

pub fn from_percent(percent: f32) -> f32 {
    percent / 100.0
}

#[cfg(test)]
mod tests {

    use super::*;
    use pipewire_audio::{PipewireAudio, PipewireError};

    #[test]
    fn it_works() -> Result<(), PipewireError> {
        let audio = PipewireAudio::new().with_target(AudioTarget::Sink(34));
        audio.set_volume(from_percent(50.0))?;
        if let Some(v) = audio.get_volume() {
            println!("volume: {} muted: {}", v.volume, v.muted);
        }
        Ok(())
    }
}
