**virtio_drivers > device > virtio_9p**

# Module: device::virtio_9p

## Contents

**Structs**

- [`VirtIO9p`](#virtio9p) - Driver for a VirtIO 9p device.

**Functions**

- [`read_mount_tag`](#read_mount_tag)

**Constants**

- [`P9_HEADER_SIZE`](#p9_header_size)
- [`QUEUE`](#queue)
- [`QUEUE_SIZE`](#queue_size)
- [`SUPPORTED_FEATURES`](#supported_features)

---

## virtio_drivers::device::virtio_9p::P9_HEADER_SIZE

*Constant*: `usize`



## virtio_drivers::device::virtio_9p::QUEUE

*Constant*: `u16`



## virtio_drivers::device::virtio_9p::QUEUE_SIZE

*Constant*: `usize`



## virtio_drivers::device::virtio_9p::SUPPORTED_FEATURES

*Constant*: `super::common::Feature`



## virtio_drivers::device::virtio_9p::VirtIO9p

*Struct*

Driver for a VirtIO 9p device.

**Generic Parameters:**
- H
- T

**Fields:**
- `transport: T`
- `queue: crate::queue::VirtQueue<H, QUEUE_SIZE>`
- `mount_tag: alloc::string::String`

**Methods:**

- `fn new(transport: T) -> Result<Self>` - Create a new VirtIO 9p driver.
- `fn mount_tag(self: &Self) -> &str` - Returns the mount tag reported by the device.
- `fn request(self: & mut Self, req: &[u8], resp: & mut [u8]) -> Result<u32>` - Sends a raw 9p request and waits for the response.



## virtio_drivers::device::virtio_9p::read_mount_tag

*Function*

```rust
fn read_mount_tag<T>(transport: &T) -> crate::Result<alloc::string::String>
```



