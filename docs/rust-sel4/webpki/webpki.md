**webpki**

# Module: webpki

## Contents

**Modules**

- [`ring`](#ring) - Signature verification algorithm implementations using the *ring* crypto library.

**Statics**

- [`ALL_VERIFICATION_ALGS`](#all_verification_algs) - An array of all the verification algorithms exported by this crate.

---

## webpki::ALL_VERIFICATION_ALGS

*Static*

An array of all the verification algorithms exported by this crate.

This will be empty if the crate is built without the `ring` and `aws-lc-rs` features.

```rust
static ALL_VERIFICATION_ALGS: &[&dyn pki_types::SignatureVerificationAlgorithm]
```



## Module: ring

Signature verification algorithm implementations using the *ring* crypto library.



