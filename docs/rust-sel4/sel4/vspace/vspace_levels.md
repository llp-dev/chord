**sel4 > vspace > vspace_levels**

# Module: vspace::vspace_levels

## Contents

**Functions**

- [`span_bits`](#span_bits) - The number of address bits spanned by the given translation table level.
- [`step_bits`](#step_bits) - The number of address bits spanned by entries of the given translation table level.

---

## sel4::vspace::vspace_levels::span_bits

*Function*

The number of address bits spanned by the given translation table level.

```rust
fn span_bits(level: usize) -> usize
```



## sel4::vspace::vspace_levels::step_bits

*Function*

The number of address bits spanned by entries of the given translation table level.

```rust
fn step_bits(level: usize) -> usize
```



