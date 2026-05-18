**num_traits > ops > bytes**

# Module: ops::bytes

## Contents

**Traits**

- [`FromBytes`](#frombytes)
- [`NumBytes`](#numbytes)
- [`ToBytes`](#tobytes)

---

## num_traits::ops::bytes::FromBytes

*Trait*

**Methods:**

- `Bytes`
- `from_be_bytes`: Create a number from its representation as a byte array in big endian.
- `from_le_bytes`: Create a number from its representation as a byte array in little endian.
- `from_ne_bytes`: Create a number from its memory representation as a byte array in native endianness.



## num_traits::ops::bytes::NumBytes

*Trait*



## num_traits::ops::bytes::ToBytes

*Trait*

**Methods:**

- `Bytes`
- `to_be_bytes`: Return the memory representation of this number as a byte array in big-endian byte order.
- `to_le_bytes`: Return the memory representation of this number as a byte array in little-endian byte order.
- `to_ne_bytes`: Return the memory representation of this number as a byte array in native byte order.



