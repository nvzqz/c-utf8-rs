use std::borrow::{Borrow, BorrowMut, ToOwned};
use std::fmt;
use std::ops::{Deref, DerefMut};

use c_utf8::CUtf8;

/// An owned, mutable UTF-8 encoded C string (akin to
/// [`String`](https://doc.rust-lang.org/std/string/struct.String.html) or
/// [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html)).
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CUtf8Buf(String);

impl Default for CUtf8Buf {
    #[inline]
    fn default() -> CUtf8Buf {
        CUtf8Buf::new()
    }
}

impl Deref for CUtf8Buf {
    type Target = CUtf8;

    #[inline]
    fn deref(&self) -> &CUtf8 {
        unsafe { CUtf8::from_str_unchecked(&self.0) }
    }
}

impl DerefMut for CUtf8Buf {
    #[inline]
    fn deref_mut(&mut self) -> &mut CUtf8 {
        unsafe { CUtf8::from_str_unchecked_mut(&mut self.0) }
    }
}

impl fmt::Write for CUtf8Buf {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.push(c);
        Ok(())
    }

    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        self.with_string(|s| s.write_fmt(args))
    }
}

impl Borrow<CUtf8> for CUtf8Buf {
    #[inline]
    fn borrow(&self) -> &CUtf8 { self }
}

impl BorrowMut<CUtf8> for CUtf8Buf {
    #[inline]
    fn borrow_mut(&mut self) -> &mut CUtf8 { self }
}

impl AsRef<CUtf8> for CUtf8Buf {
    #[inline]
    fn as_ref(&self) -> &CUtf8 { self }
}

impl AsMut<CUtf8> for CUtf8Buf {
    #[inline]
    fn as_mut(&mut self) -> &mut CUtf8 { self }
}

impl ToOwned for CUtf8 {
    type Owned = CUtf8Buf;

    #[inline]
    fn to_owned(&self) -> CUtf8Buf {
        CUtf8Buf(self.as_str_with_nul().into())
    }
}

impl<'a> From<&'a CUtf8> for CUtf8Buf {
    #[inline]
    fn from(c: &CUtf8) -> CUtf8Buf {
        c.to_owned()
    }
}

impl<'a> From<&'a mut CUtf8> for CUtf8Buf {
    #[inline]
    fn from(c: &mut CUtf8) -> CUtf8Buf {
        c.to_owned()
    }
}

impl From<String> for CUtf8Buf {
    #[inline]
    fn from(s: String) -> CUtf8Buf {
        CUtf8Buf::from_string(s)
    }
}

impl<'a> From<&'a str> for CUtf8Buf {
    #[inline]
    fn from(s: &str) -> CUtf8Buf {
        String::from(s).into()
    }
}

impl<'a> From<&'a mut str> for CUtf8Buf {
    #[inline]
    fn from(c: &mut str) -> CUtf8Buf {
        (c as &str).into()
    }
}

impl From<Box<CUtf8>> for CUtf8Buf {
    #[inline]
    fn from(b: Box<CUtf8>) -> CUtf8Buf {
        let raw = Box::into_raw(b) as *mut str;
        CUtf8Buf(unsafe { Box::from_raw(raw).into() })
    }
}

impl From<CUtf8Buf> for Box<CUtf8> {
    #[inline]
    fn from(buf: CUtf8Buf) -> Box<CUtf8> {
        let raw = Box::into_raw(buf.0.into_boxed_str()) as *mut CUtf8;
        unsafe { Box::from_raw(raw) }
    }
}

impl From<CUtf8Buf> for String {
    #[inline]
    fn from(buf: CUtf8Buf) -> String {
        buf.into_string()
    }
}

impl From<CUtf8Buf> for Vec<u8> {
    #[inline]
    fn from(buf: CUtf8Buf) -> Vec<u8> {
        buf.into_bytes()
    }
}

impl CUtf8Buf {
    /// Creates a new empty `CUtf8Buf`.
    #[inline]
    pub fn new() -> CUtf8Buf {
        CUtf8Buf(unsafe { String::from_utf8_unchecked(vec![0; 1]) })
    }

    /// Creates a new C string from a UTF-8 string, appending a nul
    /// terminator if one doesn't already exist.
    #[inline]
    pub fn from_string(mut s: String) -> CUtf8Buf {
        if !::is_nul_terminated(&s) {
            unsafe { s.as_mut_vec().push(0) };
        }
        CUtf8Buf(s)
    }

    /// Creates a new C string from a native Rust string without checking for a
    /// nul terminator.
    #[inline]
    pub unsafe fn from_string_unchecked(s: String) -> CUtf8Buf {
        CUtf8Buf(s)
    }

    #[inline]
    fn with_string<F, T>(&mut self, f: F) -> T
        where F: FnOnce(&mut String) -> T
    {
        // Remove nul byte
        unsafe { self.0.as_mut_vec().pop() };

        let val = f(&mut self.0);

        // Append nul byte
        unsafe { self.0.as_mut_vec().push(0) };

        val
    }

    /// Appends a given string slice onto the end of this `CUtf8Buf`.
    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.with_string(|inner| inner.push_str(s));
    }

    /// Appends the given `char` to the end of this `CUtf8Buf`.
    #[inline]
    pub fn push(&mut self, c: char) {
        self.with_string(|inner| inner.push(c));
    }

    /// Converts `self` into a native UTF-8 encoded Rust
    /// [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
    #[inline]
    pub fn into_string(self) -> String {
        let mut string = self.0;
        unsafe { string.as_mut_vec().pop() };
        string
    }

    /// Converts `self` into its underlying bytes.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = self.0.into_bytes();
        bytes.pop();
        bytes
    }
}
