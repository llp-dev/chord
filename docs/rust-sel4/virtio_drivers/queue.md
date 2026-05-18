**virtio_drivers > queue**

# Module: queue

## Contents

**Modules**

- [`owning`](#owning)

**Structs**

- [`AvailRing`](#availring) - The driver uses the available ring to offer buffers to the device:
- [`DescFlags`](#descflags) - Descriptor flags
- [`Descriptor`](#descriptor)
- [`InputOutputIter`](#inputoutputiter)
- [`UsedElem`](#usedelem)
- [`UsedRing`](#usedring) - The used ring is where the device returns buffers once it is done with them:
- [`VirtQueue`](#virtqueue) - The mechanism for bulk data transport on virtio devices.

**Enums**

- [`VirtQueueLayout`](#virtqueuelayout) - The inner layout of a VirtQueue.

**Functions**

- [`queue_part_sizes`](#queue_part_sizes) - Returns the size in bytes of the descriptor table, available ring and used ring for a given
- [`take_first`](#take_first)
- [`take_first_mut`](#take_first_mut)

---

## virtio_drivers::queue::AvailRing

*Struct*

The driver uses the available ring to offer buffers to the device:
each ring entry refers to the head of a descriptor chain.
It is only written by the driver and read by the device.

**Generic Parameters:**
- const SIZE

**Fields:**
- `flags: core::sync::atomic::AtomicU16`
- `idx: core::sync::atomic::AtomicU16` - A driver MUST NOT decrement the idx.
- `ring: [u16; SIZE]`
- `used_event: core::sync::atomic::AtomicU16` - Only used if `VIRTIO_F_EVENT_IDX` is negotiated.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::queue::DescFlags

*Struct*

Descriptor flags

**Tuple Struct**: `(u16)`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u16` - Get the underlying bits value.
- `fn from_bits(bits: u16) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u16) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u16) -> Self` - Convert from a bits value exactly.
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
- `fn iter(self: &Self) -> $crate::iter::Iter<DescFlags>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<DescFlags>` - Yield a set of contained named flags values.

**Traits:** Eq, FromBytes, Copy, FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in two flags values.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Flags**
  - `fn bits(self: &Self) -> u16`
  - `fn from_bits_retain(bits: u16) -> DescFlags`
- **Default**
  - `fn default() -> DescFlags`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Clone**
  - `fn clone(self: &Self) -> DescFlags`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: DescFlags) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **PartialEq**
  - `fn eq(self: &Self, other: &DescFlags) -> bool`
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.



## virtio_drivers::queue::Descriptor

*Struct*

**Fields:**
- `addr: u64`
- `len: u32`
- `flags: DescFlags`
- `next: u16`

**Methods:**

- `fn set_buf<H>(self: & mut Self, buf: NonNull<[u8]>, direction: BufferDirection, extra_flags: DescFlags)` - Sets the buffer address, length and flags, and shares it with the device.
- `fn unset_buf(self: & mut Self)` - Sets the buffer address and length to 0.
- `fn next(self: &Self) -> Option<u16>` - Returns the index of the next descriptor in the chain if the `NEXT` flag is set, or `None`

**Traits:** FromBytes, FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Descriptor`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::queue::InputOutputIter

*Struct*

**Generic Parameters:**
- 'a
- 'b

**Fields:**
- `inputs: &'a [&'b [u8]]`
- `outputs: &'a  mut [&'b  mut [u8]]`

**Methods:**

- `fn new(inputs: &'a [&'b [u8]], outputs: &'a  mut [&'b  mut [u8]]) -> Self`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## virtio_drivers::queue::UsedElem

*Struct*

**Fields:**
- `id: u32`
- `len: u32`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::queue::UsedRing

*Struct*

The used ring is where the device returns buffers once it is done with them:
it is only written to by the device, and read by the driver.

**Generic Parameters:**
- const SIZE

**Fields:**
- `flags: core::sync::atomic::AtomicU16`
- `idx: core::sync::atomic::AtomicU16`
- `ring: [UsedElem; SIZE]`
- `avail_event: core::sync::atomic::AtomicU16` - Only used if `VIRTIO_F_EVENT_IDX` is negotiated.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::queue::VirtQueue

*Struct*

The mechanism for bulk data transport on virtio devices.

Each device can have zero or more virtqueues.

* `SIZE`: The size of the queue. This is both the number of descriptors, and the number of slots
  in the available and used rings. It must be a power of 2 and fit in a [`u16`].

**Generic Parameters:**
- H
- const SIZE

**Fields:**
- `layout: VirtQueueLayout<H>` - DMA guard
- `desc: core::ptr::NonNull<[Descriptor]>` - Descriptor table
- `avail: core::ptr::NonNull<AvailRing<SIZE>>` - Available ring
- `used: core::ptr::NonNull<UsedRing<SIZE>>` - Used ring
- `queue_idx: u16` - The index of queue
- `num_used: u16` - The number of descriptors currently in use.
- `free_head: u16` - The head desc index of the free list.
- `desc_shadow: [Descriptor; SIZE]` - Our trusted copy of `desc` that the device can't access.
- `avail_idx: u16` - Our trusted copy of `avail.idx`.
- `last_used_idx: u16`
- `event_idx: bool` - Whether the `VIRTIO_F_EVENT_IDX` feature has been negotiated.
- `indirect: bool`
- `indirect_lists: [Option<core::ptr::NonNull<[Descriptor]>>; SIZE]`

**Methods:**

- `fn new<T>(transport: & mut T, idx: u16, indirect: bool, event_idx: bool) -> Result<Self>` - Creates a new VirtQueue.
- `fn add<'a, 'b>(self: & mut Self, inputs: &'a [&'b [u8]], outputs: &'a  mut [&'b  mut [u8]]) -> Result<u16>` - Add buffers to the virtqueue, return a token.
- `fn add_direct<'a, 'b>(self: & mut Self, inputs: &'a [&'b [u8]], outputs: &'a  mut [&'b  mut [u8]]) -> u16`
- `fn add_indirect<'a, 'b>(self: & mut Self, inputs: &'a [&'b [u8]], outputs: &'a  mut [&'b  mut [u8]]) -> u16`
- `fn add_notify_wait_pop<'a, impl Transport>(self: & mut Self, inputs: &'a [&'a [u8]], outputs: &'a  mut [&'a  mut [u8]], transport: & mut impl Trait) -> Result<u32>` - Add the given buffers to the virtqueue, notifies the device, blocks until the device uses
- `fn set_dev_notify(self: & mut Self, enable: bool)` - Advise the device whether used buffer notifications are needed.
- `fn should_notify(self: &Self) -> bool` - Returns whether the driver should notify the device after adding a new buffer to the
- `fn write_desc(self: & mut Self, index: u16)` - Copies the descriptor at the given index from `desc_shadow` to `desc`, so it can be seen by
- `fn can_pop(self: &Self) -> bool` - Returns whether there is a used element that can be popped.
- `fn peek_used(self: &Self) -> Option<u16>` - Returns the descriptor index (a.k.a. token) of the next used element without popping it, or
- `fn available_desc(self: &Self) -> usize` - Returns the number of free descriptors.
- `fn recycle_descriptors<'a>(self: & mut Self, head: u16, inputs: &'a [&'a [u8]], outputs: &'a  mut [&'a  mut [u8]])` - Unshares buffers in the list starting at descriptor index `head` and adds them to the free
- `fn pop_used<'a>(self: & mut Self, token: u16, inputs: &'a [&'a [u8]], outputs: &'a  mut [&'a  mut [u8]]) -> Result<u32>` - If the given token is next on the device used queue, pops it and returns the total buffer

**Traits:** Send, Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::queue::VirtQueueLayout

*Enum*

The inner layout of a VirtQueue.

Ref: 2.6 Split Virtqueues

**Generic Parameters:**
- H

**Variants:**
- `Legacy{ dma: crate::hal::Dma<H>, avail_offset: usize, used_offset: usize }`
- `Modern{ driver_to_device_dma: crate::hal::Dma<H>, device_to_driver_dma: crate::hal::Dma<H>, avail_offset: usize }`

**Methods:**

- `fn allocate_legacy(queue_size: u16) -> Result<Self>` - Allocates a single DMA region containing all parts of the virtqueue, following the layout
- `fn allocate_flexible(queue_size: u16) -> Result<Self>` - Allocates separate DMA regions for the the different parts of the virtqueue, as supported by
- `fn descriptors_paddr(self: &Self) -> PhysAddr` - Returns the physical address of the descriptor area.
- `fn descriptors_vaddr(self: &Self) -> NonNull<u8>` - Returns a pointer to the descriptor table (in the descriptor area).
- `fn driver_area_paddr(self: &Self) -> PhysAddr` - Returns the physical address of the driver area.
- `fn avail_vaddr(self: &Self) -> NonNull<u8>` - Returns a pointer to the available ring (in the driver area).
- `fn device_area_paddr(self: &Self) -> PhysAddr` - Returns the physical address of the device area.
- `fn used_vaddr(self: &Self) -> NonNull<u8>` - Returns a pointer to the used ring (in the driver area).

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: owning



## virtio_drivers::queue::queue_part_sizes

*Function*

Returns the size in bytes of the descriptor table, available ring and used ring for a given
queue size.

Ref: 2.6 Split Virtqueues

```rust
fn queue_part_sizes(queue_size: u16) -> (usize, usize, usize)
```



## virtio_drivers::queue::take_first

*Function*

```rust
fn take_first<'a, T>(slice: & mut &'a [T]) -> Option<&'a T>
```



## virtio_drivers::queue::take_first_mut

*Function*

```rust
fn take_first_mut<'a, T>(slice: & mut &'a  mut [T]) -> Option<&'a  mut T>
```



