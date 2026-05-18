*[ring](../index.md) / [hkdf](index.md)*

---

# Module `hkdf`

HMAC-based Extract-and-Expand Key Derivation Function.

HKDF is specified in [RFC 5869].


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Algorithm`](#algorithm) | struct | An HKDF algorithm. |
| [`Salt`](#salt) | struct | A salt for HKDF operations. |
| [`Prk`](#prk) | struct | A HKDF PRK (pseudorandom key). |
| [`Okm`](#okm) | struct | An HKDF OKM (Output Keying Material) |
| [`KeyType`](#keytype) | trait | The length of the OKM (Output Keying Material) for a `Prk::expand()` call. |
| [`fill_okm`](#fill-okm) | fn |  |

## Structs

### `Algorithm`

```rust
struct Algorithm(hmac::Algorithm);
```

An HKDF algorithm.

#### Implementations

- <span id="algorithm-hmac-algorithm"></span>`fn hmac_algorithm(&self) -> hmac::Algorithm` — [`Algorithm`](../hmac/index.md#algorithm)

  The underlying HMAC algorithm.

#### Trait Implementations

##### `impl Clone for Algorithm`

- <span id="algorithm-clone"></span>`fn clone(&self) -> Algorithm` — [`Algorithm`](#algorithm)

##### `impl Copy for Algorithm`

##### `impl Debug for Algorithm`

- <span id="algorithm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Algorithm`

##### `impl KeyType for Algorithm`

- <span id="algorithm-keytype-len"></span>`fn len(&self) -> usize`

##### `impl PartialEq for Algorithm`

- <span id="algorithm-partialeq-eq"></span>`fn eq(&self, other: &Algorithm) -> bool` — [`Algorithm`](#algorithm)

##### `impl StructuralPartialEq for Algorithm`

### `Salt`

```rust
struct Salt(hmac::Key);
```

A salt for HKDF operations.

#### Implementations

- <span id="salt-new"></span>`fn new(algorithm: Algorithm, value: &[u8]) -> Self` — [`Algorithm`](#algorithm)

  Constructs a new `Salt` with the given value based on the given digest

  algorithm.

  

  Constructing a `Salt` is relatively expensive so it is good to reuse a

  `Salt` object instead of re-constructing `Salt`s with the same value.

- <span id="salt-extract"></span>`fn extract(&self, secret: &[u8]) -> Prk` — [`Prk`](#prk)

  The [HKDF-Extract] operation.

- <span id="salt-algorithm"></span>`fn algorithm(&self) -> Algorithm` — [`Algorithm`](#algorithm)

  The algorithm used to derive this salt.

#### Trait Implementations

##### `impl Debug for Salt`

- <span id="salt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Prk`

```rust
struct Prk(hmac::Key);
```

A HKDF PRK (pseudorandom key).

#### Implementations

- <span id="prk-new-less-safe"></span>`fn new_less_safe(algorithm: Algorithm, value: &[u8]) -> Self` — [`Algorithm`](#algorithm)

  Construct a new `Prk` directly with the given value.

  

  Usually one can avoid using this. It is useful when the application

  intentionally wants to leak the PRK secret, e.g. to implement

  `SSLKEYLOGFILE` functionality.

- <span id="prk-expand"></span>`fn expand<'a, L: KeyType>(self: &'a Self, info: &'a [&'a [u8]], len: L) -> Result<Okm<'a, L>, error::Unspecified>` — [`Okm`](#okm), [`Unspecified`](../error/index.md#unspecified)

  The [HKDF-Expand] operation.

  

  Fails if (and only if) `len` is too large.

#### Trait Implementations

##### `impl Clone for Prk`

- <span id="prk-clone"></span>`fn clone(&self) -> Prk` — [`Prk`](#prk)

##### `impl Debug for Prk`

- <span id="prk-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Okm<'a, L: KeyType>`

```rust
struct Okm<'a, L: KeyType> {
    prk: &'a Prk,
    info: &'a [&'a [u8]],
    len: L,
    len_cached: usize,
}
```

An HKDF OKM (Output Keying Material)

Intentionally not `Clone` or `Copy` as an OKM is generally only safe to
use once.

#### Implementations

- <span id="okm-len"></span>`fn len(&self) -> &L`

  The `OkmLength` given to `Prk::expand()`.

- <span id="okm-fill"></span>`fn fill(self, out: &mut [u8]) -> Result<(), error::Unspecified>` — [`Unspecified`](../error/index.md#unspecified)

  Fills `out` with the output of the HKDF-Expand operation for the given

  inputs.

  

  Fails if (and only if) the requested output length is larger than 255

  times the size of the digest algorithm's output. (This is the limit

  imposed by the HKDF specification due to the way HKDF's counter is

  constructed.)

#### Trait Implementations

##### `impl<L: fmt::Debug + KeyType> Debug for Okm<'a, L>`

- <span id="okm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `KeyType`

```rust
trait KeyType { ... }
```

The length of the OKM (Output Keying Material) for a `Prk::expand()` call.

#### Required Methods

- `fn len(&self) -> usize`

  The length that `Prk::expand()` should expand its input to.

#### Implementors

- [`Algorithm`](#algorithm)
- [`Algorithm`](../hmac/index.md#algorithm)
- `&'static Algorithm`

## Functions

### `fill_okm`

```rust
fn fill_okm(prk: &Prk, info: &[&[u8]], out: &mut [u8], len: usize) -> Result<(), error::Unspecified>
```

