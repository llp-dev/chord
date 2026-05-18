**rkyv > ser > sharing > core**

# Module: ser::sharing::core

## Contents

**Structs**

- [`Unshare`](#unshare) - A shared pointer strategy that duplicates serializations of the same shared

---

## rkyv::ser::sharing::core::Unshare

*Struct*

A shared pointer strategy that duplicates serializations of the same shared
pointer.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Unshare`
- **Sharing**
  - `fn start_sharing(self: & mut Self, _: usize) -> SharingState`
  - `fn finish_sharing(self: & mut Self, _: usize, _: usize) -> Result<(), E>`



