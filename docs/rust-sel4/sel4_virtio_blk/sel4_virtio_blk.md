**sel4_virtio_blk**

# Module: sel4_virtio_blk

## Contents

**Structs**

- [`GetBlockDeviceLayoutWrapper`](#getblockdevicelayoutwrapper)

---

## sel4_virtio_blk::GetBlockDeviceLayoutWrapper

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Trait Implementations:**

- **GetBlockDeviceLayout**
  - `fn get_block_size(self: & mut Self) -> Result<usize, <Self as >::Error>`
  - `fn get_num_blocks(self: & mut Self) -> Result<u64, <Self as >::Error>`



