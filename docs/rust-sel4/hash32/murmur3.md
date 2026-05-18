**hash32 > murmur3**

# Module: murmur3

## Contents

**Structs**

- [`Hasher`](#hasher) - 32-bit MurmurHash3 hasher

---

## hash32::murmur3::Hasher

*Struct*

32-bit MurmurHash3 hasher

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Hasher**
  - `fn write(self: & mut Self, bytes: &[u8])`
  - `fn finish(self: &Self) -> u64`
- **Hasher**
  - `fn finish32(self: &Self) -> u32`



