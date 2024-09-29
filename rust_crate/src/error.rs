use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RinfError {
    NoDartIsolate,
    CannotDecodeMessage,
    NoSignalHandler,
}

impl fmt::Display for RinfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoDartIsolate => {
                write!(f, "Dart isolate for Rust signals was not created")
            }
            Self::CannotDecodeMessage => {
                write!(f, "Could not decode the message")
            }
            Self::NoSignalHandler => {
                write!(f, "Could not find the handler for Dart signal")
            }
        }
    }
}

impl Error for RinfError {}
