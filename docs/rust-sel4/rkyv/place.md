**rkyv > place**

# Module: place

## Contents

**Structs**

- [`Place`](#place) - A place to write a `T` paired with its position in the output buffer.

---

## rkyv::place::Place

*Struct*

A place to write a `T` paired with its position in the output buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn index(self: &Self, i: usize) -> Place<T>` - Gets a `Place` to the `i`-th element of the array.
- `fn index(self: &Self, i: usize) -> Place<T>` - Gets a `Place` to the `i`-th element of the slice.
- `fn new_unchecked(pos: usize, ptr: *mut T) -> Self` - Creates a new `Place` from an output pointer.
- `fn from_field_unchecked<U>(parent: Place<U>, ptr: *mut T) -> Self` - Creates a new `Place` from a parent pointer and the field the place
- `fn pos(self: &Self) -> usize` - Returns the position of the place.
- `fn ptr(self: &Self) -> *mut T` - Returns the pointer associated with this place.
- `fn write_unchecked(self: &Self, value: T)` - Writes the provided value to this place.
- `fn write(self: &Self, value: T)` - Writes the provided value to this place.
- `fn cast_unchecked<U>(self: &Self) -> Place<U>` - Returns this place casted to the given type.
- `fn as_slice(self: &Self) -> &[u8]` - Returns a slice of the bytes this place points to.

**Traits:** Copy

**Trait Implementations:**

- **Destructure**
  - `fn underlying(self: & mut Self) -> *mut <Self as >::Underlying`
- **Restructure**
  - `fn restructure(self: &Self, ptr: *mut U) -> <Self as >::Restructured`
- **Clone**
  - `fn clone(self: &Self) -> Self`



