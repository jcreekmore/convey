use serde_json::Error as JsonError;
use std::fmt::{self, Display};
use std::io;
use std::sync::PoisonError;
use termcolor::ParseColorError;

#[derive(Debug)]
/// Output's error type
pub struct Error {
    inner: InnerError,
}

impl ::std::error::Error for Error {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match &self.inner {
            InnerError::IoContext(_, ref e) => Some(e),
            InnerError::Io(ref e) => Some(e),
            InnerError::ParseColorError(ref e) => Some(e),
            InnerError::Json(ref e) => Some(e),
            #[cfg(feature = "log")]
            InnerError::SetLoggerError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            InnerError::IoContext(ref e, _) => write!(f, "{}", e),
            InnerError::Io(e) => write!(f, "IO error: {}", e),
            InnerError::ParseColorError(e) => write!(f, "{}", e),
            InnerError::Json(e) => write!(f, "Json error: {}", e),
            InnerError::WorkerError(e) => write!(f, "Worker error: {}", e),
            InnerError::SyncError(e) => write!(f, "Sync error: {}", e),
            InnerError::ChannelError(e) => write!(f, "Channel error: {}", e),
            #[cfg(feature = "log")]
            InnerError::SetLoggerError(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug)]
enum InnerError {
    IoContext(String, io::Error),
    Io(io::Error),
    ParseColorError(ParseColorError),
    Json(JsonError),
    WorkerError(String),
    SyncError(String),
    ChannelError(String),
    #[cfg(feature = "log")]
    SetLoggerError(log::SetLoggerError),
}

impl Error {
    pub(crate) fn worker_error(x: String) -> Self {
        Error {
            inner: InnerError::WorkerError(x),
        }
    }

    pub(crate) fn sync_error<T>(x: &PoisonError<T>) -> Self {
        Error {
            inner: InnerError::SyncError(x.to_string()),
        }
    }

    pub(crate) fn io_context<S: ToString>(s: S, e: io::Error) -> Self {
        Error {
            inner: InnerError::IoContext(s.to_string(), e),
        }
    }
}

impl From<InnerError> for Error {
    fn from(kind: InnerError) -> Error {
        Error { inner: kind }
    }
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self {
        Error {
            inner: InnerError::Io(x),
        }
    }
}

impl From<ParseColorError> for Error {
    fn from(x: ParseColorError) -> Self {
        Error {
            inner: InnerError::ParseColorError(x),
        }
    }
}

impl From<JsonError> for Error {
    fn from(x: JsonError) -> Self {
        Error {
            inner: InnerError::Json(x),
        }
    }
}

impl<T: std::fmt::Debug> From<crossbeam_channel::SendError<T>> for Error {
    fn from(x: crossbeam_channel::SendError<T>) -> Self {
        Error {
            inner: InnerError::ChannelError(x.to_string()),
        }
    }
}

#[cfg(feature = "log")]
impl From<log::SetLoggerError> for Error {
    fn from(x: log::SetLoggerError) -> Self {
        Error {
            inner: InnerError::SetLoggerError(x),
        }
    }
}
