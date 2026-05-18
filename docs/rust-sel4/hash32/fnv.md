**hash32 > fnv**

# Module: fnv

## Contents

**Structs**

- [`Hasher`](#hasher) - 32-bit Fowler-Noll-Vo hasher

---

## hash32::fnv::Hasher

*Struct*

32-bit Fowler-Noll-Vo hasher

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Hasher**
  - `fn finish32(self: &Self) -> u32`
- **Hasher**
  - `fn write(self: & mut Self, bytes: &[u8])`
  - `fn finish(self: &Self) -> u64`



