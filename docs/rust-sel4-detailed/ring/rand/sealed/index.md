*[ring](../../index.md) / [rand](../index.md) / [sealed](index.md)*

---

# Module `sealed`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SecureRandom`](#securerandom) | trait |  |
| [`RandomlyConstructable`](#randomlyconstructable) | trait |  |

## Traits

### `SecureRandom`

```rust
trait SecureRandom: core::fmt::Debug { ... }
```

#### Required Methods

- `fn fill_impl(&self, dest: &mut [u8]) -> Result<(), error::Unspecified>`

  Fills `dest` with random bytes.

#### Implementors

- [`NonceRandom`](../../ec/suite_b/ecdsa/signing/index.md#noncerandom)
- [`SystemRandom`](../index.md#systemrandom)

### `RandomlyConstructable`

```rust
trait RandomlyConstructable: Sized { ... }
```

#### Required Methods

- `fn zero() -> Self`

- `fn as_mut_bytes(&mut self) -> &mut [u8]`

#### Implementors

- `[u8; N]`

