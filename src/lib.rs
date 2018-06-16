//! Operations on [UTF-8]-encoded [C strings][c_str].
//!
//! The [`CUtf8`] and [`CUtf8Buf`] types are guaranteed to be:
//!
//! - [Nul (Ø) terminated C strings][c_str] in order to more safely ensure that
//!   C APIs only access memory that is properly owned.
//!
//! - Encoded as valid [UTF-8], allowing for passing around native Rust [`str`]
//!   strings with ease.
//!
//! # Examples
//!
//! A [`CUtf8`] slice can be created via the [`c_utf8!`](macro.c_utf8.html)
//! macro, which ensures it will _always_ end with a trailing 0 byte:
//!
//! ```
//! #[macro_use]
//! extern crate c_utf8;
//!
//! use c_utf8::CUtf8;
//!
//! static MESSAGE: &CUtf8 = c_utf8!("Heyo!");
//!
//! fn main() {
//!     let bytes = [72, 101, 121, 111, 33, 0];
//!     assert_eq!(MESSAGE.as_bytes_with_nul(), &bytes);
//! }
//! ```
//!
//! [UTF-8]:      https://en.wikipedia.org/wiki/UTF-8
//! [c_str]:      https://en.wikipedia.org/wiki/Null-terminated_string
//! [`str`]:      https://doc.rust-lang.org/std/primitive.str.html
//! [`CUtf8`]:    struct.CUtf8.html
//! [`CUtf8Buf`]: struct.CUtf8Buf.html

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "try_from", feature(try_from))]
#![cfg_attr(all(test, nightly), feature(test))]

#[cfg(all(test, nightly))]
extern crate test;

#[cfg(feature = "std")]
use std as core;

/// Creates a [`&'static CUtf8`](struct.CUtf8.html) from a native Rust [`str`]
/// string literal, making it much easier to work with C APIs that are strict
/// about encoding input as UTF-8.
///
/// # Usage
///
/// Although the input string can have a 0 byte, it is **highly recommended** to
/// not have one. This is because C APIs will only work with the memory up to
/// the first 0 byte. In the future, it will be very likely be a **hard error**
/// to have a 0 byte within the string literal.
///
/// # Examples
///
/// The resulting string will _always_ end with a 0 byte:
///
/// ```
/// #[macro_use]
/// extern crate c_utf8;
///
/// fn main() {
///     let string = c_utf8!("Hello!");
///     let bytes  = [72, 101, 108, 108, 111, 33, 0];
///
///     assert_eq!(string.as_bytes_with_nul(), &bytes);
/// }
/// ```
///
/// The macro can even be evaluated within a constant expression. This allows
/// for having instances of types with `&'static CUtf8` fields.
///
/// ```
/// # #[macro_use] extern crate c_utf8; use c_utf8::CUtf8; fn main() {
/// static APP_NAME: &CUtf8 = c_utf8!(env!("CARGO_PKG_NAME"));
///
/// assert_eq!(APP_NAME.as_str_with_nul(), "c_utf8\0");
/// # }
/// ```
///
/// [`str`]: https://doc.rust-lang.org/std/primitive.str.html
#[macro_export]
macro_rules! c_utf8 {
    ($s:expr) => {
        unsafe {
            // An internal type that allows for converting static Rust string
            // slices into static CUtf8 slices within a constant expression
            union _Ref<'a> { s: &'a str, c: &'a $crate::CUtf8 }
            _Ref { s: concat!($s, "\0") }.c
        }
    }
}

#[cfg(feature = "std")]
mod c_utf8_buf;
mod c_utf8;
mod error;

#[cfg(feature = "std")]
pub use self::c_utf8_buf::*;
pub use self::c_utf8::*;
pub use self::error::*;

/// Equivalent to C's `char` type.
#[allow(non_camel_case_types)]
#[cfg(not(feature = "std"))]
#[cfg(any(all(target_os = "linux", any(target_arch = "aarch64",
                                       target_arch = "arm",
                                       target_arch = "powerpc",
                                       target_arch = "powerpc64",
                                       target_arch = "s390x")),
          all(target_os = "android", any(target_arch = "aarch64",
                                         target_arch = "arm")),
          all(target_os = "l4re", target_arch = "x86_64"),
          all(target_os = "openbsd", target_arch = "aarch64"),
          all(target_os = "fuchsia", target_arch = "aarch64")))]
pub type c_char = u8;

/// Equivalent to C's `char` type.
#[allow(non_camel_case_types)]
#[cfg(not(feature = "std"))]
#[cfg(not(any(all(target_os = "linux", any(target_arch = "aarch64",
                                           target_arch = "arm",
                                           target_arch = "powerpc",
                                           target_arch = "powerpc64",
                                           target_arch = "s390x")),
              all(target_os = "android", any(target_arch = "aarch64",
                                             target_arch = "arm")),
              all(target_os = "l4re", target_arch = "x86_64"),
              all(target_os = "openbsd", target_arch = "aarch64"),
              all(target_os = "fuchsia", target_arch = "aarch64"))))]
pub type c_char = i8;

#[cfg(feature = "std")]
pub use std::os::raw::c_char;

#[inline]
fn is_nul_terminated(s: &str) -> bool {
    s.as_bytes().last().cloned() == Some(0)
}
