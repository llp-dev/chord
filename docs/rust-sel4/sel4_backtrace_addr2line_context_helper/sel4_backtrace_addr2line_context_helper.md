**sel4_backtrace_addr2line_context_helper**

# Module: sel4_backtrace_addr2line_context_helper

## Contents

**Enums**

- [`Error`](#error)

**Functions**

- [`new_context`](#new_context)
- [`new_context_with_sup`](#new_context_with_sup)

**Type Aliases**

- [`Context`](#context)

---

## sel4_backtrace_addr2line_context_helper::Context

*Type Alias*: `addr2line::Context<gimli::EndianRcSlice<gimli::RunTimeEndian>>`



## sel4_backtrace_addr2line_context_helper::Error

*Enum*

**Variants:**
- `ObjectError(object::Error)`
- `GimliError(gimli::Error)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Error`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **From**
  - `fn from(err: object::Error) -> Self`
- **From**
  - `fn from(err: gimli::Error) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_backtrace_addr2line_context_helper::new_context

*Function*

```rust
fn new_context<'data, 'file, O>(file: &'file O) -> Result<Context, Error>
```



## sel4_backtrace_addr2line_context_helper::new_context_with_sup

*Function*

```rust
fn new_context_with_sup<'data, 'file, O>(file: &'file O, sup_file: Option<&'file O>) -> Result<Context, Error>
```



