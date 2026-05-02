#[derive(thiserror::Error, Debug)]
pub enum AudioError {
    #[error("failed to execute {program}: {err}")]
    Execute {
        program: String,
        #[source]
        err: std::io::Error,
    },

    #[error("{program} failed with status code: {ec:?}")]
    Exit { program: String, ec: Option<i32> },
}
