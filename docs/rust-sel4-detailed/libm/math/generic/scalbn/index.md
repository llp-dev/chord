*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [scalbn](index.md)*

---

# Module `scalbn`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`scalbn`](#scalbn) | fn | Scale the exponent. |

## Functions

### `scalbn`

```rust
fn scalbn<F: Float>(x: F, n: i32) -> F
where
    u32: CastInto<<F as >::Int>,
    <F as >::Int: CastFrom<i32> + CastFrom<u32>
```

Scale the exponent.

From N3220:

> The scalbn and scalbln functions compute `x * b^n`, where `b = FLT_RADIX` if the return type
> of the function is a standard floating type, or `b = 10` if the return type of the function
> is a decimal floating type. A range error occurs for some finite x, depending on n.
>
> [...]
>
> * `scalbn(±0, n)` returns `±0`.
> * `scalbn(x, 0)` returns `x`.
> * `scalbn(±∞, n)` returns `±∞`.
>
> If the calculation does not overflow or underflow, the returned value is exact and
> independent of the current rounding direction mode.

