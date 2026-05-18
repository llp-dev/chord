**embedded_fat > filesystem > search_id**

# Module: filesystem::search_id

## Contents

**Structs**

- [`SearchId`](#searchid) - Unique ID used to search for files and directories in the open Volume/File/Directory lists
- [`SearchIdGenerator`](#searchidgenerator) - A Search ID generator.

---

## embedded_fat::filesystem::search_id::SearchId

*Struct*

Unique ID used to search for files and directories in the open Volume/File/Directory lists

**Tuple Struct**: `()`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SearchId`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SearchId) -> bool`



## embedded_fat::filesystem::search_id::SearchIdGenerator

*Struct*

A Search ID generator.

This object will always return a different ID.

Well, it will wrap after `2**32` IDs. But most systems won't open that many
files, and if they do, they are unlikely to hold one file open and then
open/close `2**32 - 1` others.

**Methods:**

- `fn new(offset: u32) -> Self` - Create a new generator of Search IDs.
- `fn get(self: & mut Self) -> SearchId` - Generate a new, unique [`SearchId`].



