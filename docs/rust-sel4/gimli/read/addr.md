**gimli > read > addr**

# Module: read::addr

## Contents

**Structs**

- [`AddrEntryIter`](#addrentryiter) - An iterator over the addresses from a `.debug_addr` section.
- [`AddrHeader`](#addrheader) - A header for a set of entries in the `.debug_addr` section.
- [`AddrHeaderIter`](#addrheaderiter) - An iterator over the headers of a `.debug_addr` section.
- [`DebugAddr`](#debugaddr) - The raw contents of the `.debug_addr` section.

---

## gimli::read::addr::AddrEntryIter

*Struct*

An iterator over the addresses from a `.debug_addr` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

**Generic Parameters:**
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<u64>>` - Advance the iterator and return the next address.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AddrEntryIter<R>`



## gimli::read::addr::AddrHeader

*Struct*

A header for a set of entries in the `.debug_addr` section.

These entries all belong to a single unit.

**Generic Parameters:**
- R
- Offset

**Methods:**

- `fn offset(self: &Self) -> DebugAddrOffset<Offset>` - Return the offset of this header within the `.debug_addr` section.
- `fn length(self: &Self) -> Offset` - Return the length of this set of entries, including the header.
- `fn encoding(self: &Self) -> Encoding` - Return the encoding parameters for this set of entries.
- `fn entries(self: &Self) -> AddrEntryIter<R>` - Return the address entries in this set.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AddrHeader<R, Offset>`
- **PartialEq**
  - `fn eq(self: &Self, other: &AddrHeader<R, Offset>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::addr::AddrHeaderIter

*Struct*

An iterator over the headers of a `.debug_addr` section.

**Generic Parameters:**
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<AddrHeader<R>>>` - Advance the iterator to the next header.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AddrHeaderIter<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::addr::DebugAddr

*Struct*

The raw contents of the `.debug_addr` section.

**Generic Parameters:**
- R

**Methods:**

- `fn get_address(self: &Self, address_size: u8, base: DebugAddrBase<<R as >::Offset>, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` - Returns the address at the given `base` and `index`.
- `fn headers(self: &Self) -> AddrHeaderIter<R>` - Iterate the sets of entries in the `.debug_addr` section.
- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugAddr<R>` - Create a `DebugAddr` section that references the data in `self`.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugAddr<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugAddr<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`
- **From**
  - `fn from(section: R) -> Self`



