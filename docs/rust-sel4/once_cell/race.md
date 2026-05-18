**once_cell > race**

# Module: race

## Contents

**Structs**

- [`OnceBool`](#oncebool) - A thread-safe cell which can be written to only once.
- [`OnceNonZeroUsize`](#oncenonzerousize) - A thread-safe cell which can be written to only once.
- [`OnceRef`](#onceref) - A thread-safe cell which can be written to only once.

---

## once_cell::race::OnceBool

*Struct*

A thread-safe cell which can be written to only once.

**Methods:**

- `fn new() -> Self` - Creates a new empty cell.
- `fn get(self: &Self) -> Option<bool>` - Gets the underlying value.
- `fn set(self: &Self, value: bool) -> Result<(), ()>` - Sets the contents of this cell to `value`.
- `fn get_or_init<F>(self: &Self, f: F) -> bool` - Gets the contents of the cell, initializing it with `f` if the cell was
- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<bool, E>` - Gets the contents of the cell, initializing it with `f` if

**Trait Implementations:**

- **Default**
  - `fn default() -> OnceBool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## once_cell::race::OnceNonZeroUsize

*Struct*

A thread-safe cell which can be written to only once.

**Methods:**

- `fn new() -> Self` - Creates a new empty cell.
- `fn get(self: &Self) -> Option<NonZeroUsize>` - Gets the underlying value.
- `fn get_unchecked(self: &Self) -> NonZeroUsize` - Get the reference to the underlying value, without checking if the cell
- `fn set(self: &Self, value: NonZeroUsize) -> Result<(), ()>` - Sets the contents of this cell to `value`.
- `fn get_or_init<F>(self: &Self, f: F) -> NonZeroUsize` - Gets the contents of the cell, initializing it with `f` if the cell was
- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<NonZeroUsize, E>` - Gets the contents of the cell, initializing it with `f` if

**Trait Implementations:**

- **Default**
  - `fn default() -> OnceNonZeroUsize`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## once_cell::race::OnceRef

*Struct*

A thread-safe cell which can be written to only once.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn new() -> Self` - Creates a new empty cell.
- `fn get(self: &Self) -> Option<&'a T>` - Gets a reference to the underlying value.
- `fn set(self: &Self, value: &'a T) -> Result<(), ()>` - Sets the contents of this cell to `value`.
- `fn get_or_init<F>(self: &Self, f: F) -> &'a T` - Gets the contents of the cell, initializing it with `f` if the cell was
- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&'a T, E>` - Gets the contents of the cell, initializing it with `f` if

**Traits:** Sync

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



