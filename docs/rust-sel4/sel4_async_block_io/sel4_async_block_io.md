**sel4_async_block_io**

# Module: sel4_async_block_io

## Contents

**Modules**

- [`access`](#access)
- [`constant_block_sizes`](#constant_block_sizes)
- [`disk`](#disk)

**Structs**

- [`BlockIOAdapter`](#blockioadapter)
- [`ByteIOAdapter`](#byteioadapter)
- [`NextBlockSizeAdapter`](#nextblocksizeadapter)
- [`Partition`](#partition)
- [`PrevBlockSizeAdapter`](#prevblocksizeadapter)
- [`SliceByteIO`](#slicebyteio)

**Functions**

- [`read_bytes`](#read_bytes)
- [`write_bytes`](#write_bytes)

**Traits**

- [`BlockIO`](#blockio)
- [`BlockIOLayout`](#blockiolayout)
- [`BlockSize`](#blocksize)
- [`ByteIO`](#byteio)
- [`ByteIOLayout`](#byteiolayout)
- [`ConstantBlockSize`](#constantblocksize)
- [`HasNextBlockSize`](#hasnextblocksize)
- [`HasPrevBlockSize`](#hasprevblocksize)

---

## sel4_async_block_io::BlockIO

*Trait*

**Methods:**

- `read_or_write_blocks`
- `read_blocks`
- `write_blocks`



## sel4_async_block_io::BlockIOAdapter

*Struct*

**Generic Parameters:**
- T
- N

**Methods:**

- `fn new(inner: T, block_size: N) -> Self`
- `fn into_inner(self: Self) -> T`
- `fn inner(self: &Self) -> &T`
- `fn inner_mut(self: & mut Self) -> & mut T`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BlockIOAdapter<T, N>`
- **BlockIO**
  - `fn read_or_write_blocks(self: &Self, start_block_idx: u64, operation: Operation<A>) -> Result<(), <Self as >::Error>`
- **BlockIOLayout**
  - `fn block_size(self: &Self) -> <Self as >::BlockSize`
  - `fn num_blocks(self: &Self) -> u64`



## sel4_async_block_io::BlockIOLayout

*Trait*

**Methods:**

- `Error`
- `BlockSize`
- `block_size`
- `num_blocks`



## sel4_async_block_io::BlockSize

*Trait*

**Methods:**

- `Block`
- `bytes`
- `bytes_u64`
- `zeroed_block`



## sel4_async_block_io::ByteIO

*Trait*

**Methods:**

- `read_or_write`
- `read`
- `write`



## sel4_async_block_io::ByteIOAdapter

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(inner: T) -> Self`
- `fn into_inner(self: Self) -> T`
- `fn inner(self: &Self) -> &T`
- `fn inner_mut(self: & mut Self) -> & mut T`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ByteIOAdapter<T>`
- **ByteIO**
  - `fn read_or_write(self: &Self, offset: u64, operation: Operation<A>) -> Result<(), <Self as >::Error>`
- **ByteIOLayout**
  - `fn size(self: &Self) -> u64`



## sel4_async_block_io::ByteIOLayout

*Trait*

**Methods:**

- `Error`
- `size`



## sel4_async_block_io::ConstantBlockSize

*Trait*

**Methods:**

- `BLOCK_SIZE`
- `BYTES`



## sel4_async_block_io::HasNextBlockSize

*Trait*

**Methods:**

- `NextBlockSize`



## sel4_async_block_io::HasPrevBlockSize

*Trait*

**Methods:**

- `PrevBlockSize`



## sel4_async_block_io::NextBlockSizeAdapter

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(inner: T) -> Self`
- `fn into_inner(self: Self) -> T`
- `fn inner(self: &Self) -> &T`
- `fn inner_mut(self: & mut Self) -> & mut T`

**Trait Implementations:**

- **BlockIO**
  - `fn read_or_write_blocks(self: &Self, start_block_idx: u64, operation: Operation<A>) -> Result<(), <Self as >::Error>`
- **BlockIOLayout**
  - `fn block_size(self: &Self) -> <Self as >::BlockSize`
  - `fn num_blocks(self: &Self) -> u64`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> NextBlockSizeAdapter<T>`



## sel4_async_block_io::Partition

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn into_inner(self: Self) -> T`
- `fn inner(self: &Self) -> &T`
- `fn inner_mut(self: & mut Self) -> & mut T`
- `fn new(inner: T, range: Range<u64>) -> Self`

**Trait Implementations:**

- **BlockIOLayout**
  - `fn block_size(self: &Self) -> <Self as >::BlockSize`
  - `fn num_blocks(self: &Self) -> u64`
- **BlockIO**
  - `fn read_or_write_blocks(self: &Self, start_block_idx: u64, operation: Operation<A>) -> Result<(), <Self as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Partition<T>`



## sel4_async_block_io::PrevBlockSizeAdapter

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(inner: T) -> Self`
- `fn into_inner(self: Self) -> T`
- `fn inner(self: &Self) -> &T`
- `fn inner_mut(self: & mut Self) -> & mut T`

**Trait Implementations:**

- **BlockIOLayout**
  - `fn block_size(self: &Self) -> <Self as >::BlockSize`
  - `fn num_blocks(self: &Self) -> u64`
- **BlockIO**
  - `fn read_or_write_blocks(self: &Self, start_block_idx: u64, operation: Operation<A>) -> Result<(), <Self as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PrevBlockSizeAdapter<T>`



## sel4_async_block_io::SliceByteIO

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(inner: T) -> Self`
- `fn into_inner(self: Self) -> T`
- `fn inner(self: &Self) -> &T`
- `fn inner_mut(self: & mut Self) -> & mut T`

**Trait Implementations:**

- **ByteIOLayout**
  - `fn size(self: &Self) -> u64`
- **ByteIO**
  - `fn read_or_write(self: &Self, offset: u64, operation: Operation<ReadOnly>) -> Result<(), <Self as >::Error>`



## Module: access



## Module: constant_block_sizes



## Module: disk



## sel4_async_block_io::read_bytes

*Function*

```rust
fn read_bytes<T, A>(io: &T, offset: u64, buf: & mut [u8]) -> Result<(), <T as >::Error>
```



## sel4_async_block_io::write_bytes

*Function*

```rust
fn write_bytes<T, A>(io: &T, offset: u64, buf: &[u8]) -> Result<(), <T as >::Error>
```



