**smoltcp > phy**

# Module: phy

## Contents

**Structs**

- [`ChecksumCapabilities`](#checksumcapabilities) - A description of checksum behavior for every supported protocol.
- [`DeviceCapabilities`](#devicecapabilities) - A description of device capabilities.
- [`PacketMeta`](#packetmeta) - Metadata associated to a packet.

**Enums**

- [`Checksum`](#checksum) - A description of checksum behavior for a particular protocol.
- [`Medium`](#medium) - Type of medium of a device.

**Traits**

- [`Device`](#device) - An interface for sending and receiving raw network frames.
- [`RxToken`](#rxtoken) - A token to receive a single network packet.
- [`TxToken`](#txtoken) - A token to transmit a single network packet.

---

## smoltcp::phy::Checksum

*Enum*

A description of checksum behavior for a particular protocol.

**Variants:**
- `Both` - Verify checksum when receiving and compute checksum when sending.
- `Rx` - Verify checksum when receiving.
- `Tx` - Compute checksum before sending.
- `None` - Ignore checksum completely.

**Methods:**

- `fn rx(self: &Self) -> bool` - Returns whether checksum should be verified when receiving.
- `fn tx(self: &Self) -> bool` - Returns whether checksum should be verified when sending.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Checksum`
- **Default**
  - `fn default() -> Checksum`



## smoltcp::phy::ChecksumCapabilities

*Struct*

A description of checksum behavior for every supported protocol.

**Fields:**
- `ipv4: Checksum`
- `udp: Checksum`
- `tcp: Checksum`
- `icmpv4: Checksum`

**Methods:**

- `fn ignored() -> Self` - Checksum behavior that results in not computing or verifying checksums

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ChecksumCapabilities`
- **Default**
  - `fn default() -> ChecksumCapabilities`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::phy::Device

*Trait*

An interface for sending and receiving raw network frames.

The interface is based on _tokens_, which are types that allow to receive/transmit a
single packet. The `receive` and `transmit` functions only construct such tokens, the
real sending/receiving operation are performed when the tokens are consumed.

**Methods:**

- `RxToken`
- `TxToken`
- `receive`: Construct a token pair consisting of one receive token and one transmit token.
- `transmit`: Construct a transmit token.
- `capabilities`: Get a description of device capabilities.



## smoltcp::phy::DeviceCapabilities

*Struct*

A description of device capabilities.

Higher-level protocols may achieve higher throughput or lower latency if they consider
the bandwidth or packet size limitations.

**Fields:**
- `medium: Medium` - Medium of the device.
- `max_transmission_unit: usize` - Maximum transmission unit.
- `max_burst_size: Option<usize>` - Maximum burst size, in terms of MTU.
- `checksum: ChecksumCapabilities` - Checksum behavior.

**Methods:**

- `fn ip_mtu(self: &Self) -> usize`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DeviceCapabilities`
- **Default**
  - `fn default() -> DeviceCapabilities`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::phy::Medium

*Enum*

Type of medium of a device.

**Variants:**
- `Ethernet` - Ethernet medium. Devices of this type send and receive Ethernet frames,

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Medium) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Medium`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Medium`



## smoltcp::phy::PacketMeta

*Struct*

Metadata associated to a packet.

The packet metadata is a set of attributes associated to network packets
as they travel up or down the stack. The metadata is get/set by the
[`Device`] implementations or by the user when sending/receiving packets from a
socket.

Metadata fields are enabled via Cargo features. If no field is enabled, this
struct becomes zero-sized, which allows the compiler to optimize it out as if
the packet metadata mechanism didn't exist at all.

Currently only UDP sockets allow setting/retrieving packet metadata. The metadata
for packets emitted with other sockets will be all default values.

This struct is marked as `#[non_exhaustive]`. This means it is not possible to
create it directly by specifying all fields. You have to instead create it with
default values and then set the fields you want. This makes adding metadata
fields a non-breaking change.

```rust
let mut meta = smoltcp::phy::PacketMeta::default();
#[cfg(feature = "packetmeta-id")]
{
    meta.id = 15;
}
```

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PacketMeta`
- **Default**
  - `fn default() -> PacketMeta`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PacketMeta) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## smoltcp::phy::RxToken

*Trait*

A token to receive a single network packet.

**Methods:**

- `consume`: Consumes the token to receive a single network packet.
- `meta`: The Packet ID associated with the frame received by this [`RxToken`]



## smoltcp::phy::TxToken

*Trait*

A token to transmit a single network packet.

**Methods:**

- `consume`: Consumes the token to send a single network packet.
- `set_meta`: The Packet ID to be associated with the frame to be transmitted by this [`TxToken`].



