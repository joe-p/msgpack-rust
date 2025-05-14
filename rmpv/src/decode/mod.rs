#[cfg(feature = "std")]
use std::fmt::{self, Display, Formatter};
#[cfg(not(feature = "std"))]
use core::fmt::{self, Display, Formatter};

use rmp::decode::{MarkerReadError, RmpReadErr, ValueReadError};

pub mod value;
pub mod value_ref;

pub use self::value::{read_value, read_value_with_max_depth};
pub use self::value_ref::{read_value_ref, read_value_ref_with_max_depth};



/// The maximum recursion depth before [`Error::DepthLimitExceeded`] is returned.
pub const MAX_DEPTH: usize = 1024;

/// This type represents all possible errors that can occur when deserializing a value.
#[derive(Debug)]
pub enum Error<E: RmpReadErr = ErrorImpl> {
    /// Error while reading marker byte.
    InvalidMarkerRead(E),
    /// Error while reading data.
    InvalidDataRead(E),
    /// The depth limit [`MAX_DEPTH`] was exceeded.
    DepthLimitExceeded,
}

#[cfg(feature = "std")]
pub type ErrorImpl = std::io::Error;
#[cfg(not(feature = "std"))]
pub type ErrorImpl = core::convert::Infallible;

#[inline]
fn decrement_depth<E: RmpReadErr>(depth: u16) -> Result<u16, Error<E>> {
    depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)
}

#[cfg(feature = "std")]
impl<E: RmpReadErr> Error<E> {
    #[cold]
    #[must_use]
    pub fn kind(&self) -> std::io::ErrorKind {
        match *self {
            Self::InvalidMarkerRead(_) => std::io::ErrorKind::Other,
            Self::InvalidDataRead(_) => std::io::ErrorKind::Other,
            Self::DepthLimitExceeded => std::io::ErrorKind::Unsupported,
        }
    }
}

#[cfg(feature = "std")]
impl<E: RmpReadErr> std::error::Error for Error<E> {
    #[cold]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::InvalidMarkerRead(ref err) => Some(err),
            Self::InvalidDataRead(ref err) => Some(err),
            Self::DepthLimitExceeded => None,
        }
    }
}

impl<E: RmpReadErr> Display for Error<E> {
    #[cold]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Self::InvalidMarkerRead(ref err) => {
                write!(fmt, "I/O error while reading marker byte: {err}")
            }
            Self::InvalidDataRead(ref err) => {
                write!(fmt, "I/O error while reading non-marker bytes: {err}")
            }
            Self::DepthLimitExceeded => {
                write!(fmt, "depth limit exceeded")
            }
        }
    }
}

impl<E: RmpReadErr> From<MarkerReadError<E>> for Error<E> {
    #[cold]
    fn from(err: MarkerReadError<E>) -> Self {
        Self::InvalidMarkerRead(err.0)
    }
}

impl<E: RmpReadErr> From<ValueReadError<E>> for Error<E> {
    #[cold]
    fn from(err: ValueReadError<E>) -> Self {
        match err {
            ValueReadError::InvalidMarkerRead(err) => Self::InvalidMarkerRead(err),
            ValueReadError::InvalidDataRead(err) => Self::InvalidDataRead(err),
            ValueReadError::TypeMismatch(..) => {
                // For both std and no_std, use DepthLimitExceeded for type mismatches
                Self::DepthLimitExceeded
            }
        }
    }
}

#[cfg(feature = "std")]
impl From<Error<std::io::Error>> for std::io::Error {
    #[cold]
    fn from(val: Error<std::io::Error>) -> Self {
        match val {
            Error::InvalidMarkerRead(err) |
            Error::InvalidDataRead(err) => err,
            Error::DepthLimitExceeded => Self::new(val.kind(), val),
        }
    }
}
