**rkyv > ser > sharing > alloc**

# Module: ser::sharing::alloc

## Contents

**Structs**

- [`Share`](#share) - A shared pointer strategy that shares serializations of the same shared

---

## rkyv::ser::sharing::alloc::Share

*Struct*

A shared pointer strategy that shares serializations of the same shared
pointer.

**Methods:**

- `fn new() -> Self` - Creates a new shared pointer unifier.
- `fn with_capacity(capacity: usize) -> Self` - Creates a new shared pointer unifier with initial capacity.
- `fn clear(self: & mut Self)` - Clears the shared pointer unifier for reuse.

**Trait Implementations:**

- **Sharing**
  - `fn start_sharing(self: & mut Self, address: usize) -> SharingState`
  - `fn finish_sharing(self: & mut Self, address: usize, pos: usize) -> Result<(), E>`
- **Default**
  - `fn default() -> Share`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



