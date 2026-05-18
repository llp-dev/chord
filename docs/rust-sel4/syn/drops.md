**syn > drops**

# Module: drops

## Contents

**Structs**

- [`NoDrop`](#nodrop)

**Traits**

- [`TrivialDrop`](#trivialdrop)

---

## syn::drops::NoDrop

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(core::mem::ManuallyDrop<T>)`

**Methods:**

- `fn new(value: T) -> Self`

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



## syn::drops::TrivialDrop

*Trait*



