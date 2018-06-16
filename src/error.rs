use core::fmt;
use core::str::Utf8Error;

#[cfg(feature = "std")]
use std::ffi::FromBytesWithNulError;

/// The error for converting types to [`CUtf8`](struct.CUtf8.html).
#[derive(Clone, Debug)]
pub enum Error {
    /// An error indicating that the nul byte was not at the end.
    Nul,
    /// An error indicating that input bytes were not encoded as UTF-8.
    Utf8(Utf8Error),
}

static NUL_ERROR: &str = "Missing nul byte at the end of the string";

impl From<Utf8Error> for Error {
    #[inline]
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

#[cfg(feature = "std")]
impl From<FromBytesWithNulError> for Error {
    #[inline]
    fn from(_: FromBytesWithNulError) -> Error {
        Error::Nul
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Nul => NUL_ERROR.fmt(f),
            Error::Utf8(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        match *self {
            Error::Nul => NUL_ERROR,
            Error::Utf8(ref err) => err.description(),
        }
    }

    #[inline]
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Utf8(ref err) => Some(err),
            _ => None,
        }
    }
}
