# Crate `sel4_panicking_env`

## Contents

- [Structs](#structs)
  - [`DebugWrite`](#debugwrite)
  - [`AbortInfo`](#abortinfo)
- [Functions](#functions)
  - [`__sel4_panicking_env__debug_put_char`](#sel4-panicking-env-debug-put-char)
  - [`__sel4_panicking_env__abort_hook`](#sel4-panicking-env-abort-hook)
  - [`__sel4_panicking_env__abort_trap`](#sel4-panicking-env-abort-trap)
  - [`default_abort_hook`](#default-abort-hook)
  - [`debug_put_char`](#debug-put-char)
  - [`abort`](#abort)
  - [`abort_without_info`](#abort-without-info)
- [Macros](#macros)
  - [`register_debug_put_char!`](#register-debug-put-char)
  - [`register_abort_hook!`](#register-abort-hook)
  - [`register_abort_trap!`](#register-abort-trap)
  - [`debug_print!`](#debug-print)
  - [`debug_println!`](#debug-println)
  - [`abort!`](#abort)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugWrite`](#debugwrite) | struct |  |
| [`AbortInfo`](#abortinfo) | struct | Information about an abort passed to an abort hook. |
| [`__sel4_panicking_env__debug_put_char`](#sel4-panicking-env-debug-put-char) | fn |  |
| [`__sel4_panicking_env__abort_hook`](#sel4-panicking-env-abort-hook) | fn |  |
| [`__sel4_panicking_env__abort_trap`](#sel4-panicking-env-abort-trap) | fn |  |
| [`default_abort_hook`](#default-abort-hook) | fn |  |
| [`debug_put_char`](#debug-put-char) | fn | Prints via a link-time hook. |
| [`abort`](#abort) | fn |  |
| [`abort_without_info`](#abort-without-info) | fn | Aborts without any [`AbortInfo`]. |
| [`register_debug_put_char!`](#register-debug-put-char) | macro | Registers a function to be used by [`debug_put_char`], [`debug_print!`], and [`debug_println!`]. |
| [`register_abort_hook!`](#register-abort-hook) | macro | Registers an abort hook to be used by [`abort!`] and [`abort_without_info`]. |
| [`register_abort_trap!`](#register-abort-trap) | macro | Registers an abort trap to be used by [`abort!`] and [`abort_without_info`]. |
| [`debug_print!`](#debug-print) | macro | Like `std::print!`, except backed by [`debug_put_char`]. |
| [`debug_println!`](#debug-println) | macro | Like `std::println!`, except backed by [`debug_put_char`]. |
| [`abort!`](#abort) | macro | Aborts execution with a message. |

## Structs

### `DebugWrite`

```rust
struct DebugWrite;
```

#### Trait Implementations

##### `impl Write for DebugWrite`

- <span id="debugwrite-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

### `AbortInfo<'a>`

```rust
struct AbortInfo<'a> {
    message: Option<&'a fmt::Arguments<'a>>,
    location: Option<&'a core::panic::Location<'a>>,
}
```

Information about an abort passed to an abort hook.

#### Implementations

- <span id="abortinfo-message"></span>`fn message(&self) -> Option<&fmt::Arguments<'_>>`

  The `core::fmt::Arguments` with which [`abort!`](#abort) was called.

- <span id="abortinfo-location"></span>`fn location(&self) -> Option<&Location<'_>>`

  The location at which [`abort!`](#abort) was called.

#### Trait Implementations

##### `impl Display for AbortInfo<'_>`

- <span id="abortinfo-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `__sel4_panicking_env__debug_put_char`

```rust
unsafe fn __sel4_panicking_env__debug_put_char(c: u8)
```

### `__sel4_panicking_env__abort_hook`

```rust
unsafe fn __sel4_panicking_env__abort_hook(info: Option<&AbortInfo<'_>>)
```

### `__sel4_panicking_env__abort_trap`

```rust
unsafe fn __sel4_panicking_env__abort_trap() -> never
```

### `default_abort_hook`

```rust
fn default_abort_hook(info: Option<&AbortInfo<'_>>)
```

### `debug_put_char`

```rust
fn debug_put_char(c: u8)
```

Prints via a link-time hook.

This function uses the following externally defined symbol:

```rust
unsafe extern "Rust" {
    fn __sel4_panicking_env__debug_put_char(c: u8);
}
```

[`register_debug_put_char`](#register-debug-put-char) provides a typesafe way to define that symbol.

### `abort`

```rust
fn abort(info: Option<&AbortInfo<'_>>) -> never
```

### `abort_without_info`

```rust
fn abort_without_info() -> never
```

Aborts without any [`AbortInfo`](#abortinfo).

This function does the same thing as [`abort!`](#abort), except it passes `None` to the abort hook.

## Macros

### `register_debug_put_char!`

Registers a function to be used by [`debug_put_char`](#debug-put-char), [`debug_print!`](#debug-print), and [`debug_println!`](#debug-println).

This macro uses the function `$path` to define the following symbol:

```rust
unsafe extern "Rust" {
    fn __sel4_panicking_env__debug_put_char(c: u8);
}
```

### `register_abort_hook!`

Registers an abort hook to be used by [`abort!`](#abort) and [`abort_without_info`](#abort-without-info).

This macro uses the function `$path` to define the following symbol:

```no_run
use sel4_panicking_env::AbortInfo;

unsafe extern "Rust" {
    fn __sel4_panicking_env__abort_hook(info: Option<&AbortInfo>);
}
```

### `register_abort_trap!`

Registers an abort trap to be used by [`abort!`](#abort) and [`abort_without_info`](#abort-without-info).

This macro uses the function `$path` to define the following symbol:

```rust
unsafe extern "Rust" {
    fn __sel4_panicking_env__abort_trap() -> !;
}
```

### `debug_print!`

Like `std::print!`, except backed by [`debug_put_char`](#debug-put-char).

### `debug_println!`

Like `std::println!`, except backed by [`debug_put_char`](#debug-put-char).

### `abort!`

Aborts execution with a message.

[`abort!`](#abort) accepts the same patterns `core::panic!`:

```no_run
use sel4_panicking_env::abort;

abort!();
abort!("uh oh!");
abort!("uh {} {}!", 123, "oh");
```

This macro first invokes an externally defined abort hook which is resolved at link time, and
then calls `core::intrinsics::abort()`.

The following externally defined symbol is used as the abort hook:

```rust
use sel4_panicking_env::AbortInfo;

unsafe extern "Rust" {
    fn __sel4_panicking_env__abort_hook(info: Option<&AbortInfo>);
}
```

The [`sel4_panicking_env` crate](crate) defines a weak version of this symbol which just prints
the [`AbortInfo`](#abortinfo) argument using [`debug_print!`](#debug-print).

[`register_abort_hook`](#register-abort-hook) provides a typesafe way to define that symbol.

