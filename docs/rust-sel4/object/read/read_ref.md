**object > read > read_ref**

# Module: read::read_ref

## Contents

**Traits**

- [`ReadRef`](#readref) - A trait for reading references to [`Pod`] types from a block of data.

---

## object::read::read_ref::ReadRef

*Trait*

A trait for reading references to [`Pod`] types from a block of data.

This allows parsers to handle both of these cases:
- the block of data exists in memory, and it is desirable
  to use references to this block instead of copying it,
- the block of data exists in storage, and it is desirable
  to read on demand to minimize I/O and memory usage.

A block of data typically exists in memory as a result of using a memory
mapped file, and the crate was written with this use case in mind.
Reading the entire file into a `Vec` is also possible, but it often uses
more I/O and memory.
Both of these are handled by the `ReadRef` implementation for `&[u8]`.

For the second use case, the `ReadRef` trait is implemented for
[`&ReadCache`](super::ReadCache). This is useful for environments where
memory mapped files are not available or not suitable, such as WebAssembly.
This differs from reading into a `Vec` in that it only reads the portions
of the file that are needed for parsing.

The methods accept `self` by value because `Self` is expected to behave
similar to a reference: it may be a reference with a lifetime of `'a`,
or it may be a wrapper of a reference.

The `Clone` and `Copy` bounds are for convenience, and since `Self` is
expected to be similar to a reference, these are easily satisfied.

Object file parsers typically use offsets to locate the structures
in the block, and will most commonly use the `*_at` methods to
read a structure at a known offset.

Occasionally file parsers will need to treat the block as a stream,
and so convenience methods are provided that update an offset with
the size that was read.

**Methods:**

- `len`: The total size of the block of data.
- `read_bytes_at`: Get a reference to a `u8` slice at the given offset.
- `read_bytes_at_until`: Get a reference to a delimited `u8` slice which starts at range.start.
- `read_bytes`: Get a reference to a `u8` slice at the given offset, and update the offset.
- `read`: Get a reference to a `Pod` type at the given offset, and update the offset.
- `read_at`: Get a reference to a `Pod` type at the given offset.
- `read_slice`: Get a reference to a slice of a `Pod` type at the given offset, and update the offset.
- `read_slice_at`: Get a reference to a slice of a `Pod` type at the given offset.



