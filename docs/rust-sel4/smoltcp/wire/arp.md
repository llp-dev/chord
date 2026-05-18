**smoltcp > wire > arp**

# Module: wire::arp

## Contents

**Structs**

- [`Packet`](#packet) - A read/write wrapper around an Address Resolution Protocol packet buffer.

**Enums**

- [`Hardware`](#hardware) - ARP hardware type.
- [`Operation`](#operation) - ARP operation type.
- [`Repr`](#repr) - A high-level representation of an Address Resolution Protocol packet.

---

## smoltcp::wire::arp::Hardware

*Enum*

ARP hardware type.

**Variants:**
- `Ethernet`
- `Unknown(u16)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Hardware) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Hardware) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u16) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Hardware) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Hardware`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## smoltcp::wire::arp::Operation

*Enum*

ARP operation type.

**Variants:**
- `Request`
- `Reply`
- `Unknown(u16)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Operation`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Operation) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Operation) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u16) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Operation) -> $crate::cmp::Ordering`



## smoltcp::wire::arp::Packet

*Struct*

A read/write wrapper around an Address Resolution Protocol packet buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn new_unchecked(buffer: T) -> Packet<T>` - Imbue a raw octet buffer with ARP packet structure.
- `fn new_checked(buffer: T) -> Result<Packet<T>>` - Shorthand for a combination of [new_unchecked] and [check_len].
- `fn check_len(self: &Self) -> Result<()>` - Ensure that no accessor method will panic if called.
- `fn into_inner(self: Self) -> T` - Consume the packet, returning the underlying buffer.
- `fn hardware_type(self: &Self) -> Hardware` - Return the hardware type field.
- `fn protocol_type(self: &Self) -> Protocol` - Return the protocol type field.
- `fn hardware_len(self: &Self) -> u8` - Return the hardware length field.
- `fn protocol_len(self: &Self) -> u8` - Return the protocol length field.
- `fn operation(self: &Self) -> Operation` - Return the operation field.
- `fn source_hardware_addr(self: &Self) -> &[u8]` - Return the source hardware address field.
- `fn source_protocol_addr(self: &Self) -> &[u8]` - Return the source protocol address field.
- `fn target_hardware_addr(self: &Self) -> &[u8]` - Return the target hardware address field.
- `fn target_protocol_addr(self: &Self) -> &[u8]` - Return the target protocol address field.
- `fn set_hardware_type(self: & mut Self, value: Hardware)` - Set the hardware type field.
- `fn set_protocol_type(self: & mut Self, value: Protocol)` - Set the protocol type field.
- `fn set_hardware_len(self: & mut Self, value: u8)` - Set the hardware length field.
- `fn set_protocol_len(self: & mut Self, value: u8)` - Set the protocol length field.
- `fn set_operation(self: & mut Self, value: Operation)` - Set the operation field.
- `fn set_source_hardware_addr(self: & mut Self, value: &[u8])` - Set the source hardware address field.
- `fn set_source_protocol_addr(self: & mut Self, value: &[u8])` - Set the source protocol address field.
- `fn set_target_hardware_addr(self: & mut Self, value: &[u8])` - Set the target hardware address field.
- `fn set_target_protocol_addr(self: & mut Self, value: &[u8])` - Set the target protocol address field.

**Traits:** Eq

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
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



## smoltcp::wire::arp::Repr

*Enum*

A high-level representation of an Address Resolution Protocol packet.

**Variants:**
- `EthernetIpv4{ operation: Operation, source_hardware_addr: super::EthernetAddress, source_protocol_addr: super::Ipv4Address, target_hardware_addr: super::EthernetAddress, target_protocol_addr: super::Ipv4Address }` - An Ethernet and IPv4 Address Resolution Protocol packet.

**Methods:**

- `fn parse<T>(packet: &Packet<T>) -> Result<Repr>` - Parse an Address Resolution Protocol packet and return a high-level representation,
- `fn buffer_len(self: &Self) -> usize` - Return the length of a packet that will be emitted from this high-level representation.
- `fn emit<T>(self: &Self, packet: & mut Packet<T>)` - Emit a high-level representation into an Address Resolution Protocol packet.

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



