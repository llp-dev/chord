**rand > distr > slice**

# Module: distr::slice

## Contents

**Structs**

- [`Choose`](#choose) - A distribution to uniformly sample elements of a slice
- [`Empty`](#empty) - Error: empty slice

---

## rand::distr::slice::Choose

*Struct*

A distribution to uniformly sample elements of a slice

Like [`IndexedRandom::choose`], this uniformly samples elements of a slice
without modification of the slice (so called "sampling with replacement").
This distribution object may be a little faster for repeated sampling (but
slower for small numbers of samples).

## Examples

Since this is a distribution, [`Rng::sample_iter`] and
[`Distribution::sample_iter`] may be used, for example:
```
use rand::distr::{Distribution, slice::Choose};

let vowels = ['a', 'e', 'i', 'o', 'u'];
let vowels_dist = Choose::new(&vowels).unwrap();

// build a string of 10 vowels
let vowel_string: String = vowels_dist
    .sample_iter(&mut rand::rng())
    .take(10)
    .collect();

println!("{}", vowel_string);
assert_eq!(vowel_string.len(), 10);
assert!(vowel_string.chars().all(|c| vowels.contains(&c)));
```

For a single sample, [`IndexedRandom::choose`] may be preferred:
```
use rand::seq::IndexedRandom;

let vowels = ['a', 'e', 'i', 'o', 'u'];
let mut rng = rand::rng();

println!("{}", vowels.choose(&mut rng).unwrap());
```

[`IndexedRandom::choose`]: crate::seq::IndexedRandom::choose
[`Rng::sample_iter`]: crate::RngExt::sample_iter

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn new(slice: &'a [T]) -> Result<Self, Empty>` - Create a new `Choose` instance which samples uniformly from the slice.
- `fn num_choices(self: &Self) -> NonZeroUsize` - Returns the count of choices in this distribution

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> &'a T`
- **Clone**
  - `fn clone(self: &Self) -> Choose<'a, T>`



## rand::distr::slice::Empty

*Struct*

Error: empty slice

This error is returned when [`Choose::new`] is given an empty slice.

**Unit Struct**

**Traits:** Copy, Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Empty`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



