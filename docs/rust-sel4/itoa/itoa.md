**itoa**

# Module: itoa

## Contents

**Structs**

- [`Buffer`](#buffer) - A correctly sized stack allocation for the formatted integer to be written

**Traits**

- [`Integer`](#integer) - An integer that can be written into an [`itoa::Buffer`][Buffer].

---

## itoa::Buffer

*Struct*

A correctly sized stack allocation for the formatted integer to be written
into.

# Example

```
let mut buffer = itoa::Buffer::new();
let printed = buffer.format(1234);
assert_eq!(printed, "1234");
```

**Methods:**

- `fn new() -> Buffer` - This is a cheap operation; you don't need to worry about reusing buffers
- `fn format<I>(self: & mut Self, i: I) -> &str` - Print an integer into this buffer and return a reference to its string

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Buffer`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## itoa::Integer

*Trait*

An integer that can be written into an [`itoa::Buffer`][Buffer].

This trait is sealed and cannot be implemented for types outside of itoa.

**Methods:**

- `MAX_STR_LEN`: The maximum length of string that formatting an integer of this type can



