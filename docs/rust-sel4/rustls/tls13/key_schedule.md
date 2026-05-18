**rustls > tls13 > key_schedule**

# Module: tls13::key_schedule

## Contents

**Functions**

- [`derive_traffic_iv`](#derive_traffic_iv) - [HKDF-Expand-Label] where the output is an IV.
- [`derive_traffic_key`](#derive_traffic_key) - [HKDF-Expand-Label] where the output is an AEAD key.

---

## rustls::tls13::key_schedule::derive_traffic_iv

*Function*

[HKDF-Expand-Label] where the output is an IV.

[HKDF-Expand-Label]: <https://www.rfc-editor.org/rfc/rfc8446#section-7.1>

```rust
fn derive_traffic_iv(expander: &dyn HkdfExpander) -> crate::crypto::cipher::Iv
```



## rustls::tls13::key_schedule::derive_traffic_key

*Function*

[HKDF-Expand-Label] where the output is an AEAD key.

[HKDF-Expand-Label]: <https://www.rfc-editor.org/rfc/rfc8446#section-7.1>

```rust
fn derive_traffic_key(expander: &dyn HkdfExpander, aead_alg: &dyn Tls13AeadAlgorithm) -> crate::crypto::cipher::AeadKey
```



