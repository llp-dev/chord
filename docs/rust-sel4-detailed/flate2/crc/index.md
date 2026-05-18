*[flate2](../index.md) / [crc](index.md)*

---

# Module `crc`

Simple CRC bindings backed by miniz.c

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impl_crc32fast`](#impl-crc32fast) | mod |  |
| [`Crc`](#crc) | struct |  |
| [`CrcReader`](#crcreader) | struct | A wrapper around a [`Read`] that calculates the CRC. |
| [`CrcWriter`](#crcwriter) | struct | A wrapper around a [`Write`] that calculates the CRC. |

## Modules

- [`impl_crc32fast`](impl_crc32fast/index.md)

## Structs

### `Crc`

```rust
struct Crc {
    amt: u32,
    hasher: crc32fast::Hasher,
}
```

The CRC calculated by a [`CrcReader`](#crcreader).


#### Implementations

- <span id="crc-new"></span>`fn new() -> Self`

  Create a new CRC.

- <span id="crc-sum"></span>`fn sum(&self) -> u32`

  Returns the current crc32 checksum.

- <span id="crc-amount"></span>`fn amount(&self) -> u32`

  The number of bytes that have been used to calculate the CRC.

  This value is only accurate if the amount is lower than 2<sup>32</sup>.

- <span id="crc-update"></span>`fn update(&mut self, data: &[u8])`

  Update the CRC with the bytes in `data`.

- <span id="crc-reset"></span>`fn reset(&mut self)`

  Reset the CRC.

- <span id="crc-combine"></span>`fn combine(&mut self, additional_crc: &Self)`

  Combine the CRC with the CRC for the subsequent block of bytes.

#### Trait Implementations

##### `impl Debug for Crc`

- <span id="crc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Crc`

- <span id="crc-default"></span>`fn default() -> Crc` — [`Crc`](impl_crc32fast/index.md#crc)

### `CrcReader<R>`

```rust
struct CrcReader<R> {
    inner: R,
    crc: Crc,
}
```

A wrapper around a `Read` that calculates the CRC.


#### Implementations

- <span id="crcreader-new"></span>`fn new(r: R) -> CrcReader<R>` — [`CrcReader`](#crcreader)

  Create a new `CrcReader`.

#### Trait Implementations

##### `impl<R: BufRead> BufRead for CrcReader<R>`

- <span id="crcreader-bufread-fill-buf"></span>`fn fill_buf(&mut self) -> io::Result<&[u8]>`

- <span id="crcreader-bufread-consume"></span>`fn consume(&mut self, amt: usize)`

##### `impl<R: fmt::Debug> Debug for CrcReader<R>`

- <span id="crcreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Read> Read for CrcReader<R>`

- <span id="crcreader-read"></span>`fn read(&mut self, into: &mut [u8]) -> io::Result<usize>`

### `CrcWriter<W>`

```rust
struct CrcWriter<W> {
    inner: W,
    crc: Crc,
}
```

A wrapper around a [`Write`]() that calculates the CRC.


#### Implementations

- <span id="crcwriter-crc"></span>`fn crc(&self) -> &Crc` — [`Crc`](impl_crc32fast/index.md#crc)

  Get the Crc for this `CrcWriter`.

- <span id="crcwriter-into-inner"></span>`fn into_inner(self) -> W`

  Get the writer that is wrapped by this `CrcWriter`.

- <span id="crcwriter-get-ref"></span>`fn get_ref(&self) -> &W`

  Get the writer that is wrapped by this `CrcWriter` by reference.

- <span id="crcwriter-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

  Get a mutable reference to the writer that is wrapped by this `CrcWriter`.

- <span id="crcwriter-reset"></span>`fn reset(&mut self)`

  Reset the Crc in this `CrcWriter`.

#### Trait Implementations

##### `impl<W: fmt::Debug> Debug for CrcWriter<W>`

- <span id="crcwriter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Write> Write for CrcWriter<W>`

- <span id="crcwriter-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="crcwriter-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

