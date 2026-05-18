**smoltcp > storage > assembler**

# Module: storage::assembler

## Contents

**Structs**

- [`Assembler`](#assembler) - A buffer (re)assembler.
- [`AssemblerIter`](#assembleriter)
- [`TooManyHolesError`](#toomanyholeserror)

---

## smoltcp::storage::assembler::Assembler

*Struct*

A buffer (re)assembler.

Currently, up to a hardcoded limit of 4 or 32 holes can be tracked in the buffer.

**Methods:**

- `fn new() -> Assembler` - Create a new buffer assembler.
- `fn clear(self: & mut Self)`
- `fn peek_front(self: &Self) -> usize` - Return length of the front contiguous range without removing it from the assembler
- `fn is_empty(self: &Self) -> bool` - Return whether the assembler contains no data.
- `fn add(self: & mut Self, offset: usize, size: usize) -> Result<(), TooManyHolesError>` - Add a new contiguous range to the assembler,
- `fn remove_front(self: & mut Self) -> usize` - Remove a contiguous range from the front of the assembler.
- `fn add_then_remove_front(self: & mut Self, offset: usize, size: usize) -> Result<usize, TooManyHolesError>` - Add a segment, then remove_front.
- `fn iter_data(self: &Self, first_offset: usize) -> AssemblerIter` - Iterate over all of the contiguous data ranges.

**Traits:** Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Assembler) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Assembler`



## smoltcp::storage::assembler::AssemblerIter

*Struct*

**Generic Parameters:**
- 'a



## smoltcp::storage::assembler::TooManyHolesError

*Struct*

**Unit Struct**



