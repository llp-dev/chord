**crc32fast**

# Module: crc32fast

## Contents

**Structs**

- [`Hasher`](#hasher) - Represents an in-progress CRC32 computation.

**Functions**

- [`hash`](#hash) - Computes the CRC32 hash of a byte slice.

---

## crc32fast::Hasher

*Struct*

Represents an in-progress CRC32 computation.

**Methods:**

- `fn new() -> Self` - Create a new `Hasher`.
- `fn new_with_initial(init: u32) -> Self` - Create a new `Hasher` with an initial CRC32 state.
- `fn new_with_initial_len(init: u32, amount: u64) -> Self` - Create a new `Hasher` with an initial CRC32 state.
- `fn update(self: & mut Self, buf: &[u8])` - Process the given byte slice and update the hash state.
- `fn finalize(self: Self) -> u32` - Finalize the hash state and return the computed CRC32 value.
- `fn reset(self: & mut Self)` - Reset the hash state.
- `fn combine(self: & mut Self, other: &Self)` - Combine the hash state with the hash state for the subsequent block of bytes.

**Trait Implementations:**

- **Hasher**
  - `fn write(self: & mut Self, bytes: &[u8])`
  - `fn finish(self: &Self) -> u64`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Hasher`



## crc32fast::hash

*Function*

Computes the CRC32 hash of a byte slice.

Check out [`Hasher`] for more advanced use-cases.

```rust
fn hash(buf: &[u8]) -> u32
```



