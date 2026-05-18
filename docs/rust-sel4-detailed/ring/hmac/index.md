*[ring](../index.md) / [hmac](index.md)*

---

# Module `hmac`

HMAC is specified in [RFC 2104].

After a `Key` is constructed, it can be used for multiple signing or
verification operations. Separating the construction of the key from the
rest of the HMAC operation allows the per-key precomputation to be done
only once, instead of it being done in every HMAC operation.

Frequently all the data to be signed in a message is available in a single
contiguous piece. In that case, the module-level `sign` function can be
used. Otherwise, if the input is in multiple parts, `Context` should be
used.

# Examples:

## Signing a value and verifying it wasn't tampered with

```rust
use ring::{hmac, rand};

let rng = rand::SystemRandom::new();
let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng)?;

let msg = "hello, world";

let tag = hmac::sign(&key, msg.as_bytes());

// [We give access to the message to an untrusted party, and they give it
// back to us. We need to verify they didn't tamper with it.]

hmac::verify(&key, msg.as_bytes(), tag.as_ref())?;

Ok::<(), ring::error::Unspecified>(())
```

## Using the one-shot API:

```rust
use ring::{digest, hmac, rand};
use ring::rand::SecureRandom;

let msg = "hello, world";

// The sender generates a secure key value and signs the message with it.
// Note that in a real protocol, a key agreement protocol would be used to
// derive `key_value`.
let rng = rand::SystemRandom::new();
let key_value: [u8; digest::SHA256_OUTPUT_LEN] = rand::generate(&rng)?.expose();

let s_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
let tag = hmac::sign(&s_key, msg.as_bytes());

// The receiver (somehow!) knows the key value, and uses it to verify the
// integrity of the message.
let v_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
hmac::verify(&v_key, msg.as_bytes(), tag.as_ref())?;

Ok::<(), ring::error::Unspecified>(())
```

## Using the multi-part API:
```rust
use ring::{digest, hmac, rand};
use ring::rand::SecureRandom;

let parts = ["hello", ", ", "world"];

// The sender generates a secure key value and signs the message with it.
// Note that in a real protocol, a key agreement protocol would be used to
// derive `key_value`.
let rng = rand::SystemRandom::new();
let mut key_value: [u8; digest::SHA384_OUTPUT_LEN] = rand::generate(&rng)?.expose();

let s_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
let mut s_ctx = hmac::Context::with_key(&s_key);
for part in &parts {
    s_ctx.update(part.as_bytes());
}
let tag = s_ctx.sign();

// The receiver (somehow!) knows the key value, and uses it to verify the
// integrity of the message.
let v_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
let mut msg = Vec::<u8>::new();
for part in &parts {
    msg.extend(part.as_bytes());
}
hmac::verify(&v_key, &msg.as_ref(), tag.as_ref())?;

Ok::<(), ring::error::Unspecified>(())
```




## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Algorithm`](#algorithm) | struct | An HMAC algorithm. |
| [`Tag`](#tag) | struct | An HMAC tag. |
| [`Key`](#key) | struct | A key to use for HMAC signing. |
| [`Context`](#context) | struct | A context for multi-step (Init-Update-Finish) HMAC signing. |
| [`sign`](#sign) | fn | Calculates the HMAC of `data` using the key `key` in one step. |
| [`verify`](#verify) | fn | Calculates the HMAC of `data` using the signing key `key`, and verifies whether the resultant value equals `tag`, in one step. |

## Structs

### `Algorithm`

```rust
struct Algorithm(&'static digest::Algorithm);
```

An HMAC algorithm.

#### Implementations

- <span id="algorithm-digest-algorithm"></span>`fn digest_algorithm(&self) -> &'static digest::Algorithm` — [`Algorithm`](../digest/index.md#algorithm)

  The digest algorithm this HMAC algorithm is based on.

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

### `Tag`

```rust
struct Tag(digest::Digest);
```

An HMAC tag.

For a given tag `t`, use `t.as_ref()` to get the tag value as a byte slice.

#### Trait Implementations

##### `impl AsRef for Tag`

- <span id="tag-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for Tag`

- <span id="tag-clone"></span>`fn clone(&self) -> Tag` — [`Tag`](#tag)

##### `impl Copy for Tag`

##### `impl Debug for Tag`

- <span id="tag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Key`

```rust
struct Key {
    inner: digest::BlockContext,
    outer: digest::BlockContext,
}
```

A key to use for HMAC signing.

#### Implementations

- <span id="key-generate"></span>`fn generate(algorithm: Algorithm, rng: &dyn rand::SecureRandom) -> Result<Self, error::Unspecified>` — [`Algorithm`](#algorithm), [`SecureRandom`](../rand/index.md#securerandom), [`Unspecified`](../error/index.md#unspecified)

  Generate an HMAC signing key using the given digest algorithm with a

  random value generated from `rng`.

  

  The key will be `digest_alg.output_len` bytes long, based on the

  recommendation in [RFC 2104 Section 3].

- <span id="key-construct"></span>`fn construct<F>(algorithm: Algorithm, fill: F) -> Result<Self, error::Unspecified>` — [`Algorithm`](#algorithm), [`Unspecified`](../error/index.md#unspecified)

- <span id="key-new"></span>`fn new(algorithm: Algorithm, key_value: &[u8]) -> Self` — [`Algorithm`](#algorithm)

  Construct an HMAC signing key using the given digest algorithm and key

  value.

  

  `key_value` should be a value generated using a secure random number

  generator (e.g. the `key_value` output by

  `SealingKey::generate_serializable()`) or derived from a random key by

  a key derivation function (e.g. `ring::hkdf`). In particular,

  `key_value` shouldn't be a password.

  

  As specified in RFC 2104, if `key_value` is shorter than the digest

  algorithm's block length (as returned by `digest::Algorithm::block_len()`,

  not the digest length returned by `digest::Algorithm::output_len()`) then

  it will be padded with zeros. Similarly, if it is longer than the block

  length then it will be compressed using the digest algorithm.

  

  You should not use keys larger than the `digest_alg.block_len` because

  the truncation described above reduces their strength to only

  `digest_alg.output_len * 8` bits. Support for such keys is likely to be

  removed in a future version of *ring*.

- <span id="key-algorithm"></span>`fn algorithm(&self) -> Algorithm` — [`Algorithm`](#algorithm)

  The digest algorithm for the key.

#### Trait Implementations

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](#key)

##### `impl Debug for Key`

- <span id="key-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `Context`

```rust
struct Context {
    inner: digest::Context,
    outer: digest::BlockContext,
}
```

A context for multi-step (Init-Update-Finish) HMAC signing.

Use `sign` for single-step HMAC signing.

#### Implementations

- <span id="context-with-key"></span>`fn with_key(signing_key: &Key) -> Self` — [`Key`](#key)

  Constructs a new HMAC signing context using the given digest algorithm

  and key.

- <span id="context-update"></span>`fn update(&mut self, data: &[u8])`

  Updates the HMAC with all the data in `data`. `update` may be called

  zero or more times until `finish` is called.

- <span id="context-sign"></span>`fn sign(self) -> Tag` — [`Tag`](#tag)

  Finalizes the HMAC calculation and returns the HMAC value. `sign`

  consumes the context so it cannot be (mis-)used after `sign` has been

  called.

  

  It is generally not safe to implement HMAC verification by comparing

  the return value of `sign` to a tag. Use `verify` for verification

  instead.

#### Trait Implementations

##### `impl Clone for Context`

- <span id="context-clone"></span>`fn clone(&self) -> Context` — [`Context`](#context)

##### `impl Debug for Context`

- <span id="context-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

## Functions

### `sign`

```rust
fn sign(key: &Key, data: &[u8]) -> Tag
```

Calculates the HMAC of `data` using the key `key` in one step.

Use `Context` to calculate HMACs where the input is in multiple parts.

It is generally not safe to implement HMAC verification by comparing the
return value of `sign` to a tag. Use `verify` for verification instead.

### `verify`

```rust
fn verify(key: &Key, data: &[u8], tag: &[u8]) -> Result<(), error::Unspecified>
```

Calculates the HMAC of `data` using the signing key `key`, and verifies
whether the resultant value equals `tag`, in one step.

This is logically equivalent to, but more efficient than, constructing a
`Key` with the same value as `key` and then using `verify`.

The verification will be done in constant time to prevent timing attacks.

