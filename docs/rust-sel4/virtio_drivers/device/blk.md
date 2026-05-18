**virtio_drivers > device > blk**

# Module: device::blk

## Contents

**Structs**

- [`BlkConfig`](#blkconfig)
- [`BlkFeature`](#blkfeature)
- [`BlkReq`](#blkreq) - A VirtIO block device request.
- [`BlkResp`](#blkresp) - Response of a VirtIOBlk request.
- [`RespStatus`](#respstatus) - Status of a VirtIOBlk request.
- [`VirtIOBlk`](#virtioblk) - Driver for a VirtIO block device.

**Enums**

- [`ReqType`](#reqtype)

**Constants**

- [`QUEUE`](#queue)
- [`QUEUE_SIZE`](#queue_size)
- [`SECTOR_SIZE`](#sector_size) - The standard sector size of a VirtIO block device. Data is read and written in multiples of this
- [`SUPPORTED_FEATURES`](#supported_features)

---

## virtio_drivers::device::blk::BlkConfig

*Struct*

**Fields:**
- `capacity_low: crate::config::ReadOnly<u32>` - Number of 512 Bytes sectors
- `capacity_high: crate::config::ReadOnly<u32>`
- `size_max: crate::config::ReadOnly<u32>`
- `seg_max: crate::config::ReadOnly<u32>`
- `cylinders: crate::config::ReadOnly<u16>`
- `heads: crate::config::ReadOnly<u8>`
- `sectors: crate::config::ReadOnly<u8>`
- `blk_size: crate::config::ReadOnly<u32>`
- `physical_block_exp: crate::config::ReadOnly<u8>`
- `alignment_offset: crate::config::ReadOnly<u8>`
- `min_io_size: crate::config::ReadOnly<u16>`
- `opt_io_size: crate::config::ReadOnly<u32>`

**Traits:** FromBytes, FromZeros, IntoBytes, TryFromBytes, Immutable



## virtio_drivers::device::blk::BlkFeature

*Struct*

**Tuple Struct**: `(<BlkFeature as $crate::__private::PublicFlags>::Internal)`

**Methods:**

- `fn iter(self: &Self) -> $crate::iter::Iter<BlkFeature>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<BlkFeature>` - Yield a set of contained named flags values.
- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u64` - Get the underlying bits value.
- `fn from_bits(bits: u64) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u64) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u64) -> Self` - Convert from a bits value exactly.
- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>` - Get a flags value with the bits of a flag with the given name set.
- `fn is_empty(self: &Self) -> bool` - Whether all bits in this flags value are unset.
- `fn is_all(self: &Self) -> bool` - Whether all known bits in this flags value are set.
- `fn intersects(self: &Self, other: Self) -> bool` - Whether any set bits in a source flags value are also set in a target flags value.
- `fn contains(self: &Self, other: Self) -> bool` - Whether all set bits in a source flags value are also set in a target flags value.
- `fn insert(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- `fn remove(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags
- `fn toggle(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- `fn set(self: & mut Self, other: Self, value: bool)` - Call `insert` when `value` is `true` or `remove` when `value` is `false`.
- `fn intersection(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- `fn union(self: Self, other: Self) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- `fn difference(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags
- `fn symmetric_difference(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- `fn complement(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.

**Traits:** Copy, PublicFlags, Eq

**Trait Implementations:**

- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Default**
  - `fn default() -> BlkFeature`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> BlkFeature`
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **BitOr**
  - `fn bitor(self: Self, other: BlkFeature) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **PartialEq**
  - `fn eq(self: &Self, other: &BlkFeature) -> bool`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Flags**
  - `fn bits(self: &Self) -> u64`
  - `fn from_bits_retain(bits: u64) -> BlkFeature`



## virtio_drivers::device::blk::BlkReq

*Struct*

A VirtIO block device request.

**Fields:**
- `type_: ReqType`
- `reserved: u32`
- `sector: u64`

**Traits:** IntoBytes, KnownLayout, Immutable

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## virtio_drivers::device::blk::BlkResp

*Struct*

Response of a VirtIOBlk request.

**Fields:**
- `status: RespStatus`

**Methods:**

- `fn status(self: &Self) -> RespStatus` - Return the status of a VirtIOBlk request.

**Traits:** TryFromBytes, Immutable, KnownLayout, FromBytes, FromZeros, IntoBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## virtio_drivers::device::blk::QUEUE

*Constant*: `u16`



## virtio_drivers::device::blk::QUEUE_SIZE

*Constant*: `u16`



## virtio_drivers::device::blk::ReqType

*Enum*

**Variants:**
- `In`
- `Out`
- `Flush`
- `GetId`
- `GetLifetime`
- `Discard`
- `WriteZeroes`
- `SecureErase`

**Traits:** IntoBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::blk::RespStatus

*Struct*

Status of a VirtIOBlk request.

**Tuple Struct**: `(u8)`

**Methods:**


**Traits:** TryFromBytes, Immutable, KnownLayout, Copy, FromBytes, Eq, FromZeros, IntoBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RespStatus`
- **PartialEq**
  - `fn eq(self: &Self, other: &RespStatus) -> bool`



## virtio_drivers::device::blk::SECTOR_SIZE

*Constant*: `usize`

The standard sector size of a VirtIO block device. Data is read and written in multiples of this
size.



## virtio_drivers::device::blk::SUPPORTED_FEATURES

*Constant*: `BlkFeature`



## virtio_drivers::device::blk::VirtIOBlk

*Struct*

Driver for a VirtIO block device.

This is a simple virtual block device, e.g. disk.

Read and write requests (and other exotic requests) are placed in the queue and serviced
(probably out of order) by the device except where noted.

# Example

```
# use virtio_drivers::{Error, Hal};
# use virtio_drivers::transport::Transport;
use virtio_drivers::device::blk::{VirtIOBlk, SECTOR_SIZE};

# fn example<HalImpl: Hal, T: Transport>(transport: T) -> Result<(), Error> {
let mut disk = VirtIOBlk::<HalImpl, _>::new(transport)?;

println!("VirtIO block device: {} kB", disk.capacity() * SECTOR_SIZE as u64 / 2);

// Read sector 0 and then copy it to sector 1.
let mut buf = [0; SECTOR_SIZE];
disk.read_blocks(0, &mut buf)?;
disk.write_blocks(1, &buf)?;
# Ok(())
# }
```

**Generic Parameters:**
- H
- T

**Fields:**
- `transport: T`
- `queue: crate::queue::VirtQueue<H, { _ }>`
- `capacity: u64`
- `negotiated_features: BlkFeature`

**Methods:**

- `fn new(transport: T) -> Result<Self>` - Create a new VirtIO-Blk driver.
- `fn capacity(self: &Self) -> u64` - Gets the capacity of the block device, in 512 byte ([`SECTOR_SIZE`]) sectors.
- `fn readonly(self: &Self) -> bool` - Returns true if the block device is read-only, or false if it allows writes.
- `fn ack_interrupt(self: & mut Self) -> InterruptStatus` - Acknowledges a pending interrupt, if any.
- `fn enable_interrupts(self: & mut Self)` - Enables interrupts from the device.
- `fn disable_interrupts(self: & mut Self)` - Disables interrupts from the device.
- `fn request(self: & mut Self, request: BlkReq) -> Result` - Sends the given request to the device and waits for a response, with no extra data.
- `fn request_read(self: & mut Self, request: BlkReq, data: & mut [u8]) -> Result` - Sends the given request to the device and waits for a response, including the given data.
- `fn request_write(self: & mut Self, request: BlkReq, data: &[u8]) -> Result` - Sends the given request and data to the device and waits for a response.
- `fn flush(self: & mut Self) -> Result` - Requests the device to flush any pending writes to storage.
- `fn device_id(self: & mut Self, id: & mut [u8; 20]) -> Result<usize>` - Gets the device ID.
- `fn read_blocks(self: & mut Self, block_id: usize, buf: & mut [u8]) -> Result` - Reads one or more blocks into the given buffer.
- `fn read_blocks_nb(self: & mut Self, block_id: usize, req: & mut BlkReq, buf: & mut [u8], resp: & mut BlkResp) -> Result<u16>` - Submits a request to read one or more blocks, but returns immediately without waiting for
- `fn complete_read_blocks(self: & mut Self, token: u16, req: &BlkReq, buf: & mut [u8], resp: & mut BlkResp) -> Result<()>` - Completes a read operation which was started by `read_blocks_nb`.
- `fn write_blocks(self: & mut Self, block_id: usize, buf: &[u8]) -> Result` - Writes the contents of the given buffer to a block or blocks.
- `fn write_blocks_nb(self: & mut Self, block_id: usize, req: & mut BlkReq, buf: &[u8], resp: & mut BlkResp) -> Result<u16>` - Submits a request to write one or more blocks, but returns immediately without waiting for
- `fn complete_write_blocks(self: & mut Self, token: u16, req: &BlkReq, buf: &[u8], resp: & mut BlkResp) -> Result<()>` - Completes a write operation which was started by `write_blocks_nb`.
- `fn peek_used(self: & mut Self) -> Option<u16>` - Fetches the token of the next completed request from the used ring and returns it, without
- `fn virt_queue_size(self: &Self) -> u16` - Returns the size of the device's VirtQueue.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`



