**object > read > elf > dynamic**

# Module: read::elf::dynamic

## Contents

**Traits**

- [`Dyn`](#dyn) - A trait for generic access to [`elf::Dyn32`] and [`elf::Dyn64`].

---

## object::read::elf::dynamic::Dyn

*Trait*

A trait for generic access to [`elf::Dyn32`] and [`elf::Dyn64`].

**Methods:**

- `Word`
- `Endian`
- `d_tag`
- `d_val`
- `tag32`: Try to convert the tag to a `u32`.
- `val32`: Try to convert the value to a `u32`.
- `is_string`: Return true if the value is an offset in the dynamic string table.
- `string`: Use the value to get a string in a string table.
- `is_address`: Return true if the value is an address.



