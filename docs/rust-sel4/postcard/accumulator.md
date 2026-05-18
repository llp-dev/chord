**postcard > accumulator**

# Module: accumulator

## Contents

**Structs**

- [`CobsAccumulator`](#cobsaccumulator) - An accumulator used to collect chunked COBS data and deserialize it.

**Enums**

- [`FeedResult`](#feedresult) - The result of feeding the accumulator.

---

## postcard::accumulator::CobsAccumulator

*Struct*

An accumulator used to collect chunked COBS data and deserialize it.

This is often useful when you receive "parts" of the message at a time, for example when draining
a serial port buffer that may not contain an entire uninterrupted message.

# Examples

Deserialize a struct by reading chunks from a [`Read`]er.

```rust
use postcard::accumulator::{CobsAccumulator, FeedResult};
use serde::Deserialize;
use std::io::Read;

# let mut input_buf = [0u8; 256];
# #[derive(serde::Serialize, Deserialize, Debug, PartialEq, Eq)]
# struct MyData {
#     a: u32,
#     b: bool,
#     c: [u8; 16],
# }
let input = /* Anything that implements the `Read` trait */
# postcard::to_slice_cobs(&MyData {
#     a: 0xabcdef00,
#     b: true,
#     c: [0xab; 16],
# }, &mut input_buf).unwrap();
# let mut input = &input[..];

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

[`Read`]: std::io::Read

**Generic Parameters:**
- const N

**Methods:**

- `fn new() -> Self` - Create a new accumulator.
- `fn feed<'a, T>(self: & mut Self, input: &'a [u8]) -> FeedResult<'a, T>` - Appends data to the internal buffer and attempts to deserialize the accumulated data into
- `fn feed_ref<'de, 'a, T>(self: &'de  mut Self, input: &'a [u8]) -> FeedResult<'a, T>` - Appends data to the internal buffer and attempts to deserialize the accumulated data into

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## postcard::accumulator::FeedResult

*Enum*

The result of feeding the accumulator.

**Generic Parameters:**
- 'a
- T

**Variants:**
- `Consumed` - Consumed all data, still pending.
- `OverFull(&'a [u8])` - Buffer was filled. Contains remaining section of input, if any.
- `DeserError(&'a [u8])` - Reached end of chunk, but deserialization failed. Contains remaining section of input, if any.
- `Success{ data: T, remaining: &'a [u8] }` - Deserialization complete. Contains deserialized data and remaining section of input, if any.



