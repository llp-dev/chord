*[ruzstd](../../index.md) / [decoding](../index.md) / [decode_buffer](index.md)*

---

# Module `decode_buffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DecodeBuffer`](#decodebuffer) | struct |  |
| [`write_all_bytes`](#write-all-bytes) | fn | Like Write::write_all but returns partial write length even on error |

## Structs

### `DecodeBuffer`

```rust
struct DecodeBuffer {
    buffer: super::ringbuffer::RingBuffer,
    pub dict_content: alloc::vec::Vec<u8>,
    pub window_size: usize,
    total_output_counter: u64,
    pub hash: twox_hash::XxHash64,
}
```

#### Implementations

- <span id="decodebuffer-new"></span>`fn new(window_size: usize) -> DecodeBuffer` — [`DecodeBuffer`](#decodebuffer)

- <span id="decodebuffer-reset"></span>`fn reset(&mut self, window_size: usize)`

- <span id="decodebuffer-len"></span>`fn len(&self) -> usize`

- <span id="decodebuffer-push"></span>`fn push(&mut self, data: &[u8])`

- <span id="decodebuffer-repeat"></span>`fn repeat(&mut self, offset: usize, match_length: usize) -> Result<(), DecodeBufferError>` — [`DecodeBufferError`](../errors/index.md#decodebuffererror)

- <span id="decodebuffer-repeat-in-chunks"></span>`fn repeat_in_chunks(&mut self, offset: usize, match_length: usize, start_idx: usize)`

- <span id="decodebuffer-repeat-from-dict"></span>`fn repeat_from_dict(&mut self, offset: usize, match_length: usize) -> Result<(), DecodeBufferError>` — [`DecodeBufferError`](../errors/index.md#decodebuffererror)

- <span id="decodebuffer-can-drain-to-window-size"></span>`fn can_drain_to_window_size(&self) -> Option<usize>`

  Check if and how many bytes can currently be drawn from the buffer

- <span id="decodebuffer-can-drain"></span>`fn can_drain(&self) -> usize`

- <span id="decodebuffer-drain-to-window-size"></span>`fn drain_to_window_size(&mut self) -> Option<Vec<u8>>`

  Drain as much as possible while retaining enough so that decoding si still possible with the required window_size

  At best call only if can_drain_to_window_size reports a 'high' number of bytes to reduce allocations

- <span id="decodebuffer-drain-to-window-size-writer"></span>`fn drain_to_window_size_writer(&mut self, sink: impl Write) -> Result<usize, Error>` — [`Write`](../../io_std/index.md#write), [`Error`](../../io_std/index.md#error)

- <span id="decodebuffer-drain"></span>`fn drain(&mut self) -> Vec<u8>`

  drain the buffer completely

- <span id="decodebuffer-drain-to-writer"></span>`fn drain_to_writer(&mut self, sink: impl Write) -> Result<usize, Error>` — [`Write`](../../io_std/index.md#write), [`Error`](../../io_std/index.md#error)

- <span id="decodebuffer-read-all"></span>`fn read_all(&mut self, target: &mut [u8]) -> Result<usize, Error>` — [`Error`](../../io_std/index.md#error)

- <span id="decodebuffer-drain-to"></span>`fn drain_to(&mut self, amount: usize, write_bytes: impl FnMut(&[u8]) -> (usize, Result<(), Error>)) -> Result<usize, Error>` — [`Error`](../../io_std/index.md#error)

  Semantics of write_bytes:

  Should dump as many of the provided bytes as possible to whatever sink until no bytes are left or an error is encountered

  Return how many bytes have actually been dumped to the sink.

#### Trait Implementations

##### `impl Read for DecodeBuffer`

- <span id="decodebuffer-read"></span>`fn read(&mut self, target: &mut [u8]) -> Result<usize, Error>` — [`Error`](../../io_std/index.md#error)

## Functions

### `write_all_bytes`

```rust
fn write_all_bytes(sink: impl Write, buf: &[u8]) -> (usize, Result<(), crate::io::Error>)
```

Like Write::write_all but returns partial write length even on error

