**sel4 > printing**

# Module: printing

## Contents

**Structs**

- [`DebugWrite`](#debugwrite) - Implements `core::fmt::Write` using [`debug_put_char`].

**Functions**

- [`debug_put_char`](#debug_put_char) - Corresponds to `seL4_DebugPutChar`.

---

## sel4::printing::DebugWrite

*Struct*

Implements `core::fmt::Write` using [`debug_put_char`].

**Unit Struct**

**Trait Implementations:**

- **Write**
  - `fn write_str(self: & mut Self, s: &str) -> fmt::Result`



## sel4::printing::debug_put_char

*Function*

Corresponds to `seL4_DebugPutChar`.

```rust
fn debug_put_char(c: u8)
```



