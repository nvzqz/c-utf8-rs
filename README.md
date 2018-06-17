# C-Style UTF-8 Strings for Rust

[![Build status][travis-badge]][travis]
![Crate version](https://img.shields.io/crates/v/c_utf8.svg)
![rustc version](https://img.shields.io/badge/rustc-^1.20.0-blue.svg)

This project makes it easier to establish guarantees when interfacing with
[nul-terminated C string][c_str] APIs that require [UTF-8] encoding.

[Documentation](https://docs.rs/c_utf8/)

## What is UTF-8?

[UTF-8] is the character encoding chosen by much of the programming community
since 2008, including Rust with its [`str`] primitive.

<p align="center">
    <a href="https://en.wikipedia.org/wiki/File:Utf8webgrowth.svg">
        <img src="https://upload.wikimedia.org/wikipedia/commons/c/c4/Utf8webgrowth.svg"
             alt="The usage of the main encodings on the web as recorded by Google"
             width=550>
    </a>
</p>

[UTF-8] is capable of representing all 1,112,064 code points of the [Unicode]
standard. Code points are variable-width, ranging from 8 to 32 bits wide.

## Where does UTF-8 appear in C?

### UTF-8 in SDL

The [Simple DirectMedia Layer (SDL)][sdl] library exposes certain APIs that only
interface with UTF-8 encoded C strings. Here's a potential wrapper one could
create around SDL:

```rust
impl Window {
    /* ... */

    fn title(&self) -> &CUtf8 {
        unsafe {
            let title = SDL_GetWindowTitle(self.inner);
            CUtf8::from_ptr(title).unwrap()
        }
    }

    fn set_title(&mut self, title: &CUtf8) {
        unsafe {
            SDL_SetWindowTitle(self.inner, title.as_ptr());
        }
    }

    /* ... */
}
```

Creating a [`&CUtf8`](https://docs.rs/c_utf8/*/c_utf8/struct.CUtf8.html)
instance to interface with the above code can be done easily via the
[`c_utf8!`](https://docs.rs/c_utf8/*/c_utf8/macro.c_utf8.html) macro:

```rust
window.set_title(c_utf8!("MyAwesomeApp"));
```

## Installation

This crate is available [on crates.io][crate] and can be used by adding the
following to your project's [`Cargo.toml`]:

```toml
[dependencies]
c_utf8 = "0.1.0"
```

and this to your crate root (`lib.rs` or `main.rs`):

```rust
#[macro_use]
extern crate c_utf8;
```

## License

This project is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`] or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT License ([`LICENSE-MIT`] or http://opensource.org/licenses/MIT)

at your option.

[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
[`str`]:        https://doc.rust-lang.org/std/primitive.str.html
[sdl]:          https://en.wikipedia.org/wiki/Simple_DirectMedia_Layer
[c_str]:        https://en.wikipedia.org/wiki/Null-terminated_string
[UTF-8]:        https://en.wikipedia.org/wiki/UTF-8
[Unicode]:      https://en.wikipedia.org/wiki/Unicode

[crate]: https://crates.io/crates/c_utf8

[travis]:       https://travis-ci.com/nvzqz/c-utf8-rs
[travis-badge]: https://travis-ci.com/nvzqz/c-utf8-rs.svg?branch=master

[`LICENSE-APACHE`]: https://github.com/nvzqz/c-utf8-rs/blob/master/LICENSE-APACHE
[`LICENSE-MIT`]:    https://github.com/nvzqz/c-utf8-rs/blob/master/LICENSE-MIT
