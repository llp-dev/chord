*[untrusted](../index.md) / [reader](index.md)*

---

# Module `reader`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Reader`](#reader) | struct | A read-only, forward-only cursor into the data in an `Input`. |
| [`EndOfInput`](#endofinput) | struct | The error type used to indicate the end of the input was reached before the operation could be completed. |

## Structs

### `Reader<'a>`

```rust
struct Reader<'a> {
    input: no_panic::Slice<'a>,
    i: usize,
}
```

A read-only, forward-only cursor into the data in an `Input`.

Using `Reader` to parse input helps to ensure that no byte of the input
will be accidentally processed more than once. Using `Reader` in
conjunction with `read_all` and `read_all_optional` helps ensure that no
byte of the input is accidentally left unprocessed. The methods of `Reader`
never panic, so `Reader` also assists the writing of panic-free code.

Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit
non-constant-time comparisons.

#### Implementations

- <span id="reader-new"></span>`fn new(input: Input<'a>) -> Self` — [`Input`](../input/index.md#input)

  Construct a new Reader for the given input. Use `read_all` or

  `read_all_optional` instead of `Reader::new` whenever possible.

- <span id="reader-at-end"></span>`fn at_end(&self) -> bool`

  Returns `true` if the reader is at the end of the input, and `false`

  otherwise.

- <span id="reader-peek"></span>`fn peek(&self, b: u8) -> bool`

  Returns `true` if there is at least one more byte in the input and that

  byte is equal to `b`, and false otherwise.

- <span id="reader-read-byte"></span>`fn read_byte(&mut self) -> Result<u8, EndOfInput>` — [`EndOfInput`](#endofinput)

  Reads the next input byte.

  

  Returns `Ok(b)` where `b` is the next input byte, or `Err(EndOfInput)`

  if the `Reader` is at the end of the input.

- <span id="reader-read-bytes"></span>`fn read_bytes(&mut self, num_bytes: usize) -> Result<Input<'a>, EndOfInput>` — [`Input`](../input/index.md#input), [`EndOfInput`](#endofinput)

  Skips `num_bytes` of the input, returning the skipped input as an

  `Input`.

  

  Returns `Ok(i)` if there are at least `num_bytes` of input remaining,

  and `Err(EndOfInput)` otherwise.

- <span id="reader-read-bytes-to-end"></span>`fn read_bytes_to_end(&mut self) -> Input<'a>` — [`Input`](../input/index.md#input)

  Skips the reader to the end of the input, returning the skipped input

  as an `Input`.

- <span id="reader-read-partial"></span>`fn read_partial<F, R, E>(&mut self, read: F) -> Result<(Input<'a>, R), E>` — [`Input`](../input/index.md#input)

  Calls `read()` with the given input as a `Reader`. On success, returns a

  pair `(bytes_read, r)` where `bytes_read` is what `read()` consumed and

  `r` is `read()`'s return value.

- <span id="reader-skip"></span>`fn skip(&mut self, num_bytes: usize) -> Result<(), EndOfInput>` — [`EndOfInput`](#endofinput)

  Skips `num_bytes` of the input.

  

  Returns `Ok(i)` if there are at least `num_bytes` of input remaining,

  and `Err(EndOfInput)` otherwise.

- <span id="reader-skip-to-end"></span>`fn skip_to_end(&mut self)`

  Skips the reader to the end of the input.

#### Trait Implementations

##### `impl Debug for Reader<'_>`

- <span id="reader-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `EndOfInput`

```rust
struct EndOfInput;
```

The error type used to indicate the end of the input was reached before the
operation could be completed.

#### Trait Implementations

##### `impl Clone for EndOfInput`

- <span id="endofinput-clone"></span>`fn clone(&self) -> EndOfInput` — [`EndOfInput`](#endofinput)

##### `impl Copy for EndOfInput`

##### `impl Debug for EndOfInput`

- <span id="endofinput-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EndOfInput`

##### `impl PartialEq for EndOfInput`

- <span id="endofinput-partialeq-eq"></span>`fn eq(&self, other: &EndOfInput) -> bool` — [`EndOfInput`](#endofinput)

##### `impl StructuralPartialEq for EndOfInput`

