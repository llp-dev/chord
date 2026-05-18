**virtio_drivers > hal**

# Module: hal

## Contents

**Structs**

- [`Dma`](#dma) - A region of contiguous physical memory used for DMA.

**Enums**

- [`BufferDirection`](#bufferdirection) - The direction in which a buffer is passed.

**Traits**

- [`Hal`](#hal) - The interface which a particular hardware implementation must implement.

**Type Aliases**

- [`PhysAddr`](#physaddr) - A physical address as used for virtio.

---

## virtio_drivers::hal::BufferDirection

*Enum*

The direction in which a buffer is passed.

**Variants:**
- `DriverToDevice` - The buffer may be read or written by the driver, but only read by the device.
- `DeviceToDriver` - The buffer may be read or written by the device, but only read by the driver.
- `Both` - The buffer may be read or written by both the device and the driver.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> BufferDirection`
- **PartialEq**
  - `fn eq(self: &Self, other: &BufferDirection) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::hal::Dma

*Struct*

A region of contiguous physical memory used for DMA.

**Generic Parameters:**
- H

**Fields:**
- `paddr: PhysAddr`
- `vaddr: core::ptr::NonNull<u8>`
- `pages: usize`
- `_hal: core::marker::PhantomData<H>`

**Methods:**

- `fn new(pages: usize, direction: BufferDirection) -> Result<Self>` - Allocates the given number of pages of physically contiguous memory to be used for DMA in
- `fn paddr(self: &Self) -> PhysAddr` - Returns the physical address of the start of the DMA region, as seen by devices.
- `fn vaddr(self: &Self, offset: usize) -> NonNull<u8>` - Returns a pointer to the given offset within the DMA region.
- `fn raw_slice(self: &Self) -> NonNull<[u8]>` - Returns a pointer to the entire DMA region as a slice.

**Traits:** Sync, Send

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::hal::Hal

*Trait*

The interface which a particular hardware implementation must implement.

# Safety

Implementations of this trait must follow the "implementation safety" requirements documented
for each method. Callers must follow the safety requirements documented for the unsafe methods.

**Methods:**

- `dma_alloc`: Allocates and zeroes the given number of contiguous physical pages of DMA memory for VirtIO
- `dma_dealloc`: Deallocates the given contiguous physical DMA memory pages.
- `mmio_phys_to_virt`: Converts a physical address used for MMIO to a virtual address which the driver can access.
- `share`: Shares the given memory range with the device, and returns the physical address that the
- `unshare`: Unshares the given memory range from the device and (if necessary) copies it back to the



## virtio_drivers::hal::PhysAddr

*Type Alias*: `u64`

A physical address as used for virtio.



