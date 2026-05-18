**smoltcp > wire > icmpv4**

# Module: wire::icmpv4

## Contents

**Structs**

- [`Packet`](#packet) - A read/write wrapper around an Internet Control Message Protocol version 4 packet buffer.

**Enums**

- [`DstUnreachable`](#dstunreachable) - Internet protocol control message subtype for type "Destination Unreachable".
- [`Message`](#message) - Internet protocol control message type.
- [`ParamProblem`](#paramproblem) - Internet protocol control message subtype for type "Parameter Problem".
- [`Redirect`](#redirect) - Internet protocol control message subtype for type "Redirect Message".
- [`Repr`](#repr) - A high-level representation of an Internet Control Message Protocol version 4 packet header.
- [`TimeExceeded`](#timeexceeded) - Internet protocol control message subtype for type "Time Exceeded".

---

## smoltcp::wire::icmpv4::DstUnreachable

*Enum*

Internet protocol control message subtype for type "Destination Unreachable".

**Variants:**
- `NetUnreachable` - Destination network unreachable
- `HostUnreachable` - Destination host unreachable
- `ProtoUnreachable` - Destination protocol unreachable
- `PortUnreachable` - Destination port unreachable
- `FragRequired` - Fragmentation required, and DF flag set
- `SrcRouteFailed` - Source route failed
- `DstNetUnknown` - Destination network unknown
- `DstHostUnknown` - Destination host unknown
- `SrcHostIsolated` - Source host isolated
- `NetProhibited` - Network administratively prohibited
- `HostProhibited` - Host administratively prohibited
- `NetUnreachToS` - Network unreachable for ToS
- `HostUnreachToS` - Host unreachable for ToS
- `CommProhibited` - Communication administratively prohibited
- `HostPrecedViol` - Host precedence violation
- `PrecedCutoff` - Precedence cutoff in effect
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DstUnreachable) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DstUnreachable) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &DstUnreachable) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> DstUnreachable`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## smoltcp::wire::icmpv4::Message

*Enum*

Internet protocol control message type.

**Variants:**
- `EchoReply` - Echo reply
- `DstUnreachable` - Destination unreachable
- `Redirect` - Message redirect
- `EchoRequest` - Echo request
- `RouterAdvert` - Router advertisement
- `RouterSolicit` - Router solicitation
- `TimeExceeded` - Time exceeded
- `ParamProblem` - Parameter problem
- `Timestamp` - Timestamp
- `TimestampReply` - Timestamp reply
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Message) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Message) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Message`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Message) -> bool`



## smoltcp::wire::icmpv4::Packet

*Struct*

A read/write wrapper around an Internet Control Message Protocol version 4 packet buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn new_unchecked(buffer: T) -> Packet<T>` - Imbue a raw octet buffer with ICMPv4 packet structure.
- `fn new_checked(buffer: T) -> Result<Packet<T>>` - Shorthand for a combination of [new_unchecked] and [check_len].
- `fn check_len(self: &Self) -> Result<()>` - Ensure that no accessor method will panic if called.
- `fn into_inner(self: Self) -> T` - Consume the packet, returning the underlying buffer.
- `fn msg_type(self: &Self) -> Message` - Return the message type field.
- `fn msg_code(self: &Self) -> u8` - Return the message code field.
- `fn checksum(self: &Self) -> u16` - Return the checksum field.
- `fn echo_ident(self: &Self) -> u16` - Return the identifier field (for echo request and reply packets).
- `fn echo_seq_no(self: &Self) -> u16` - Return the sequence number field (for echo request and reply packets).
- `fn header_len(self: &Self) -> usize` - Return the header length.
- `fn verify_checksum(self: &Self) -> bool` - Validate the header checksum.
- `fn set_msg_type(self: & mut Self, value: Message)` - Set the message type field.
- `fn set_msg_code(self: & mut Self, value: u8)` - Set the message code field.
- `fn set_checksum(self: & mut Self, value: u16)` - Set the checksum field.
- `fn set_echo_ident(self: & mut Self, value: u16)` - Set the identifier field (for echo request and reply packets).
- `fn set_echo_seq_no(self: & mut Self, value: u16)` - Set the sequence number field (for echo request and reply packets).
- `fn fill_checksum(self: & mut Self)` - Compute and fill in the header checksum.
- `fn data(self: &Self) -> &'a [u8]` - Return a pointer to the type-specific data.
- `fn data_mut(self: & mut Self) -> & mut [u8]` - Return a mutable pointer to the type-specific data.

**Traits:** Eq

**Trait Implementations:**

- **PrettyPrint**
  - `fn pretty_print(buffer: &dyn AsRef, f: & mut fmt::Formatter, indent: & mut PrettyIndent) -> fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Packet<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Packet<T>`



## smoltcp::wire::icmpv4::ParamProblem

*Enum*

Internet protocol control message subtype for type "Parameter Problem".

**Variants:**
- `AtPointer` - Pointer indicates the error
- `MissingOption` - Missing a required option
- `BadLength` - Bad length
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &ParamProblem) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> ParamProblem`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParamProblem) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ParamProblem) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`



## smoltcp::wire::icmpv4::Redirect

*Enum*

Internet protocol control message subtype for type "Redirect Message".

**Variants:**
- `Net` - Redirect Datagram for the Network
- `Host` - Redirect Datagram for the Host
- `NetToS` - Redirect Datagram for the ToS & network
- `HostToS` - Redirect Datagram for the ToS & host
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Redirect) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Redirect) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Redirect) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Redirect`



## smoltcp::wire::icmpv4::Repr

*Enum*

A high-level representation of an Internet Control Message Protocol version 4 packet header.

**Generic Parameters:**
- 'a

**Variants:**
- `EchoRequest{ ident: u16, seq_no: u16, data: &'a [u8] }`
- `EchoReply{ ident: u16, seq_no: u16, data: &'a [u8] }`
- `DstUnreachable{ reason: DstUnreachable, header: crate::wire::Ipv4Repr, data: &'a [u8] }`
- `TimeExceeded{ reason: TimeExceeded, header: crate::wire::Ipv4Repr, data: &'a [u8] }`

**Methods:**

- `fn parse<T>(packet: &Packet<&'a T>, checksum_caps: &ChecksumCapabilities) -> Result<Repr<'a>>` - Parse an Internet Control Message Protocol version 4 packet and return
- `fn buffer_len(self: &Self) -> usize` - Return the length of a packet that will be emitted from this high-level representation.
- `fn emit<T>(self: &Self, packet: & mut Packet<& mut T>, checksum_caps: &ChecksumCapabilities)` - Emit a high-level representation into an Internet Control Message Protocol version 4

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Repr<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Repr<'a>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::wire::icmpv4::TimeExceeded

*Enum*

Internet protocol control message subtype for type "Time Exceeded".

**Variants:**
- `TtlExpired` - TTL expired in transit
- `FragExpired` - Fragment reassembly time exceeded
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TimeExceeded`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TimeExceeded) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &TimeExceeded) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &TimeExceeded) -> $crate::cmp::Ordering`



