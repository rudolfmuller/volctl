mod lexer;
mod pulse_audio;

pub mod audio_target;
pub mod error;
pub mod pipewire_audio;
pub mod volume;

#[cfg(test)]
mod tests {

    use super::*;
    use audio_target::AudioTarget;
    use error::AudioError;
    use pipewire_audio::PipewireAudio;
    use volume::Volume;

    #[test]
    fn it_works() -> Result<(), AudioError> {
        let audio = PipewireAudio::new()
            .with_target(AudioTarget::Default)
            .with_bin("/usr/bin/wpctl");
        audio.set_volume(Volume(40.0).from_percent())?;
        let volume = audio.state().volume.to_percent();
        let muted = audio.state().muted;

        println!("volume: {} muted: {}", volume, muted);

        Ok(())
    }
}
