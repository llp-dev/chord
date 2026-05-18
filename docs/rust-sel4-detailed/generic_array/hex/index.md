*[generic_array](../index.md) / [hex](index.md)*

---

# Module `hex`

Generic array are commonly used as a return value for hash digests, so
it's a good idea to allow to hexlify them easily. This module implements
`std::fmt::LowerHex` and `std::fmt::UpperHex` traits.

Example:

```rust
#[macro_use]
extern crate generic_array;
extern crate typenum;
fn main() {
let array = arr![u8; 10, 20, 30];
assert_eq!(format!("{:x}", array), "0a141e");
}
```


