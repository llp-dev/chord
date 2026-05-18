**rand_core > utils**

# Module: utils

## Contents

**Functions**

- [`fill_bytes_via_next_word`](#fill_bytes_via_next_word) - Fill `dst` with bytes using `next_word`
- [`next_u64_via_u32`](#next_u64_via_u32) - Generate a `u64` using `next_u32`, little-endian order.
- [`next_word_via_fill`](#next_word_via_fill) - Generate a `u32` or `u64` word using `fill_bytes`
- [`read_words`](#read_words) - Reads an array of words from a byte slice

---

## rand_core::utils::fill_bytes_via_next_word

*Function*

Fill `dst` with bytes using `next_word`

This may be used to implement `fill_bytes` over `next_u32` or
`next_u64`. Words are used in order of generation. The last word may be
partially discarded.

```rust
fn fill_bytes_via_next_word<E, W, impl FnMut() -> Result<W, E>>(dst: & mut [u8], next_word: impl Trait) -> Result<(), E>
```



## rand_core::utils::next_u64_via_u32

*Function*

Generate a `u64` using `next_u32`, little-endian order.

```rust
fn next_u64_via_u32<R>(rng: & mut R) -> Result<u64, <R as >::Error>
```



## rand_core::utils::next_word_via_fill

*Function*

Generate a `u32` or `u64` word using `fill_bytes`

```rust
fn next_word_via_fill<W, R>(rng: & mut R) -> Result<W, <R as >::Error>
```



## rand_core::utils::read_words

*Function*

Reads an array of words from a byte slice

Words are read from `src` in order, using LE conversion from bytes.

# Panics

Panics if `size_of_val(src) != size_of::<[W; N]>()`.

```rust
fn read_words<W, const N>(src: &[u8]) -> [W; N]
```



