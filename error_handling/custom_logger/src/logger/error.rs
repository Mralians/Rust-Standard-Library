use log::SetLoggerError;
use std::error;
use std::io;
use std::result;
#[derive(Debug)]
pub enum FileLoggerError {
    Io(io::Error),
    SetLogger(SetLoggerError),
}

pub type Result<T> = result::Result<T, FileLoggerError>;
impl error::Error for FileLoggerError {
    fn description(&self) -> &str {
        match *self {
            Self::Io(ref error) => error.description(),
            Self::SetLogger(ref error) => error.description(),
        }
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Self::Io(ref error) => Some(error),
            Self::SetLogger(ref error) => Some(error),
        }
    }
}

impl std::fmt::Display for FileLoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Io(ref error) => write!(f, "{}", error),
            Self::SetLogger(ref error) => write!(f, "{}", error),
        }
    }
}

impl From<io::Error> for FileLoggerError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<SetLoggerError> for FileLoggerError {
    fn from(value: SetLoggerError) -> Self {
        Self::SetLogger(value)
    }
}
