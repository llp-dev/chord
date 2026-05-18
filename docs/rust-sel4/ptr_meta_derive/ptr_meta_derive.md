**ptr_meta_derive**

# Module: ptr_meta_derive

## Contents

**Proc Macros**

- [`pointee`](#pointee) - Generates a `Pointee` implementation for trait object of the labeled trait.

---

## ptr_meta_derive::pointee

*Attribute Macro*

Generates a `Pointee` implementation for trait object of the labeled trait.

# Arguments

`#[pointee(...)]` takes the following arguments:

- `crate = ...`: Chooses an alternative crate path to import ptr_meta from.

```rust
#[pointee]
```



