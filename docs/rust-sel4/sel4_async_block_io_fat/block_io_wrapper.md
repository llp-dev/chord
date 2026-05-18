**sel4_async_block_io_fat > block_io_wrapper**

# Module: block_io_wrapper

## Contents

**Structs**

- [`BlockIOWrapper`](#blockiowrapper)

---

## sel4_async_block_io_fat::block_io_wrapper::BlockIOWrapper

*Struct*

**Generic Parameters:**
- T
- A

**Methods:**

- `fn new(inner: T) -> Self`

**Trait Implementations:**

- **BlockDevice**
  - `fn read(self: &Self, blocks: & mut [fat::Block], start_block_idx: fat::BlockIdx, _reason: &str) -> Result<(), <Self as >::Error>`
  - `fn write(self: &Self, blocks: &[fat::Block], start_block_idx: fat::BlockIdx) -> Result<(), <Self as >::Error>`
  - `fn num_blocks(self: &Self) -> Result<fat::BlockCount, <Self as >::Error>`



