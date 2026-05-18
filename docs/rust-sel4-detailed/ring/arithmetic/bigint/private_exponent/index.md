*[ring](../../../index.md) / [arithmetic](../../index.md) / [bigint](../index.md) / [private_exponent](index.md)*

---

# Module `private_exponent`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PrivateExponent`](#privateexponent) | struct |  |

## Structs

### `PrivateExponent`

```rust
struct PrivateExponent {
    limbs: alloc::boxed::Box<[u64]>,
}
```

#### Implementations

- <span id="privateexponent-from-be-bytes-padded"></span>`fn from_be_bytes_padded<M>(input: untrusted::Input<'_>, p: &Modulus<'_, M>) -> Result<Self, error::Unspecified>` — [`Modulus`](../modulus/index.md#modulus), [`Unspecified`](../../../error/index.md#unspecified)

- <span id="privateexponent-limbs"></span>`fn limbs(&self) -> &[u64]`

