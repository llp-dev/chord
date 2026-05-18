**ttf_parser > parser**

# Module: parser

## Contents

**Structs**

- [`F2DOT14`](#f2dot14) - A 16-bit signed fixed number with the low 14 bits of fraction (2.14).
- [`Fixed`](#fixed) - A 32-bit signed fixed-point number (16.16).
- [`LazyArray16`](#lazyarray16) - A slice-like container that converts internal binary data only on access.
- [`LazyArray32`](#lazyarray32) - A slice-like container that converts internal binary data only on access.
- [`LazyArrayIter16`](#lazyarrayiter16) - An iterator over `LazyArray16`.
- [`LazyArrayIter32`](#lazyarrayiter32) - An iterator over `LazyArray32`.
- [`LazyOffsetArray16`](#lazyoffsetarray16) - A [`LazyArray16`]-like container, but data is accessed by offsets.
- [`LazyOffsetArrayIter16`](#lazyoffsetarrayiter16) - An iterator over [`LazyOffsetArray16`] values.
- [`Offset16`](#offset16) - A type-safe u16 offset.
- [`Offset24`](#offset24) - A type-safe u24 offset.
- [`Offset32`](#offset32) - A type-safe u32 offset.
- [`Stream`](#stream) - A streaming binary parser.
- [`U24`](#u24) - A u24 number.

**Functions**

- [`f32_bound`](#f32_bound)
- [`i16_bound`](#i16_bound)

**Traits**

- [`FromData`](#fromdata) - A trait for parsing raw binary data of fixed size.
- [`FromSlice`](#fromslice) - A trait for parsing raw binary data of variable size.
- [`NumFrom`](#numfrom) - A safe u32 to usize casting.
- [`Offset`](#offset) - A common offset methods.
- [`TryNumFrom`](#trynumfrom) - Just like TryFrom<N>, but for numeric types not supported by the Rust's std.

---

## ttf_parser::parser::F2DOT14

*Struct*

A 16-bit signed fixed number with the low 14 bits of fraction (2.14).

**Tuple Struct**: `(i16)`

**Methods:**

- `fn to_f32(self: Self) -> f32` - Converts i16 to f32.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> F2DOT14`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::parser::Fixed

*Struct*

A 32-bit signed fixed-point number (16.16).

**Tuple Struct**: `(f32)`

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Fixed`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::parser::FromData

*Trait*

A trait for parsing raw binary data of fixed size.

This is a low-level, internal trait that should not be used directly.

**Methods:**

- `SIZE`: Object's raw data size.
- `parse`: Parses an object from a raw data.



## ttf_parser::parser::FromSlice

*Trait*

A trait for parsing raw binary data of variable size.

This is a low-level, internal trait that should not be used directly.

**Methods:**

- `parse`: Parses an object from a raw data.



## ttf_parser::parser::LazyArray16

*Struct*

A slice-like container that converts internal binary data only on access.

Array values are stored in a continuous data chunk.

**Generic Parameters:**
- 'a
- T

**Fields:**
- `data: &'a [u8]`
- `data_type: core::marker::PhantomData<T>`

**Methods:**

- `fn new(data: &'a [u8]) -> Self` - Creates a new `LazyArray`.
- `fn get(self: &Self, index: u16) -> Option<T>` - Returns a value at `index`.
- `fn last(self: &Self) -> Option<T>` - Returns the last value.
- `fn slice(self: &Self, range: Range<u16>) -> Option<Self>` - Returns sub-array.
- `fn len(self: &Self) -> u16` - Returns array's length.
- `fn is_empty(self: &Self) -> bool` - Checks if array is empty.
- `fn binary_search(self: &Self, key: &T) -> Option<(u16, T)>` - Performs a binary search by specified `key`.
- `fn binary_search_by<F>(self: &Self, f: F) -> Option<(u16, T)>` - Performs a binary search using specified closure.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LazyArray16<'a, T>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`



## ttf_parser::parser::LazyArray32

*Struct*

A slice-like container that converts internal binary data only on access.

This is a low-level, internal structure that should not be used directly.

**Generic Parameters:**
- 'a
- T

**Fields:**
- `data: &'a [u8]`
- `data_type: core::marker::PhantomData<T>`

**Methods:**

- `fn new(data: &'a [u8]) -> Self` - Creates a new `LazyArray`.
- `fn get(self: &Self, index: u32) -> Option<T>` - Returns a value at `index`.
- `fn len(self: &Self) -> u32` - Returns array's length.
- `fn is_empty(self: &Self) -> bool` - Checks if the array is empty.
- `fn binary_search(self: &Self, key: &T) -> Option<(u32, T)>` - Performs a binary search by specified `key`.
- `fn binary_search_by<F>(self: &Self, f: F) -> Option<(u32, T)>` - Performs a binary search using specified closure.

**Traits:** Copy

**Trait Implementations:**

- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Clone**
  - `fn clone(self: &Self) -> LazyArray32<'a, T>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::parser::LazyArrayIter16

*Struct*

An iterator over `LazyArray16`.

**Generic Parameters:**
- 'a
- T

**Fields:**
- `data: LazyArray16<'a, T>`
- `index: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LazyArrayIter16<'a, T>`
- **Default**
  - `fn default() -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn count(self: Self) -> usize`



## ttf_parser::parser::LazyArrayIter32

*Struct*

An iterator over `LazyArray32`.

**Generic Parameters:**
- 'a
- T

**Fields:**
- `data: LazyArray32<'a, T>`
- `index: u32`

**Traits:** Copy

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn count(self: Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> LazyArrayIter32<'a, T>`



## ttf_parser::parser::LazyOffsetArray16

*Struct*

A [`LazyArray16`]-like container, but data is accessed by offsets.

Unlike [`LazyArray16`], internal storage is not continuous.

Multiple offsets can point to the same data.

**Generic Parameters:**
- 'a
- T

**Fields:**
- `data: &'a [u8]`
- `offsets: LazyArray16<'a, Option<Offset16>>`
- `data_type: core::marker::PhantomData<T>`

**Methods:**

- `fn new(data: &'a [u8], offsets: LazyArray16<'a, Option<Offset16>>) -> Self` - Creates a new `LazyOffsetArray16`.
- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses `LazyOffsetArray16` from raw data.
- `fn get(self: &Self, index: u16) -> Option<T>` - Returns a value at `index`.
- `fn len(self: &Self) -> u16` - Returns array's length.
- `fn is_empty(self: &Self) -> bool` - Checks if array is empty.

**Traits:** Copy

**Trait Implementations:**

- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Clone**
  - `fn clone(self: &Self) -> LazyOffsetArray16<'a, T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::parser::LazyOffsetArrayIter16

*Struct*

An iterator over [`LazyOffsetArray16`] values.

**Generic Parameters:**
- 'a
- T

**Fields:**
- `array: LazyOffsetArray16<'a, T>`
- `index: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LazyOffsetArrayIter16<'a, T>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn count(self: Self) -> usize`



## ttf_parser::parser::NumFrom

*Trait*

A safe u32 to usize casting.

Rust doesn't implement `From<u32> for usize`,
because it has to support 16 bit targets.
We don't, so we can allow this.

**Methods:**

- `num_from`: Converts u32 into usize.



## ttf_parser::parser::Offset

*Trait*

A common offset methods.

**Methods:**

- `to_usize`: Converts the offset to `usize`.



## ttf_parser::parser::Offset16

*Struct*

A type-safe u16 offset.

**Tuple Struct**: `(u16)`

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Offset**
  - `fn to_usize(self: &Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> Offset16`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::parser::Offset24

*Struct*

A type-safe u24 offset.

**Tuple Struct**: `(u32)`

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Offset**
  - `fn to_usize(self: &Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> Offset24`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::parser::Offset32

*Struct*

A type-safe u32 offset.

**Tuple Struct**: `(u32)`

**Traits:** Copy

**Trait Implementations:**

- **Offset**
  - `fn to_usize(self: &Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> Offset32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::parser::Stream

*Struct*

A streaming binary parser.

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `offset: usize`

**Methods:**

- `fn new(data: &'a [u8]) -> Self` - Creates a new `Stream` parser.
- `fn new_at(data: &'a [u8], offset: usize) -> Option<Self>` - Creates a new `Stream` parser at offset.
- `fn at_end(self: &Self) -> bool` - Checks that stream reached the end of the data.
- `fn jump_to_end(self: & mut Self)` - Jumps to the end of the stream.
- `fn offset(self: &Self) -> usize` - Returns the current offset.
- `fn tail(self: &Self) -> Option<&'a [u8]>` - Returns the trailing data.
- `fn skip<T>(self: & mut Self)` - Advances by `FromData::SIZE`.
- `fn advance(self: & mut Self, len: usize)` - Advances by the specified `len`.
- `fn advance_checked(self: & mut Self, len: usize) -> Option<()>` - Advances by the specified `len` and checks for bounds.
- `fn read<T>(self: & mut Self) -> Option<T>` - Parses the type from the steam.
- `fn read_at<T>(data: &[u8], offset: usize) -> Option<T>` - Parses the type from the steam at offset.
- `fn read_bytes(self: & mut Self, len: usize) -> Option<&'a [u8]>` - Reads N bytes from the stream.
- `fn read_array16<T>(self: & mut Self, count: u16) -> Option<LazyArray16<'a, T>>` - Reads the next `count` types as a slice.
- `fn read_array32<T>(self: & mut Self, count: u32) -> Option<LazyArray32<'a, T>>` - Reads the next `count` types as a slice.
- `fn read_at_offset16(self: & mut Self, data: &'a [u8]) -> Option<&'a [u8]>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Stream<'a>`
- **Default**
  - `fn default() -> Stream<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::parser::TryNumFrom

*Trait*

Just like TryFrom<N>, but for numeric types not supported by the Rust's std.

**Methods:**

- `try_num_from`: Casts between numeric types.



## ttf_parser::parser::U24

*Struct*

A u24 number.

Stored as u32, but encoded as 3 bytes in the font.

<https://docs.microsoft.com/en-us/typography/opentype/spec/otff#data-types>

**Tuple Struct**: `(u32)`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> U24`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::parser::f32_bound

*Function*

```rust
fn f32_bound(min: f32, val: f32, max: f32) -> f32
```



## ttf_parser::parser::i16_bound

*Function*

```rust
fn i16_bound(min: i16, val: i16, max: i16) -> i16
```



