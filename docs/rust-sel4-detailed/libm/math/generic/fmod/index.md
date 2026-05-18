*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [fmod](index.md)*

---

# Module `fmod`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fmod`](#fmod) | fn |  |
| [`into_sig_exp`](#into-sig-exp) | fn | Given the bits of a finite float, return a tuple of - the mantissa with the implicit bit (0 if subnormal, 1 otherwise) - the additional exponent past 1, (0 for subnormal, 0 or more otherwise) |
| [`reduction`](#reduction) | fn | Compute the remainder `(x * 2.pow(e)) % y` without overflow. |

## Functions

### `fmod`

```rust
fn fmod<F: Float>(x: F, y: F) -> F
where
    <F as >::Int: HInt,
    <<F as >::Int as HInt>::D: NarrowingDiv
```

### `into_sig_exp`

```rust
fn into_sig_exp<F: Float>(bits: <F as >::Int) -> (<F as >::Int, u32)
```

Given the bits of a finite float, return a tuple of
 - the mantissa with the implicit bit (0 if subnormal, 1 otherwise)
 - the additional exponent past 1, (0 for subnormal, 0 or more otherwise)

### `reduction`

```rust
fn reduction<F>(x: <F as >::Int, e: u32, y: <F as >::Int) -> <F as >::Int
where
    F: Float,
    <F as >::Int: HInt,
    <<F as Float>::Int as HInt>::D: NarrowingDiv
```

Compute the remainder `(x * 2.pow(e)) % y` without overflow.

