**rkyv > validation**

# Module: validation

## Contents

**Modules**

- [`archive`](#archive) - Basic archive buffer validation.
- [`shared`](#shared) - Shared pointer validation.

**Structs**

- [`Validator`](#validator) - The default validator.

---

## rkyv::validation::Validator

*Struct*

The default validator.

**Generic Parameters:**
- A
- S

**Methods:**

- `fn new(archive: A, shared: S) -> Self` - Creates a new validator from a byte range.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **SharedContext**
  - `fn start_shared(self: & mut Self, address: usize, type_id: TypeId) -> Result<shared::ValidationState, E>`
  - `fn finish_shared(self: & mut Self, address: usize, type_id: TypeId) -> Result<(), E>`
- **ArchiveContext**
  - `fn check_subtree_ptr(self: & mut Self, ptr: *const u8, layout: &core::alloc::Layout) -> Result<(), E>`
  - `fn push_subtree_range(self: & mut Self, root: *const u8, end: *const u8) -> Result<Range<usize>, E>`
  - `fn pop_subtree_range(self: & mut Self, range: Range<usize>) -> Result<(), E>`



## Module: archive

Basic archive buffer validation.



## Module: shared

Shared pointer validation.



