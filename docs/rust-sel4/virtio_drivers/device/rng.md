**virtio_drivers > device > rng**

# Module: device::rng

## Contents

**Structs**

- [`VirtIORng`](#virtiorng) - Driver for a VirtIO random number generator device.

**Constants**

- [`QUEUE_IDX`](#queue_idx)
- [`QUEUE_SIZE`](#queue_size)
- [`SUPPORTED_FEATURES`](#supported_features)

---

## virtio_drivers::device::rng::QUEUE_IDX

*Constant*: `u16`



## virtio_drivers::device::rng::QUEUE_SIZE

*Constant*: `usize`



## virtio_drivers::device::rng::SUPPORTED_FEATURES

*Constant*: `super::common::Feature`



## virtio_drivers::device::rng::VirtIORng

*Struct*

Driver for a VirtIO random number generator device.

**Generic Parameters:**
- H
- T

**Fields:**
- `transport: T`
- `queue: crate::queue::VirtQueue<H, QUEUE_SIZE>`

**Methods:**

- `fn new(transport: T) -> Result<Self>` - Create a new driver with the given transport.
- `fn request_entropy(self: & mut Self, dst: & mut [u8]) -> Result<usize>` - Request random bytes from the device to be stored into `dst`.
- `fn enable_interrupts(self: & mut Self)` - Enable interrupts.
- `fn disable_interrupts(self: & mut Self)` - Disable interrupts.
- `fn ack_interrupt(self: & mut Self) -> InterruptStatus` - Acknowledge interrupt.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`



