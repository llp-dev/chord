# Crate `rancor`

rancor provides scalable and efficient error handling without using type
composition. This makes it best-suited for situations where:

- Programmatic error introspection is not useful
- Functions may error, but succeed most of the time
- Errors should provide as much useful detail as possible when emitted
- Use cases include both `no_std` and targets with support for `std`

## Features

- `alloc`: Provides the [`BoxedError`](boxed_error/index.md) type. Enabled by default.

## Contents

- [Modules](#modules)
  - [`boxed_error`](#boxed-error)
  - [`thin_box`](#thin-box)
- [Structs](#structs)
  - [`Strategy`](#strategy)
  - [`NoneError`](#noneerror)
  - [`Failure`](#failure)
  - [`BoxedError`](#boxederror)
  - [`Error`](#error)
- [Enums](#enums)
  - [`Panic`](#panic)
- [Traits](#traits)
  - [`Trace`](#trace)
  - [`Source`](#source)
  - [`Fallible`](#fallible)
  - [`ResultExt`](#resultext)
  - [`OptionExt`](#optionext)
  - [`Never`](#never)
- [Functions](#functions)
  - [`unreachable_checked`](#unreachable-checked)
  - [`Infallible`](#infallible)
- [Type Aliases](#type-aliases)
  - [`ErrorType`](#errortype)
- [Macros](#macros)
  - [`fail!`](#fail)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`boxed_error`](#boxed-error) | mod |  |
| [`thin_box`](#thin-box) | mod |  |
| [`Strategy`](#strategy) | struct | Equips a type with a `Fallible` implementation that chooses `E` as its error type. |
| [`NoneError`](#noneerror) | struct |  |
| [`Failure`](#failure) | struct | An error type that only preserves success or failure, throwing away any more detailed error messages. |
| [`BoxedError`](#boxederror) | struct |  |
| [`Error`](#error) | struct | A good general-purpose error type. |
| [`Panic`](#panic) | enum | An error type that does not occupy any space, panicking on creation instead. |
| [`Trace`](#trace) | trait | An error type which can add additional "trace" information to itself. |
| [`Source`](#source) | trait | An error type which can be uniformly constructed from an [`Error`] and additional trace information. |
| [`Fallible`](#fallible) | trait | A type with fallible operations that return its associated error type. |
| [`ResultExt`](#resultext) | trait | Helper methods for `Result`s. |
| [`OptionExt`](#optionext) | trait | Helper methods for `Option`s. |
| [`Never`](#never) | trait | A type that can never be produced. |
| [`unreachable_checked`](#unreachable-checked) | fn | Consumes a `Never` type, returning a primitive `!`. |
| [`Infallible`](#infallible) | fn | A re-export of `core::convert::Infallible`. |
| [`ErrorType`](#errortype) | type |  |
| [`fail!`](#fail) | macro | Returns the given error from this function. |

## Modules

- [`boxed_error`](boxed_error/index.md)
- [`thin_box`](thin_box/index.md)

## Structs

### `Strategy<T: ?Sized, E>`

```rust
struct Strategy<T: ?Sized, E> {
    _error: core::marker::PhantomData<E>,
    inner: T,
}
```

Equips a type with a `Fallible` implementation that chooses `E` as its error
type.

# Example

```rust
use rancor::{Failure, Fallible, Strategy};

trait Print<E = <Self as Fallible>::Error> {
    fn print(&self, message: &str) -> Result<(), E>;
}

impl<T: Print<E> + ?Sized, E> Print<E> for Strategy<T, E> {
    fn print(&self, message: &str) -> Result<(), E> {
        T::print(self, message)
    }
}

struct StdOut;

impl<E> Print<E> for StdOut {
    fn print(&self, message: &str) -> Result<(), E> {
        println!("{message}");
        Ok(())
    }
}

Strategy::<_, Failure>::wrap(&mut StdOut)
    .print("hello world")
    .unwrap();
```

#### Implementations

- <span id="strategy-wrap"></span>`fn wrap(inner: &mut T) -> &mut Self`

  Wraps the given mutable reference, returning a mutable reference to a

  `Strategy`.

  

  ## Example

  ```rust

  use core::ops::Deref;

  

  use rancor::{Failure, Strategy};

  fn test() {

      struct Inner {

          value: u64,

      }

  

      let mut inner = Inner { value: 10 };

  

      let inner_value_ptr = &inner.value as *const u64;

      let strategy: &mut Strategy<Inner, Failure> =

          Strategy::wrap(&mut inner);

      let strategy_value_ptr = (&strategy.deref().value) as *const u64;

      assert_eq!(inner_value_ptr, strategy_value_ptr);

      // Strategy wraps a type but does not change its memory layout.

  }

  

  test();

  ```

#### Trait Implementations

##### `impl<T: ?Sized, E> Deref for Strategy<T, E>`

- <span id="strategy-deref-type-target"></span>`type Target = T`

- <span id="strategy-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: ?Sized, E> DerefMut for Strategy<T, E>`

- <span id="strategy-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T: ?Sized, E> Fallible for Strategy<T, E>`

- <span id="strategy-fallible-type-error"></span>`type Error = E`

##### `impl<T> Pointee for Strategy<T, E>`

- <span id="strategy-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl<T> Receiver for Strategy<T, E>`

- <span id="strategy-receiver-type-target"></span>`type Target = T`

### `NoneError`

```rust
struct NoneError;
```

#### Trait Implementations

##### `impl Debug for NoneError`

- <span id="noneerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for NoneError`

- <span id="noneerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for NoneError`

##### `impl Pointee for NoneError`

- <span id="noneerror-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NoneError`

- <span id="noneerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Failure`

```rust
struct Failure;
```

An error type that only preserves success or failure, throwing away any more
detailed error messages.

#### Trait Implementations

##### `impl Clone for Failure`

- <span id="failure-clone"></span>`fn clone(&self) -> Failure` — [`Failure`](#failure)

##### `impl Copy for Failure`

##### `impl Debug for Failure`

- <span id="failure-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Failure`

- <span id="failure-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Failure`

##### `impl Error for Failure`

##### `impl Hash for Failure`

- <span id="failure-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Failure`

- <span id="failure-ord-cmp"></span>`fn cmp(&self, other: &Failure) -> cmp::Ordering` — [`Failure`](#failure)

##### `impl PartialEq for Failure`

- <span id="failure-partialeq-eq"></span>`fn eq(&self, other: &Failure) -> bool` — [`Failure`](#failure)

##### `impl PartialOrd for Failure`

- <span id="failure-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Failure) -> option::Option<cmp::Ordering>` — [`Failure`](#failure)

##### `impl Pointee for Failure`

- <span id="failure-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Source for Failure`

- <span id="failure-source-new"></span>`fn new<T: error::Error + Send + Sync + 'static>(_: T) -> Self`

##### `impl StructuralPartialEq for Failure`

##### `impl ToString for Failure`

- <span id="failure-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl Trace for Failure`

- <span id="failure-trace"></span>`fn trace<R>(self, _: R) -> Self`

### `BoxedError`

```rust
struct BoxedError {
    inner: crate::thin_box::ThinBox<dyn error::Error + Send + Sync>,
}
```

An error type that preserves all detailed error messages. It is optimized
to fit in a single pointer.

#### Trait Implementations

##### `impl Debug for BoxedError`

- <span id="boxederror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BoxedError`

- <span id="boxederror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for BoxedError`

- <span id="boxederror-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl Pointee for BoxedError`

- <span id="boxederror-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Source for BoxedError`

- <span id="boxederror-source-new"></span>`fn new<T: error::Error + Send + Sync + 'static>(source: T) -> Self`

##### `impl ToString for BoxedError`

- <span id="boxederror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl Trace for BoxedError`

- <span id="boxederror-trace"></span>`fn trace<R>(self, trace: R) -> Self`

### `Error`

```rust
struct Error {
    inner: BoxedError,
}
```

A good general-purpose error type.

If `debug_assertions` and the `alloc` feature are enabled, then this error
will have the same behavior as [`BoxedError`](boxed_error/index.md). Otherwise, it will behave
like [`Failure`](#failure).

#### Trait Implementations

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl Pointee for Error`

- <span id="error-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Source for Error`

- <span id="error-source-new"></span>`fn new<T: error::Error + Send + Sync + 'static>(source: T) -> Self`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl Trace for Error`

- <span id="error-trace"></span>`fn trace<R>(self, trace: R) -> Self`

## Enums

### `Panic`

```rust
enum Panic {
}
```

An error type that does not occupy any space, panicking on creation instead.

Because panicking occurs immediately upon creation, this error type will not
print any additional trace information.

#### Trait Implementations

##### `impl Clone for Panic`

- <span id="panic-clone"></span>`fn clone(&self) -> Panic` — [`Panic`](#panic)

##### `impl Copy for Panic`

##### `impl Debug for Panic`

- <span id="panic-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Panic`

- <span id="panic-display-fmt"></span>`fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Panic`

##### `impl Error for Panic`

##### `impl Hash for Panic`

- <span id="panic-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Never for Panic`

##### `impl Ord for Panic`

- <span id="panic-ord-cmp"></span>`fn cmp(&self, other: &Panic) -> cmp::Ordering` — [`Panic`](#panic)

##### `impl PartialEq for Panic`

- <span id="panic-partialeq-eq"></span>`fn eq(&self, other: &Panic) -> bool` — [`Panic`](#panic)

##### `impl PartialOrd for Panic`

- <span id="panic-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Panic) -> option::Option<cmp::Ordering>` — [`Panic`](#panic)

##### `impl Pointee for Panic`

- <span id="panic-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Source for Panic`

- <span id="panic-source-new"></span>`fn new<T: fmt::Display>(error: T) -> Self`

##### `impl StructuralPartialEq for Panic`

##### `impl ToString for Panic`

- <span id="panic-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl Trace for Panic`

- <span id="panic-trace"></span>`fn trace<R>(self, _: R) -> Self`

## Traits

### `Trace`

```rust
trait Trace: Sized + Send + Sync + 'static { ... }
```

An error type which can add additional "trace" information to itself.

Some functions only add additional context to errors created by other
functions, rather than creating errors themselves. With generics, it's
therefore possible to have a generic function which can produce errors with
some type arguments but not with others. In these cases, `Trace` allows
those functions to add context if an error can occur, and compile out the
context if the error type is [`Infallible`](#infallible) or [`Panic`](#panic).

# Example

```rust
use rancor::{ResultExt, Trace};

trait Print<E> {
    fn print(&self, message: &str) -> Result<(), E>;
}

fn print_hello_world<T: Print<E>, E: Trace>(printer: &T) -> Result<(), E> {
    printer.print("hello").trace("failed to print hello")?;
    printer.print("world").trace("failed to print world")?;
    Ok(())
}
```

#### Required Methods

- `fn trace<R>(self, trace: R) -> Self`

  Adds an additional trace to this error, returning a new error.

#### Implementors

- [`BoxedError`](boxed_error/index.md#boxederror)
- [`Error`](#error)
- [`Failure`](#failure)
- [`Infallible`](#infallible)
- [`Panic`](#panic)

### `Source`

```rust
trait Source: Trace + error::Error { ... }
```

An error type which can be uniformly constructed from an [`Error`](#error) and
additional trace information.

# Example

```rust
use core::{error::Error, fmt};

use rancor::{fail, Source};

#[derive(Debug)]
struct DivideByZeroError;

impl fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attempted to divide by zero")
    }
}

impl Error for DivideByZeroError {}

fn try_divide<E: Source>(a: i32, b: i32) -> Result<i32, E> {
    if b == 0 {
        fail!(DivideByZeroError);
    }
    Ok(a / b)
}
```

#### Required Methods

- `fn new<T: error::Error + Send + Sync + 'static>(source: T) -> Self`

  Returns a new `Self` using the given [`Error`](#error).

#### Implementors

- [`BoxedError`](boxed_error/index.md#boxederror)
- [`Error`](#error)
- [`Failure`](#failure)
- [`Panic`](#panic)

### `Fallible`

```rust
trait Fallible { ... }
```

A type with fallible operations that return its associated error type.

`Fallible` turns an error type parameter into an associated type of another
parameter. You can equip an existing type with a `Fallible` implementation
by wrapping it in a [`Strategy`](#strategy).

# Example

```rust
use rancor::{Failure, Fallible, Strategy};

trait Operator<E = <Self as Fallible>::Error> {
    fn operate(&self, lhs: i32, rhs: i32) -> Result<i32, E>;
}

impl<T: Operator<E> + ?Sized, E> Operator<E> for Strategy<T, E> {
    fn operate(&self, lhs: i32, rhs: i32) -> Result<i32, E> {
        T::operate(self, lhs, rhs)
    }
}

struct Add;

impl<E> Operator<E> for Add {
    fn operate(&self, lhs: i32, rhs: i32) -> Result<i32, E> {
        Ok(lhs + rhs)
    }
}

fn operate_one_one<T: Operator + Fallible>(
    operator: &T,
) -> Result<i32, T::Error> {
    operator.operate(1, 1)
}

assert_eq!(
    operate_one_one(Strategy::<_, Failure>::wrap(&mut Add)),
    Ok(2)
);
```

#### Associated Types

- `type Error`

#### Implementors

- [`Strategy`](#strategy)

### `ResultExt<T, E>`

```rust
trait ResultExt<T, E> { ... }
```

Helper methods for `Result`s.

#### Required Methods

- `fn into_error<U>(self) -> Result<T, U>`

  Returns a `Result` with this error type converted to `U`.

- `fn into_trace<U, R>(self, trace: R) -> Result<T, U>`

  Returns a `Result` with this error type converted to `U` and with an

- `fn into_with_trace<U, R, F>(self, f: F) -> Result<T, U>`

  Returns a `Result` with this error type converted to `U` and with an

- `fn trace<R>(self, trace: R) -> Result<T, E>`

  Adds an additional `trace` message to the error value of this type.

- `fn with_trace<R, F>(self, f: F) -> Result<T, E>`

  Adds an additional trace message to the error value of this type by

- `fn always_ok(self) -> T`

  Safely unwraps a result that is always `Ok`.

#### Implementors

- `Result<T, E>`

### `OptionExt<T>`

```rust
trait OptionExt<T> { ... }
```

Helper methods for `Option`s.

#### Required Methods

- `fn into_error<E>(self) -> Result<T, E>`

  Returns a `Result` with an error indicating that `Some` was expected but

- `fn into_trace<E, R>(self, trace: R) -> Result<T, E>`

  Returns a `Result` with an error indicating that `Some` was expected but

- `fn into_with_trace<E, R, F>(self, f: F) -> Result<T, E>`

  Returns a `Result` with an error indicating that `Some` was expected but

#### Implementors

- `Option<T>`

### `Never`

```rust
trait Never { ... }
```

A type that can never be produced.

Never types include the unstable `!` type, enums with no variants, or any
type that always contains a never type (e.g. a struct with a `Never` field).

# Safety

It must be impossible to produce a value of this type.

#### Implementors

- [`Infallible`](#infallible)
- [`Panic`](#panic)

## Functions

### `unreachable_checked`

```rust
const fn unreachable_checked<T: Never>(_: T) -> never
```

Consumes a `Never` type, returning a primitive `!`.

This is a safe version of `unreachable_unchecked` for `Never` types.

# Example

```rust
use rancor::{unreachable_checked, Infallible};

let result = Ok::<i32, Infallible>(10);
match result {
    Ok(i) => println!("i"),
    Err(e) => unreachable_checked(e),
}
```

### `Infallible`

```rust
fn Infallible(value: U) -> Result<T, <T as TryFrom>::Error>
```

## Type Aliases

### `ErrorType`

```rust
type ErrorType = BoxedError;
```

## Macros

### `fail!`

Returns the given error from this function.

The current function must return a `Result<_, E>` where `E` implements
[`Source`](#source).

# Example

```rust
use core::{error::Error, fmt};

use rancor::{fail, Source};

#[derive(Debug)]
struct DivideByZeroError;

impl fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attempted to divide by zero")
    }
}

impl Error for DivideByZeroError {}

fn divide<E: Source>(a: i32, b: i32) -> Result<i32, E> {
    if b == 0 {
        fail!(DivideByZeroError);
    }
    Ok(a / b)
}
```

