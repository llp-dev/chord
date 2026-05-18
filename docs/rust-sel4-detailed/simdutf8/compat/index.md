*[simdutf8](../index.md) / [compat](index.md)*

---

# Module `compat`

The `compat` API flavor provides full compatibility with `std::str::from_utf8()` and detailed validation errors.

In particular, [`from_utf8()`](#from-utf8)
returns an [`Utf8Error`](#utf8error), which has the [`valid_up_to()`](Utf8Error#method.valid_up_to) and
[`error_len()`](Utf8Error#method.error_len) methods. The first is useful for verification of streamed data. The
second is useful e.g. for replacing invalid byte sequences with a replacement character.

The functions in this module also fail early: errors are checked on-the-fly as the string is processed and once
an invalid UTF-8 sequence is encountered, it returns without processing the rest of the data.
This comes at a slight performance penalty compared to the [`crate::basic`](../basic/index.md) module if the input is valid UTF-8.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Utf8Error`](#utf8error) | struct | UTF-8 error information compatible with [`std::str::Utf8Error`]. |
| [`from_utf8`](#from-utf8) | fn | Analogue to [`std::str::from_utf8()`]. |
| [`from_utf8_mut`](#from-utf8-mut) | fn | Analogue to [`std::str::from_utf8_mut()`]. |

## Structs

### `Utf8Error`

```rust
struct Utf8Error {
    valid_up_to: usize,
    error_len: Option<u8>,
}
```

UTF-8 error information compatible with `std::str::Utf8Error`.

Contains information on the location of the encountered validation error and the length of the
invalid UTF-8 sequence.

#### Implementations

- <span id="utf8error-valid-up-to"></span>`fn valid_up_to(&self) -> usize`

  Analogue to [`std::str::Utf8Error::valid_up_to()`](std::str::Utf8Error#method.valid_up_to).

  

  ...

- <span id="utf8error-error-len"></span>`fn error_len(&self) -> Option<usize>`

  Analogue to [`std::str::Utf8Error::error_len()`](std::str::Utf8Error#method.error_len).

  

  ...

#### Trait Implementations

##### `impl Clone for Utf8Error`

- <span id="utf8error-clone"></span>`fn clone(&self) -> Utf8Error` — [`Utf8Error`](#utf8error)

##### `impl Copy for Utf8Error`

##### `impl Debug for Utf8Error`

- <span id="utf8error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utf8Error`

- <span id="utf8error-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

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
Will return Err([`Utf8Error`](#utf8error)) on if the input contains invalid UTF-8 with
detailed error information.

### `from_utf8_mut`

```rust
fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error>
```

Analogue to `std::str::from_utf8_mut()`.

Checks if the passed mutable byte sequence is valid UTF-8 and returns a mutable
`std::str` reference to the passed byte slice wrapped in `Ok()` if it is.

# Errors
Will return Err([`Utf8Error`](#utf8error)) on if the input contains invalid UTF-8 with
detailed error information.

