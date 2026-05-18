**rancor**

# Module: rancor

## Contents

**Macros**

- [`fail`](#fail) - Returns the given error from this function.

**Structs**

- [`Error`](#error) - A good general-purpose error type.
- [`Failure`](#failure) - An error type that only preserves success or failure, throwing away any more
- [`Strategy`](#strategy) - Equips a type with a `Fallible` implementation that chooses `E` as its error

**Enums**

- [`Panic`](#panic) - An error type that does not occupy any space, panicking on creation instead.

**Functions**

- [`unreachable_checked`](#unreachable_checked) - Consumes a `Never` type, returning a primitive `!`.

**Traits**

- [`Fallible`](#fallible) - A type with fallible operations that return its associated error type.
- [`Never`](#never) - A type that can never be produced.
- [`OptionExt`](#optionext) - Helper methods for `Option`s.
- [`ResultExt`](#resultext) - Helper methods for `Result`s.
- [`Source`](#source) - An error type which can be uniformly constructed from an [`Error`] and
- [`Trace`](#trace) - An error type which can add additional "trace" information to itself.

---

## rancor::Error

*Struct*

A good general-purpose error type.

If `debug_assertions` and the `alloc` feature are enabled, then this error
will have the same behavior as [`BoxedError`]. Otherwise, it will behave
like [`Failure`].

**Trait Implementations:**

- **Source**
  - `fn new<T>(source: T) -> Self`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn error::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Trace**
  - `fn trace<R>(self: Self, trace: R) -> Self`



## rancor::Failure

*Struct*

An error type that only preserves success or failure, throwing away any more
detailed error messages.

**Unit Struct**

**Traits:** Error, Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Source**
  - `fn new<T>(_: T) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Failure`
- **Ord**
  - `fn cmp(self: &Self, other: &Failure) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Failure) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Failure) -> bool`
- **Trace**
  - `fn trace<R>(self: Self, _: R) -> Self`



## rancor::Fallible

*Trait*

A type with fallible operations that return its associated error type.

`Fallible` turns an error type parameter into an associated type of another
parameter. You can equip an existing type with a `Fallible` implementation
by wrapping it in a [`Strategy`].

# Example

```
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

**Methods:**

- `Error`: The error type associated with this type's operations.



## rancor::Never

*Trait*

A type that can never be produced.

Never types include the unstable `!` type, enums with no variants, or any
type that always contains a never type (e.g. a struct with a `Never` field).

# Safety

It must be impossible to produce a value of this type.



## rancor::OptionExt

*Trait*

Helper methods for `Option`s.

**Methods:**

- `into_error`: Returns a `Result` with an error indicating that `Some` was expected but
- `into_trace`: Returns a `Result` with an error indicating that `Some` was expected but
- `into_with_trace`: Returns a `Result` with an error indicating that `Some` was expected but



## rancor::Panic

*Enum*

An error type that does not occupy any space, panicking on creation instead.

Because panicking occurs immediately upon creation, this error type will not
print any additional trace information.

**Traits:** Eq, Never, Copy, Error

**Trait Implementations:**

- **Trace**
  - `fn trace<R>(self: Self, _: R) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Panic) -> bool`
- **Source**
  - `fn new<T>(error: T) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Panic) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Panic) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Panic`
- **Display**
  - `fn fmt(self: &Self, _: & mut fmt::Formatter) -> fmt::Result`



## rancor::ResultExt

*Trait*

Helper methods for `Result`s.

**Methods:**

- `into_error`: Returns a `Result` with this error type converted to `U`.
- `into_trace`: Returns a `Result` with this error type converted to `U` and with an
- `into_with_trace`: Returns a `Result` with this error type converted to `U` and with an
- `trace`: Adds an additional `trace` message to the error value of this type.
- `with_trace`: Adds an additional trace message to the error value of this type by
- `always_ok`: Safely unwraps a result that is always `Ok`.



## rancor::Source

*Trait*

An error type which can be uniformly constructed from an [`Error`] and
additional trace information.

# Example

```
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

**Methods:**

- `new`: Returns a new `Self` using the given [`Error`].



## rancor::Strategy

*Struct*

Equips a type with a `Fallible` implementation that chooses `E` as its error
type.

# Example

```
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

**Generic Parameters:**
- T
- E

**Methods:**

- `fn wrap(inner: & mut T) -> & mut Self` - Wraps the given mutable reference, returning a mutable reference to a

**Traits:** Fallible

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



## rancor::Trace

*Trait*

An error type which can add additional "trace" information to itself.

Some functions only add additional context to errors created by other
functions, rather than creating errors themselves. With generics, it's
therefore possible to have a generic function which can produce errors with
some type arguments but not with others. In these cases, `Trace` allows
those functions to add context if an error can occur, and compile out the
context if the error type is [`Infallible`] or [`Panic`].

# Example

```
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

**Methods:**

- `trace`: Adds an additional trace to this error, returning a new error.



## rancor::fail

*Declarative Macro*

Returns the given error from this function.

The current function must return a `Result<_, E>` where `E` implements
[`Source`].

# Example

```
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

```rust
macro_rules! fail {
    ($($x:tt)*) => { ... };
}
```



## rancor::unreachable_checked

*Function*

Consumes a `Never` type, returning a primitive `!`.

This is a safe version of [`unreachable_unchecked`] for `Never` types.

# Example

```
use rancor::{unreachable_checked, Infallible};

let result = Ok::<i32, Infallible>(10);
match result {
    Ok(i) => println!("i"),
    Err(e) => unreachable_checked(e),
}
```

```rust
fn unreachable_checked<T>(_: T) -> never
```



