**cobs**

# Module: cobs

## Contents

**Functions**

- [`max_encoding_length`](#max_encoding_length) - Calculates the maximum possible size of an encoded message given the length
- [`max_encoding_overhead`](#max_encoding_overhead) - Calculates the maximum overhead when encoding a message with the given length.

---

## cobs::max_encoding_length

*Function*

Calculates the maximum possible size of an encoded message given the length
of the source message. This may be useful for calculating how large the
`dest` buffer needs to be in the encoding functions.

```rust
fn max_encoding_length(source_len: usize) -> usize
```



## cobs::max_encoding_overhead

*Function*

Calculates the maximum overhead when encoding a message with the given length.
The overhead is a maximum of [n/254] bytes (one in 254 bytes) rounded up.

```rust
fn max_encoding_overhead(source_len: usize) -> usize
```



