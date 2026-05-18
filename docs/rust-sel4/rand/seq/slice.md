**rand > seq > slice**

# Module: seq::slice

## Contents

**Traits**

- [`IndexedMutRandom`](#indexedmutrandom) - Extension trait on indexable lists, providing random sampling methods.
- [`IndexedRandom`](#indexedrandom) - Extension trait on indexable lists, providing random sampling methods.
- [`SliceRandom`](#slicerandom) - Extension trait on slices, providing shuffling methods.

---

## rand::seq::slice::IndexedMutRandom

*Trait*

Extension trait on indexable lists, providing random sampling methods.

This trait is implemented automatically for every type implementing
[`IndexedRandom`] and [`std::ops::IndexMut<usize>`].

**Methods:**

- `choose_mut`: Uniformly sample one element (mut)



## rand::seq::slice::IndexedRandom

*Trait*

Extension trait on indexable lists, providing random sampling methods.

This trait is implemented on `[T]` slice types. Other types supporting
[`std::ops::Index<usize>`] may implement this (only [`Self::len`] must be
specified).

**Methods:**

- `len`: The length
- `is_empty`: True when the length is zero
- `choose`: Uniformly sample one element
- `choose_iter`: Return an iterator which samples from `self` with replacement
- `sample_array`: Uniformly sample a fixed-size array of distinct elements from self
- `choose_multiple_array`: Deprecated: use [`Self::sample_array`] instead



## rand::seq::slice::SliceRandom

*Trait*

Extension trait on slices, providing shuffling methods.

This trait is implemented on all `[T]` slice types, providing several
methods for choosing and shuffling elements. You must `use` this trait:

```
use rand::seq::SliceRandom;

let mut rng = rand::rng();
let mut bytes = "Hello, random!".to_string().into_bytes();
bytes.shuffle(&mut rng);
let str = String::from_utf8(bytes).unwrap();
println!("{}", str);
```
Example output (non-deterministic):
```none
l,nmroHado !le
```

**Methods:**

- `shuffle`: Shuffle a mutable slice in place.
- `partial_shuffle`: Shuffle a slice in place, but exit early.



