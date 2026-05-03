use std::borrow::Cow;

/// Sink enum; allows you to use the default sink or a custom sink
#[derive(Debug, Clone, Copy)]
pub enum AudioSink {
    /// `@DEFAULT_AUDIO_SINK@` OR `@DEFAULT_SINK@` (it depends on the backend)
    Default,
    /// Specify where the audio sink (output) should be directed
    Device(u32),
}
impl AudioSink {
    pub(crate) fn as_wpctl(&self) -> Cow<'static, str> {
        match self {
            AudioSink::Default => "@DEFAULT_AUDIO_SINK@".into(),
            AudioSink::Device(id) => id.to_string().into(),
        }
    }
    pub(crate) fn as_pactl(&self) -> Cow<'static, str> {
        match self {
            AudioSink::Default => "@DEFAULT_SINK@".into(),
            AudioSink::Device(id) => id.to_string().into(),
        }
    }
}
