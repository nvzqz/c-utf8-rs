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
