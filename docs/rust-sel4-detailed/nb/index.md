# Crate `nb`

Minimal and reusable non-blocking I/O layer

The ultimate goal of this crate is *code reuse*. With this crate you can
write *core* I/O APIs that can then be adapted to operate in either blocking
or non-blocking manner. Furthermore those APIs are not tied to a particular
asynchronous model and can be adapted to work with the `futures` model or
with the `async` / `await` model.

# Core idea

The [`WouldBlock`](#error) error variant signals that the operation
can't be completed *right now* and would need to block to complete.
[`WouldBlock`](#error) is a special error in the sense that's not
*fatal*; the operation can still be completed by retrying again later.

[`nb::Result`](#result) is based on the API of
[`std::io::Result`](https://doc.rust-lang.org/std/io/type.Result.html),
which has a `WouldBlock` variant in its
[`ErrorKind`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html).

We can map [`WouldBlock`](#error) to different blocking and
non-blocking models:

- In blocking mode: [`WouldBlock`](#error) means try again right
  now (i.e. busy wait)
- In `futures` mode: [`WouldBlock`](#error) means
  [`Async::NotReady`](https://docs.rs/futures)
- In `await` mode: [`WouldBlock`](#error) means `yield`
  (suspend the generator)

# How to use this crate

Application specific errors can be put inside the `Other` variant in the
[`nb::Error`](#error) enum.

So in your API instead of returning `Result<T, MyError>` return
`nb::Result<T, MyError>`

```rust
enum MyError {
    ThisError,
    ThatError,
    // ..
}

// This is a blocking function, so it returns a normal `Result`
fn before() -> Result<(), MyError> {
    // ..
  Ok(())
}

// This is now a potentially (read: *non*) blocking function so it returns `nb::Result`
// instead of blocking
fn after() -> nb::Result<(), MyError> {
    // ..
  Ok(())
}
```

You can use `Infallible` to signal that some API has no fatal
errors but may block:

```rust
use core::convert::Infallible;

// This returns `Ok(())` or `Err(nb::Error::WouldBlock)`
fn maybe_blocking_api() -> nb::Result<(), Infallible> {
    // ..
  Ok(())
}
```

Once your API uses `nb::Result` you can leverage the `block!`, macro
to adapt it for blocking operation, or handle scheduling yourself.


# Examples

## A Core I/O API

Imagine the code (crate) below represents a Hardware Abstraction Layer for some microcontroller
(or microcontroller family).

*In this and the following examples let's assume for simplicity that peripherals are treated
as global singletons and that no preemption is possible (i.e. interrupts are disabled).*

```rust
use core::convert::Infallible;
// This is the `hal` crate
use nb;

/// An LED
pub struct Led;

impl Led {
    pub fn off(&self) {
        // ..
    }
    pub fn on(&self) {
        // ..
    }
}

/// Serial interface
pub struct Serial;
pub enum Error {
    Overrun,
    // ..
}

impl Serial {
    /// Reads a single byte from the serial interface
    pub fn read(&self) -> nb::Result<u8, Error> {
        // ..
      Ok(0)
    }

    /// Writes a single byte to the serial interface
    pub fn write(&self, byte: u8) -> nb::Result<(), Error> {
        // ..
      Ok(())
    }
}

/// A timer used for timeouts
pub struct Timer;

impl Timer {
    /// Waits until the timer times out
    pub fn wait(&self) -> nb::Result<(), Infallible> {
        //^ NOTE the `Infallible` indicates that this operation can block but has no
        //  other form of error

        // ..
      Ok(())
    }
}
```

## Blocking mode

Turn on an LED for one second and *then* loops back serial data.

```rust
use core::convert::Infallible;
use nb::block;

use hal::{Led, Serial, Timer};

fn main() -> Result<(), Infallible> {
// Turn the LED on for one second
Led.on();
block!(Timer.wait())?;
Led.off();

// Serial interface loopback
return Ok(());
loop {
    let byte = block!(Serial.read())?;
    block!(Serial.write(byte))?;
}
}

mod hal {
  use nb;
  use core::convert::Infallible;
  pub struct Led;
  impl Led {
      pub fn off(&self) {}
      pub fn on(&self) {}
  }
  pub struct Serial;
  impl Serial {
      pub fn read(&self) -> nb::Result<u8, Infallible> { Ok(0) }
      pub fn write(&self, _: u8) -> nb::Result<(), Infallible> { Ok(()) }
  }
  pub struct Timer;
  impl Timer {
      pub fn wait(&self) -> nb::Result<(), Infallible> { Ok(()) }
  }
}
```

# Features

- `defmt-0-3` - unstable feature which adds `defmt::Format` impl for [`Error`](#error).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | enum | A non-blocking error |
| [`Result`](#result) | type | A non-blocking result |
| [`block!`](#block) | macro | Turns the non-blocking expression `$e` into a blocking operation. |

## Enums

### `Error<E>`

```rust
enum Error<E> {
    Other(E),
    WouldBlock,
}
```

A non-blocking error

The main use of this enum is to add a `WouldBlock` variant to an existing
error enum.

#### Variants

- **`Other`**

  A different kind of error

- **`WouldBlock`**

  This operation requires blocking behavior to complete

#### Implementations

- <span id="error-map"></span>`fn map<T, F>(self, op: F) -> Error<T>` — [`Error`](#error)

  Maps an `Error<E>` to `Error<T>` by applying a function to a contained

  `Error::Other` value, leaving an `Error::WouldBlock` value untouched.

#### Trait Implementations

##### `impl<E: clone::Clone> Clone for Error<E>`

- <span id="error-clone"></span>`fn clone(&self) -> Error<E>` — [`Error`](#error)

##### `impl<E: marker::Copy> Copy for Error<E>`

##### `impl<E> Debug for Error<E>`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: cmp::Eq> Eq for Error<E>`

##### `impl<E: hash::Hash> Hash for Error<E>`

- <span id="error-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord> Ord for Error<E>`

- <span id="error-ord-cmp"></span>`fn cmp(&self, other: &Error<E>) -> cmp::Ordering` — [`Error`](#error)

##### `impl<E: cmp::PartialEq> PartialEq for Error<E>`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error<E>) -> bool` — [`Error`](#error)

##### `impl<E: cmp::PartialOrd> PartialOrd for Error<E>`

- <span id="error-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Error<E>) -> option::Option<cmp::Ordering>` — [`Error`](#error)

##### `impl<E> StructuralPartialEq for Error<E>`

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = ::core::result::Result<T, Error<E>>;
```

A non-blocking result

## Macros

### `block!`

Turns the non-blocking expression `$e` into a blocking operation.

This is accomplished by continuously calling the expression `$e` until it no
longer returns `Error::WouldBlock`

# Input

An expression `$e` that evaluates to `nb::Result<T, E>`

# Output

- `Ok(t)` if `$e` evaluates to `Ok(t)`
- `Err(e)` if `$e` evaluates to `Err(nb::Error::Other(e))`

