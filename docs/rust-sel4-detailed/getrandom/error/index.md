*[getrandom](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct | A small and `no_std` compatible error type |
| [`internal_error`](#internal-error) | fn |  |
| [`os_err`](#os-err) | fn |  |
| [`internal_desc`](#internal-desc) | fn |  |

## Structs

### `Error`

```rust
struct Error(core::num::NonZeroU32);
```

A small and `no_std` compatible error type

The `Error::raw_os_error()` will indicate if the error is from the OS, and
if so, which error code the OS gave the application. If such an error is
encountered, please consult with your system documentation.

Internally this type is a NonZeroU32, with certain values reserved for
certain purposes, see `Error::INTERNAL_START` and `Error::CUSTOM_START`.

*If this crate's `"std"` Cargo feature is enabled*, then:
- [`getrandom::Error`][Error] implements
  [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
- [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) implements
  [`From<getrandom::Error>`](https://doc.rust-lang.org/std/convert/trait.From.html).

#### Implementations

- <span id="error-const-unsupported"></span>`const UNSUPPORTED: Error`

- <span id="error-const-errno-not-positive"></span>`const ERRNO_NOT_POSITIVE: Error`

- <span id="error-const-unexpected"></span>`const UNEXPECTED: Error`

- <span id="error-const-ios-sec-random"></span>`const IOS_SEC_RANDOM: Error`

- <span id="error-const-windows-rtl-gen-random"></span>`const WINDOWS_RTL_GEN_RANDOM: Error`

- <span id="error-const-failed-rdrand"></span>`const FAILED_RDRAND: Error`

- <span id="error-const-no-rdrand"></span>`const NO_RDRAND: Error`

- <span id="error-const-web-crypto"></span>`const WEB_CRYPTO: Error`

- <span id="error-const-web-get-random-values"></span>`const WEB_GET_RANDOM_VALUES: Error`

- <span id="error-const-vxworks-rand-secure"></span>`const VXWORKS_RAND_SECURE: Error`

- <span id="error-const-node-crypto"></span>`const NODE_CRYPTO: Error`

- <span id="error-const-node-random-fill-sync"></span>`const NODE_RANDOM_FILL_SYNC: Error`

- <span id="error-const-node-es-module"></span>`const NODE_ES_MODULE: Error`

- <span id="error-const-internal-start"></span>`const INTERNAL_START: u32`

- <span id="error-const-custom-start"></span>`const CUSTOM_START: u32`

- <span id="error-raw-os-error"></span>`fn raw_os_error(self) -> Option<i32>`

  Extract the raw OS error code (if this error came from the OS)

  

  This method is identical to [`std::io::Error::raw_os_error()`][1], except

  that it works in `no_std` contexts. If this method returns `None`, the

  error value can still be formatted via the `Display` implementation.

- <span id="error-code"></span>`const fn code(self) -> NonZeroU32`

  Extract the bare error code.

  

  This code can either come from the underlying OS, or be a custom error.

  Use `Error::raw_os_error()` to disambiguate.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

## Functions

### `internal_error`

```rust
const fn internal_error(n: u16) -> Error
```

### `os_err`

```rust
fn os_err(errno: i32, buf: &mut [u8]) -> Option<&str>
```

### `internal_desc`

```rust
fn internal_desc(error: Error) -> Option<&'static str>
```

