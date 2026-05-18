**sel4_stack**

# Module: sel4_stack

## Contents

**Structs**

- [`Stack`](#stack)
- [`StackBottom`](#stackbottom)

---

## sel4_stack::Stack

*Struct*

**Generic Parameters:**
- const N

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self`
- `fn bottom(self: &Self) -> StackBottom`

**Traits:** Sync

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## sel4_stack::StackBottom

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn ptr(self: &Self) -> *mut u8`

**Traits:** Sync



