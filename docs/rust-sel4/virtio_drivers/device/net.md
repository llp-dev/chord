**virtio_drivers > device > net**

# Module: device::net

## Contents

**Modules**

- [`dev`](#dev)
- [`dev_raw`](#dev_raw)
- [`net_buf`](#net_buf)

**Structs**

- [`Config`](#config)
- [`Features`](#features)
- [`Flags`](#flags)
- [`GsoType`](#gsotype)
- [`InterruptStatus`](#interruptstatus)
- [`Status`](#status)
- [`VirtioNetHdr`](#virtionethdr) - VirtIO 5.1.6 Device Operation:
- [`VirtioNetHdrLegacy`](#virtionethdrlegacy) - VirtIO 5.1.6 Device Operation:

**Constants**

- [`MAX_BUFFER_LEN`](#max_buffer_len)
- [`MIN_BUFFER_LEN`](#min_buffer_len)
- [`QUEUE_RECEIVE`](#queue_receive)
- [`QUEUE_TRANSMIT`](#queue_transmit)
- [`SUPPORTED_FEATURES`](#supported_features)

**Type Aliases**

- [`EthernetAddress`](#ethernetaddress)

---

## virtio_drivers::device::net::Config

*Struct*

**Fields:**
- `mac: crate::config::ReadOnly<[u8; 6]>`
- `status: crate::config::ReadOnly<Status>`
- `max_virtqueue_pairs: crate::config::ReadOnly<u16>`
- `mtu: crate::config::ReadOnly<u16>`



## virtio_drivers::device::net::EthernetAddress

*Type Alias*: `[u8; 6]`



## virtio_drivers::device::net::Features

*Struct*

**Tuple Struct**: `(<Features as $crate::__private::PublicFlags>::Internal)`

**Methods:**

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
- `fn iter(self: &Self) -> $crate::iter::Iter<Features>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Features>` - Yield a set of contained named flags values.

**Traits:** PublicFlags, Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Features`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u64`
  - `fn from_bits_retain(bits: u64) -> Features`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> Features`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: Features) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in two flags values.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **PartialEq**
  - `fn eq(self: &Self, other: &Features) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.



## virtio_drivers::device::net::Flags

*Struct*

**Tuple Struct**: `(u8)`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u8` - Get the underlying bits value.
- `fn from_bits(bits: u8) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u8) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u8) -> Self` - Convert from a bits value exactly.
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
- `fn iter(self: &Self) -> $crate::iter::Iter<Flags>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Flags>` - Yield a set of contained named flags values.

**Traits:** FromBytes, KnownLayout, Copy, FromZeros, IntoBytes, TryFromBytes, Immutable, Eq

**Trait Implementations:**

- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: Flags) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **PartialEq**
  - `fn eq(self: &Self, other: &Flags) -> bool`
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Default**
  - `fn default() -> Flags`
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
- **Clone**
  - `fn clone(self: &Self) -> Flags`
- **Flags**
  - `fn bits(self: &Self) -> u8`
  - `fn from_bits_retain(bits: u8) -> Flags`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.



## virtio_drivers::device::net::GsoType

*Struct*

**Tuple Struct**: `(u8)`

**Methods:**


**Traits:** Eq, FromBytes, KnownLayout, FromZeros, IntoBytes, Copy, TryFromBytes, Immutable

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> GsoType`
- **PartialEq**
  - `fn eq(self: &Self, other: &GsoType) -> bool`
- **Default**
  - `fn default() -> GsoType`



## virtio_drivers::device::net::InterruptStatus

*Struct*

**Tuple Struct**: `(<InterruptStatus as $crate::__private::PublicFlags>::Internal)`

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

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: InterruptStatus) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in two flags values.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **PartialEq**
  - `fn eq(self: &Self, other: &InterruptStatus) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Default**
  - `fn default() -> InterruptStatus`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u32`
  - `fn from_bits_retain(bits: u32) -> InterruptStatus`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> InterruptStatus`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.



## virtio_drivers::device::net::MAX_BUFFER_LEN

*Constant*: `usize`



## virtio_drivers::device::net::MIN_BUFFER_LEN

*Constant*: `usize`



## virtio_drivers::device::net::QUEUE_RECEIVE

*Constant*: `u16`



## virtio_drivers::device::net::QUEUE_TRANSMIT

*Constant*: `u16`



## virtio_drivers::device::net::SUPPORTED_FEATURES

*Constant*: `Features`



## virtio_drivers::device::net::Status

*Struct*

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
- `fn iter(self: &Self) -> $crate::iter::Iter<Status>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Status>` - Yield a set of contained named flags values.

**Traits:** Eq, FromBytes, Copy, FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout

**Trait Implementations:**

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
  - `fn from_bits_retain(bits: u16) -> Status`
- **Default**
  - `fn default() -> Status`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Clone**
  - `fn clone(self: &Self) -> Status`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: Status) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **PartialEq**
  - `fn eq(self: &Self, other: &Status) -> bool`
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::net::VirtioNetHdr

*Struct*

VirtIO 5.1.6 Device Operation:

Packets are transmitted by placing them in the transmitq1. . .transmitqN,
and buffers for incoming packets are placed in the receiveq1. . .receiveqN.
In each case, the packet itself is preceded by a header.

**Fields:**
- `flags: Flags`
- `gso_type: GsoType`
- `hdr_len: u16`
- `gso_size: u16`
- `csum_start: u16`
- `csum_offset: u16`
- `num_buffers: u16`

**Traits:** Copy, FromBytes, FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Default**
  - `fn default() -> VirtioNetHdr`
- **Clone**
  - `fn clone(self: &Self) -> VirtioNetHdr`
- **From**
  - `fn from(legacy: &VirtioNetHdrLegacy) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::net::VirtioNetHdrLegacy

*Struct*

VirtIO 5.1.6 Device Operation:

Packets are transmitted by placing them in the transmitq1. . .transmitqN,
and buffers for incoming packets are placed in the receiveq1. . .receiveqN.
In each case, the packet itself is preceded by a header.

**Fields:**
- `flags: Flags`
- `gso_type: GsoType`
- `hdr_len: u16`
- `gso_size: u16`
- `csum_start: u16`
- `csum_offset: u16`

**Traits:** TryFromBytes, Immutable, KnownLayout, FromBytes, FromZeros, IntoBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> VirtioNetHdrLegacy`



## Module: dev



## Module: dev_raw



## Module: net_buf



