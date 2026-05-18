**object > read > xcoff > segment**

# Module: read::xcoff::segment

## Contents

**Structs**

- [`XcoffSegment`](#xcoffsegment) - A loadable section in an [`XcoffFile`].
- [`XcoffSegmentIterator`](#xcoffsegmentiterator) - An iterator for the segments in an [`XcoffFile`].

**Type Aliases**

- [`XcoffSegment32`](#xcoffsegment32) - A segment in an [`XcoffFile32`](super::XcoffFile32).
- [`XcoffSegment64`](#xcoffsegment64) - A segment in an [`XcoffFile64`](super::XcoffFile64).
- [`XcoffSegmentIterator32`](#xcoffsegmentiterator32) - An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32).
- [`XcoffSegmentIterator64`](#xcoffsegmentiterator64) - An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64).

---

## object::read::xcoff::segment::XcoffSegment

*Struct*

A loadable section in an [`XcoffFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- Xcoff
- R

**Trait Implementations:**

- **ObjectSegment**
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> (u64, u64)`
  - `fn data(self: &Self) -> Result<&'data [u8]>`
  - `fn data_range(self: &Self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>`
  - `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>`
  - `fn name(self: &Self) -> Result<Option<&str>>`
  - `fn flags(self: &Self) -> SegmentFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::xcoff::segment::XcoffSegment32

*Type Alias*: `XcoffSegment<'data, 'file, xcoff::FileHeader32, R>`

A segment in an [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::segment::XcoffSegment64

*Type Alias*: `XcoffSegment<'data, 'file, xcoff::FileHeader64, R>`

A segment in an [`XcoffFile64`](super::XcoffFile64).



## object::read::xcoff::segment::XcoffSegmentIterator

*Struct*

An iterator for the segments in an [`XcoffFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- Xcoff
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::xcoff::segment::XcoffSegmentIterator32

*Type Alias*: `XcoffSegmentIterator<'data, 'file, xcoff::FileHeader32, R>`

An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::segment::XcoffSegmentIterator64

*Type Alias*: `XcoffSegmentIterator<'data, 'file, xcoff::FileHeader64, R>`

An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64).



