use std::io::Error;
use std::process::ExitStatus;
use std::string::FromUtf8Error;

use crate::lexer::Token;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("unexpected token: {unexpected:?}, expected: {expected:?}")]
    UnexpectedToken { expected: Token, unexpected: Token },

    #[error("the token: {0:?} cannot be found ")]
    TokenNotFound(Token),
}

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

    #[error(transparent)]
    Parse(#[from] ParseError),
}
