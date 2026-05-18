**sel4_capdl_initializer_add_spec**

# Module: sel4_capdl_initializer_add_spec

## Contents

**Enums**

- [`ObjectNamesLevel`](#objectnameslevel)

**Functions**

- [`add_spec`](#add_spec)

---

## sel4_capdl_initializer_add_spec::ObjectNamesLevel

*Enum*

**Variants:**
- `All`
- `JustTcbs`
- `None`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectNamesLevel) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ObjectNamesLevel`



## sel4_capdl_initializer_add_spec::add_spec

*Function*

```rust
fn add_spec<impl AsRef<Path>>(initializer_without_spec: &[u8], spec: &sel4_capdl_initializer_types::InputSpec, fill_dirs: &[impl Trait], object_names_level: &ObjectNamesLevel, embed_frames: bool, deflate: bool, initializer_verbosity: u8) -> Vec<u8>
```



