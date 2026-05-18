*[base64ct](../index.md) / [decoder](index.md)*

---

# Module `decoder`

Buffered Base64 decoder.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Decoder`](#decoder) | struct | Stateful Base64 decoder with support for buffered, incremental decoding. |
| [`BlockBuffer`](#blockbuffer) | struct | Base64 decode buffer for a 1-block input. |
| [`Line`](#line) | struct | A single line of linewrapped data, providing a read buffer. |
| [`LineReader`](#linereader) | struct | Iterator over multi-line Base64 input. |

## Structs

### `Decoder<'i, E: Encoding>`

```rust
struct Decoder<'i, E: Encoding> {
    line: Line<'i>,
    line_reader: LineReader<'i>,
    remaining_len: usize,
    block_buffer: BlockBuffer,
    encoding: core::marker::PhantomData<E>,
}
```

Stateful Base64 decoder with support for buffered, incremental decoding.

The `E` type parameter can be any type which impls [`Encoding`](../encoding/index.md) such as
[`Base64`](../alphabet/standard/index.md) or [`Base64Unpadded`](../alphabet/standard/index.md).

#### Fields

- **`line`**: `Line<'i>`

  Current line being processed.

- **`line_reader`**: `LineReader<'i>`

  Base64 input data reader.

- **`remaining_len`**: `usize`

  Length of the remaining data after Base64 decoding.

- **`block_buffer`**: `BlockBuffer`

  Block buffer used for non-block-aligned data.

- **`encoding`**: `core::marker::PhantomData<E>`

  Phantom parameter for the Base64 encoding in use.

#### Implementations

