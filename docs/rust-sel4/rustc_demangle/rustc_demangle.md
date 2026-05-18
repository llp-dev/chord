**rustc_demangle**

# Module: rustc_demangle

## Contents

**Structs**

- [`Demangle`](#demangle) - Representation of a demangled symbol name.
- [`TryDemangleError`](#trydemangleerror) - Error returned from the `try_demangle` function below when demangling fails.

**Functions**

- [`demangle`](#demangle) - De-mangles a Rust symbol into a more readable version
- [`try_demangle`](#try_demangle) - The same as `demangle`, except return an `Err` if the string does not appear

---

## rustc_demangle::Demangle

*Struct*

Representation of a demangled symbol name.

**Generic Parameters:**
- 'a

**Methods:**

- `fn as_str(self: &Self) -> &'a str` - Returns the underlying string that's being demangled.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## rustc_demangle::TryDemangleError

*Struct*

Error returned from the `try_demangle` function below when demangling fails.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TryDemangleError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustc_demangle::demangle

*Function*

De-mangles a Rust symbol into a more readable version

This function will take a **mangled** symbol and return a value. When printed,
the de-mangled version will be written. If the symbol does not look like
a mangled symbol, the original value will be written instead.

# Examples

```
use rustc_demangle::demangle;

assert_eq!(demangle("_ZN4testE").to_string(), "test");
assert_eq!(demangle("_ZN3foo3barE").to_string(), "foo::bar");
assert_eq!(demangle("foo").to_string(), "foo");
```

```rust
fn demangle(s: &str) -> Demangle
```



## rustc_demangle::try_demangle

*Function*

The same as `demangle`, except return an `Err` if the string does not appear
to be a Rust symbol, rather than "demangling" the given string as a no-op.

```
extern crate rustc_demangle;

let not_a_rust_symbol = "la la la";

// The `try_demangle` function will reject strings which are not Rust symbols.
assert!(rustc_demangle::try_demangle(not_a_rust_symbol).is_err());

// While `demangle` will just pass the non-symbol through as a no-op.
assert_eq!(rustc_demangle::demangle(not_a_rust_symbol).as_str(), not_a_rust_symbol);
```

```rust
fn try_demangle(s: &str) -> Result<Demangle, TryDemangleError>
```



