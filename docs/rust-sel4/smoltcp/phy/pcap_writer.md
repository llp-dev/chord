**smoltcp > phy > pcap_writer**

# Module: phy::pcap_writer

## Contents

**Structs**

- [`PcapWriter`](#pcapwriter) - A packet capture writer device.

**Enums**

- [`PcapLinkType`](#pcaplinktype) - Captured packet header type.
- [`PcapMode`](#pcapmode) - Packet capture mode.

**Traits**

- [`PcapSink`](#pcapsink) - A packet capture sink.

---

## smoltcp::phy::pcap_writer::PcapLinkType

*Enum*

Captured packet header type.

**Variants:**
- `Ethernet` - Ethernet frames
- `Ip` - IPv4 or IPv6 packets (depending on the version field)
- `Ieee802154WithoutFcs` - IEEE 802.15.4 packets without FCS.
- `Unknown(u32)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: u32) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &PcapLinkType) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> PcapLinkType`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PcapLinkType) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PcapLinkType) -> $crate::option::Option<$crate::cmp::Ordering>`



## smoltcp::phy::pcap_writer::PcapMode

*Enum*

Packet capture mode.

**Variants:**
- `Both` - Capture both received and transmitted packets.
- `RxOnly` - Capture only received packets.
- `TxOnly` - Capture only transmitted packets.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PcapMode) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> PcapMode`



## smoltcp::phy::pcap_writer::PcapSink

*Trait*

A packet capture sink.

**Methods:**

- `write`: Write data into the sink.
- `flush`: Flush data written into the sync.
- `write_u16`: Write an `u16` into the sink, in native byte order.
- `write_u32`: Write an `u32` into the sink, in native byte order.
- `global_header`: Write the libpcap global header into the sink.
- `packet_header`: Write the libpcap packet header into the sink.
- `packet`: Write the libpcap packet header followed by packet data into the sink.



## smoltcp::phy::pcap_writer::PcapWriter

*Struct*

A packet capture writer device.

Every packet transmitted or received through this device is timestamped
and written (in the [libpcap] format) using the provided [sink].
Note that writes are fine-grained, and buffering is recommended.

[libpcap]: https://wiki.wireshark.org/Development/LibpcapFileFormat
[sink]: trait.PcapSink.html

**Generic Parameters:**
- D
- S

**Methods:**

- `fn new(lower: D, sink: S, mode: PcapMode) -> PcapWriter<D, S>` - Creates a packet capture writer.
- `fn get_ref(self: &Self) -> &D` - Get a reference to the underlying device.
- `fn get_mut(self: & mut Self) -> & mut D` - Get a mutable reference to the underlying device.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Device**
  - `fn capabilities(self: &Self) -> DeviceCapabilities`
  - `fn receive(self: & mut Self, timestamp: Instant) -> Option<(<Self as >::RxToken, <Self as >::TxToken)>`
  - `fn transmit(self: & mut Self, timestamp: Instant) -> Option<<Self as >::TxToken>`



