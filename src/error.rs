#[derive(thiserror::Error, Debug)]
pub enum AudioError {
    #[error("failed to execute {program}: {err}")]
    Execute {
        program: &'static str,
        #[source]
        err: std::io::Error,
    },

    #[error("{program} failed with status code: {ec:?}")]
    Exit {
        program: &'static str,
        ec: Option<i32>,
    },
}
