*[uguid](../index.md) / [guid](index.md)*

---

# Module `guid`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Guid`](#guid) | struct | Globally-unique identifier. |
| [`Variant`](#variant) | enum | Variant or type of GUID, as defined in [RFC4122]. |

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

- <span id="guid-variant"></span>`const fn variant(self) -> Variant` — [`Variant`](#variant)

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

- <span id="guid-try-parse"></span>`const fn try_parse(s: &str) -> Result<Self, GuidFromStrError>` — [`GuidFromStrError`](../error/index.md#guidfromstrerror)

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

- <span id="guid-clone"></span>`fn clone(&self) -> Guid` — [`Guid`](#guid)

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

- <span id="guid-ord-cmp"></span>`fn cmp(&self, other: &Guid) -> cmp::Ordering` — [`Guid`](#guid)

##### `impl PartialEq for Guid`

- <span id="guid-partialeq-eq"></span>`fn eq(&self, other: &Guid) -> bool` — [`Guid`](#guid)

##### `impl PartialOrd for Guid`

- <span id="guid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Guid) -> option::Option<cmp::Ordering>` — [`Guid`](#guid)

##### `impl Pod for Guid`

##### `impl StructuralPartialEq for Guid`

##### `impl Zeroable for Guid`

## Enums

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

- <span id="variant-clone"></span>`fn clone(&self) -> Variant` — [`Variant`](#variant)

##### `impl Copy for Variant`

##### `impl Debug for Variant`

- <span id="variant-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Variant`

##### `impl Ord for Variant`

- <span id="variant-ord-cmp"></span>`fn cmp(&self, other: &Variant) -> cmp::Ordering` — [`Variant`](#variant)

##### `impl PartialEq for Variant`

- <span id="variant-partialeq-eq"></span>`fn eq(&self, other: &Variant) -> bool` — [`Variant`](#variant)

##### `impl PartialOrd for Variant`

- <span id="variant-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Variant) -> option::Option<cmp::Ordering>` — [`Variant`](#variant)

##### `impl StructuralPartialEq for Variant`

