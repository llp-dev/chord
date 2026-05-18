**object > read > read_cache**

# Module: read::read_cache

## Contents

**Structs**

- [`ReadCache`](#readcache) - An implementation of [`ReadRef`] for data in a stream that implements
- [`ReadCacheRange`](#readcacherange) - An implementation of [`ReadRef`] for a range of data in a stream that

**Traits**

- [`ReadCacheOps`](#readcacheops) - Operations required to implement [`ReadCache`].

---

## object::read::read_cache::ReadCache

*Struct*

An implementation of [`ReadRef`] for data in a stream that implements
`Read + Seek`.

Contains a cache of read-only blocks of data, allowing references to
them to be returned. Entries in the cache are never removed.
Entries are keyed on the offset and size of the read.
Currently overlapping reads are considered separate reads.

This is primarily intended for environments where memory mapped files
are not available or not suitable, such as WebAssembly.

Note that malformed files can cause the cache to grow much larger than
the file size.

**Generic Parameters:**
- R

**Methods:**

- `fn new(read: R) -> Self` - Create an empty `ReadCache` for the given stream.
- `fn range(self: &Self, offset: u64, size: u64) -> ReadCacheRange<R>` - Return an implementation of `ReadRef` that restricts reads
- `fn clear(self: & mut Self)` - Free buffers used by the cache.
- `fn into_inner(self: Self) -> R` - Unwrap this `ReadCache<R>`, returning the underlying reader.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::read_cache::ReadCacheOps

*Trait*

Operations required to implement [`ReadCache`].

This is a subset of the `Read` and `Seek` traits.
A blanket implementation is provided for all types that implement
`Read + Seek`.

**Methods:**

- `len`: Return the length of the stream.
- `seek`: Seek to the given position in the stream.
- `read`: Read up to `buf.len()` bytes into `buf`.
- `read_exact`: Read exactly `buf.len()` bytes into `buf`.



## object::read::read_cache::ReadCacheRange

*Struct*

An implementation of [`ReadRef`] for a range of data in a stream that
implements `Read + Seek`.

Shares an underlying [`ReadCache`] with a lifetime of `'a`.

**Generic Parameters:**
- 'a
- R

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ReadRef**
  - `fn len(self: Self) -> Result<u64, ()>`
  - `fn read_bytes_at(self: Self, offset: u64, size: u64) -> Result<&'a [u8], ()>`
  - `fn read_bytes_at_until(self: Self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



