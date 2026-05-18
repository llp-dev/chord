**banscii_assistant_core**

# Module: banscii_assistant_core

## Contents

**Structs**

- [`Draft`](#draft)

**Functions**

- [`layout`](#layout)

---

## banscii_assistant_core::Draft

*Struct*

**Fields:**
- `width: usize`
- `height: usize`
- `pixel_data: alloc::vec::Vec<u8>`

**Methods:**

- `fn new(subject: &str) -> Self`



## banscii_assistant_core::layout

*Function*

```rust
fn layout<F, SF>(font: SF, position: ab_glyph::Point, text: &str, target: & mut alloc::vec::Vec<ab_glyph::Glyph>)
```



