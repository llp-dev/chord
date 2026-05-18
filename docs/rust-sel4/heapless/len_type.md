**heapless > len_type**

# Module: len_type

## Contents

**Traits**

- [`LenType`](#lentype) - A sealed trait representing a valid type to use as a length for a container.
- [`Sealed`](#sealed)

---

## heapless::len_type::LenType

*Trait*

A sealed trait representing a valid type to use as a length for a container.

This cannot be implemented in user code, and is restricted to `u8`, `u16`, `u32`, and `usize`.



## heapless::len_type::Sealed

*Trait*

**Methods:**

- `ZERO`: The zero value of the integer type.
- `MAX`: The one value of the integer type.
- `MAX_USIZE`: The maximum value of this type, as a `usize`.
- `one`: The one value of the integer type.
- `from_usize`: An infallible conversion from `usize` to `LenT`.
- `into_usize`: An infallible conversion from `LenT` to `usize`.
- `to_non_max`: Converts `LenT` into `Some(usize)`, unless it's `Self::MAX`, where it returns `None`.



