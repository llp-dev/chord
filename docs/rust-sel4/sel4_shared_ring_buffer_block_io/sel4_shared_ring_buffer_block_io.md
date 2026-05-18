**sel4_shared_ring_buffer_block_io**

# Module: sel4_shared_ring_buffer_block_io

## Contents

**Structs**

- [`RequestFuture`](#requestfuture)
- [`SharedRingBufferBlockIO`](#sharedringbufferblockio)

---

## sel4_shared_ring_buffer_block_io::RequestFuture

*Struct*

**Generic Parameters:**
- 'a
- N
- P
- A
- F

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## sel4_shared_ring_buffer_block_io::SharedRingBufferBlockIO

*Struct*

**Generic Parameters:**
- N
- P
- A
- F

**Methods:**

- `fn new(block_size: N, num_blocks: u64, dma_region: SharedMemoryRef<'static, [u8]>, bounce_buffer_allocator: A, ring_buffers: RingBuffers<'static, Provide, F, BlockIORequest>) -> Self`
- `fn poll(self: &Self) -> Result<bool, Error>`

**Trait Implementations:**

- **BlockIOLayout**
  - `fn block_size(self: &Self) -> <Self as >::BlockSize`
  - `fn num_blocks(self: &Self) -> u64`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **BlockIO**
  - `fn read_or_write_blocks(self: &Self, start_block_idx: u64, operation: Operation<P>) -> Result<(), <Self as >::Error>`



