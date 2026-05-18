# Crate `uguid`

Library providing a GUID (Globally Unique Identifier) type. The
format is defined in [RFC 4122]. However, unlike "normal" UUIDs
(such as those provided by the `uuid` crate), the first three
fields are little-endian. See [Appendix A] of the UEFI
Specification. This format of GUID is also used in Microsoft
Windows.



# Features

No features are enabled by default.

* `bytemuck`: Implements bytemuck's `Pod` and `Zeroable` traits for `Guid`.
* `serde`: Implements serde's `Serialize` and `Deserialize` traits for `Guid`.
* `std`: Currently has no effect.

# Examples

Construct a GUID at compile time with the `guid!` macro:

```rust
use uguid::guid;

let guid = guid!("01234567-89ab-cdef-0123-456789abcdef");
```

Parse a GUID at runtime from a string:

```rust
use uguid::Guid;

let guid: Guid = "01234567-89ab-cdef-0123-456789abcdef".parse().unwrap();
```

Construct a GUID from its components or a byte array:

```rust
use uguid::Guid;

##[rustfmt::skip]
let guid1 = Guid::from_bytes([
    0x01, 0x02, 0x03, 0x04,
    0x05, 0x06, 0x07, 0x08,
    0x09, 0x10, 0x11, 0x12,
    0x13, 0x14, 0x15, 0x16,
]);
let guid2 = Guid::new(
    [0x01, 0x02, 0x03, 0x04],
    [0x05, 0x06],
    [0x07, 0x08],
    0x09,
    0x10,
    [0x11, 0x12, 0x13, 0x14, 0x15, 0x16],
);
assert_eq!(guid1, guid2);
```

Convert to a string or a byte array:

```rust
use uguid::guid;

let guid = guid!("01234567-89ab-cdef-0123-456789abcdef");
assert_eq!(guid.to_string(), "01234567-89ab-cdef-0123-456789abcdef");
assert_eq!(
    guid.to_bytes(),
    [
        0x67, 0x45, 0x23, 0x01, 0xab, 0x89, 0xef, 0xcd, 0x01, 0x23, 0x45,
        0x67, 0x89, 0xab, 0xcd, 0xef
    ]
);
```

## Contents

