**sel4_panicking_env**

# Module: sel4_panicking_env

## Contents

**Macros**

- [`abort`](#abort) - Aborts execution with a message.
- [`debug_print`](#debug_print) - Like `std::print!`, except backed by [`debug_put_char`].
- [`debug_println`](#debug_println) - Like `std::println!`, except backed by [`debug_put_char`].
- [`register_abort_hook`](#register_abort_hook) - Registers an abort hook to be used by [`abort!`] and [`abort_without_info`].
- [`register_abort_trap`](#register_abort_trap) - Registers an abort trap to be used by [`abort!`] and [`abort_without_info`].
- [`register_debug_put_char`](#register_debug_put_char) - Registers a function to be used by [`debug_put_char`], [`debug_print!`], and [`debug_println!`].

**Structs**

- [`AbortInfo`](#abortinfo) - Information about an abort passed to an abort hook.

**Functions**

- [`abort_without_info`](#abort_without_info) - Aborts without any [`AbortInfo`].
- [`debug_put_char`](#debug_put_char) - Prints via a link-time hook.

---

## sel4_panicking_env::AbortInfo

*Struct*

Information about an abort passed to an abort hook.

**Generic Parameters:**
- 'a

**Methods:**

- `fn message(self: &Self) -> Option<&fmt::Arguments>` - The `core::fmt::Arguments` with which [`abort!`] was called.
- `fn location(self: &Self) -> Option<&Location>` - The location at which [`abort!`] was called.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## sel4_panicking_env::abort

*Declarative Macro*

Aborts execution with a message.

[`abort!`] accepts the same patterns `core::panic!`:

```no_run
use sel4_panicking_env::abort;

abort!();
abort!("uh oh!");
abort!("uh {} {}!", 123, "oh");
```

This macro first invokes an externally defined abort hook which is resolved at link time, and
then calls `core::intrinsics::abort()`.

The following externally defined symbol is used as the abort hook:

```
use sel4_panicking_env::AbortInfo;

unsafe extern "Rust" {
    fn __sel4_panicking_env__abort_hook(info: Option<&AbortInfo>);
}
```

The [`sel4_panicking_env` crate](crate) defines a weak version of this symbol which just prints
the [`AbortInfo`] argument using [`debug_print!`].

[`register_abort_hook`] provides a typesafe way to define that symbol.

```rust
macro_rules! abort {
    () => { ... };
    ($($arg:tt)*) => { ... };
}
```



## sel4_panicking_env::abort_without_info

*Function*

Aborts without any [`AbortInfo`].

This function does the same thing as [`abort!`], except it passes `None` to the abort hook.

```rust
fn abort_without_info() -> never
```



## sel4_panicking_env::debug_print

*Declarative Macro*

Like `std::print!`, except backed by [`debug_put_char`].

```rust
macro_rules! debug_print {
    ($($arg:tt)*) => { ... };
}
```



## sel4_panicking_env::debug_println

*Declarative Macro*

Like `std::println!`, except backed by [`debug_put_char`].

```rust
macro_rules! debug_println {
    () => { ... };
    ($($arg:tt)*) => { ... };
}
```



## sel4_panicking_env::debug_put_char

*Function*

Prints via a link-time hook.

This function uses the following externally defined symbol:

```
unsafe extern "Rust" {
    fn __sel4_panicking_env__debug_put_char(c: u8);
}
```

[`register_debug_put_char`] provides a typesafe way to define that symbol.

```rust
fn debug_put_char(c: u8)
```



## sel4_panicking_env::register_abort_hook

*Declarative Macro*

Registers an abort hook to be used by [`abort!`] and [`abort_without_info`].

This macro uses the function `$path` to define the following symbol:

```no_run
use sel4_panicking_env::AbortInfo;

unsafe extern "Rust" {
    fn __sel4_panicking_env__abort_hook(info: Option<&AbortInfo>);
}
```

```rust
macro_rules! register_abort_hook {
    ($(#[$attrs:meta])* $path:path) => { ... };
}
```



## sel4_panicking_env::register_abort_trap

*Declarative Macro*

Registers an abort trap to be used by [`abort!`] and [`abort_without_info`].

This macro uses the function `$path` to define the following symbol:

```
unsafe extern "Rust" {
    fn __sel4_panicking_env__abort_trap() -> !;
}
```

```rust
macro_rules! register_abort_trap {
    ($(#[$attrs:meta])* $path:path) => { ... };
}
```



## sel4_panicking_env::register_debug_put_char

*Declarative Macro*

Registers a function to be used by [`debug_put_char`], [`debug_print!`], and [`debug_println!`].

This macro uses the function `$path` to define the following symbol:

```
unsafe extern "Rust" {
    fn __sel4_panicking_env__debug_put_char(c: u8);
}
```

```rust
macro_rules! register_debug_put_char {
    ($(#[$attrs:meta])* $path:path) => { ... };
}
```



