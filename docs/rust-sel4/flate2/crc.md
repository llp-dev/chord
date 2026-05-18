**flate2 > crc**

# Module: crc

## Contents

**Structs**

- [`CrcReader`](#crcreader) - A wrapper around a [`Read`] that calculates the CRC.
- [`CrcWriter`](#crcwriter) - A wrapper around a [`Write`] that calculates the CRC.

---

## flate2::crc::CrcReader

*Struct*

A wrapper around a [`Read`] that calculates the CRC.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html

**Generic Parameters:**
- R

**Methods:**

- `fn crc(self: &Self) -> &Crc` - Get the Crc for this `CrcReader`.
- `fn into_inner(self: Self) -> R` - Get the reader that is wrapped by this `CrcReader`.
- `fn get_ref(self: &Self) -> &R` - Get the reader that is wrapped by this `CrcReader` by reference.
- `fn get_mut(self: & mut Self) -> & mut R` - Get a mutable reference to the reader that is wrapped by this `CrcReader`.
- `fn reset(self: & mut Self)` - Reset the Crc in this `CrcReader`.
- `fn new(r: R) -> CrcReader<R>` - Create a new `CrcReader`.

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, into: & mut [u8]) -> io::Result<usize>`
- **BufRead**
  - `fn fill_buf(self: & mut Self) -> io::Result<&[u8]>`
  - `fn consume(self: & mut Self, amt: usize)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## flate2::crc::CrcWriter

*Struct*

A wrapper around a [`Write`] that calculates the CRC.

[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> CrcWriter<W>` - Create a new `CrcWriter`.
- `fn crc(self: &Self) -> &Crc` - Get the Crc for this `CrcWriter`.
- `fn into_inner(self: Self) -> W` - Get the writer that is wrapped by this `CrcWriter`.
- `fn get_ref(self: &Self) -> &W` - Get the writer that is wrapped by this `CrcWriter` by reference.
- `fn get_mut(self: & mut Self) -> & mut W` - Get a mutable reference to the writer that is wrapped by this `CrcWriter`.
- `fn reset(self: & mut Self)` - Reset the Crc in this `CrcWriter`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`