- [Modules](#modules)
  - [`error`](#error)
  - [`guid`](#guid)
  - [`util`](#util)
- [Structs](#structs)
  - [`Guid`](#guid)
- [Enums](#enums)
  - [`GuidFromStrError`](#guidfromstrerror)
  - [`Variant`](#variant)
- [Macros](#macros)
  - [`mtry!`](#mtry)
  - [`guid!`](#guid)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`error`](#error) | mod |  |
| [`guid`](#guid) | mod |  |
| [`util`](#util) | mod |  |
| [`Guid`](#guid) | struct |  |
| [`GuidFromStrError`](#guidfromstrerror) | enum |  |
| [`Variant`](#variant) | enum |  |
| [`mtry!`](#mtry) | macro | Macro replacement for the `?` operator, which cannot be used in const functions. |
| [`guid!`](#guid) | macro | Create a [`Guid`] from a string at compile time. |

## Modules

- [`error`](error/index.md)
- [`guid`](guid/index.md)
- [`util`](util/index.md)

## Structs

### `Guid`

```rust
struct Guid {
    time_low: u32,
    time_mid: [u8; 2],
    time_high_and_version: [u8; 2],
    clock_seq_high_and_reserved: u8,
    clock_seq_low: u8,
    node: [u8; 6],
}
```

Globally-unique identifier.

The format is defined in [RFC 4122]. However, unlike "normal" UUIDs
(such as those provided by the `uuid` crate), the first three
fields are little-endian. See also [Appendix A] of the UEFI
Specification.

This type is 4-byte aligned. The UEFI Specification says the GUID
type should be 8-byte aligned, but most C implementations have
4-byte alignment, so we do the same here for compatibility.




#### Implementations

- <span id="guid-const-zero"></span>`const ZERO: Self`

- <span id="guid-new"></span>`const fn new(time_low: [u8; 4], time_mid: [u8; 2], time_high_and_version: [u8; 2], clock_seq_high_and_reserved: u8, clock_seq_low: u8, node: [u8; 6]) -> Self`

  Create a new GUID.

- <span id="guid-from-random-bytes"></span>`const fn from_random_bytes(random_bytes: [u8; 16]) -> Self`

  Create a version 4 GUID from provided random bytes.

  

  See [RFC 4122 section 4.4][rfc] for the definition of a version

  4 GUID.

  

  This constructor does not itself generate random bytes, but

  instead expects the caller to provide suitably random bytes.

  

  # Example

  

  ```rust

  use uguid::{Guid, Variant};

  

  let guid = Guid::from_random_bytes([

      104, 192, 95, 215, 120, 33, 249, 1, 102, 21, 171, 84, 233, 204, 68, 176,

  ]);

  assert_eq!(guid.variant(), Variant::Rfc4122);

  assert_eq!(guid.version(), 4);

  ```

- <span id="guid-is-zero"></span>`const fn is_zero(self) -> bool`

  True if all bits are zero, false otherwise.

  

  # Example

  

  ```rust

  use uguid::guid;

  

  assert!(guid!("00000000-0000-0000-0000-000000000000").is_zero());

  assert!(!guid!("308bbc16-a308-47e8-8977-5e5646c5291f").is_zero());

  ```

- <span id="guid-time-low"></span>`const fn time_low(self) -> [u8; 4]`

  The little-endian low field of the timestamp.

- <span id="guid-time-mid"></span>`const fn time_mid(self) -> [u8; 2]`

  The little-endian middle field of the timestamp.

- <span id="guid-time-high-and-version"></span>`const fn time_high_and_version(self) -> [u8; 2]`

  The little-endian high field of the timestamp multiplexed with

  the version number.

- <span id="guid-clock-seq-high-and-reserved"></span>`const fn clock_seq_high_and_reserved(self) -> u8`

  The high field of the clock sequence multiplexed with the

  variant.

- <span id="guid-clock-seq-low"></span>`const fn clock_seq_low(self) -> u8`

  The low field of the clock sequence.

- <span id="guid-node"></span>`const fn node(self) -> [u8; 6]`

  The spatially unique node identifier.

- <span id="guid-variant"></span>`const fn variant(self) -> Variant` — [`Variant`](guid/index.md#variant)

  Get the GUID variant.

  

  # Example

  

  ```rust

  use uguid::{guid, Variant};

  

  assert_eq!(

      guid!("308bbc16-a308-47e8-8977-5e5646c5291f").variant(),

      Variant::Rfc4122

  );

  ```

- <span id="guid-version"></span>`const fn version(self) -> u8`

  Get the GUID version. This is a sub-type of the variant as

  defined in [RFC4122].

  

  # Example

  

  ```rust

  use uguid::guid;

  

  assert_eq!(guid!("308bbc16-a308-47e8-8977-5e5646c5291f").version(), 4);

  ```

- <span id="guid-try-parse"></span>`const fn try_parse(s: &str) -> Result<Self, GuidFromStrError>` — [`GuidFromStrError`](error/index.md#guidfromstrerror)

  Parse a GUID from a string.

  

  This is functionally the same as `Self::from_str`, but is

  exposed separately to provide a `const` method for parsing.

- <span id="guid-parse-or-panic"></span>`const fn parse_or_panic(s: &str) -> Self`

  Parse a GUID from a string, panicking on failure.

  

  The input must be in "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"

  format, where each `x` is a hex digit (any of `0-9`, `a-f`, or

  `A-F`).

  

  This function is marked `track_caller` so that error messages

  point directly to the invalid GUID string.

  

  # Panics

  

  This function will panic if the input is not in the format shown

  above. In particular, it will panic if the input is not exactly

  36 bytes long, or if the input does not have separators at the

  expected positions, or if any of the remaining characters are

  not valid hex digits.

- <span id="guid-from-bytes"></span>`const fn from_bytes(bytes: [u8; 16]) -> Self`

  Create a GUID from a 16-byte array. No changes to byte order are made.

- <span id="guid-to-bytes"></span>`const fn to_bytes(self) -> [u8; 16]`

  Convert to a 16-byte array.

- <span id="guid-to-ascii-hex-lower"></span>`const fn to_ascii_hex_lower(self) -> [u8; 36]`

  Convert to a lower-case hex ASCII string.

  

  The output is in "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx" format.

#### Trait Implementations

##### `impl AnyBitPattern for Guid`

##### `impl CheckedBitPattern for Guid`

- <span id="guid-checkedbitpattern-type-bits"></span>`type Bits = T`

- <span id="guid-checkedbitpattern-is-valid-bit-pattern"></span>`fn is_valid_bit_pattern(_bits: &T) -> bool`

##### `impl Clone for Guid`

- <span id="guid-clone"></span>`fn clone(&self) -> Guid` — [`Guid`](guid/index.md#guid)

##### `impl Copy for Guid`

##### `impl Debug for Guid`

- <span id="guid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Guid`

- <span id="guid-default"></span>`fn default() -> Self`

##### `impl Display for Guid`

- <span id="guid-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Guid`

##### `impl FromStr for Guid`

- <span id="guid-fromstr-type-err"></span>`type Err = GuidFromStrError`

- <span id="guid-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

  Parse a GUID from a string, panicking on failure.

  

  The input must be in "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"

  format, where each `x` is a hex digit (any of `0-9`, `a-f`, or

  `A-F`).

##### `impl Hash for Guid`

- <span id="guid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl NoUninit for Guid`

##### `impl Ord for Guid`

- <span id="guid-ord-cmp"></span>`fn cmp(&self, other: &Guid) -> cmp::Ordering` — [`Guid`](guid/index.md#guid)

##### `impl PartialEq for Guid`

- <span id="guid-partialeq-eq"></span>`fn eq(&self, other: &Guid) -> bool` — [`Guid`](guid/index.md#guid)

##### `impl PartialOrd for Guid`

- <span id="guid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Guid) -> option::Option<cmp::Ordering>` — [`Guid`](guid/index.md#guid)

##### `impl Pod for Guid`

##### `impl StructuralPartialEq for Guid`

##### `impl Zeroable for Guid`

## Enums

### `GuidFromStrError`

```rust
enum GuidFromStrError {
    Length,
    Separator(u8),
    Hex(u8),
}
```

Error type for `Guid::try_parse` and `Guid::from_str`.



#### Variants

- **`Length`**

  Input has the wrong length, expected 36 bytes.

- **`Separator`**

  Input is missing a separator (`-`) at this byte index.

- **`Hex`**

  Input contains invalid ASCII hex at this byte index.

#### Trait Implementations

##### `impl Clone for GuidFromStrError`

- <span id="guidfromstrerror-clone"></span>`fn clone(&self) -> GuidFromStrError` — [`GuidFromStrError`](error/index.md#guidfromstrerror)

##### `impl Copy for GuidFromStrError`

##### `impl Debug for GuidFromStrError`

- <span id="guidfromstrerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GuidFromStrError`

- <span id="guidfromstrerror-default"></span>`fn default() -> Self`

##### `impl Display for GuidFromStrError`

- <span id="guidfromstrerror-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for GuidFromStrError`

##### `impl Error for GuidFromStrError`

##### `impl Hash for GuidFromStrError`

- <span id="guidfromstrerror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for GuidFromStrError`

- <span id="guidfromstrerror-ord-cmp"></span>`fn cmp(&self, other: &GuidFromStrError) -> cmp::Ordering` — [`GuidFromStrError`](error/index.md#guidfromstrerror)

##### `impl PartialEq for GuidFromStrError`

- <span id="guidfromstrerror-partialeq-eq"></span>`fn eq(&self, other: &GuidFromStrError) -> bool` — [`GuidFromStrError`](error/index.md#guidfromstrerror)

##### `impl PartialOrd for GuidFromStrError`

- <span id="guidfromstrerror-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &GuidFromStrError) -> option::Option<cmp::Ordering>` — [`GuidFromStrError`](error/index.md#guidfromstrerror)

##### `impl StructuralPartialEq for GuidFromStrError`

### `Variant`

```rust
enum Variant {
    ReservedNcs,
    Rfc4122,
    ReservedMicrosoft,
    ReservedFuture,
}
```

Variant or type of GUID, as defined in [RFC4122].


#### Variants

- **`ReservedNcs`**

  Reserved, NCS backward compatibility.

- **`Rfc4122`**

  The GUID variant described by RFC4122.

- **`ReservedMicrosoft`**

  Reserved, Microsoft Corporation backward compatibility.

- **`ReservedFuture`**

  Reserved for future use.

#### Trait Implementations

##### `impl Clone for Variant`

- <span id="variant-clone"></span>`fn clone(&self) -> Variant` — [`Variant`](guid/index.md#variant)

##### `impl Copy for Variant`

##### `impl Debug for Variant`

- <span id="variant-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Variant`

##### `impl Ord for Variant`

- <span id="variant-ord-cmp"></span>`fn cmp(&self, other: &Variant) -> cmp::Ordering` — [`Variant`](guid/index.md#variant)

##### `impl PartialEq for Variant`

- <span id="variant-partialeq-eq"></span>`fn eq(&self, other: &Variant) -> bool` — [`Variant`](guid/index.md#variant)

##### `impl PartialOrd for Variant`

- <span id="variant-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Variant) -> option::Option<cmp::Ordering>` — [`Variant`](guid/index.md#variant)

##### `impl StructuralPartialEq for Variant`

## Macros

### `mtry!`

Macro replacement for the `?` operator, which cannot be used in
const functions.

### `guid!`

Create a [`Guid`](guid/index.md) from a string at compile time.

# Examples

```rust
use uguid::{guid, Guid};
assert_eq!(
    guid!("01234567-89ab-cdef-0123-456789abcdef"),
    Guid::new(
        [0x67, 0x45, 0x23, 0x01],
        [0xab, 0x89],
        [0xef, 0xcd],
        0x01,
        0x23,
        [0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
    )
);
```

