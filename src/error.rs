use std::io::Error;
use std::process::ExitStatus;
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum AudioError {
    #[error("failed to execute {program}: {err}")]
    Execute {
        program: String,
        #[source]
        err: Error,
    },

    #[error("{program} failed with status code: {ec:?}")]
    Exit { program: String, ec: ExitStatus },

    #[error("invalid UTF-8")]
    InvalidUtf8(#[from] FromUtf8Error),
}
