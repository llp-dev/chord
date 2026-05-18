**smoltcp > wire > dhcpv4**

# Module: wire::dhcpv4

## Contents

**Structs**

- [`DhcpOption`](#dhcpoption) - A representation of a single DHCP option.
- [`DhcpOptionWriter`](#dhcpoptionwriter) - A buffer for DHCP options.
- [`Flags`](#flags)
- [`Packet`](#packet) - A read/write wrapper around a Dynamic Host Configuration Protocol packet buffer.
- [`Repr`](#repr) - A high-level representation of a Dynamic Host Configuration Protocol packet.

**Enums**

- [`MessageType`](#messagetype) - The possible message types of a DHCP packet.
- [`OpCode`](#opcode) - The possible opcodes of a DHCP packet.

**Constants**

- [`CLIENT_PORT`](#client_port)
- [`MAX_DNS_SERVER_COUNT`](#max_dns_server_count)
- [`SERVER_PORT`](#server_port)

---

## smoltcp::wire::dhcpv4::CLIENT_PORT

*Constant*: `u16`



## smoltcp::wire::dhcpv4::DhcpOption

*Struct*

A representation of a single DHCP option.

**Generic Parameters:**
- 'a

**Fields:**
- `kind: u8`
- `data: &'a [u8]`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DhcpOption<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DhcpOption<'a>`



## smoltcp::wire::dhcpv4::DhcpOptionWriter

*Struct*

A buffer for DHCP options.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(buffer: &'a  mut [u8]) -> Self`
- `fn emit(self: & mut Self, option: DhcpOption) -> Result<()>` - Emit a  [`DhcpOption`] into a [`DhcpOptionWriter`].
- `fn end(self: & mut Self) -> Result<()>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::wire::dhcpv4::Flags

*Struct*

**Methods:**

- `fn empty() -> Self` - Returns an empty set of flags.
- `fn all() -> Self` - Returns the set containing all flags.
- `fn bits(self: &Self) -> u16` - Returns the raw value of the flags currently stored.
- `fn from_bits(bits: u16) -> $crate::_core::option::Option<Self>` - Convert from underlying bit representation, unless that
- `fn from_bits_truncate(bits: u16) -> Self` - Convert from underlying bit representation, dropping any bits
- `fn from_bits_unchecked(bits: u16) -> Self` - Convert from underlying bit representation, preserving all
- `fn is_empty(self: &Self) -> bool` - Returns `true` if no flags are currently stored.
- `fn is_all(self: &Self) -> bool` - Returns `true` if all flags are currently set.
- `fn intersects(self: &Self, other: Self) -> bool` - Returns `true` if there are flags common to both `self` and `other`.
- `fn contains(self: &Self, other: Self) -> bool` - Returns `true` if all of the flags in `other` are contained within `self`.
- `fn insert(self: & mut Self, other: Self)` - Inserts the specified flags in-place.
- `fn remove(self: & mut Self, other: Self)` - Removes the specified flags in-place.
- `fn toggle(self: & mut Self, other: Self)` - Toggles the specified flags in-place.
- `fn set(self: & mut Self, other: Self, value: bool)` - Inserts or removes the specified flags depending on the passed value.
- `fn intersection(self: Self, other: Self) -> Self` - Returns the intersection between the flags in `self` and
- `fn union(self: Self, other: Self) -> Self` - Returns the union of between the flags in `self` and `other`.
- `fn difference(self: Self, other: Self) -> Self` - Returns the difference between the flags in `self` and `other`.
- `fn symmetric_difference(self: Self, other: Self) -> Self` - Returns the [symmetric difference][sym-diff] between the flags
- `fn complement(self: Self) -> Self` - Returns the complement of this set of flags.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - Returns the intersection between the two sets of flags.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Flags) -> $crate::option::Option<$crate::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - Disables all flags disabled in the set.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - Disables all flags enabled in the set.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - Returns the left flags, but with all the right flags toggled.
- **Not**
  - `fn not(self: Self) -> Self` - Returns the complement of this set of flags.
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - Toggles the set of flags.
- **Ord**
  - `fn cmp(self: &Self, other: &Flags) -> $crate::cmp::Ordering`
- **BitOr**
  - `fn bitor(self: Self, other: Flags) -> Self` - Returns the union of the two sets of flags.
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - Adds the set of flags.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Flags) -> bool`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Flags`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - Returns the set difference of the two sets of flags.



## smoltcp::wire::dhcpv4::MAX_DNS_SERVER_COUNT

*Constant*: `usize`



## smoltcp::wire::dhcpv4::MessageType

*Enum*

The possible message types of a DHCP packet.

**Variants:**
- `Discover`
- `Offer`
- `Request`
- `Decline`
- `Ack`
- `Nak`
- `Release`
- `Inform`
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &MessageType) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> MessageType`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &MessageType) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &MessageType) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`



## smoltcp::wire::dhcpv4::OpCode

*Enum*

The possible opcodes of a DHCP packet.

**Variants:**
- `Request`
- `Reply`
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &OpCode) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &OpCode) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &OpCode) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> OpCode`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## smoltcp::wire::dhcpv4::Packet

*Struct*

A read/write wrapper around a Dynamic Host Configuration Protocol packet buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn options_mut(self: & mut Self) -> DhcpOptionWriter` - Return a pointer to the options.
- `fn new_unchecked(buffer: T) -> Packet<T>` - Imbue a raw octet buffer with DHCP packet structure.
- `fn new_checked(buffer: T) -> Result<Packet<T>>` - Shorthand for a combination of [new_unchecked] and [check_len].
- `fn check_len(self: &Self) -> Result<()>` - Ensure that no accessor method will panic if called.
- `fn into_inner(self: Self) -> T` - Consume the packet, returning the underlying buffer.
- `fn opcode(self: &Self) -> OpCode` - Returns the operation code of this packet.
- `fn hardware_type(self: &Self) -> Hardware` - Returns the hardware protocol type (e.g. ethernet).
- `fn hardware_len(self: &Self) -> u8` - Returns the length of a hardware address in bytes (e.g. 6 for ethernet).
- `fn transaction_id(self: &Self) -> u32` - Returns the transaction ID.
- `fn client_hardware_address(self: &Self) -> EthernetAddress` - Returns the hardware address of the client (called `chaddr` in the specification).
- `fn hops(self: &Self) -> u8` - Returns the value of the `hops` field.
- `fn secs(self: &Self) -> u16` - Returns the value of the `secs` field.
- `fn magic_number(self: &Self) -> u32` - Returns the value of the `magic cookie` field in the DHCP options.
- `fn client_ip(self: &Self) -> Ipv4Address` - Returns the Ipv4 address of the client, zero if not set.
- `fn your_ip(self: &Self) -> Ipv4Address` - Returns the value of the `yiaddr` field, zero if not set.
- `fn server_ip(self: &Self) -> Ipv4Address` - Returns the value of the `siaddr` field, zero if not set.
- `fn relay_agent_ip(self: &Self) -> Ipv4Address` - Returns the value of the `giaddr` field, zero if not set.
- `fn flags(self: &Self) -> Flags`
- `fn options(self: &Self) -> impl Trait` - Return an iterator over the options.
- `fn get_sname(self: &Self) -> Result<&str>`
- `fn get_boot_file(self: &Self) -> Result<&str>`
- `fn set_sname_and_boot_file_to_zero(self: & mut Self)` - Sets the optional `sname` (“server name”) and `file` (“boot file name”) fields to zero.
- `fn set_opcode(self: & mut Self, value: OpCode)` - Sets the `OpCode` for the packet.
- `fn set_hardware_type(self: & mut Self, value: Hardware)` - Sets the hardware address type (only ethernet is supported).
- `fn set_hardware_len(self: & mut Self, value: u8)` - Sets the hardware address length.
- `fn set_transaction_id(self: & mut Self, value: u32)` - Sets the transaction ID.
- `fn set_client_hardware_address(self: & mut Self, value: EthernetAddress)` - Sets the ethernet address of the client.
- `fn set_hops(self: & mut Self, value: u8)` - Sets the hops field.
- `fn set_secs(self: & mut Self, value: u16)` - Sets the `secs` field.
- `fn set_magic_number(self: & mut Self, value: u32)` - Sets the value of the `magic cookie` field in the DHCP options.
- `fn set_client_ip(self: & mut Self, value: Ipv4Address)` - Sets the Ipv4 address of the client.
- `fn set_your_ip(self: & mut Self, value: Ipv4Address)` - Sets the value of the `yiaddr` field.
- `fn set_server_ip(self: & mut Self, value: Ipv4Address)` - Sets the value of the `siaddr` field.
- `fn set_relay_agent_ip(self: & mut Self, value: Ipv4Address)` - Sets the value of the `giaddr` field.
- `fn set_flags(self: & mut Self, val: Flags)` - Sets the flags to the specified value.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Packet<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Packet<T>) -> bool`



## smoltcp::wire::dhcpv4::Repr

*Struct*

A high-level representation of a Dynamic Host Configuration Protocol packet.

DHCP messages have the following layout (see [RFC 2131](https://tools.ietf.org/html/rfc2131)
for details):

```no_rust
0                   1                   2                   3
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| message_type  | htype (N/A)   |   hlen (N/A)  |   hops        |
+---------------+---------------+---------------+---------------+
|                       transaction_id                          |
+-------------------------------+-------------------------------+
|           secs                |           flags               |
+-------------------------------+-------------------------------+
|                           client_ip                           |
+---------------------------------------------------------------+
|                            your_ip                            |
+---------------------------------------------------------------+
|                           server_ip                           |
+---------------------------------------------------------------+
|                        relay_agent_ip                         |
+---------------------------------------------------------------+
|                                                               |
|                    client_hardware_address                    |
|                                                               |
|                                                               |
+---------------------------------------------------------------+
|                                                               |
|                          sname  (N/A)                         |
+---------------------------------------------------------------+
|                                                               |
|                          file    (N/A)                        |
+---------------------------------------------------------------+
|                                                               |
|                          options                              |
+---------------------------------------------------------------+
```

It is assumed that the access layer is Ethernet, so `htype` (the field representing the
hardware address type) is always set to `1`, and `hlen` (which represents the hardware address
length) is set to `6`.

The `options` field has a variable length.

**Generic Parameters:**
- 'a

**Fields:**
- `message_type: MessageType` - This field is also known as `op` in the RFC. It indicates the type of DHCP message this
- `transaction_id: u32` - This field is also known as `xid` in the RFC. It is a random number chosen by the client,
- `secs: u16` - seconds elapsed since client began address acquisition or renewal
- `client_hardware_address: crate::wire::EthernetAddress` - This field is also known as `chaddr` in the RFC and for networks where the access layer is
- `client_ip: crate::wire::Ipv4Address` - This field is also known as `ciaddr` in the RFC. It is only filled in if client is in
- `your_ip: crate::wire::Ipv4Address` - This field is also known as `yiaddr` in the RFC.
- `server_ip: crate::wire::Ipv4Address` - This field is also known as `siaddr` in the RFC. It may be set by the server in DHCPOFFER
- `router: Option<crate::wire::Ipv4Address>` - Default gateway
- `subnet_mask: Option<crate::wire::Ipv4Address>` - This field comes from a corresponding DhcpOption.
- `relay_agent_ip: crate::wire::Ipv4Address` - This field is also known as `giaddr` in the RFC. In order to allow DHCP clients on subnets
- `broadcast: bool` - Broadcast flags. It can be set in DHCPDISCOVER, DHCPINFORM and DHCPREQUEST message if the
- `requested_ip: Option<crate::wire::Ipv4Address>` - The "requested IP address" option. It can be used by clients in DHCPREQUEST or DHCPDISCOVER
- `client_identifier: Option<crate::wire::EthernetAddress>` - The "client identifier" option.
- `server_identifier: Option<crate::wire::Ipv4Address>` - The "server identifier" option. It is used both to identify a DHCP server
- `parameter_request_list: Option<&'a [u8]>` - The parameter request list informs the server about which configuration parameters
- `dns_servers: Option<heapless::Vec<crate::wire::Ipv4Address, MAX_DNS_SERVER_COUNT>>` - DNS servers
- `max_size: Option<u16>` - The maximum size dhcp packet the interface can receive
- `lease_duration: Option<u32>` - The DHCP IP lease duration, specified in seconds.
- `renew_duration: Option<u32>` - The DHCP IP renew duration (T1 interval), in seconds, if specified in the packet.
- `rebind_duration: Option<u32>` - The DHCP IP rebind duration (T2 interval), in seconds, if specified in the packet.
- `additional_options: &'a [DhcpOption<'a>]` - When returned from [`Repr::parse`], this field will be `None`.

**Methods:**

- `fn buffer_len(self: &Self) -> usize` - Return the length of a packet that will be emitted from this high-level representation.
- `fn parse<T>(packet: &'a Packet<&'a T>) -> Result<Self>` - Parse a DHCP packet and return a high-level representation.
- `fn emit<T>(self: &Self, packet: & mut Packet<& mut T>) -> Result<()>` - Emit a high-level representation into a Dynamic Host

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Repr<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Repr<'a>`



## smoltcp::wire::dhcpv4::SERVER_PORT

*Constant*: `u16`



