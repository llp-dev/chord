**smoltcp > wire > ip**

# Module: wire::ip

## Contents

**Structs**

- [`Endpoint`](#endpoint) - An internet endpoint address.
- [`ListenEndpoint`](#listenendpoint) - An internet endpoint address for listening.
- [`Packet`](#packet) - A read/write wrapper around a generic Internet Protocol packet buffer.

**Enums**

- [`Address`](#address) - An internetworking address.
- [`Cidr`](#cidr) - A specification of a CIDR block, containing an address and a variable-length
- [`Protocol`](#protocol) - IP datagram encapsulated protocol.
- [`Repr`](#repr) - An IP packet representation.
- [`Version`](#version) - Internet protocol version.

---

## smoltcp::wire::ip::Address

*Enum*

An internetworking address.

**Variants:**
- `Ipv4(crate::wire::Ipv4Address)` - An IPv4 address.

**Methods:**

- `fn v4(a0: u8, a1: u8, a2: u8, a3: u8) -> Address` - Create an address wrapping an IPv4 address with the given octets.
- `fn version(self: &Self) -> Version` - Return the protocol version.
- `fn is_unicast(self: &Self) -> bool` - Query whether the address is a valid unicast address.
- `fn is_multicast(self: &Self) -> bool` - Query whether the address is a valid multicast address.
- `fn is_broadcast(self: &Self) -> bool` - Query whether the address is the broadcast address.
- `fn is_unspecified(self: &Self) -> bool` - Query whether the address falls into the "unspecified" range.
- `fn prefix_len(self: &Self) -> Option<u8>` - If `self` is a CIDR-compatible subnet mask, return `Some(prefix_len)`,

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Address) -> $crate::option::Option<$crate::cmp::Ordering>`
- **FromStr**
  - `fn from_str(s: &str) -> result::Result<IpAddress, ()>` - Parse a string representation of an IP address.
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Address) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Address`
- **From**
  - `fn from(ipv4: Ipv4Address) -> Address`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Address) -> bool`



## smoltcp::wire::ip::Cidr

*Enum*

A specification of a CIDR block, containing an address and a variable-length
subnet masking prefix length.

**Variants:**
- `Ipv4(crate::wire::Ipv4Cidr)`

**Methods:**

- `fn new(addr: Address, prefix_len: u8) -> Cidr` - Create a CIDR block from the given address and prefix length.
- `fn address(self: &Self) -> Address` - Return the IP address of this CIDR block.
- `fn prefix_len(self: &Self) -> u8` - Return the prefix length of this CIDR block.
- `fn contains_addr(self: &Self, addr: &Address) -> bool` - Query whether the subnetwork described by this CIDR block contains
- `fn contains_subnet(self: &Self, subnet: &Cidr) -> bool` - Query whether the subnetwork described by this CIDR block contains

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Cidr`
- **From**
  - `fn from(addr: Ipv4Cidr) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Cidr) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Cidr) -> $crate::option::Option<$crate::cmp::Ordering>`
- **FromStr**
  - `fn from_str(s: &str) -> result::Result<IpCidr, ()>` - Parse a string representation of an IP CIDR.
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Cidr) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::wire::ip::Endpoint

*Struct*

An internet endpoint address.

`Endpoint` always fully specifies both the address and the port.

See also ['ListenEndpoint'], which allows not specifying the address
in order to listen on a given port on any address.

**Fields:**
- `addr: Address`
- `port: u16`

**Methods:**

- `fn new(addr: Address, port: u16) -> Endpoint` - Create an endpoint address from given address and port.

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from((addr, port): (T, u16)) -> Endpoint`
- **Ord**
  - `fn cmp(self: &Self, other: &Endpoint) -> $crate::cmp::Ordering`
- **From**
  - `fn from(x: ::core::net::SocketAddrV4) -> Endpoint`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Endpoint`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Endpoint) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Endpoint) -> $crate::option::Option<$crate::cmp::Ordering>`
- **FromStr**
  - `fn from_str(s: &str) -> result::Result<IpEndpoint, ()>`



## smoltcp::wire::ip::ListenEndpoint

*Struct*

An internet endpoint address for listening.

In contrast with [`Endpoint`], `ListenEndpoint` allows not specifying the address,
in order to listen on a given port at all our addresses.

An endpoint can be constructed from a port, in which case the address is unspecified.

**Fields:**
- `addr: Option<Address>`
- `port: u16`

**Methods:**

- `fn is_specified(self: &Self) -> bool` - Query whether the endpoint has a specified address and port.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &ListenEndpoint) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ListenEndpoint`
- **From**
  - `fn from(endpoint: Endpoint) -> ListenEndpoint`
- **Default**
  - `fn default() -> ListenEndpoint`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(port: u16) -> ListenEndpoint`
- **PartialEq**
  - `fn eq(self: &Self, other: &ListenEndpoint) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ListenEndpoint) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(x: ::core::net::SocketAddrV4) -> ListenEndpoint`
- **From**
  - `fn from((addr, port): (T, u16)) -> ListenEndpoint`



## smoltcp::wire::ip::Packet

*Struct*

A read/write wrapper around a generic Internet Protocol packet buffer.

**Generic Parameters:**
- T



## smoltcp::wire::ip::Protocol

*Enum*

IP datagram encapsulated protocol.

**Variants:**
- `HopByHop`
- `Icmp`
- `Igmp`
- `Tcp`
- `Udp`
- `Ipv6Route`
- `Ipv6Frag`
- `IpSecEsp`
- `IpSecAh`
- `Icmpv6`
- `Ipv6NoNxt`
- `Ipv6Opts`
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Protocol) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Protocol`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Protocol) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Protocol) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`



## smoltcp::wire::ip::Repr

*Enum*

An IP packet representation.

This enum abstracts the various versions of IP packets. It either contains an IPv4
or IPv6 concrete high-level representation.

**Variants:**
- `Ipv4(crate::wire::Ipv4Repr)`

**Methods:**

- `fn new(src_addr: Address, dst_addr: Address, next_header: Protocol, payload_len: usize, hop_limit: u8) -> Self` - Create a new IpRepr, choosing the right IP version for the src/dst addrs.
- `fn parse<T>(packet: &Packet<&T>, checksum_caps: &ChecksumCapabilities) -> Result<Repr>` - Parse an Internet Protocol packet and return an [IpRepr] containing either an Internet
- `fn version(self: &Self) -> Version` - Return the protocol version.
- `fn src_addr(self: &Self) -> Address` - Return the source address.
- `fn dst_addr(self: &Self) -> Address` - Return the destination address.
- `fn next_header(self: &Self) -> Protocol` - Return the next header (protocol).
- `fn payload_len(self: &Self) -> usize` - Return the payload length.
- `fn set_payload_len(self: & mut Self, length: usize)` - Set the payload length.
- `fn hop_limit(self: &Self) -> u8` - Return the TTL value.
- `fn header_len(self: &Self) -> usize` - Return the length of a header that will be emitted from this high-level representation.
- `fn emit<T>(self: &Self, buffer: T, _checksum_caps: &ChecksumCapabilities)` - Emit this high-level representation into a buffer.
- `fn buffer_len(self: &Self) -> usize` - Return the total length of a packet that will be emitted from this

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Repr`
- **PartialEq**
  - `fn eq(self: &Self, other: &Repr) -> bool`
- **From**
  - `fn from(repr: Ipv4Repr) -> Repr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::wire::ip::Version

*Enum*

Internet protocol version.

**Variants:**
- `Ipv4`

**Methods:**

- `fn of_packet(data: &[u8]) -> Result<Version>` - Return the version of an IP packet stored in the provided buffer.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Version`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Version) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Version) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Version) -> $crate::cmp::Ordering`



