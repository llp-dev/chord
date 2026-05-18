**rkyv > util**

# Module: util

## Contents

**Structs**

- [`Align`](#align) - A wrapper which aligns its inner value to 16 bytes.

---

## rkyv::util::Align

*Struct*

A wrapper which aligns its inner value to 16 bytes.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Copy

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Clone**
  - `fn clone(self: &Self) -> Align<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



