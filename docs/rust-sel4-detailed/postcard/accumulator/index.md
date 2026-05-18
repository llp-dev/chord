*[postcard](../index.md) / [accumulator](index.md)*

---

# Module `accumulator`

An accumulator used to collect chunked COBS data and deserialize it.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CobsAccumulator`](#cobsaccumulator) | struct | An accumulator used to collect chunked COBS data and deserialize it. |
| [`FeedResult`](#feedresult) | enum | The result of feeding the accumulator. |

## Structs

### `CobsAccumulator<const N: usize>`

```rust
struct CobsAccumulator<const N: usize> {
    buf: [u8; N],
    idx: usize,
}
```

An accumulator used to collect chunked COBS data and deserialize it.

This is often useful when you receive "parts" of the message at a time, for example when draining
a serial port buffer that may not contain an entire uninterrupted message.

# Examples

Deserialize a struct by reading chunks from a [`Read`]()er.

```rust
use postcard::accumulator::{CobsAccumulator, FeedResult};
use serde::Deserialize;
use std::io::Read;

let mut input_buf = [0u8; 256];
#[derive(serde::Serialize, Deserialize, Debug, PartialEq, Eq)]
struct MyData {
    a: u32,
    b: bool,
    c: [u8; 16],
}
let input = /* Anything that implements the `Read` trait */
postcard::to_slice_cobs(&MyData {
    a: 0xabcdef00,
    b: true,
    c: [0xab; 16],
}, &mut input_buf).unwrap();
let mut input = &input[..];

let mut raw_buf = [0u8; 32];
let mut cobs_buf: CobsAccumulator<256> = CobsAccumulator::new();

while let Ok(ct) = input.read(&mut raw_buf) {
    // Finished reading input
    if ct == 0 {
        break;
    }

    let buf = &raw_buf[..ct];
    let mut window = &buf[..];

    'cobs: while !window.is_empty() {
        window = match cobs_buf.feed::<MyData>(&window) {
            FeedResult::Consumed => break 'cobs,
            FeedResult::OverFull(new_wind) => new_wind,
            FeedResult::DeserError(new_wind) => new_wind,
            FeedResult::Success { data, remaining } => {
                // Do something with `data: MyData` here.

                dbg!(data);

                remaining
            }
        };
    }
}
```


#### Implementations

- <span id="cobsaccumulator-new"></span>`const fn new() -> Self`

  Create a new accumulator.

- <span id="cobsaccumulator-feed"></span>`fn feed<'a, T>(&mut self, input: &'a [u8]) -> FeedResult<'a, T>` — [`FeedResult`](#feedresult)

  Appends data to the internal buffer and attempts to deserialize the accumulated data into

  `T`.

- <span id="cobsaccumulator-feed-ref"></span>`fn feed_ref<'de, 'a, T>(self: &'de mut Self, input: &'a [u8]) -> FeedResult<'a, T>` — [`FeedResult`](#feedresult)

  Appends data to the internal buffer and attempts to deserialize the accumulated data into

  `T`.

  

  This differs from feed, as it allows the `T` to reference data within the internal buffer, but

  mutably borrows the accumulator for the lifetime of the deserialization.

  If `T` does not require the reference, the borrow of `self` ends at the end of the function.

- <span id="cobsaccumulator-extend-unchecked"></span>`fn extend_unchecked(&mut self, input: &[u8])`

  Extend the internal buffer with the given input.

  

  # Panics

  

  Will panic if the input does not fit in the internal buffer.

#### Trait Implementations

##### `impl Default for CobsAccumulator<N>`

- <span id="cobsaccumulator-default"></span>`fn default() -> Self`

## Enums

### `FeedResult<'a, T>`

```rust
enum FeedResult<'a, T> {
    Consumed,
    OverFull(&'a [u8]),
    DeserError(&'a [u8]),
    Success {
        data: T,
        remaining: &'a [u8],
    },
}
```

The result of feeding the accumulator.

#### Variants

- **`Consumed`**

  Consumed all data, still pending.

- **`OverFull`**

  Buffer was filled. Contains remaining section of input, if any.

- **`DeserError`**

  Reached end of chunk, but deserialization failed. Contains remaining section of input, if any.

- **`Success`**

  Deserialization complete. Contains deserialized data and remaining section of input, if any.

