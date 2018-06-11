use std::ffi::{CStr, FromBytesWithNulError, OsStr};
use std::fmt;
use std::os::raw::c_char;
use std::path::Path;
use std::str::{self, Utf8Error};

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

impl AsRef<str> for CUtf8 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<CStr> for CUtf8 {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl AsRef<[u8]> for CUtf8 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<Path> for CUtf8 {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_str().as_ref()
    }
}

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
//    #[inline]
    fn default() -> &'a CUtf8 { c_utf8!("") }
}

/// The error for [`CUtf8::from_bytes`](struct.CUtf8.html#method.from_bytes).
#[derive(Clone, Debug)]
pub enum FromBytesError {
    /// An error indicating that a nul byte was not in the expected position.
    Nul(FromBytesWithNulError),
    /// An error indicating that input bytes were not encoded as UTF-8.
    Utf8(Utf8Error),
}

impl From<Utf8Error> for FromBytesError {
    #[inline]
    fn from(err: Utf8Error) -> FromBytesError {
        FromBytesError::Utf8(err)
    }
}

impl From<FromBytesWithNulError> for FromBytesError {
    #[inline]
    fn from(err: FromBytesWithNulError) -> FromBytesError {
        FromBytesError::Nul(err)
    }
}

impl CUtf8 {
    /// Returns a C string containing `bytes`, or an error if a nul byte is in
    /// an unexpected position or if the bytes are not encoded as UTF-8.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<&CUtf8, FromBytesError> {
        if let Err(err) = CStr::from_bytes_with_nul(bytes) {
            Err(err.into())
        } else {
            let s = str::from_utf8(bytes)?;
            unsafe { Ok(CUtf8::from_str_unchecked(s)) }
        }
    }

    /// Returns the C string if it is valid UTF-8.
    #[inline]
    pub fn from_c_str(c: &CStr) -> Result<&CUtf8, Utf8Error> {
        let s = str::from_utf8(c.to_bytes_with_nul())?;
        unsafe { Ok(CUtf8::from_str_unchecked(s)) }
    }

    /// Returns the raw C string if it is valid UTF-8 up to the first nul byte.
    #[inline]
    pub unsafe fn from_ptr<'a>(raw: *const c_char) -> Result<&'a CUtf8, Utf8Error> {
        CUtf8::from_c_str(CStr::from_ptr(raw))
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
