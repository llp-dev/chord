*[gimli](../../index.md) / [read](../index.md) / [endian_reader](index.md)*

---

# Module `endian_reader`

Defining custom `Reader`s quickly.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EndianReader`](#endianreader) | struct | An easy way to define a custom `Reader` implementation with a reference to a generic buffer of bytes and an associated endianity. |
| [`SubRange`](#subrange) | struct |  |
| [`EndianRcSlice`](#endianrcslice) | type | A reference counted, non-thread-safe slice of bytes and associated endianity. |
| [`EndianArcSlice`](#endianarcslice) | type | An atomically reference counted, thread-safe slice of bytes and associated endianity. |

## Structs

### `EndianReader<Endian, T>`

```rust
struct EndianReader<Endian, T>
where
    Endian: Endianity,
    T: CloneStableDeref<Target = [u8]> + Debug {
    range: SubRange<T>,
    endian: Endian,
}
```

An easy way to define a custom `Reader` implementation with a reference to a
generic buffer of bytes and an associated endianity.

Note that the whole original buffer is kept alive in memory even if there is
only one reader that references only a handful of bytes from that original
buffer. That is, `EndianReader` will not do any copying, moving, or
compacting in order to free up unused regions of the original buffer. If you
require this kind of behavior, it is up to you to implement `Reader`
directly by-hand.

# Example

Say you have an `mmap`ed file that you want to serve as a `gimli::Reader`.
You can wrap that `mmap`ed file up in a `MmapFile` type and use
`EndianReader<Rc<MmapFile>>` or `EndianReader<Arc<MmapFile>>` as readers as
long as `MmapFile` dereferences to the underlying `[u8]` data.

```rust
use std::io;
use std::ops::Deref;
use std::path::Path;
use std::slice;
use std::sync::Arc;

/// A type that represents an `mmap`ed file.
#[derive(Debug)]
pub struct MmapFile {
    ptr: *const u8,
    len: usize,
}

impl MmapFile {
    pub fn new(path: &Path) -> io::Result<MmapFile> {
        // Call `mmap` and check for errors and all that...
      unimplemented!()
    }
}

impl Drop for MmapFile {
    fn drop(&mut self) {
        // Call `munmap` to clean up after ourselves...
      unimplemented!()
    }
}

