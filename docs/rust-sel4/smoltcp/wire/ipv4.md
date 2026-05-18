**smoltcp > wire > ipv4**

# Module: wire::ipv4

## Contents

**Structs**

- [`Cidr`](#cidr) - A specification of an IPv4 CIDR block, containing an address and a variable-length
- [`Key`](#key)
- [`Packet`](#packet) - A read/write wrapper around an Internet Protocol version 4 packet buffer.
- [`Repr`](#repr) - A high-level representation of an Internet Protocol version 4 packet header.

**Constants**

- [`HEADER_LEN`](#header_len)
- [`MIN_MTU`](#min_mtu) - Minimum MTU required of all links supporting IPv4. See [RFC 791 § 3.1].
- [`MULTICAST_ALL_ROUTERS`](#multicast_all_routers) - All multicast-capable routers
- [`MULTICAST_ALL_SYSTEMS`](#multicast_all_systems) - All multicast-capable nodes

---

## smoltcp::wire::ipv4::Cidr

*Struct*

A specification of an IPv4 CIDR block, containing an address and a variable-length
subnet masking prefix length.

**Methods:**

- `fn new(address: Address, prefix_len: u8) -> Cidr` - Create an IPv4 CIDR block from the given address and prefix length.
- `fn from_netmask(addr: Address, netmask: Address) -> Result<Cidr>` - Create an IPv4 CIDR block from the given address and network mask.
- `fn address(self: &Self) -> Address` - Return the address of this IPv4 CIDR block.
- `fn prefix_len(self: &Self) -> u8` - Return the prefix length of this IPv4 CIDR block.
- `fn netmask(self: &Self) -> Address` - Return the network mask of this IPv4 CIDR.
- `fn broadcast(self: &Self) -> Option<Address>` - Return the broadcast address of this IPv4 CIDR.
- `fn network(self: &Self) -> Cidr` - Return the network block of this IPv4 CIDR.
- `fn contains_addr(self: &Self, addr: &Address) -> bool` - Query whether the subnetwork described by this IPv4 CIDR block contains
- `fn contains_subnet(self: &Self, subnet: &Cidr) -> bool` - Query whether the subnetwork described by this IPv4 CIDR block contains

**Traits:** Copy, Eq

**Trait Implementations:**

- **FromStr**
  - `fn from_str(s: &str) -> result::Result<Ipv4Cidr, ()>` - Parse a string representation of an IPv4 CIDR.
- **Ord**
  - `fn cmp(self: &Self, other: &Cidr) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Cidr`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Cidr) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Cidr) -> $crate::option::Option<$crate::cmp::Ordering>`



## smoltcp::wire::ipv4::HEADER_LEN

*Constant*: `usize`



## smoltcp::wire::ipv4::Key

*Struct*

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Key) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Key) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Key) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Key`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::wire::ipv4::MIN_MTU

*Constant*: `usize`

Minimum MTU required of all links supporting IPv4. See [RFC 791 § 3.1].

[RFC 791 § 3.1]: https://tools.ietf.org/html/rfc791#section-3.1



## smoltcp::wire::ipv4::MULTICAST_ALL_ROUTERS

*Constant*: `Address`

All multicast-capable routers



## smoltcp::wire::ipv4::MULTICAST_ALL_SYSTEMS

*Constant*: `Address`

All multicast-capable nodes



## smoltcp::wire::ipv4::Packet

*Struct*

A read/write wrapper around an Internet Protocol version 4 packet buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn payload(self: &Self) -> &'a [u8]` - Return a pointer to the payload.
- `fn set_version(self: & mut Self, value: u8)` - Set the version field.
- `fn set_header_len(self: & mut Self, value: u8)` - Set the header length, in octets.
- `fn set_dscp(self: & mut Self, value: u8)` - Set the Differential Services Code Point field.
- `fn set_ecn(self: & mut Self, value: u8)` - Set the Explicit Congestion Notification field.
- `fn set_total_len(self: & mut Self, value: u16)` - Set the total length field.
- `fn set_ident(self: & mut Self, value: u16)` - Set the fragment identification field.
- `fn clear_flags(self: & mut Self)` - Clear the entire flags field.
- `fn set_dont_frag(self: & mut Self, value: bool)` - Set the "don't fragment" flag.
- `fn set_more_frags(self: & mut Self, value: bool)` - Set the "more fragments" flag.
- `fn set_frag_offset(self: & mut Self, value: u16)` - Set the fragment offset, in octets.
- `fn set_hop_limit(self: & mut Self, value: u8)` - Set the time to live field.
- `fn set_next_header(self: & mut Self, value: Protocol)` - Set the next header (protocol) field.
- `fn set_checksum(self: & mut Self, value: u16)` - Set the header checksum field.
- `fn set_src_addr(self: & mut Self, value: Address)` - Set the source address field.
- `fn set_dst_addr(self: & mut Self, value: Address)` - Set the destination address field.
- `fn fill_checksum(self: & mut Self)` - Compute and fill in the header checksum.
- `fn payload_mut(self: & mut Self) -> & mut [u8]` - Return a mutable pointer to the payload.
- `fn new_unchecked(buffer: T) -> Packet<T>` - Imbue a raw octet buffer with IPv4 packet structure.
- `fn new_checked(buffer: T) -> Result<Packet<T>>` - Shorthand for a combination of [new_unchecked] and [check_len].
- `fn check_len(self: &Self) -> Result<()>` - Ensure that no accessor method will panic if called.
- `fn into_inner(self: Self) -> T` - Consume the packet, returning the underlying buffer.
- `fn version(self: &Self) -> u8` - Return the version field.
- `fn header_len(self: &Self) -> u8` - Return the header length, in octets.
- `fn dscp(self: &Self) -> u8` - Return the Differential Services Code Point field.
- `fn ecn(self: &Self) -> u8` - Return the Explicit Congestion Notification field.
- `fn total_len(self: &Self) -> u16` - Return the total length field.
- `fn ident(self: &Self) -> u16` - Return the fragment identification field.
- `fn dont_frag(self: &Self) -> bool` - Return the "don't fragment" flag.
- `fn more_frags(self: &Self) -> bool` - Return the "more fragments" flag.
- `fn frag_offset(self: &Self) -> u16` - Return the fragment offset, in octets.
- `fn hop_limit(self: &Self) -> u8` - Return the time to live field.
- `fn next_header(self: &Self) -> Protocol` - Return the next_header (protocol) field.
- `fn checksum(self: &Self) -> u16` - Return the header checksum field.
- `fn src_addr(self: &Self) -> Address` - Return the source address field.
- `fn dst_addr(self: &Self) -> Address` - Return the destination address field.
- `fn verify_checksum(self: &Self) -> bool` - Validate the header checksum.
- `fn get_key(self: &Self) -> Key` - Returns the key for identifying the packet.

**Traits:** Eq

**Trait Implementations:**

- **PrettyPrint**
  - `fn pretty_print(buffer: &dyn AsRef, f: & mut fmt::Formatter, indent: & mut PrettyIndent) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Packet<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Packet<T>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## smoltcp::wire::ipv4::Repr

*Struct*

A high-level representation of an Internet Protocol version 4 packet header.

**Fields:**
- `src_addr: Address`
- `dst_addr: Address`
- `next_header: Protocol`
- `payload_len: usize`
- `hop_limit: u8`

**Methods:**

- `fn parse<T>(packet: &Packet<&T>, checksum_caps: &ChecksumCapabilities) -> Result<Repr>` - Parse an Internet Protocol version 4 packet and return a high-level representation.
- `fn buffer_len(self: &Self) -> usize` - Return the length of a header that will be emitted from this high-level representation.
- `fn emit<T>(self: &Self, packet: & mut Packet<T>, checksum_caps: &ChecksumCapabilities)` - Emit a high-level representation into an Internet Protocol version 4 packet.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Repr) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Repr`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



