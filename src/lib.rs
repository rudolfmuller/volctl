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
    Sink(u16),
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

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let audio = pipewire_audio::PipewireAudio {
            id: AudioTarget::Default,
        };
        if let Some(v) = audio.get_volume() {
            println!("volume: {} muted: {}", v.volume, v.muted);
        }
        Ok(())
    }
}
