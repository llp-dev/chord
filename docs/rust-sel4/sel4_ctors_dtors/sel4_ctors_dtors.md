**sel4_ctors_dtors**

# Module: sel4_ctors_dtors

## Contents

**Enums**

- [`Error`](#error)

**Functions**

- [`run_ctors`](#run_ctors)
- [`run_dtors`](#run_dtors)

---

## sel4_ctors_dtors::Error

*Enum*

**Variants:**
- `Misaligned{ section_name: &'static str }`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`



## sel4_ctors_dtors::run_ctors

*Function*

```rust
fn run_ctors() -> Result<(), Error>
```



## sel4_ctors_dtors::run_dtors

*Function*

```rust
fn run_dtors() -> Result<(), Error>
```



