**rand > distr > uniform > other**

# Module: distr::uniform::other

## Contents

**Structs**

- [`UniformChar`](#uniformchar) - The back-end implementing [`UniformSampler`] for `char`.
- [`UniformDuration`](#uniformduration) - The back-end implementing [`UniformSampler`] for `Duration`.

---

## rand::distr::uniform::other::UniformChar

*Struct*

The back-end implementing [`UniformSampler`] for `char`.

Unless you are implementing [`UniformSampler`] for your own type, this type
should not be used directly, use [`Uniform`] instead.

This differs from integer range sampling since the range `0xD800..=0xDFFF`
are used for surrogate pairs in UCS and UTF-16, and consequently are not
valid Unicode code points. We must therefore avoid sampling values in this
range.

**Traits:** Eq, Copy

**Trait Implementations:**

- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X`
- **Clone**
  - `fn clone(self: &Self) -> UniformChar`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UniformChar) -> bool`



## rand::distr::uniform::other::UniformDuration

*Struct*

The back-end implementing [`UniformSampler`] for `Duration`.

Unless you are implementing [`UniformSampler`] for your own types, this type
should not be used directly, use [`Uniform`] instead.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UniformDuration) -> bool`
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> Duration`
- **Clone**
  - `fn clone(self: &Self) -> UniformDuration`



