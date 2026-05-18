**rustls > crypto > tls12**

# Module: crypto::tls12

## Contents

**Structs**

- [`PrfUsingHmac`](#prfusinghmac) - Implements [`Prf`] using a [`hmac::Hmac`].

**Traits**

- [`Prf`](#prf) - An instantiation of the TLS1.2 PRF with a specific, implicit hash function.

---

## rustls::crypto::tls12::Prf

*Trait*

An instantiation of the TLS1.2 PRF with a specific, implicit hash function.

See the definition in [RFC5246 section 5](https://www.rfc-editor.org/rfc/rfc5246#section-5).

See [`PrfUsingHmac`] as a route to implementing this trait with just
an implementation of [`hmac::Hmac`].

**Methods:**

- `for_key_exchange`: Computes `PRF(secret, label, seed)` using the secret from a completed key exchange.
- `for_secret`: Computes `PRF(secret, label, seed)`, writing the result into `output`.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.



## rustls::crypto::tls12::PrfUsingHmac

*Struct*

Implements [`Prf`] using a [`hmac::Hmac`].

**Generic Parameters:**
- 'a

**Tuple Struct**: `(&'a dyn hmac::Hmac)`

**Trait Implementations:**

- **Prf**
  - `fn for_key_exchange(self: &Self, output: & mut [u8; 48], kx: Box<dyn ActiveKeyExchange>, peer_pub_key: &[u8], label: &[u8], seed: &[u8]) -> Result<(), Error>`
  - `fn for_secret(self: &Self, output: & mut [u8], secret: &[u8], label: &[u8], seed: &[u8])`



