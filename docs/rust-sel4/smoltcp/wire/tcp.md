**smoltcp > wire > tcp**

# Module: wire::tcp

## Contents

**Structs**

- [`Packet`](#packet) - A read/write wrapper around a Transmission Control Protocol packet buffer.
- [`Repr`](#repr) - A high-level representation of a Transmission Control Protocol packet.
- [`SeqNumber`](#seqnumber) - A TCP sequence number.
- [`TcpTimestampRepr`](#tcptimestamprepr)

**Enums**

- [`Control`](#control) - The possible control flags of a Transmission Control Protocol packet.
- [`TcpOption`](#tcpoption) - A representation of a single TCP option.

**Constants**

- [`HEADER_LEN`](#header_len)

**Type Aliases**

- [`TcpTimestampGenerator`](#tcptimestampgenerator)

---

## smoltcp::wire::tcp::Control

*Enum*

The possible control flags of a Transmission Control Protocol packet.

**Variants:**
- `None`
- `Psh`
- `Syn`
- `Fin`
- `Rst`

**Methods:**

- `fn len(self: Self) -> usize` - Return the length of a control flag, in terms of sequence space.
- `fn quash_psh(self: Self) -> Control` - Turn the PSH flag into no flag, and keep the rest as-is.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Control) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Control`



## smoltcp::wire::tcp::HEADER_LEN

*Constant*: `usize`



## smoltcp::wire::tcp::Packet

*Struct*

A read/write wrapper around a Transmission Control Protocol packet buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn new_unchecked(buffer: T) -> Packet<T>` - Imbue a raw octet buffer with TCP packet structure.
- `fn new_checked(buffer: T) -> Result<Packet<T>>` - Shorthand for a combination of [new_unchecked] and [check_len].
- `fn check_len(self: &Self) -> Result<()>` - Ensure that no accessor method will panic if called.
- `fn into_inner(self: Self) -> T` - Consume the packet, returning the underlying buffer.
- `fn src_port(self: &Self) -> u16` - Return the source port field.
- `fn dst_port(self: &Self) -> u16` - Return the destination port field.
- `fn seq_number(self: &Self) -> SeqNumber` - Return the sequence number field.
- `fn ack_number(self: &Self) -> SeqNumber` - Return the acknowledgement number field.
- `fn fin(self: &Self) -> bool` - Return the FIN flag.
- `fn syn(self: &Self) -> bool` - Return the SYN flag.
- `fn rst(self: &Self) -> bool` - Return the RST flag.
- `fn psh(self: &Self) -> bool` - Return the PSH flag.
- `fn ack(self: &Self) -> bool` - Return the ACK flag.
- `fn urg(self: &Self) -> bool` - Return the URG flag.
- `fn ece(self: &Self) -> bool` - Return the ECE flag.
- `fn cwr(self: &Self) -> bool` - Return the CWR flag.
- `fn ns(self: &Self) -> bool` - Return the NS flag.
- `fn header_len(self: &Self) -> u8` - Return the header length, in octets.
- `fn window_len(self: &Self) -> u16` - Return the window size field.
- `fn checksum(self: &Self) -> u16` - Return the checksum field.
- `fn urgent_at(self: &Self) -> u16` - Return the urgent pointer field.
- `fn segment_len(self: &Self) -> usize` - Return the length of the segment, in terms of sequence space.
- `fn selective_ack_permitted(self: &Self) -> Result<bool>` - Returns whether the selective acknowledgement SYN flag is set or not.
- `fn selective_ack_ranges(self: &Self) -> Result<[Option<(u32, u32)>; 3]>` - Return the selective acknowledgement ranges, if any. If there are none in the packet, an
- `fn verify_partial_checksum(self: &Self, src_addr: &IpAddress, dst_addr: &IpAddress) -> bool` - Validate the partial checksum.
- `fn verify_checksum(self: &Self, src_addr: &IpAddress, dst_addr: &IpAddress) -> bool` - Validate the packet checksum.
- `fn options(self: &Self) -> &'a [u8]` - Return a pointer to the options.
- `fn payload(self: &Self) -> &'a [u8]` - Return a pointer to the payload.
- `fn set_src_port(self: & mut Self, value: u16)` - Set the source port field.
- `fn set_dst_port(self: & mut Self, value: u16)` - Set the destination port field.
- `fn set_seq_number(self: & mut Self, value: SeqNumber)` - Set the sequence number field.
- `fn set_ack_number(self: & mut Self, value: SeqNumber)` - Set the acknowledgement number field.
- `fn clear_flags(self: & mut Self)` - Clear the entire flags field.
- `fn set_fin(self: & mut Self, value: bool)` - Set the FIN flag.
- `fn set_syn(self: & mut Self, value: bool)` - Set the SYN flag.
- `fn set_rst(self: & mut Self, value: bool)` - Set the RST flag.
- `fn set_psh(self: & mut Self, value: bool)` - Set the PSH flag.
- `fn set_ack(self: & mut Self, value: bool)` - Set the ACK flag.
- `fn set_urg(self: & mut Self, value: bool)` - Set the URG flag.
- `fn set_ece(self: & mut Self, value: bool)` - Set the ECE flag.
- `fn set_cwr(self: & mut Self, value: bool)` - Set the CWR flag.
- `fn set_ns(self: & mut Self, value: bool)` - Set the NS flag.
- `fn set_header_len(self: & mut Self, value: u8)` - Set the header length, in octets.
- `fn set_window_len(self: & mut Self, value: u16)` - Set the window size field.
- `fn set_checksum(self: & mut Self, value: u16)` - Set the checksum field.
- `fn set_urgent_at(self: & mut Self, value: u16)` - Set the urgent pointer field.
- `fn fill_checksum(self: & mut Self, src_addr: &IpAddress, dst_addr: &IpAddress)` - Compute and fill in the header checksum.
- `fn options_mut(self: & mut Self) -> & mut [u8]` - Return a pointer to the options.
- `fn payload_mut(self: & mut Self) -> & mut [u8]` - Return a mutable pointer to the payload data.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Packet<T>`
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



## smoltcp::wire::tcp::Repr

*Struct*

A high-level representation of a Transmission Control Protocol packet.

**Generic Parameters:**
- 'a

**Fields:**
- `src_port: u16`
- `dst_port: u16`
- `control: Control`
- `seq_number: SeqNumber`
- `ack_number: Option<SeqNumber>`
- `window_len: u16`
- `window_scale: Option<u8>`
- `max_seg_size: Option<u16>`
- `sack_permitted: bool`
- `sack_ranges: [Option<(u32, u32)>; 3]`
- `timestamp: Option<TcpTimestampRepr>`
- `payload: &'a [u8]`

**Methods:**

- `fn parse<T>(packet: &Packet<&'a T>, src_addr: &IpAddress, dst_addr: &IpAddress, checksum_caps: &ChecksumCapabilities) -> Result<Repr<'a>>` - Parse a Transmission Control Protocol packet and return a high-level representation.
- `fn header_len(self: &Self) -> usize` - Return the length of a header that will be emitted from this high-level representation.
- `fn buffer_len(self: &Self) -> usize` - Return the length of a packet that will be emitted from this high-level representation.
- `fn emit<T>(self: &Self, packet: & mut Packet<& mut T>, src_addr: &IpAddress, dst_addr: &IpAddress, checksum_caps: &ChecksumCapabilities)` - Emit a high-level representation into a Transmission Control Protocol packet.
- `fn segment_len(self: &Self) -> usize` - Return the length of the segment, in terms of sequence space.
- `fn is_empty(self: &Self) -> bool` - Return whether the segment has no flags set (except PSH) and no data.

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



## smoltcp::wire::tcp::SeqNumber

*Struct*

A TCP sequence number.

A sequence number is a monotonically advancing integer modulo 2<sup>32</sup>.
Sequence numbers do not have a discontiguity when compared pairwise across a signed overflow.

**Tuple Struct**: `(i32)`

**Methods:**

- `fn max(self: Self, rhs: Self) -> Self`
- `fn min(self: Self, rhs: Self) -> Self`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: usize)`
- **PartialEq**
  - `fn eq(self: &Self, other: &SeqNumber) -> bool`
- **Sub**
  - `fn sub(self: Self, rhs: SeqNumber) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> SeqNumber`
- **Default**
  - `fn default() -> SeqNumber`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SeqNumber) -> Option<cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Add**
  - `fn add(self: Self, rhs: usize) -> SeqNumber`
- **Sub**
  - `fn sub(self: Self, rhs: usize) -> SeqNumber`



## smoltcp::wire::tcp::TcpOption

*Enum*

A representation of a single TCP option.

**Generic Parameters:**
- 'a

**Variants:**
- `EndOfList`
- `NoOperation`
- `MaxSegmentSize(u16)`
- `WindowScale(u8)`
- `SackPermitted`
- `SackRange([Option<(u32, u32)>; 3])`
- `TimeStamp{ tsval: u32, tsecr: u32 }`
- `Unknown{ kind: u8, data: &'a [u8] }`

**Methods:**

- `fn parse(buffer: &'a [u8]) -> Result<(&'a [u8], TcpOption<'a>)>`
- `fn buffer_len(self: &Self) -> usize`
- `fn emit<'b>(self: &Self, buffer: &'b  mut [u8]) -> &'b  mut [u8]`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TcpOption<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TcpOption<'a>`



## smoltcp::wire::tcp::TcpTimestampGenerator

*Type Alias*: `fn(...)`



## smoltcp::wire::tcp::TcpTimestampRepr

*Struct*

**Fields:**
- `tsval: u32`
- `tsecr: u32`

**Methods:**

- `fn new(tsval: u32, tsecr: u32) -> Self`
- `fn generate_reply(self: &Self, generator: Option<TcpTimestampGenerator>) -> Option<Self>`
- `fn generate_reply_with_tsval(generator: Option<TcpTimestampGenerator>, tsval: u32) -> Option<Self>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TcpTimestampRepr) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TcpTimestampRepr`



