**sel4_virtio_hal_impl**

# Module: sel4_virtio_hal_impl

## Contents

**Structs**

- [`HalImpl`](#halimpl)

---

## sel4_virtio_hal_impl::HalImpl

*Struct*

**Unit Struct**

**Methods:**

- `fn init(dma_region_size: usize, dma_region_vaddr: usize, dma_region_paddr: usize)`

**Trait Implementations:**

- **Hal**
  - `fn dma_alloc(pages: usize, _direction: BufferDirection) -> (PhysAddr, NonNull<u8>)`
  - `fn dma_dealloc(paddr: PhysAddr, _vaddr: NonNull<u8>, pages: usize) -> i32`
  - `fn mmio_phys_to_virt(_paddr: PhysAddr, _size: usize) -> NonNull<u8>`
  - `fn share(buffer: NonNull<[u8]>, _direction: BufferDirection) -> PhysAddr`
  - `fn unshare(paddr: PhysAddr, buffer: NonNull<[u8]>, direction: BufferDirection)`



