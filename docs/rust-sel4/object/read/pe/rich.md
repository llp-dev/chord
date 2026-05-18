**object > read > pe > rich**

# Module: read::pe::rich

## Contents

**Structs**

- [`RichHeaderEntry`](#richheaderentry) - A PE rich header entry after it has been unmasked.
- [`RichHeaderInfo`](#richheaderinfo) - Parsed information about a Rich Header.

---

## object::read::pe::rich::RichHeaderEntry

*Struct*

A PE rich header entry after it has been unmasked.

See [`pe::MaskedRichHeaderEntry`].

**Fields:**
- `comp_id: u32` - ID of the component.
- `count: u32` - Number of times this component has been used when building this PE.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RichHeaderEntry`



## object::read::pe::rich::RichHeaderInfo

*Struct*

Parsed information about a Rich Header.

**Generic Parameters:**
- 'data

**Fields:**
- `offset: usize` - The offset at which the rich header starts.
- `length: usize` - The length (in bytes) of the rich header.
- `xor_key: u32` - The XOR key used to mask the rich header.

**Methods:**

- `fn parse<R>(data: R, nt_header_offset: u64) -> Option<Self>` - Try to locate a rich header and its entries in the current PE file.
- `fn unmasked_entries(self: &Self) -> impl Trait` - Returns an iterator over the unmasked entries.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RichHeaderInfo<'data>`



