*[base64ct](../index.md) / [encoder](index.md)*

---

# Module `encoder`

Buffered Base64 encoder.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Encoder`](#encoder) | struct | Stateful Base64 encoder with support for buffered, incremental encoding. |
| [`BlockBuffer`](#blockbuffer) | struct | Base64 encode buffer for a 1-block output. |
| [`LineWrapper`](#linewrapper) | struct | Helper for wrapping Base64 at a given line width. |

## Structs

### `Encoder<'o, E: Encoding>`

```rust
struct Encoder<'o, E: Encoding> {
    output: &'o mut [u8],
    position: usize,
    block_buffer: BlockBuffer,
    line_wrapper: Option<LineWrapper>,
    encoding: core::marker::PhantomData<E>,
}
```

Stateful Base64 encoder with support for buffered, incremental encoding.

The `E` type parameter can be any type which impls [`Encoding`](../encoding/index.md) such as
[`Base64`](../alphabet/standard/index.md) or [`Base64Unpadded`](../alphabet/standard/index.md).

#### Fields

- **`output`**: `&'o mut [u8]`

  Output buffer.

- **`position`**: `usize`

  Cursor within the output buffer.

- **`block_buffer`**: `BlockBuffer`

  Block buffer used for non-block-aligned data.

- **`line_wrapper`**: `Option<LineWrapper>`

  Configuration and state for line-wrapping the output at a specified
  column.

- **`encoding`**: `core::marker::PhantomData<E>`

  Phantom parameter for the Base64 encoding in use.

#### Implementations

- <span id="encoder-new"></span>`fn new(output: &'o mut [u8]) -> Result<Self, Error>` — [`Error`](../errors/index.md#error)

  Create a new encoder which writes output to the given byte slice.

  

  Output constructed using this method is not line-wrapped.

- <span id="encoder-new-wrapped"></span>`fn new_wrapped(output: &'o mut [u8], width: usize, ending: LineEnding) -> Result<Self, Error>` — [`LineEnding`](../line_ending/index.md#lineending), [`Error`](../errors/index.md#error)

  Create a new encoder which writes line-wrapped output to the given byte

  slice.

  

  Output will be wrapped at the specified interval, using the provided

  line ending. Use `LineEnding::default()` to use the conventional line

  ending for the target OS.

  

  Minimum allowed line width is 4.

- <span id="encoder-encode"></span>`fn encode(&mut self, input: &[u8]) -> Result<(), Error>` — [`Error`](../errors/index.md#error)

  Encode the provided buffer as Base64, writing it to the output buffer.

  

  # Returns

  - `Ok(())` if the expected amount of data was read

  - `Err(Error::InvalidLength)` if there is insufficient space in the output buffer

- <span id="encoder-position"></span>`fn position(&self) -> usize`

  Get the position inside of the output buffer where the write cursor

  is currently located.

- <span id="encoder-finish"></span>`fn finish(self) -> Result<&'o str, Error>` — [`Error`](../errors/index.md#error)

  Finish encoding data, returning the resulting Base64 as a `str`.

- <span id="encoder-finish-with-remaining"></span>`fn finish_with_remaining(self) -> Result<(&'o str, &'o mut [u8]), Error>` — [`Error`](../errors/index.md#error)

  Finish encoding data, returning the resulting Base64 as a `str`

  along with the remaining space in the output buffer.

- <span id="encoder-remaining"></span>`fn remaining(&mut self) -> &mut [u8]`

  Borrow the remaining data in the buffer.

- <span id="encoder-process-buffer"></span>`fn process_buffer(&mut self, input: &mut &[u8]) -> Result<(), Error>` — [`Error`](../errors/index.md#error)

  Fill the block buffer with data, consuming and encoding it when the

  buffer is full.

- <span id="encoder-perform-encode"></span>`fn perform_encode(&mut self, input: &[u8]) -> Result<usize, Error>` — [`Error`](../errors/index.md#error)

  Perform Base64 encoding operation.

### `BlockBuffer`

```rust
struct BlockBuffer {
    bytes: [u8; 3],
    position: usize,
}
```

Base64 encode buffer for a 1-block output.

This handles a partial block of data, i.e. data which hasn't been

#### Fields

- **`bytes`**: `[u8; 3]`

  3 decoded bytes to be encoded to a 4-byte Base64-encoded input.

- **`position`**: `usize`

  Position within the buffer.

#### Implementations

- <span id="blockbuffer-const-size"></span>`const SIZE: usize`

- <span id="blockbuffer-fill"></span>`fn fill(&mut self, input: &mut &[u8]) -> Result<(), Error>` — [`Error`](../errors/index.md#error)

  Fill the remaining space in the buffer with the input data.

- <span id="blockbuffer-take"></span>`fn take(&mut self) -> [u8; 3]`

  Take the output buffer, resetting the position to 0.

- <span id="blockbuffer-is-empty"></span>`fn is_empty(&self) -> bool`

  Is the buffer empty?

- <span id="blockbuffer-is-full"></span>`fn is_full(&self) -> bool`

  Is the buffer full?

#### Trait Implementations

##### `impl Clone for BlockBuffer`

- <span id="blockbuffer-clone"></span>`fn clone(&self) -> BlockBuffer` — [`BlockBuffer`](#blockbuffer)

##### `impl Debug for BlockBuffer`

- <span id="blockbuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BlockBuffer`

- <span id="blockbuffer-default"></span>`fn default() -> BlockBuffer` — [`BlockBuffer`](#blockbuffer)

### `LineWrapper`

```rust
struct LineWrapper {
    remaining: usize,
    width: usize,
    ending: crate::LineEnding,
}
```

Helper for wrapping Base64 at a given line width.

#### Fields

- **`remaining`**: `usize`

  Number of bytes remaining in the current line.

- **`width`**: `usize`

  Column at which Base64 should be wrapped.

- **`ending`**: `crate::LineEnding`

  Newline characters to use at the end of each line.

#### Implementations

- <span id="linewrapper-new"></span>`fn new(width: usize, ending: LineEnding) -> Result<Self, Error>` — [`LineEnding`](../line_ending/index.md#lineending), [`Error`](../errors/index.md#error)

  Create a new linewrapper.

- <span id="linewrapper-wrap-blocks"></span>`fn wrap_blocks(&self, blocks: &mut usize) -> Result<(), Error>` — [`Error`](../errors/index.md#error)

  Wrap the number of blocks to encode near/at EOL.

- <span id="linewrapper-insert-newlines"></span>`fn insert_newlines(&mut self, buffer: &mut [u8], len: &mut usize) -> Result<(), Error>` — [`Error`](../errors/index.md#error)

  Insert newlines into the output buffer as needed.

#### Trait Implementations

##### `impl Debug for LineWrapper`

- <span id="linewrapper-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

