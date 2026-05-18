**ring > aead > chacha20_poly1305**

# Module: aead::chacha20_poly1305

## Contents

**Statics**

- [`CHACHA20_POLY1305`](#chacha20_poly1305) - ChaCha20-Poly1305 as described in [RFC 8439].

---

## ring::aead::chacha20_poly1305::CHACHA20_POLY1305

*Static*

ChaCha20-Poly1305 as described in [RFC 8439].

The keys are 256 bits long and the nonces are 96 bits long.

[RFC 8439]: https://tools.ietf.org/html/rfc8439

```rust
static CHACHA20_POLY1305: aead::Algorithm
```



