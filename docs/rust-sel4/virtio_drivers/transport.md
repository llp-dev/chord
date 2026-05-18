**virtio_drivers > transport**

# Module: transport

## Contents

**Modules**

- [`mmio`](#mmio) - MMIO transport for VirtIO.
- [`pci`](#pci) - PCI transport for VirtIO.
- [`some`](#some)
- [`x86_64`](#x86_64) - x86-64 specific transports.

**Structs**

- [`DeviceStatus`](#devicestatus) - The device status field. Writing 0 into this field resets the device.
- [`InterruptStatus`](#interruptstatus) - The set of interrupts which were pending

**Enums**

- [`DeviceType`](#devicetype) - Types of virtio devices.
- [`DeviceTypeError`](#devicetypeerror) - Errors converting a number to a virtio device type.

**Traits**

- [`Transport`](#transport) - A VirtIO transport layer.

---

## virtio_drivers::transport::DeviceStatus

*Struct*

The device status field. Writing 0 into this field resets the device.

**Tuple Struct**: `(u32)`

**Methods:**

- `fn iter(self: &Self) -> $crate::iter::Iter<DeviceStatus>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<DeviceStatus>` - Yield a set of contained named flags values.
- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u32` - Get the underlying bits value.
- `fn from_bits(bits: u32) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u32) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u32) -> Self` - Convert from a bits value exactly.
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

**Traits:** Eq, FromBytes, FromZeros, IntoBytes, Copy, TryFromBytes, Immutable

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DeviceStatus`
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
  - `fn bits(self: &Self) -> u32`
  - `fn from_bits_retain(bits: u32) -> DeviceStatus`
- **Default**
  - `fn default() -> DeviceStatus`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **PartialEq**
  - `fn eq(self: &Self, other: &DeviceStatus) -> bool`
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: DeviceStatus) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.



## virtio_drivers::transport::DeviceType

*Enum*

Types of virtio devices.

**Variants:**
- `Network`
- `Block`
- `Console`
- `EntropySource`
- `MemoryBallooning`
- `IoMemory`
- `Rpmsg`
- `ScsiHost`
- `_9P`
- `Mac80211`
- `RprocSerial`
- `VirtioCAIF`
- `MemoryBalloon`
- `GPU`
- `Timer`
- `Input`
- `Socket`
- `Crypto`
- `SignalDistributionModule`
- `Pstore`
- `IOMMU`
- `Memory`
- `Sound`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DeviceType) -> $crate::option::Option<$crate::cmp::Ordering>`
- **TryFrom**
  - `fn try_from(virtio_device_id: u32) -> core::result::Result<Self, <Self as >::Error>`
- **TryFrom**
  - `fn try_from(virtio_device_id: u16) -> core::result::Result<Self, <Self as >::Error>`
- **TryFrom**
  - `fn try_from(virtio_device_id: u8) -> core::result::Result<Self, <Self as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &DeviceType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DeviceType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &DeviceType) -> $crate::cmp::Ordering`



## virtio_drivers::transport::DeviceTypeError

*Enum*

Errors converting a number to a virtio device type.

**Variants:**
- `InvalidDeviceType(u32)` - Invalid or unknown virtio device type.

**Traits:** Copy, Error, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DeviceTypeError`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DeviceTypeError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::InterruptStatus

*Struct*

The set of interrupts which were pending

Ref: 4.1.4.5 ISR status capability

**Tuple Struct**: `(u32)`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u32` - Get the underlying bits value.
- `fn from_bits(bits: u32) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u32) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u32) -> Self` - Convert from a bits value exactly.
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
- `fn iter(self: &Self) -> $crate::iter::Iter<InterruptStatus>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<InterruptStatus>` - Yield a set of contained named flags values.

**Traits:** TryFromBytes, Eq, FromBytes, FromZeros, Copy

**Trait Implementations:**

- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> InterruptStatus`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: InterruptStatus) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **PartialEq**
  - `fn eq(self: &Self, other: &InterruptStatus) -> bool`
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in two flags values.
- **Default**
  - `fn default() -> InterruptStatus`
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Flags**
  - `fn bits(self: &Self) -> u32`
  - `fn from_bits_retain(bits: u32) -> InterruptStatus`



## virtio_drivers::transport::Transport

*Trait*

A VirtIO transport layer.

**Methods:**

- `device_type`: Gets the device type.
- `read_device_features`: Reads device features.
- `write_driver_features`: Writes device features.
- `max_queue_size`: Gets the max size of the given queue.
- `notify`: Notifies the given queue on the device.
- `get_status`: Gets the device status.
- `set_status`: Sets the device status.
- `set_guest_page_size`: Sets the guest page size.
- `requires_legacy_layout`: Returns whether the transport requires queues to use the legacy layout.
- `queue_set`: Sets up the given queue.
- `queue_unset`: Disables and resets the given queue.
- `queue_used`: Returns whether the queue is in use, i.e. has a nonzero PFN or is marked as ready.
- `ack_interrupt`: Acknowledges an interrupt.
- `begin_init`: Begins initializing the device.
- `finish_init`: Finishes initializing the device.
- `read_config_generation`: Reads the configuration space generation.
- `read_config_space`: Reads a value from the device config space.
- `write_config_space`: Writes a value to the device config space.
- `read_consistent`: Safely reads multiple fields from config space by ensuring that the config generation is the



## Module: mmio

MMIO transport for VirtIO.



## Module: pci

PCI transport for VirtIO.



## Module: some



## Module: x86_64

x86-64 specific transports.



