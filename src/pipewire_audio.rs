use crate::VolumeState;
use crate::lexer;
use std::process::Command;

pub struct PipewireAudio {}
impl PipewireAudio {
    pub fn get_volume() -> Option<VolumeState> {
        let output = Command::new("wpctl")
            .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
            .output()
            .ok()?;
        let utf8_lossy = String::from_utf8_lossy(&output.stdout);
        let tokens = lexer::lex(&utf8_lossy);
        let volume: f32 = tokens.get(1)?.parse().ok()?;
        let muted = if tokens.get(2).is_some() { true } else { false };

        Some(VolumeState { volume, muted })
    }
}