// And `MmapFile` can deref to a slice of the `mmap`ed region of memory.
impl Deref for MmapFile {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

/// A type that represents a shared `mmap`ed file.
#[derive(Debug, Clone)]
pub struct ArcMmapFile(Arc<MmapFile>);

// And `ArcMmapFile` can deref to a slice of the `mmap`ed region of memory.
impl Deref for ArcMmapFile {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

// These are both valid for any `Rc` or `Arc`.
unsafe impl gimli::StableDeref for ArcMmapFile {}
unsafe impl gimli::CloneStableDeref for ArcMmapFile {}

/// A `gimli::Reader` that is backed by an `mmap`ed file!
pub type MmapFileReader<Endian> = gimli::EndianReader<Endian, ArcMmapFile>;
fn test(_: &MmapFileReader<gimli::NativeEndian>) { }
```

#### Implementations

- <span id="endianreader-new"></span>`fn new(bytes: T, endian: Endian) -> EndianReader<Endian, T>` — [`EndianReader`](../index.md#endianreader)

  Construct a new `EndianReader` with the given bytes.

- <span id="endianreader-bytes"></span>`fn bytes(&self) -> &[u8]`

  Return a reference to the raw bytes underlying this reader.

#### Trait Implementations

##### `impl<Endian, T> Clone for EndianReader<Endian, T>`

- <span id="endianreader-clone"></span>`fn clone(&self) -> EndianReader<Endian, T>` — [`EndianReader`](../index.md#endianreader)

##### `impl<Endian, T> Copy for EndianReader<Endian, T>`

##### `impl<Endian, T> Debug for EndianReader<Endian, T>`

- <span id="endianreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Endian, T> Deref for EndianReader<Endian, T>`

- <span id="endianreader-deref-type-target"></span>`type Target = [u8]`

- <span id="endianreader-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<Endian, T> Eq for EndianReader<Endian, T>`

##### `impl<Endian, T> Hash for EndianReader<Endian, T>`

- <span id="endianreader-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<Endian, T> Index for EndianReader<Endian, T>`

- <span id="endianreader-index-type-output"></span>`type Output = u8`

- <span id="endianreader-index"></span>`fn index(&self, idx: usize) -> &<Self as >::Output`

##### `impl<Endian, T1, T2> PartialEq for EndianReader<Endian, T1>`

- <span id="endianreader-partialeq-eq"></span>`fn eq(&self, rhs: &EndianReader<Endian, T2>) -> bool` — [`EndianReader`](../index.md#endianreader)

##### `impl<Endian, T> Reader for EndianReader<Endian, T>`

- <span id="endianreader-reader-type-endian"></span>`type Endian = Endian`

- <span id="endianreader-reader-type-offset"></span>`type Offset = usize`

- <span id="endianreader-reader-endian"></span>`fn endian(&self) -> Endian`

- <span id="endianreader-reader-len"></span>`fn len(&self) -> usize`

- <span id="endianreader-reader-empty"></span>`fn empty(&mut self)`

- <span id="endianreader-reader-truncate"></span>`fn truncate(&mut self, len: usize) -> Result<()>` — [`Result`](../../index.md#result)

- <span id="endianreader-reader-offset-from"></span>`fn offset_from(&self, base: &EndianReader<Endian, T>) -> usize` — [`EndianReader`](../index.md#endianreader)

- <span id="endianreader-reader-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](../index.md#readeroffsetid)

- <span id="endianreader-reader-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](../index.md#readeroffsetid), [`Reader`](../index.md#reader)

- <span id="endianreader-reader-find"></span>`fn find(&self, byte: u8) -> Result<usize>` — [`Result`](../../index.md#result)

- <span id="endianreader-reader-skip"></span>`fn skip(&mut self, len: usize) -> Result<()>` — [`Result`](../../index.md#result)

- <span id="endianreader-reader-split"></span>`fn split(&mut self, len: usize) -> Result<Self>` — [`Result`](../../index.md#result)

- <span id="endianreader-reader-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../../index.md#result)

- <span id="endianreader-reader-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md#result)

- <span id="endianreader-reader-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md#result)

- <span id="endianreader-reader-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../../index.md#result)

##### `impl<T> Receiver for EndianReader<Endian, T>`

- <span id="endianreader-receiver-type-target"></span>`type Target = T`

### `SubRange<T>`

```rust
struct SubRange<T>
where
    T: CloneStableDeref<Target = [u8]> + Debug {
    bytes: T,
    ptr: *const u8,
    len: usize,
}
```

#### Implementations

- <span id="subrange-new"></span>`fn new(bytes: T) -> Self`

- <span id="subrange-bytes"></span>`fn bytes(&self) -> &[u8]`

- <span id="subrange-len"></span>`fn len(&self) -> usize`

- <span id="subrange-truncate"></span>`fn truncate(&mut self, len: usize)`

- <span id="subrange-skip"></span>`fn skip(&mut self, len: usize)`

- <span id="subrange-read-slice"></span>`fn read_slice(&mut self, len: usize) -> Option<&[u8]>`

#### Trait Implementations

##### `impl<T> Clone for SubRange<T>`

- <span id="subrange-clone"></span>`fn clone(&self) -> SubRange<T>` — [`SubRange`](#subrange)

##### `impl<T> Copy for SubRange<T>`

##### `impl<T> Debug for SubRange<T>`

- <span id="subrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Eq for SubRange<T>`

##### `impl<T> Hash for SubRange<T>`

- <span id="subrange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> PartialEq for SubRange<T>`

- <span id="subrange-partialeq-eq"></span>`fn eq(&self, other: &SubRange<T>) -> bool` — [`SubRange`](#subrange)

##### `impl<T> Send for SubRange<T>`

##### `impl<T> StructuralPartialEq for SubRange<T>`

##### `impl<T> Sync for SubRange<T>`

## Type Aliases

### `EndianRcSlice<Endian>`

```rust
type EndianRcSlice<Endian> = EndianReader<Endian, alloc::rc::Rc<[u8]>>;
```

A reference counted, non-thread-safe slice of bytes and associated
endianity.

```rust
#[cfg(feature = "std")] {
use std::rc::Rc;

let buf = Rc::from(&[1, 2, 3, 4][..]);
let reader = gimli::EndianRcSlice::new(buf, gimli::NativeEndian);
let _ = reader;
}
```

### `EndianArcSlice<Endian>`

```rust
type EndianArcSlice<Endian> = EndianReader<Endian, alloc::sync::Arc<[u8]>>;
```

An atomically reference counted, thread-safe slice of bytes and associated
endianity.

```rust
#[cfg(feature = "std")] {
use std::sync::Arc;

let buf = Arc::from(&[1, 2, 3, 4][..]);
let reader = gimli::EndianArcSlice::new(buf, gimli::NativeEndian);
let _ = reader;
}
```

