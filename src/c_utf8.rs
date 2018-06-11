use core::fmt;
use core::str::{self, Utf8Error};

#[cfg(feature = "std")]
use std::ffi::{CStr, FromBytesWithNulError, OsStr};

#[cfg(feature = "std")]
use std::path::Path;

use c_char;

/// Like [`CStr`](https://doc.rust-lang.org/std/ffi/struct.CStr.html), except
/// with the guarantee of being encoded as valid [UTF-8].
///
/// Use the [`c_utf8!`](macro.c_utf8.html) macro to conveniently create static
/// instances.
///
/// # Guarantees
///
/// This type guarantees that instances are:
///
/// - [Nul (Ø) terminated C strings](https://en.wikipedia.org/wiki/Null-terminated_string)
///   in order for C APIs to safely access memory that is properly owned. It is
///   generally preferable for there to be no other nul byte prior to the very
///   end of the string.
///
/// - Encoded as valid [UTF-8], which allows for passing around native Rust
///   &#8220;[`str`](https://doc.rust-lang.org/std/primitive.str.html)&#8221;
///   strings with ease.
///
/// [UTF-8]: https://en.wikipedia.org/wiki/UTF-8
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CUtf8(str);

#[cfg(feature = "try_from")]
mod try_from {
    use super::*;
    use core::convert::TryFrom;

    impl<'a> TryFrom<&'a [u8]> for &'a CUtf8 {
        type Error = Error;

        #[inline]
        fn try_from(bytes: &[u8]) -> Result<&CUtf8, Self::Error> {
            CUtf8::from_bytes(bytes)
        }
    }

    #[cfg(feature = "std")]
    impl<'a> TryFrom<&'a CStr> for &'a CUtf8 {
        type Error = Utf8Error;

        #[inline]
        fn try_from(c: &CStr) -> Result<&CUtf8, Self::Error> {
            CUtf8::from_c_str(c)
        }
    }

    impl<'a> TryFrom<&'a str> for &'a CUtf8 {
        type Error = Error;

        #[inline]
        fn try_from(s: &str) -> Result<&CUtf8, Self::Error> {
            CUtf8::from_str(s)
        }
    }
}

#[cfg(feature = "std")]
mod std {

}

impl AsRef<str> for CUtf8 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(feature = "std")]
impl AsRef<CStr> for CUtf8 {
    #[inline]
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_ptr() as *const c_char) }
    }
}

impl AsRef<[u8]> for CUtf8 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "std")]
impl AsRef<Path> for CUtf8 {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_str().as_ref()
    }
}

#[cfg(feature = "std")]
impl AsRef<OsStr> for CUtf8 {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_str().as_ref()
    }
}

impl<'a> fmt::Debug for &'a CUtf8 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> fmt::Debug for &'a mut CUtf8 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> fmt::Display for &'a CUtf8 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> fmt::Display for &'a mut CUtf8 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> Default for &'a CUtf8 {
    #[inline]
    fn default() -> &'a CUtf8 { c_utf8!("") }
}

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

impl CUtf8 {
    /// Returns a C string containing `bytes`, or an error if a nul byte is in
    /// an unexpected position or if the bytes are not encoded as UTF-8.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<&CUtf8, Error> {
        CUtf8::from_str(str::from_utf8(bytes)?)
    }

    /// Returns the UTF-8 string if it is terminated by a nul byte.
    #[inline]
    pub fn from_str(s: &str) -> Result<&CUtf8, Error> {
        if let Some(&0) = s.as_bytes().last() {
            unsafe { Ok(CUtf8::from_str_unchecked(s)) }
        } else {
            Err(Error::Nul)
        }
    }

    /// Returns the C string if it is valid UTF-8.
    #[cfg(feature = "std")]
    #[inline]
    pub fn from_c_str(c: &CStr) -> Result<&CUtf8, Utf8Error> {
        let s = str::from_utf8(c.to_bytes_with_nul())?;
        unsafe { Ok(CUtf8::from_str_unchecked(s)) }
    }

    /// Returns the raw C string if it is valid UTF-8 up to the first nul byte.
    #[inline]
    pub unsafe fn from_ptr<'a>(raw: *const c_char) -> Result<&'a CUtf8, Utf8Error> {
        #[cfg(feature = "std")] {
            CUtf8::from_c_str(CStr::from_ptr(raw))
        }
        #[cfg(not(feature = "std"))] {
            use core::slice;

            extern {
                fn strlen(cs: *const c_char) -> usize;
            }

            let n = strlen(raw) + 1;
            let s = str::from_utf8(slice::from_raw_parts(raw as *const u8, n))?;
            Ok(CUtf8::from_str_unchecked(s))
        }
    }

    /// Returns a C string without checking UTF-8 validity or for a trailing
    /// nul byte.
    #[inline]
    pub unsafe fn from_bytes_unchecked(b: &[u8]) -> &CUtf8 {
        &*(b as *const [u8] as *const CUtf8)
    }

    /// Returns a C string without checking for a trailing nul byte.
    #[inline]
    pub unsafe fn from_str_unchecked(s: &str) -> &CUtf8 {
        &*(s as *const str as *const CUtf8)
    }

    /// Returns a C string without checking UTF-8 validity.
    #[cfg(feature = "std")]
    #[inline]
    pub unsafe fn from_c_str_unchecked(c: &CStr) -> &CUtf8 {
        Self::from_bytes_unchecked(c.to_bytes_with_nul())
    }

    /// Returns a pointer to the start of the raw C string.
    #[inline]
    pub fn as_ptr(&self) -> *const c_char {
        self.as_bytes().as_ptr() as *const c_char
    }

    /// Returns `self` as a normal C string.
    #[cfg(feature = "std")]
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul()) }
    }

    /// Returns `self` as a normal UTF-8 encoded string.
    #[inline]
    pub fn as_str(&self) -> &str {
        // Remove nul
        let len = self.0.len().saturating_sub(1);
        unsafe { self.0.get_unchecked(..len) }
    }

    /// Returns `self` as a UTF-8 encoded string with a trailing 0 byte.
    #[inline]
    pub fn as_str_with_nul(&self) -> &str {
        &self.0
    }

    /// Returns the bytes of `self` without a trailing 0 byte.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    /// Returns the bytes of `self` with a trailing 0 byte.
    #[inline]
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.as_str_with_nul().as_bytes()
    }
}
