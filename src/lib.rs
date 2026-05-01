mod lexer;
mod pulse_audio;

pub mod error;
pub mod pipewire_audio;

use std::borrow::Cow;

#[derive(Debug)]
pub struct Volume(pub f32);

impl Volume {
    pub fn to_percent(&self) -> f32 {
        self.0 * 100.0
    }
    pub fn from_percent(percent: f32) -> f32 {
        percent / 100.0
    }
}

pub struct VolumeState {
    pub volume: Volume,
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
    use pipewire_audio::PipewireAudio;

    #[test]
    fn it_works() -> Result<(), AudioError> {
        let audio = PipewireAudio::new().with_target(AudioTarget::Default);
        audio.set_volume(Volume::from_percent(50.0))?;
        audio.set_mute(true)?;
        if let Some(v) = audio.get_volume() {
            println!("volume: {:?} muted: {:?}", v.volume.to_percent(), v.muted);
        }
        Ok(())
    }
}
