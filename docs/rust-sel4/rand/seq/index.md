**rand > seq > index**

# Module: seq::index

## Contents

**Functions**

- [`sample_array`](#sample_array) - Randomly sample exactly `N` distinct indices from `0..len`, and

---

## rand::seq::index::sample_array

*Function*

Randomly sample exactly `N` distinct indices from `0..len`, and
return them in random order (fully shuffled).

This is implemented via Floyd's algorithm. Time complexity is `O(N^2)`
and memory complexity is `O(N)`.

Returns `None` if (and only if) `N > len`.

```rust
fn sample_array<R, const N>(rng: & mut R, len: usize) -> Option<[usize; N]>
```



