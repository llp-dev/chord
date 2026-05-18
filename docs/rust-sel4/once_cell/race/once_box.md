**once_cell > race > once_box**

# Module: race::once_box

## Contents

**Structs**

- [`OnceBox`](#oncebox) - A thread-safe cell which can be written to only once.

---

## once_cell::race::once_box::OnceBox

*Struct*

A thread-safe cell which can be written to only once.

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Creates a new empty cell.
- `fn with_value(value: Box<T>) -> Self` - Creates a new cell with the given value.
- `fn get(self: &Self) -> Option<&T>` - Gets a reference to the underlying value.
- `fn set(self: &Self, value: Box<T>) -> Result<(), Box<T>>` - Sets the contents of this cell to `value`.
- `fn get_or_init<F>(self: &Self, f: F) -> &T` - Gets the contents of the cell, initializing it with `f` if the cell was
- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&T, E>` - Gets the contents of the cell, initializing it with `f` if

**Traits:** Sync

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



