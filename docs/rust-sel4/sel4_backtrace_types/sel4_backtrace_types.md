**sel4_backtrace_types**

# Module: sel4_backtrace_types

## Contents

**Structs**

- [`Entry`](#entry)
- [`Error`](#error)
- [`Postamble`](#postamble)
- [`Preamble`](#preamble)
- [`StackFrame`](#stackframe)

---

## sel4_backtrace_types::Entry

*Struct*

**Fields:**
- `stack_frame: StackFrame`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Entry) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Entry`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_backtrace_types::Error

*Struct*

**Fields:**
- `unwind_reason_code: i32`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_backtrace_types::Postamble

*Struct*

**Fields:**
- `error: Option<Error>`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Postamble) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Postamble`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_backtrace_types::Preamble

*Struct*

**Generic Parameters:**
- T

**Fields:**
- `image: T`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Preamble<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Preamble<T>`



## sel4_backtrace_types::StackFrame

*Struct*

**Fields:**
- `ip: usize`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &StackFrame) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> StackFrame`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



