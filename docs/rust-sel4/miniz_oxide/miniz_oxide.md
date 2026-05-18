**miniz_oxide**

# Module: miniz_oxide

## Contents

**Modules**

- [`deflate`](#deflate) - This module contains functionality for compression.
- [`inflate`](#inflate) - This module contains functionality for decompression.

**Structs**

- [`StreamResult`](#streamresult) - A structure containing the result of a call to the inflate or deflate streaming functions.

**Enums**

- [`DataFormat`](#dataformat) - How compressed data is wrapped.
- [`MZError`](#mzerror) - A list of miniz failed status codes.
- [`MZFlush`](#mzflush) - A list of flush types.
- [`MZStatus`](#mzstatus) - A list of miniz successful status codes.

**Type Aliases**

- [`MZResult`](#mzresult) - `Result` alias for all miniz status codes both successful and failed.

---

## miniz_oxide::DataFormat

*Enum*

How compressed data is wrapped.

**Variants:**
- `Zlib` - Wrapped using the [zlib](http://www.zlib.org/rfc-zlib.html) format.
- `ZLibIgnoreChecksum` - Zlib wrapped but ignore and don't compute the adler32 checksum.
- `Raw` - Raw DEFLATE.

**Methods:**

- `fn from_window_bits(window_bits: i32) -> DataFormat`
- `fn to_window_bits(self: Self) -> i32`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> DataFormat`
- **PartialEq**
  - `fn eq(self: &Self, other: &DataFormat) -> bool`



## miniz_oxide::MZError

*Enum*

A list of miniz failed status codes.

These are emitted as the [`Err`] side of a [`MZResult`] in the [`StreamResult`] returned from
[`deflate::stream::deflate()`] or [`inflate::stream::inflate()`].

**Variants:**
- `ErrNo` - Unused
- `Stream` - General stream error.
- `Data` - Error in inflation; see [`inflate::stream::inflate()`] for details.
- `Mem` - Unused
- `Buf` - Buffer-related error.
- `Version` - Unused
- `Param` - Bad parameters.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &MZError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> MZError`



## miniz_oxide::MZFlush

*Enum*

A list of flush types.

See <http://www.bolet.org/~pornin/deflate-flush.html> for more in-depth info.

**Variants:**
- `None` - Don't force any flushing.
- `Partial` - Zlib partial flush.
- `Sync` - Finish compressing the currently buffered data, and output an empty raw block.
- `Full` - Same as [`Sync`], but resets the compression dictionary so that further compressed
- `Finish` - Attempt to flush the remaining data and end the stream.
- `Block` - Not implemented.

**Methods:**

- `fn new(flush: i32) -> Result<Self, MZError>` - Create an MZFlush value from an integer value.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> MZFlush`
- **PartialEq**
  - `fn eq(self: &Self, other: &MZFlush) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## miniz_oxide::MZResult

*Type Alias*: `Result<MZStatus, MZError>`

`Result` alias for all miniz status codes both successful and failed.



## miniz_oxide::MZStatus

*Enum*

A list of miniz successful status codes.

These are emitted as the [`Ok`] side of a [`MZResult`] in the [`StreamResult`] returned from
[`deflate::stream::deflate()`] or [`inflate::stream::inflate()`].

**Variants:**
- `Ok` - Operation succeeded.
- `StreamEnd` - Operation succeeded and end of deflate stream was found.
- `NeedDict` - Unused

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &MZStatus) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> MZStatus`



## miniz_oxide::StreamResult

*Struct*

A structure containing the result of a call to the inflate or deflate streaming functions.

**Fields:**
- `bytes_consumed: usize` - The number of bytes consumed from the input slice.
- `bytes_written: usize` - The number of bytes written to the output slice.
- `status: MZResult` - The return status of the call.

**Methods:**

- `fn error(error: MZError) -> StreamResult`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &StreamResult) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> StreamResult`



## Module: deflate

This module contains functionality for compression.



## Module: inflate

This module contains functionality for decompression.



