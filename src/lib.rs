//! Operations on [C strings][c_str] that are [UTF-8]-encoded.
//!
//! The [`CUtf8`] type allows for:
//!
//! - [Nul (Ø) terminated C strings][c_str]
//!   in order for C APIs to only access memory that is properly owned.
//!
//! - Encoded as valid [UTF-8], allowing for passing around native Rust [`str`]
//!   strings with ease.
//!
//! # Examples
//!
//! A [`CUtf8`] slice will _always_ end with a trailing 0 byte:
//!
//! ```
//! #[macro_use]
//! extern crate c_utf8;
//!
//! fn main() {
//!     let string = c_utf8!("Heyo!");
//!     let bytes  = [72, 101, 121, 111, 33, 0];
//!
//!     assert_eq!(string.as_bytes_with_nul(), &bytes);
//! }
//! ```
//!
//! [UTF-8]:   https://en.wikipedia.org/wiki/UTF-8
//! [c_str]:   https://en.wikipedia.org/wiki/Null-terminated_string
//! [`str`]:   https://doc.rust-lang.org/std/primitive.str.html
//! [`CUtf8`]: struct.CUtf8.html

#![deny(missing_docs)]
#![cfg_attr(feature = "try_from", feature(try_from))]

/// Creates a [`&'static CUtf8`](struct.CUtf8.html) from a native Rust [`str`]
/// string literal, making it much easier to work with C APIs that are strict
/// about encoding input as UTF-8.
///
/// # Usage
///
/// Although the input string can have a 0 byte, it is highly recommended to not
/// have one. This is because C APIs will only work with the memory up to the
/// first 0 byte.
///
/// # Examples
///
/// The input string will _always_ end with a trailing 0 byte:
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
/// [`str`]: https://doc.rust-lang.org/std/primitive.str.html
#[macro_export]
macro_rules! c_utf8 {
    ($s:expr) => {
        unsafe { $crate::CUtf8::from_str_unchecked(concat!($s, "\0")) }
    }
}

mod c_utf8;

pub use self::{c_utf8::*};
