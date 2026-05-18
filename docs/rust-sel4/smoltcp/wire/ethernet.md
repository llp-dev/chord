**smoltcp > wire > ethernet**

# Module: wire::ethernet

## Contents

**Structs**

- [`Address`](#address) - A six-octet Ethernet II address.
- [`Frame`](#frame) - A read/write wrapper around an Ethernet II frame buffer.
- [`Repr`](#repr) - A high-level representation of an Internet Protocol version 4 packet header.

**Enums**

- [`EtherType`](#ethertype) - Ethernet protocol type.

**Constants**

- [`HEADER_LEN`](#header_len) - The Ethernet header length

---

## smoltcp::wire::ethernet::Address

*Struct*

A six-octet Ethernet II address.

**Tuple Struct**: `([u8; 6])`

**Methods:**

- `fn from_bytes(data: &[u8]) -> Address` - Construct an Ethernet address from a sequence of octets, in big-endian.
- `fn as_bytes(self: &Self) -> &[u8]` - Return an Ethernet address as a sequence of octets, in big-endian.
- `fn is_unicast(self: &Self) -> bool` - Query whether the address is an unicast address.
- `fn is_broadcast(self: &Self) -> bool` - Query whether this address is the broadcast address.
- `fn is_multicast(self: &Self) -> bool` - Query whether the "multicast" bit in the OUI is set.
- `fn is_local(self: &Self) -> bool` - Query whether the "locally administered" bit in the OUI is set.
- `fn as_eui_64(self: &Self) -> Option<[u8; 8]>` - Convert the address to an Extended Unique Identifier (EUI-64)

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Address) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Address) -> $crate::cmp::Ordering`
- **FromStr**
  - `fn from_str(s: &str) -> result::Result<EthernetAddress, ()>` - Parse a string representation of an Ethernet address.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Address`
- **Default**
  - `fn default() -> Address`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Address) -> bool`



## smoltcp::wire::ethernet::EtherType

*Enum*

Ethernet protocol type.

**Variants:**
- `Ipv4`
- `Arp`
- `Ipv6`
- `Unknown(u16)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &EtherType) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &EtherType) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u16) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &EtherType) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> EtherType`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## smoltcp::wire::ethernet::Frame

*Struct*

A read/write wrapper around an Ethernet II frame buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn new_unchecked(buffer: T) -> Frame<T>` - Imbue a raw octet buffer with Ethernet frame structure.
- `fn new_checked(buffer: T) -> Result<Frame<T>>` - Shorthand for a combination of [new_unchecked] and [check_len].
- `fn check_len(self: &Self) -> Result<()>` - Ensure that no accessor method will panic if called.
- `fn into_inner(self: Self) -> T` - Consumes the frame, returning the underlying buffer.
- `fn header_len() -> usize` - Return the length of a frame header.
- `fn buffer_len(payload_len: usize) -> usize` - Return the length of a buffer required to hold a packet with the payload
- `fn dst_addr(self: &Self) -> Address` - Return the destination address field.
- `fn src_addr(self: &Self) -> Address` - Return the source address field.
- `fn ethertype(self: &Self) -> EtherType` - Return the EtherType field, without checking for 802.1Q.
- `fn payload(self: &Self) -> &'a [u8]` - Return a pointer to the payload, without checking for 802.1Q.
- `fn set_dst_addr(self: & mut Self, value: Address)` - Set the destination address field.
- `fn set_src_addr(self: & mut Self, value: Address)` - Set the source address field.
- `fn set_ethertype(self: & mut Self, value: EtherType)` - Set the EtherType field.
- `fn payload_mut(self: & mut Self) -> & mut [u8]` - Return a mutable pointer to the payload.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PrettyPrint**
  - `fn pretty_print(buffer: &dyn AsRef, f: & mut fmt::Formatter, indent: & mut PrettyIndent) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Frame<T>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::wire::ethernet::HEADER_LEN

*Constant*: `usize`

The Ethernet header length



## smoltcp::wire::ethernet::Repr

*Struct*

A high-level representation of an Internet Protocol version 4 packet header.

**Fields:**
- `src_addr: Address`
- `dst_addr: Address`
- `ethertype: EtherType`

**Methods:**

- `fn parse<T>(frame: &Frame<&T>) -> Result<Repr>` - Parse an Ethernet II frame and return a high-level representation.
- `fn buffer_len(self: &Self) -> usize` - Return the length of a header that will be emitted from this high-level representation.
- `fn emit<T>(self: &Self, frame: & mut Frame<T>)` - Emit a high-level representation into an Ethernet II frame.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Repr) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Repr`



