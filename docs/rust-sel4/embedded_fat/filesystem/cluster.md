**embedded_fat > filesystem > cluster**

# Module: filesystem::cluster

## Contents

**Structs**

- [`ClusterId`](#clusterid) - Identifies a cluster on disk.

---

## embedded_fat::filesystem::cluster::ClusterId

*Struct*

Identifies a cluster on disk.

A cluster is a consecutive group of blocks. Each cluster has a a numeric ID.
Some numeric IDs are reserved for special purposes.

**Tuple Struct**: `()`

**Methods:**


**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ClusterId) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> ClusterId`
- **Add**
  - `fn add(self: Self, rhs: u32) -> ClusterId`
- **PartialEq**
  - `fn eq(self: &Self, other: &ClusterId) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &ClusterId) -> $crate::cmp::Ordering`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: u32)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



