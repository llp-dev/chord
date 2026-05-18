**virtio_drivers > queue > owning**

# Module: queue::owning

## Contents

**Structs**

- [`OwningQueue`](#owningqueue) - A wrapper around [`VirtQueue`] that owns all the buffers that are passed to the queue.

---

## virtio_drivers::queue::owning::OwningQueue

*Struct*

A wrapper around [`VirtQueue`] that owns all the buffers that are passed to the queue.

**Generic Parameters:**
- H
- const SIZE
- const BUFFER_SIZE

**Fields:**
- `queue: super::VirtQueue<H, SIZE>`
- `buffers: [core::ptr::NonNull<[u8; BUFFER_SIZE]>; SIZE]`

**Methods:**

- `fn new(queue: VirtQueue<H, SIZE>) -> Result<Self>` - Constructs a new `OwningQueue` wrapping around the given `VirtQueue`.
- `fn should_notify(self: &Self) -> bool` - Returns whether the driver should notify the device after adding a new buffer to the
- `fn set_dev_notify(self: & mut Self, enable: bool)` - Tells the device whether to send used buffer notifications.
- `fn add_buffer_to_queue<impl Transport>(self: & mut Self, index: u16, transport: & mut impl Trait) -> Result` - Adds the buffer at the given index in `buffers` back to the queue.
- `fn pop(self: & mut Self) -> Result<Option<(&[u8], u16)>>`
- `fn poll<T, impl Transport, impl FnOnce(&[u8]) -> Result<Option<T>>>(self: & mut Self, transport: & mut impl Trait, handler: impl Trait) -> Result<Option<T>>` - Checks whether there are any buffers which the device has marked as used so the driver

**Traits:** Sync, Send

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



