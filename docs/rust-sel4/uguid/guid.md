**uguid > guid**

# Module: guid

## Contents

**Structs**

- [`Guid`](#guid) - Globally-unique identifier.

**Enums**

- [`Variant`](#variant) - Variant or type of GUID, as defined in [RFC4122].

---

## uguid::guid::Guid

*Struct*

Globally-unique identifier.

The format is defined in [RFC 4122]. However, unlike "normal" UUIDs
(such as those provided by the [`uuid`] crate), the first three
fields are little-endian. See also [Appendix A] of the UEFI
Specification.

This type is 4-byte aligned. The UEFI Specification says the GUID
type should be 8-byte aligned, but most C implementations have
4-byte alignment, so we do the same here for compatibility.

[Appendix A]: https://uefi.org/specs/UEFI/2.10/Apx_A_GUID_and_Time_Formats.html
[RFC 4122]: https://datatracker.ietf.org/doc/html/rfc4122
[`uuid`]: https://docs.rs/uuid/latest/uuid

**Fields:**
- `time_low: u32`
- `time_mid: [u8; 2]`
- `time_high_and_version: [u8; 2]`
- `clock_seq_high_and_reserved: u8`
- `clock_seq_low: u8`
- `node: [u8; 6]`

**Methods:**

- `fn new(time_low: [u8; 4], time_mid: [u8; 2], time_high_and_version: [u8; 2], clock_seq_high_and_reserved: u8, clock_seq_low: u8, node: [u8; 6]) -> Self` - Create a new GUID.
- `fn from_random_bytes(random_bytes: [u8; 16]) -> Self` - Create a version 4 GUID from provided random bytes.
- `fn is_zero(self: Self) -> bool` - True if all bits are zero, false otherwise.
- `fn time_low(self: Self) -> [u8; 4]` - The little-endian low field of the timestamp.
- `fn time_mid(self: Self) -> [u8; 2]` - The little-endian middle field of the timestamp.
- `fn time_high_and_version(self: Self) -> [u8; 2]` - The little-endian high field of the timestamp multiplexed with
- `fn clock_seq_high_and_reserved(self: Self) -> u8` - The high field of the clock sequence multiplexed with the
- `fn clock_seq_low(self: Self) -> u8` - The low field of the clock sequence.
- `fn node(self: Self) -> [u8; 6]` - The spatially unique node identifier.
- `fn variant(self: Self) -> Variant` - Get the GUID variant.
- `fn version(self: Self) -> u8` - Get the GUID version. This is a sub-type of the variant as
- `fn try_parse(s: &str) -> Result<Self, GuidFromStrError>` - Parse a GUID from a string.
- `fn parse_or_panic(s: &str) -> Self` - Parse a GUID from a string, panicking on failure.
- `fn from_bytes(bytes: [u8; 16]) -> Self` - Create a GUID from a 16-byte array. No changes to byte order are made.
- `fn to_bytes(self: Self) -> [u8; 16]` - Convert to a 16-byte array.
- `fn to_ascii_hex_lower(self: Self) -> [u8; 36]` - Convert to a lower-case hex ASCII string.

**Traits:** Eq, Copy, Zeroable, Pod

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Guid) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Guid`
- **Ord**
  - `fn cmp(self: &Self, other: &Guid) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>` - Parse a GUID from a string, panicking on failure.
- **PartialEq**
  - `fn eq(self: &Self, other: &Guid) -> bool`



## uguid::guid::Variant

*Enum*

Variant or type of GUID, as defined in [RFC4122].

[RFC4122]: https://datatracker.ietf.org/doc/html/rfc4122#section-4.1.3

**Variants:**
- `ReservedNcs` - Reserved, NCS backward compatibility.
- `Rfc4122` - The GUID variant described by RFC4122.
- `ReservedMicrosoft` - Reserved, Microsoft Corporation backward compatibility.
- `ReservedFuture` - Reserved for future use.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Variant`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Variant) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Variant) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Variant) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



