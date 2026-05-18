**smoltcp > storage**

# Module: storage

## Contents

**Structs**

- [`Empty`](#empty) - Error returned when dequeuing from an empty buffer.
- [`Full`](#full) - Error returned when enqueuing into a full buffer.

**Traits**

- [`Resettable`](#resettable) - A trait for setting a value to a known state.

---

## smoltcp::storage::Empty

*Struct*

Error returned when dequeuing from an empty buffer.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Empty`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Empty) -> bool`



## smoltcp::storage::Full

*Struct*

Error returned when enqueuing into a full buffer.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Full) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Full`



## smoltcp::storage::Resettable

*Trait*

A trait for setting a value to a known state.

In-place analog of Default.

**Methods:**

- `reset`



