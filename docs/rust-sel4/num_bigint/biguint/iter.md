**num_bigint > biguint > iter**

# Module: biguint::iter

## Contents

**Structs**

- [`U32Digits`](#u32digits) - An iterator of `u32` digits representation of a `BigUint` or `BigInt`,
- [`U64Digits`](#u64digits) - An iterator of `u64` digits representation of a `BigUint` or `BigInt`,

---

## num_bigint::biguint::iter::U32Digits

*Struct*

An iterator of `u32` digits representation of a `BigUint` or `BigInt`,
ordered least significant digit first.

**Generic Parameters:**
- 'a

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<u32>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn last(self: Self) -> Option<u32>`
  - `fn count(self: Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## num_bigint::biguint::iter::U64Digits

*Struct*

An iterator of `u64` digits representation of a `BigUint` or `BigInt`,
ordered least significant digit first.

**Generic Parameters:**
- 'a

**Traits:** FusedIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<u64>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn nth(self: & mut Self, n: usize) -> Option<u64>`
  - `fn last(self: Self) -> Option<u64>`
  - `fn count(self: Self) -> usize`