- <span id="decoder-new"></span>`fn new(input: &'i [u8]) -> Result<Self, Error>` — [`Error`](../errors/index.md#error)

  Create a new decoder for a byte slice containing contiguous

  (non-newline-delimited) Base64-encoded data.

  

  # Returns

  - `Ok(decoder)` on success.

  - `Err(Error::InvalidLength)` if the input buffer is empty.

- <span id="decoder-new-wrapped"></span>`fn new_wrapped(input: &'i [u8], line_width: usize) -> Result<Self, Error>` — [`Error`](../errors/index.md#error)

  Create a new decoder for a byte slice containing Base64 which

  line wraps at the given line length.

  

  Trailing newlines are not supported and must be removed in advance.

  

  Newlines are handled according to what are roughly [RFC7468] conventions:

  

  ```text

  [parsers] MUST handle different newline conventions

  ```

  

  RFC7468 allows any of the following as newlines, and allows a mixture

  of different types of newlines:

  

  ```text

  eol        = CRLF / CR / LF

  ```

  

  # Returns

  - `Ok(decoder)` on success.

  - `Err(Error::InvalidLength)` if the input buffer is empty or the line

    width is zero.

- <span id="decoder-decode"></span>`fn decode<'o>(&mut self, out: &'o mut [u8]) -> Result<&'o [u8], Error>` — [`Error`](../errors/index.md#error)

  Fill the provided buffer with data decoded from Base64.

  

  Enough Base64 input data must remain to fill the entire buffer.

  

  # Returns

  - `Ok(bytes)` if the expected amount of data was read

  - `Err(Error::InvalidLength)` if the exact amount of data couldn't be read

- <span id="decoder-decode-to-end"></span>`fn decode_to_end<'o>(&mut self, buf: &'o mut Vec<u8>) -> Result<&'o [u8], Error>` — [`Error`](../errors/index.md#error)

  Decode all remaining Base64 data, placing the result into `buf`.

  

  If successful, this function will return the total number of bytes

  decoded into `buf`.

- <span id="decoder-remaining-len"></span>`fn remaining_len(&self) -> usize`

  Get the length of the remaining data after Base64 decoding.

  

  Decreases every time data is decoded.

- <span id="decoder-is-finished"></span>`fn is_finished(&self) -> bool`

  Has all of the input data been decoded?

- <span id="decoder-fill-block-buffer"></span>`fn fill_block_buffer(&mut self) -> Result<(), Error>` — [`Error`](../errors/index.md#error)

  Fill the block buffer with data.

- <span id="decoder-advance-line"></span>`fn advance_line(&mut self) -> Result<(), Error>` — [`Error`](../errors/index.md#error)

  Advance the internal buffer to the next line.

- <span id="decoder-perform-decode"></span>`fn perform_decode<'o>(&self, src: &[u8], dst: &'o mut [u8]) -> Result<&'o [u8], Error>` — [`Error`](../errors/index.md#error)

  Perform Base64 decoding operation.

#### Trait Implementations

##### `impl<E: clone::Clone + Encoding> Clone for Decoder<'i, E>`

- <span id="decoder-clone"></span>`fn clone(&self) -> Decoder<'i, E>` — [`Decoder`](#decoder)

### `BlockBuffer`

```rust
struct BlockBuffer {
    decoded: [u8; 3],
    length: usize,
    position: usize,
}
```

Base64 decode buffer for a 1-block input.

This handles a partially decoded block of data, i.e. data which has been
decoded but not read.

#### Fields

- **`decoded`**: `[u8; 3]`

  3 decoded bytes from a 4-byte Base64-encoded input.

- **`length`**: `usize`

  Length of the buffer.

- **`position`**: `usize`

  Position within the buffer.

#### Implementations

- <span id="blockbuffer-const-size"></span>`const SIZE: usize`

- <span id="blockbuffer-fill"></span>`fn fill(&mut self, decoded_input: &[u8]) -> Result<(), Error>` — [`Error`](../errors/index.md#error)

  Fill the buffer by decoding up to 3 bytes of decoded Base64 input.

- <span id="blockbuffer-take"></span>`fn take(&mut self, nbytes: usize) -> Result<&[u8], Error>` — [`Error`](../errors/index.md#error)

  Take a specified number of bytes from the buffer.

  

  Returns as many bytes as possible, or an empty slice if the buffer has

  already been read to completion.

- <span id="blockbuffer-is-empty"></span>`fn is_empty(&self) -> bool`

  Have all of the bytes in this buffer been consumed?

#### Trait Implementations

##### `impl Clone for BlockBuffer`

- <span id="blockbuffer-clone"></span>`fn clone(&self) -> BlockBuffer` — [`BlockBuffer`](#blockbuffer)

##### `impl Debug for BlockBuffer`

- <span id="blockbuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BlockBuffer`

- <span id="blockbuffer-default"></span>`fn default() -> BlockBuffer` — [`BlockBuffer`](#blockbuffer)

### `Line<'i>`

```rust
struct Line<'i> {
    remaining: &'i [u8],
}
```

A single line of linewrapped data, providing a read buffer.

#### Fields

- **`remaining`**: `&'i [u8]`

  Remaining data in the line

#### Implementations

- <span id="line-new"></span>`fn new(bytes: &'i [u8]) -> Self`

  Create a new line which wraps the given input data.

- <span id="line-take"></span>`fn take(&mut self, nbytes: usize) -> &'i [u8]`

  Take up to `nbytes` from this line buffer.

- <span id="line-slice-tail"></span>`fn slice_tail(&self, nbytes: usize) -> Result<&'i [u8], Error>` — [`Error`](../errors/index.md#error)

  Slice off a tail of a given length.

- <span id="line-len"></span>`fn len(&self) -> usize`

  Get the number of bytes remaining in this line.

- <span id="line-is-empty"></span>`fn is_empty(&self) -> bool`

  Is the buffer for this line empty?

- <span id="line-trim-end"></span>`fn trim_end(&self) -> Self`

  Trim the newline off the end of this line.

#### Trait Implementations

##### `impl Clone for Line<'i>`

- <span id="line-clone"></span>`fn clone(&self) -> Line<'i>` — [`Line`](#line)

##### `impl Debug for Line<'i>`

- <span id="line-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Line<'_>`

- <span id="line-default"></span>`fn default() -> Self`

### `LineReader<'i>`

```rust
struct LineReader<'i> {
    remaining: &'i [u8],
    line_width: Option<usize>,
}
```

Iterator over multi-line Base64 input.

#### Fields

- **`remaining`**: `&'i [u8]`

  Remaining linewrapped data to be processed.

- **`line_width`**: `Option<usize>`

  Line width.

#### Implementations

- <span id="linereader-new-unwrapped"></span>`fn new_unwrapped(bytes: &'i [u8]) -> Result<Self, Error>` — [`Error`](../errors/index.md#error)

  Create a new reader which operates over continugous unwrapped data.

- <span id="linereader-new-wrapped"></span>`fn new_wrapped(bytes: &'i [u8], line_width: usize) -> Result<Self, Error>` — [`Error`](../errors/index.md#error)

  Create a new reader which operates over linewrapped data.

- <span id="linereader-is-empty"></span>`fn is_empty(&self) -> bool`

  Is this line reader empty?

- <span id="linereader-decoded-len"></span>`fn decoded_len<E: Encoding>(&self) -> Result<usize, Error>` — [`Error`](../errors/index.md#error)

  Get the total length of the data decoded from this line reader.

#### Trait Implementations

##### `impl Clone for LineReader<'i>`

- <span id="linereader-clone"></span>`fn clone(&self) -> LineReader<'i>` — [`LineReader`](#linereader)

##### `impl IntoIterator for LineReader<'i>`

- <span id="linereader-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="linereader-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="linereader-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LineReader<'i>`

- <span id="linereader-iterator-type-item"></span>`type Item = Result<Line<'i>, Error>`

- <span id="linereader-iterator-next"></span>`fn next(&mut self) -> Option<Result<Line<'i>, Error>>` — [`Line`](#line), [`Error`](../errors/index.md#error)

