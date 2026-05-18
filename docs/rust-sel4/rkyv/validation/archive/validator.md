**rkyv > validation > archive > validator**

# Module: validation::archive::validator

## Contents

**Structs**

- [`ArchiveValidator`](#archivevalidator) - A validator that can verify archives with nonlocal memory.

---

## rkyv::validation::archive::validator::ArchiveValidator

*Struct*

A validator that can verify archives with nonlocal memory.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(bytes: &'a [u8]) -> Self` - Creates a new bounds validator for the given bytes.
- `fn with_max_depth(bytes: &'a [u8], max_subtree_depth: Option<NonZeroUsize>) -> Self` - Crates a new bounds validator for the given bytes with a maximum

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ArchiveContext**
  - `fn check_subtree_ptr(self: & mut Self, ptr: *const u8, layout: &Layout) -> Result<(), E>`
  - `fn push_subtree_range(self: & mut Self, root: *const u8, end: *const u8) -> Result<Range<usize>, E>`
  - `fn pop_subtree_range(self: & mut Self, range: Range<usize>) -> Result<(), E>`



