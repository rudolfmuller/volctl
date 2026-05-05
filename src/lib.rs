mod lexer;
mod pulse_audio;

pub mod audio_sink;
pub mod error;
pub mod pipewire_audio;
pub mod volume;

#[cfg(test)]
mod tests {

    use crate::volume::VolumeState;

    use super::*;
    use audio_sink::AudioSink;
    use error::AudioError;
    use pipewire_audio::PipewireAudio;
    use volume::Volume;

    #[test]
    fn test_pw_state() -> Result<(), AudioError> {
        let pw_audio = PipewireAudio::default()
            .with_sink(AudioSink::Default)
            .with_bin("/usr/bin/wpctl");

        let audio_state = pw_audio.get_state().unwrap_or_default();
        eprintln!(
            "volume: {}% muted: {}",
            audio_state.volume.to_percent(),
            audio_state.muted
        );

        Ok(())
    }
    #[test]
    fn test_pw_volume() -> Result<(), AudioError> {
        let pw_audio = PipewireAudio::default()
            .with_sink(AudioSink::Default)
            .with_bin("/usr/bin/wpctl");

        pw_audio.set_volume(Volume(0.4))?; // 0.4
        pw_audio.set_volume(Volume::from_percent(40.0))?; // (40.0 / 100) = 0.4

        Ok(())
    }
    #[test]
    fn test_pw_mute() -> Result<(), AudioError> {
        let pw_audio = PipewireAudio::default()
            .with_sink(AudioSink::Default)
            .with_bin("/usr/bin/wpctl");

        pw_audio.set_mute(true)?;
        Ok(())
    }
    #[test]
    fn test_pw_set_state() -> Result<(), AudioError> {
        let pw_audio = PipewireAudio::default()
            .with_sink(AudioSink::Default)
            .with_bin("/usr/bin/wpctl");

        pw_audio.set_state(VolumeState {
            volume: Volume::from_percent(40.0),
            muted: false,
        })?;
        Ok(())
    }
}
