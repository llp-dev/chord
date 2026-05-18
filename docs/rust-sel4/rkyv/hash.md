**rkyv > hash**

# Module: hash

## Contents

**Structs**

- [`FxHasher64`](#fxhasher64) - A cross-platform 64-bit implementation of fxhash.

**Functions**

- [`hash_value`](#hash_value) - Hashes the given value with the default value of the specified `Hasher`.

---

## rkyv::hash::FxHasher64

*Struct*

A cross-platform 64-bit implementation of fxhash.

**Trait Implementations:**

- **Default**
  - `fn default() -> FxHasher64`
- **Hasher**
  - `fn write(self: & mut Self, bytes: &[u8])`
  - `fn finish(self: &Self) -> u64`
  - `fn write_u8(self: & mut Self, i: u8)`
  - `fn write_u16(self: & mut Self, i: u16)`
  - `fn write_u32(self: & mut Self, i: u32)`
  - `fn write_u64(self: & mut Self, i: u64)`
  - `fn write_u128(self: & mut Self, i: u128)`
  - `fn write_usize(self: & mut Self, i: usize)`
  - `fn write_isize(self: & mut Self, i: isize)`



## rkyv::hash::hash_value

*Function*

Hashes the given value with the default value of the specified `Hasher`.

```rust
fn hash_value<Q, H>(value: &Q) -> u64
```



