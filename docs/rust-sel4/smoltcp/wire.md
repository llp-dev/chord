**smoltcp > wire**

# Module: wire

## Contents

**Modules**

- [`pretty_print`](#pretty_print) -  Pretty-printing of packet representation.

**Structs**

- [`Error`](#error) - Parsing a packet failed.
- [`RawHardwareAddress`](#rawhardwareaddress) - Unparsed hardware address.

**Enums**

- [`HardwareAddress`](#hardwareaddress) - Representation of an hardware address, such as an Ethernet address or an IEEE802.15.4 address.

**Constants**

- [`MAX_HARDWARE_ADDRESS_LEN`](#max_hardware_address_len)

**Type Aliases**

- [`Result`](#result)

---

## smoltcp::wire::Error

*Struct*

Parsing a packet failed.

Either it is malformed, or it is not supported by smoltcp.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::wire::HardwareAddress

*Enum*

Representation of an hardware address, such as an Ethernet address or an IEEE802.15.4 address.

**Variants:**
- `Ethernet(EthernetAddress)`

**Methods:**

- `fn as_bytes(self: &Self) -> &[u8]`
- `fn is_unicast(self: &Self) -> bool` - Query whether the address is an unicast address.
- `fn is_broadcast(self: &Self) -> bool` - Query whether the address is a broadcast address.
- `fn as_eui_64(self: &Self) -> Option<[u8; 8]>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &HardwareAddress) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> HardwareAddress`
- **From**
  - `fn from(addr: EthernetAddress) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::wire::MAX_HARDWARE_ADDRESS_LEN

*Constant*: `usize`



## smoltcp::wire::RawHardwareAddress

*Struct*

Unparsed hardware address.

Used to make NDISC parsing agnostic of the hardware medium in use.

**Methods:**

- `fn from_bytes(addr: &[u8]) -> Self` - Create a new `RawHardwareAddress` from a byte slice.
- `fn as_bytes(self: &Self) -> &[u8]`
- `fn len(self: &Self) -> usize`
- `fn is_empty(self: &Self) -> bool`
- `fn parse(self: &Self, medium: Medium) -> Result<HardwareAddress>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(addr: HardwareAddress) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RawHardwareAddress) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RawHardwareAddress`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(addr: EthernetAddress) -> Self`



## smoltcp::wire::Result

*Type Alias*: `core::result::Result<T, Error>`



## Module: pretty_print

 Pretty-printing of packet representation.

The `pretty_print` module provides bits and pieces for printing concise,
easily human readable packet listings.

# Example

A packet can be formatted using the `PrettyPrinter` wrapper:

```rust
use smoltcp::wire::*;
let buffer = vec![
    // Ethernet II
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
    0x08, 0x00,
    // IPv4
    0x45, 0x00, 0x00, 0x20,
    0x00, 0x00, 0x40, 0x00,
    0x40, 0x01, 0xd2, 0x79,
    0x11, 0x12, 0x13, 0x14,
    0x21, 0x22, 0x23, 0x24,
    // ICMPv4
    0x08, 0x00, 0x8e, 0xfe,
    0x12, 0x34, 0xab, 0xcd,
    0xaa, 0x00, 0x00, 0xff
];

let result = "\
EthernetII src=11-12-13-14-15-16 dst=01-02-03-04-05-06 type=IPv4\n\
\\ IPv4 src=17.18.19.20 dst=33.34.35.36 proto=ICMP (checksum incorrect)\n \
 \\ ICMPv4 echo request id=4660 seq=43981 len=4\
";

#[cfg(all(feature = "medium-ethernet", feature = "proto-ipv4"))]
assert_eq!(
    result,
    &format!("{}", PrettyPrinter::<EthernetFrame<&'static [u8]>>::new("", &buffer))
);
```



