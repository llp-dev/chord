**smoltcp > wire > udp**

# Module: wire::udp

## Contents

**Structs**

- [`Packet`](#packet) - A read/write wrapper around an User Datagram Protocol packet buffer.
- [`Repr`](#repr) - A high-level representation of an User Datagram Protocol packet.

**Constants**

- [`HEADER_LEN`](#header_len)

---

## smoltcp::wire::udp::HEADER_LEN

*Constant*: `usize`



## smoltcp::wire::udp::Packet

*Struct*

A read/write wrapper around an User Datagram Protocol packet buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn set_src_port(self: & mut Self, value: u16)` - Set the source port field.
- `fn set_dst_port(self: & mut Self, value: u16)` - Set the destination port field.
- `fn set_len(self: & mut Self, value: u16)` - Set the length field.
- `fn set_checksum(self: & mut Self, value: u16)` - Set the checksum field.
- `fn fill_checksum(self: & mut Self, src_addr: &IpAddress, dst_addr: &IpAddress)` - Compute and fill in the header checksum.
- `fn payload_mut(self: & mut Self) -> & mut [u8]` - Return a mutable pointer to the payload.
- `fn payload(self: &Self) -> &'a [u8]` - Return a pointer to the payload.
- `fn new_unchecked(buffer: T) -> Packet<T>` - Imbue a raw octet buffer with UDP packet structure.
- `fn new_checked(buffer: T) -> Result<Packet<T>>` - Shorthand for a combination of [new_unchecked] and [check_len].
- `fn check_len(self: &Self) -> Result<()>` - Ensure that no accessor method will panic if called.
- `fn into_inner(self: Self) -> T` - Consume the packet, returning the underlying buffer.
- `fn src_port(self: &Self) -> u16` - Return the source port field.
- `fn dst_port(self: &Self) -> u16` - Return the destination port field.
- `fn len(self: &Self) -> u16` - Return the length field.
- `fn checksum(self: &Self) -> u16` - Return the checksum field.
- `fn verify_partial_checksum(self: &Self, src_addr: &IpAddress, dst_addr: &IpAddress) -> bool` - Validate the partial packet checksum.
- `fn verify_checksum(self: &Self, src_addr: &IpAddress, dst_addr: &IpAddress) -> bool` - Validate the packet checksum.

**Traits:** Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Packet<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Packet<T>`
- **PrettyPrint**
  - `fn pretty_print(buffer: &dyn AsRef, f: & mut fmt::Formatter, indent: & mut PrettyIndent) -> fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## smoltcp::wire::udp::Repr

*Struct*

A high-level representation of an User Datagram Protocol packet.

**Fields:**
- `src_port: u16`
- `dst_port: u16`

**Methods:**

- `fn parse<T>(packet: &Packet<&T>, src_addr: &IpAddress, dst_addr: &IpAddress, checksum_caps: &ChecksumCapabilities) -> Result<Repr>` - Parse an User Datagram Protocol packet and return a high-level representation.
- `fn header_len(self: &Self) -> usize` - Return the length of the packet header that will be emitted from this high-level representation.
- `fn emit<T, impl FnOnce(&mut [u8])>(self: &Self, packet: & mut Packet<& mut T>, src_addr: &IpAddress, dst_addr: &IpAddress, payload_len: usize, emit_payload: impl Trait, checksum_caps: &ChecksumCapabilities)` - Emit a high-level representation into an User Datagram Protocol packet.

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



