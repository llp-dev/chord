**sel4_async_block_io > when_alloc**

# Module: when_alloc

## Contents

**Structs**

- [`CachedBlockIO`](#cachedblockio)
- [`DynamicBlockSize`](#dynamicblocksize)

---

## sel4_async_block_io::when_alloc::CachedBlockIO

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(inner: T, cache_size_in_blocks: usize) -> Self`
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



## sel4_async_block_io::when_alloc::DynamicBlockSize

*Struct*

**Methods:**

- `fn new(bits: usize) -> Self`

**Trait Implementations:**

- **BlockSize**
  - `fn bytes(self: &Self) -> usize`
  - `fn zeroed_block(self: &Self) -> <Self as >::Block`



