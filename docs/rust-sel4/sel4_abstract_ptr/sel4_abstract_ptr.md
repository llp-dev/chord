**sel4_abstract_ptr**

# Module: sel4_abstract_ptr

## Contents

**Modules**

- [`access`](#access) - Marker types for limiting access.
- [`memory_type`](#memory_type)

**Macros**

- [`map_field`](#map_field) - Provides safe field projection for abstract pointers referencing structs.

---

## Module: access

Marker types for limiting access.



## sel4_abstract_ptr::map_field

*Declarative Macro*

Provides safe field projection for abstract pointers referencing structs.

## Examples

Accessing a struct field:

```ignore
use sel4_abstract_ptr::{AbstractPtr, map_field};

struct Example { field_1: u32, field_2: u8, }
let mut value = Example { field_1: 15, field_2: 255 };
let ptr = unsafe { AbstractPtr::new((&mut value).into()) };

// construct an abstract reference to a field
let field_2 = map_field!(ptr.field_2);
assert_eq!(field_2.read(), 255);
```

Creating `AbstractPtr`s to unaligned field in packed structs is not allowed:
```ignore
use sel4_abstract_ptr::{AbstractPtr, map_field};

#[repr(packed)]
struct Example { field_1: u8, field_2: usize, }
let mut value = Example { field_1: 15, field_2: 255 };
let ptr = unsafe { AbstractPtr::new((&mut value).into()) };

// Constructing an abstract reference to an unaligned field doesn't compile.
let field_2 = map_field!(ptr.field_2);
```

```rust
macro_rules! map_field {
    ($abstract_ptr:ident.$($place:ident).+) => { ... };
}
```



## Module: memory_type



