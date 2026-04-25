mod lexer;
mod pipewire_audio;
mod pulse_audio;

pub struct VolumeState {
    pub volume: f32,
    pub muted: bool,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        if let Some(v) = pipewire_audio::PipewireAudio::get_volume() {
            println!("volume: {} muted: {}", v.volume, v.muted);
        }
        Ok(())
    }
}
