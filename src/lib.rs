mod error;
mod lexer;
mod math;
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

#[cfg(test)]
mod tests {

    use super::*;
    use error::AudioError;
    use math::{from_percent, to_percent};
    use pipewire_audio::PipewireAudio;

    #[test]
    fn it_works() -> Result<(), AudioError> {
        let audio = PipewireAudio::new().with_target(AudioTarget::Default);
        audio.set_volume(from_percent(50.0))?;
        audio.set_mute(true)?;
        if let Some(v) = audio.get_volume() {
            println!("volume: {} muted: {}", to_percent(v.volume), v.muted);
        }
        Ok(())
    }
}
