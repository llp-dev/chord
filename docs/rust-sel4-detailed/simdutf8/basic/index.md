*[simdutf8](../index.md) / [basic](index.md)*

---

# Module `basic`

The `basic` API flavor provides barebones UTF-8 checking at the highest speed.

It is fastest on valid UTF-8, but only checks for errors after processing the whole byte sequence
and does not provide detailed information if the data is not valid UTF-8. [`Utf8Error`](#utf8error) is a zero-sized error struct.

If you need detailed error information use the functions from the [`crate::compat`](../compat/index.md) module instead.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Utf8Error`](#utf8error) | struct | Simple zero-sized UTF-8 error. |
| [`from_utf8`](#from-utf8) | fn | Analogue to [`std::str::from_utf8()`]. |
| [`from_utf8_mut`](#from-utf8-mut) | fn | Analogue to [`std::str::from_utf8_mut()`]. |

## Structs

### `Utf8Error`

```rust
struct Utf8Error;
```

Simple zero-sized UTF-8 error.

No information is provided where the error occurred or how long the invalid byte
byte sequence is.

#### Trait Implementations

##### `impl Clone for Utf8Error`

- <span id="utf8error-clone"></span>`fn clone(&self) -> Utf8Error` — [`Utf8Error`](#utf8error)

##### `impl Copy for Utf8Error`

##### `impl Debug for Utf8Error`

- <span id="utf8error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utf8Error`

- <span id="utf8error-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Utf8Error`

##### `impl PartialEq for Utf8Error`

- <span id="utf8error-partialeq-eq"></span>`fn eq(&self, other: &Utf8Error) -> bool` — [`Utf8Error`](#utf8error)

##### `impl StructuralPartialEq for Utf8Error`

## Functions

### `from_utf8`

```rust
fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error>
```

Analogue to `std::str::from_utf8()`.

Checks if the passed byte sequence is valid UTF-8 and returns an
`std::str` reference to the passed byte slice wrapped in `Ok()` if it is.

# Errors
Will return the zero-sized Err([`Utf8Error`](#utf8error)) on if the input contains invalid UTF-8.

### `from_utf8_mut`

```rust
fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error>
```

Analogue to `std::str::from_utf8_mut()`.

Checks if the passed mutable byte sequence is valid UTF-8 and returns a mutable
`std::str` reference to the passed byte slice wrapped in `Ok()` if it is.

# Errors
Will return the zero-sized Err([`Utf8Error`](#utf8error)) on if the input contains invalid UTF-8.

