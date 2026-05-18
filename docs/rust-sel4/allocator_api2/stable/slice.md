**allocator_api2 > stable > slice**

# Module: stable::slice

## Contents

**Traits**

- [`SliceExt`](#sliceext) - Slice methods that use `Box` and `Vec` from this crate.

---

## allocator_api2::stable::slice::SliceExt

*Trait*

Slice methods that use `Box` and `Vec` from this crate.

**Methods:**

- `to_vec`: Copies `self` into a new `Vec`.
- `to_vec_in`: Copies `self` into a new `Vec` with an allocator.
- `repeat`: Creates a vector by copying a slice `n` times.



