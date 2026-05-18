**flate2 > crc > impl_crc32fast**

# Module: crc::impl_crc32fast

## Contents

**Structs**

- [`Crc`](#crc) - The CRC calculated by a [`CrcReader`].

---

## flate2::crc::impl_crc32fast::Crc

*Struct*

The CRC calculated by a [`CrcReader`].

[`CrcReader`]: struct.CrcReader.html

**Methods:**

- `fn new() -> Self` - Create a new CRC.
- `fn sum(self: &Self) -> u32` - Returns the current crc32 checksum.
- `fn amount(self: &Self) -> u32` - The number of bytes that have been used to calculate the CRC.
- `fn update(self: & mut Self, data: &[u8])` - Update the CRC with the bytes in `data`.
- `fn reset(self: & mut Self)` - Reset the CRC.
- `fn combine(self: & mut Self, additional_crc: &Self)` - Combine the CRC with the CRC for the subsequent block of bytes.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Crc`



