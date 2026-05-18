**virtio_drivers > device > socket > protocol**

# Module: device::socket::protocol

## Contents

**Structs**

- [`Feature`](#feature)
- [`StreamShutdown`](#streamshutdown) - Flags sent with a shutdown request to hint that the peer won't send or receive more data.
- [`VirtioVsockConfig`](#virtiovsockconfig) - VirtioVsockConfig is the vsock device configuration space.
- [`VirtioVsockEvent`](#virtiovsockevent) - An event sent to the event queue
- [`VirtioVsockHdr`](#virtiovsockhdr) - The message header for data packets sent on the tx/rx queues
- [`VsockAddr`](#vsockaddr) - Socket address.

**Enums**

- [`SocketType`](#sockettype) - Currently only stream sockets are supported. type is 1 for stream socket types.
- [`VirtioVsockOp`](#virtiovsockop)

**Constants**

- [`VMADDR_CID_HOST`](#vmaddr_cid_host) - Well-known CID for the host.

---

## virtio_drivers::device::socket::protocol::Feature

*Struct*

**Tuple Struct**: `(<Feature as $crate::__private::PublicFlags>::Internal)`

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
- `fn iter(self: &Self) -> $crate::iter::Iter<Feature>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Feature>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

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
  - `fn eq(self: &Self, other: &Feature) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Default**
  - `fn default() -> Feature`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u64`
  - `fn from_bits_retain(bits: u64) -> Feature`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> Feature`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: Feature) -> Self` - The bitwise or (`|`) of the bits in two flags values.



## virtio_drivers::device::socket::protocol::SocketType

*Enum*

Currently only stream sockets are supported. type is 1 for stream socket types.

**Variants:**
- `Stream` - Stream sockets provide in-order, guaranteed, connection-oriented delivery without message boundaries.
- `SeqPacket` - seqpacket socket type introduced in virtio-v1.2.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SocketType`



## virtio_drivers::device::socket::protocol::StreamShutdown

*Struct*

Flags sent with a shutdown request to hint that the peer won't send or receive more data.

**Tuple Struct**: `(<StreamShutdown as $crate::__private::PublicFlags>::Internal)`

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
- `fn iter(self: &Self) -> $crate::iter::Iter<StreamShutdown>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<StreamShutdown>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> StreamShutdown`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: StreamShutdown) -> Self` - The bitwise or (`|`) of the bits in two flags values.
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
  - `fn eq(self: &Self, other: &StreamShutdown) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Default**
  - `fn default() -> StreamShutdown`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u32`
  - `fn from_bits_retain(bits: u32) -> StreamShutdown`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.



## virtio_drivers::device::socket::protocol::VMADDR_CID_HOST

*Constant*: `u64`

Well-known CID for the host.



## virtio_drivers::device::socket::protocol::VirtioVsockConfig

*Struct*

VirtioVsockConfig is the vsock device configuration space.

**Fields:**
- `guest_cid_low: crate::config::ReadOnly<u32>` - The guest_cid field contains the guest’s context ID, which uniquely identifies
- `guest_cid_high: crate::config::ReadOnly<u32>`

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, FromBytes



## virtio_drivers::device::socket::protocol::VirtioVsockEvent

*Struct*

An event sent to the event queue

**Fields:**
- `id: zerocopy::byteorder::U32<zerocopy::byteorder::LittleEndian>`

**Traits:** Copy, FromZeros, TryFromBytes, Immutable, IntoBytes, FromBytes, KnownLayout

**Trait Implementations:**

- **Default**
  - `fn default() -> VirtioVsockEvent`
- **Clone**
  - `fn clone(self: &Self) -> VirtioVsockEvent`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::socket::protocol::VirtioVsockHdr

*Struct*

The message header for data packets sent on the tx/rx queues

**Fields:**
- `src_cid: zerocopy::byteorder::U64<zerocopy::byteorder::LittleEndian>`
- `dst_cid: zerocopy::byteorder::U64<zerocopy::byteorder::LittleEndian>`
- `src_port: zerocopy::byteorder::U32<zerocopy::byteorder::LittleEndian>`
- `dst_port: zerocopy::byteorder::U32<zerocopy::byteorder::LittleEndian>`
- `len: zerocopy::byteorder::U32<zerocopy::byteorder::LittleEndian>`
- `socket_type: zerocopy::byteorder::U16<zerocopy::byteorder::LittleEndian>`
- `op: zerocopy::byteorder::U16<zerocopy::byteorder::LittleEndian>`
- `flags: zerocopy::byteorder::U32<zerocopy::byteorder::LittleEndian>`
- `buf_alloc: zerocopy::byteorder::U32<zerocopy::byteorder::LittleEndian>` - Total receive buffer space for this socket. This includes both free and in-use buffers.
- `fwd_cnt: zerocopy::byteorder::U32<zerocopy::byteorder::LittleEndian>` - Free-running bytes received counter.

**Methods:**

- `fn len(self: &Self) -> u32` - Returns the length of the data.
- `fn op(self: &Self) -> result::Result<VirtioVsockOp, SocketError>`
- `fn source(self: &Self) -> VsockAddr`
- `fn destination(self: &Self) -> VsockAddr`
- `fn check_data_is_empty(self: &Self) -> result::Result<(), SocketError>`

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout, Copy, Eq, FromBytes

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> VirtioVsockHdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VirtioVsockHdr) -> bool`



## virtio_drivers::device::socket::protocol::VirtioVsockOp

*Enum*

**Variants:**
- `Invalid`
- `Request`
- `Response`
- `Rst`
- `Shutdown`
- `Rw`
- `CreditUpdate`
- `CreditRequest`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &VirtioVsockOp) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> VirtioVsockOp`
- **TryFrom**
  - `fn try_from(v: U16<LittleEndian>) -> Result<Self, <Self as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## virtio_drivers::device::socket::protocol::VsockAddr

*Struct*

Socket address.

**Fields:**
- `cid: u64` - Context Identifier.
- `port: u32` - Port number.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> VsockAddr`
- **PartialEq**
  - `fn eq(self: &Self, other: &VsockAddr) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> VsockAddr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



