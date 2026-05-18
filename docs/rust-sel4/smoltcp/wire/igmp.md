**smoltcp > wire > igmp**

# Module: wire::igmp

## Contents

**Structs**

- [`Packet`](#packet) - A read/write wrapper around an Internet Group Management Protocol v1/v2 packet buffer.

**Enums**

- [`IgmpVersion`](#igmpversion) - Type of IGMP membership report version
- [`Message`](#message) - Internet Group Management Protocol v1/v2 message version/type.
- [`Repr`](#repr) - A high-level representation of an Internet Group Management Protocol v1/v2 header.

---

## smoltcp::wire::igmp::IgmpVersion

*Enum*

Type of IGMP membership report version

**Variants:**
- `Version1` - IGMPv1
- `Version2` - IGMPv2

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IgmpVersion) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> IgmpVersion`



## smoltcp::wire::igmp::Message

*Enum*

Internet Group Management Protocol v1/v2 message version/type.

**Variants:**
- `MembershipQuery` - Membership Query
- `MembershipReportV2` - Version 2 Membership Report
- `LeaveGroup` - Leave Group
- `MembershipReportV1` - Version 1 Membership Report
- `Unknown(u8)`



## smoltcp::wire::igmp::Packet

*Struct*

A read/write wrapper around an Internet Group Management Protocol v1/v2 packet buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn new_unchecked(buffer: T) -> Packet<T>` - Imbue a raw octet buffer with IGMPv2 packet structure.
- `fn new_checked(buffer: T) -> Result<Packet<T>>` - Shorthand for a combination of [new_unchecked] and [check_len].
- `fn check_len(self: &Self) -> Result<()>` - Ensure that no accessor method will panic if called.
- `fn into_inner(self: Self) -> T` - Consume the packet, returning the underlying buffer.
- `fn msg_type(self: &Self) -> Message` - Return the message type field.
- `fn max_resp_code(self: &Self) -> u8` - Return the maximum response time, using the encoding specified in
- `fn checksum(self: &Self) -> u16` - Return the checksum field.
- `fn group_addr(self: &Self) -> Ipv4Address` - Return the source address field.
- `fn verify_checksum(self: &Self) -> bool` - Validate the header checksum.
- `fn set_msg_type(self: & mut Self, value: Message)` - Set the message type field.
- `fn set_max_resp_code(self: & mut Self, value: u8)` - Set the maximum response time, using the encoding specified in
- `fn set_checksum(self: & mut Self, value: u16)` - Set the checksum field.
- `fn set_group_address(self: & mut Self, addr: Ipv4Address)` - Set the group address field
- `fn fill_checksum(self: & mut Self)` - Compute and fill in the header checksum.

**Trait Implementations:**

- **PrettyPrint**
  - `fn pretty_print(buffer: &dyn AsRef, f: & mut fmt::Formatter, indent: & mut PrettyIndent) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## smoltcp::wire::igmp::Repr

*Enum*

A high-level representation of an Internet Group Management Protocol v1/v2 header.

**Variants:**
- `MembershipQuery{ max_resp_time: crate::time::Duration, group_addr: crate::wire::Ipv4Address, version: IgmpVersion }`
- `MembershipReport{ group_addr: crate::wire::Ipv4Address, version: IgmpVersion }`
- `LeaveGroup{ group_addr: crate::wire::Ipv4Address }`

**Methods:**

- `fn parse<T>(packet: &Packet<&T>) -> Result<Repr>` - Parse an Internet Group Management Protocol v1/v2 packet and return
- `fn buffer_len(self: &Self) -> usize` - Return the length of a packet that will be emitted from this high-level representation.
- `fn emit<T>(self: &Self, packet: & mut Packet<& mut T>)` - Emit a high-level representation into an Internet Group Management Protocol v2 packet.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Repr) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Repr`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



