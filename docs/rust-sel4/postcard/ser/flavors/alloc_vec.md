**postcard > ser > flavors > alloc_vec**

# Module: ser::flavors::alloc_vec

## Contents

**Structs**

- [`AllocVec`](#allocvec) - The `AllocVec` flavor is a wrapper type around an [`alloc::vec::Vec`].

---

## postcard::ser::flavors::alloc_vec::AllocVec

*Struct*

The `AllocVec` flavor is a wrapper type around an [`alloc::vec::Vec`].

This type is only available when the (non-default) `alloc` feature is active

**Methods:**

- `fn new() -> Self` - Create a new, currently empty, [`alloc::vec::Vec`] to be used for storing serialized

**Trait Implementations:**

- **Default**
  - `fn default() -> AllocVec`
- **Index**
  - `fn index(self: &Self, idx: usize) -> &u8`
- **IndexMut**
  - `fn index_mut(self: & mut Self, idx: usize) -> & mut u8`
- **Flavor**
  - `fn try_extend(self: & mut Self, data: &[u8]) -> Result<()>`
  - `fn try_push(self: & mut Self, data: u8) -> Result<()>`
  - `fn finalize(self: Self) -> Result<<Self as >::Output>`



