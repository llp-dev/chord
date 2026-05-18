*[ring](../index.md) / [limb](index.md)*

---

# Module `limb`

Unsigned multi-precision integer arithmetic.

Limbs ordered least-significant-limb to most-significant-limb. The bits
limbs use the native endianness.

## Contents

- [Enums](#enums)
  - [`LimbMask`](#limbmask)
  - [`AllowZero`](#allowzero)
- [Functions](#functions)
  - [`LIMBS_are_zero`](#limbs-are-zero)
  - [`LIMBS_less_than`](#limbs-less-than)
  - [`LIMBS_reduce_once`](#limbs-reduce-once)
  - [`LIMB_shr`](#limb-shr)
  - [`LIMBS_are_even`](#limbs-are-even)
  - [`LIMBS_equal_limb`](#limbs-equal-limb)
  - [`LIMBS_less_than_limb`](#limbs-less-than-limb)
  - [`limbs_equal_limbs_consttime`](#limbs-equal-limbs-consttime)
  - [`limbs_less_than_limbs_consttime`](#limbs-less-than-limbs-consttime)
  - [`limbs_less_than_limbs_vartime`](#limbs-less-than-limbs-vartime)
  - [`limbs_less_than_limb_constant_time`](#limbs-less-than-limb-constant-time)
  - [`limbs_are_zero_constant_time`](#limbs-are-zero-constant-time)
  - [`limbs_are_even_constant_time`](#limbs-are-even-constant-time)
  - [`limbs_equal_limb_constant_time`](#limbs-equal-limb-constant-time)
  - [`limbs_minimal_bits`](#limbs-minimal-bits)
  - [`limbs_reduce_once_constant_time`](#limbs-reduce-once-constant-time)
  - [`parse_big_endian_in_range_and_pad_consttime`](#parse-big-endian-in-range-and-pad-consttime)
  - [`parse_big_endian_and_pad_consttime`](#parse-big-endian-and-pad-consttime)
  - [`big_endian_from_limbs`](#big-endian-from-limbs)
  - [`unstripped_be_bytes`](#unstripped-be-bytes)
  - [`fold_5_bit_windows`](#fold-5-bit-windows)
  - [`limbs_add_assign_mod`](#limbs-add-assign-mod)
  - [`limbs_double_mod`](#limbs-double-mod)
  - [`limbs_negative_odd`](#limbs-negative-odd)
- [Type Aliases](#type-aliases)
  - [`Limb`](#limb)
  - [`Window`](#window)
- [Constants](#constants)
  - [`LIMB_BITS`](#limb-bits)
  - [`LIMB_BYTES`](#limb-bytes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LimbMask`](#limbmask) | enum |  |
| [`AllowZero`](#allowzero) | enum |  |
| [`LIMBS_are_zero`](#limbs-are-zero) | fn |  |
| [`LIMBS_less_than`](#limbs-less-than) | fn |  |
| [`LIMBS_reduce_once`](#limbs-reduce-once) | fn |  |
| [`LIMB_shr`](#limb-shr) | fn |  |
| [`LIMBS_are_even`](#limbs-are-even) | fn |  |
| [`LIMBS_equal_limb`](#limbs-equal-limb) | fn |  |
| [`LIMBS_less_than_limb`](#limbs-less-than-limb) | fn |  |
| [`limbs_equal_limbs_consttime`](#limbs-equal-limbs-consttime) | fn |  |
| [`limbs_less_than_limbs_consttime`](#limbs-less-than-limbs-consttime) | fn |  |
| [`limbs_less_than_limbs_vartime`](#limbs-less-than-limbs-vartime) | fn |  |
| [`limbs_less_than_limb_constant_time`](#limbs-less-than-limb-constant-time) | fn |  |
| [`limbs_are_zero_constant_time`](#limbs-are-zero-constant-time) | fn |  |
| [`limbs_are_even_constant_time`](#limbs-are-even-constant-time) | fn |  |
| [`limbs_equal_limb_constant_time`](#limbs-equal-limb-constant-time) | fn |  |
| [`limbs_minimal_bits`](#limbs-minimal-bits) | fn | Returns the number of bits in `a`. |
| [`limbs_reduce_once_constant_time`](#limbs-reduce-once-constant-time) | fn | Equivalent to `if (r >= m) { r -= m; }` |
| [`parse_big_endian_in_range_and_pad_consttime`](#parse-big-endian-in-range-and-pad-consttime) | fn | Parses `input` into `result`, verifies that the value is less than `max_exclusive`, and pads `result` with zeros to its length. |
| [`parse_big_endian_and_pad_consttime`](#parse-big-endian-and-pad-consttime) | fn | Parses `input` into `result`, padding `result` with zeros to its length. |
| [`big_endian_from_limbs`](#big-endian-from-limbs) | fn |  |
| [`unstripped_be_bytes`](#unstripped-be-bytes) | fn | Returns an iterator of the big-endian encoding of `limbs`. |
| [`fold_5_bit_windows`](#fold-5-bit-windows) | fn | Processes `limbs` as a sequence of 5-bit windows, folding the windows from most significant to least significant and returning the accumulated result. |
| [`limbs_add_assign_mod`](#limbs-add-assign-mod) | fn |  |
| [`limbs_double_mod`](#limbs-double-mod) | fn |  |
| [`limbs_negative_odd`](#limbs-negative-odd) | fn |  |
| [`Limb`](#limb) | type |  |
| [`Window`](#window) | type |  |
| [`LIMB_BITS`](#limb-bits) | const |  |
| [`LIMB_BYTES`](#limb-bytes) | const |  |

## Enums

### `LimbMask`

```rust
enum LimbMask {
    True,
    False,
}
```

#### Trait Implementations

##### `impl Debug for LimbMask`

- <span id="limbmask-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for LimbMask`

- <span id="limbmask-partialeq-eq"></span>`fn eq(&self, other: &LimbMask) -> bool` — [`LimbMask`](#limbmask)

##### `impl StructuralPartialEq for LimbMask`

### `AllowZero`

```rust
enum AllowZero {
    No,
    Yes,
}
```

#### Trait Implementations

##### `impl Clone for AllowZero`

- <span id="allowzero-clone"></span>`fn clone(&self) -> AllowZero` — [`AllowZero`](#allowzero)

##### `impl Copy for AllowZero`

##### `impl PartialEq for AllowZero`

- <span id="allowzero-partialeq-eq"></span>`fn eq(&self, other: &AllowZero) -> bool` — [`AllowZero`](#allowzero)

##### `impl StructuralPartialEq for AllowZero`

## Functions

### `LIMBS_are_zero`

```rust
unsafe fn LIMBS_are_zero(a: *const u64, num_limbs: usize) -> LimbMask
```

### `LIMBS_less_than`

```rust
unsafe fn LIMBS_less_than(a: *const u64, b: *const u64, num_limbs: usize) -> LimbMask
```

### `LIMBS_reduce_once`

```rust
unsafe fn LIMBS_reduce_once(r: *mut u64, m: *const u64, num_limbs: usize)
```

### `LIMB_shr`

```rust
unsafe fn LIMB_shr(a: u64, shift: usize) -> u64
```

### `LIMBS_are_even`

```rust
unsafe fn LIMBS_are_even(a: *const u64, num_limbs: usize) -> LimbMask
```

### `LIMBS_equal_limb`

```rust
unsafe fn LIMBS_equal_limb(a: *const u64, b: u64, num_limbs: usize) -> LimbMask
```

### `LIMBS_less_than_limb`

```rust
unsafe fn LIMBS_less_than_limb(a: *const u64, b: u64, num_limbs: usize) -> LimbMask
```

### `limbs_equal_limbs_consttime`

```rust
fn limbs_equal_limbs_consttime(a: &[u64], b: &[u64]) -> LimbMask
```

### `limbs_less_than_limbs_consttime`

```rust
fn limbs_less_than_limbs_consttime(a: &[u64], b: &[u64]) -> LimbMask
```

### `limbs_less_than_limbs_vartime`

```rust
fn limbs_less_than_limbs_vartime(a: &[u64], b: &[u64]) -> bool
```

### `limbs_less_than_limb_constant_time`

```rust
fn limbs_less_than_limb_constant_time(a: &[u64], b: u64) -> LimbMask
```

### `limbs_are_zero_constant_time`

```rust
fn limbs_are_zero_constant_time(limbs: &[u64]) -> LimbMask
```

### `limbs_are_even_constant_time`

```rust
fn limbs_are_even_constant_time(limbs: &[u64]) -> LimbMask
```

### `limbs_equal_limb_constant_time`

```rust
fn limbs_equal_limb_constant_time(a: &[u64], b: u64) -> LimbMask
```

### `limbs_minimal_bits`

```rust
fn limbs_minimal_bits(a: &[u64]) -> bits::BitLength
```

Returns the number of bits in `a`.

### `limbs_reduce_once_constant_time`

```rust
fn limbs_reduce_once_constant_time(r: &mut [u64], m: &[u64])
```

Equivalent to `if (r >= m) { r -= m; }`

### `parse_big_endian_in_range_and_pad_consttime`

```rust
fn parse_big_endian_in_range_and_pad_consttime(input: untrusted::Input<'_>, allow_zero: AllowZero, max_exclusive: &[u64], result: &mut [u64]) -> Result<(), error::Unspecified>
```

Parses `input` into `result`, verifies that the value is less than
`max_exclusive`, and pads `result` with zeros to its length. If `allow_zero`
is not `AllowZero::Yes`, zero values are rejected.

This attempts to be constant-time with respect to the actual value *only if*
the value is actually in range. In other words, this won't leak anything
about a valid value, but it might leak small amounts of information about an
invalid value (which constraint it failed).

### `parse_big_endian_and_pad_consttime`

```rust
fn parse_big_endian_and_pad_consttime(input: untrusted::Input<'_>, result: &mut [u64]) -> Result<(), error::Unspecified>
```

Parses `input` into `result`, padding `result` with zeros to its length.
This attempts to be constant-time with respect to the value but not with
respect to the length; it is assumed that the length is public knowledge.

### `big_endian_from_limbs`

```rust
fn big_endian_from_limbs(limbs: &[u64], out: &mut [u8])
```

### `unstripped_be_bytes`

```rust
fn unstripped_be_bytes(limbs: &[u64]) -> impl ExactSizeIterator<Item = u8> + Clone + '_
```

Returns an iterator of the big-endian encoding of `limbs`.

The number of bytes returned will be a multiple of `LIMB_BYTES`
and thus may be padded with leading zeros.

### `fold_5_bit_windows`

```rust
fn fold_5_bit_windows<R, I: FnOnce(u64) -> R, F: Fn(R, u64) -> R>(limbs: &[u64], init: I, fold: F) -> R
```

Processes `limbs` as a sequence of 5-bit windows, folding the windows from
most significant to least significant and returning the accumulated result.
The first window will be mapped by `init` to produce the initial value for
the accumulator. Then `f` will be called to fold the accumulator and the
next window until all windows are processed. When the input's bit length
isn't divisible by 5, the window passed to `init` will be partial; all
windows passed to `fold` will be full.

This is designed to avoid leaking the contents of `limbs` through side
channels as long as `init` and `fold` are side-channel free.

Panics if `limbs` is empty.

### `limbs_add_assign_mod`

```rust
fn limbs_add_assign_mod(a: &mut [u64], b: &[u64], m: &[u64])
```

### `limbs_double_mod`

```rust
fn limbs_double_mod(r: &mut [u64], m: &[u64])
```

### `limbs_negative_odd`

```rust
fn limbs_negative_odd(r: &mut [u64], a: &[u64])
```

## Type Aliases

### `Limb`

```rust
type Limb = u64;
```

### `Window`

```rust
type Window = u64;
```

## Constants

### `LIMB_BITS`
```rust
const LIMB_BITS: usize = 64usize;
```

### `LIMB_BYTES`
```rust
const LIMB_BYTES: usize = 8usize;
```

