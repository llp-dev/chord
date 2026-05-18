**ring > ec > suite_b > ecdh**

# Module: ec::suite_b::ecdh

## Contents

**Statics**

- [`ECDH_P256`](#ecdh_p256) - ECDH using the NSA Suite B
- [`ECDH_P384`](#ecdh_p384) - ECDH using the NSA Suite B

---

## ring::ec::suite_b::ecdh::ECDH_P256

*Static*

ECDH using the NSA Suite B
P-256 (secp256r1)
curve.

Public keys are encoding in uncompressed form using the
Octet-String-to-Elliptic-Curve-Point algorithm in
[SEC 1: Elliptic Curve Cryptography, Version 2.0]. Public keys are
validated during key agreement according to
[NIST Special Publication 800-56A, revision 2] and Appendix B.3 of
the NSA's [Suite B Implementer's Guide to NIST SP 800-56A].

[SEC 1: Elliptic Curve Cryptography, Version 2.0]:
    http://www.secg.org/sec1-v2.pdf
[NIST Special Publication 800-56A, revision 2]:
    http://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Ar2.pdf
[Suite B Implementer's Guide to NIST SP 800-56A]:
    https://github.com/briansmith/ring/blob/main/doc/ecdh.pdf

```rust
static ECDH_P256: agreement::Algorithm
```



## ring::ec::suite_b::ecdh::ECDH_P384

*Static*

ECDH using the NSA Suite B
P-384 (secp384r1)
curve.

Public keys are encoding in uncompressed form using the
Octet-String-to-Elliptic-Curve-Point algorithm in
[SEC 1: Elliptic Curve Cryptography, Version 2.0]. Public keys are
validated during key agreement according to
[NIST Special Publication 800-56A, revision 2] and Appendix B.3 of
the NSA's [Suite B Implementer's Guide to NIST SP 800-56A].

[SEC 1: Elliptic Curve Cryptography, Version 2.0]:
    http://www.secg.org/sec1-v2.pdf
[NIST Special Publication 800-56A, revision 2]:
    http://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Ar2.pdf
[Suite B Implementer's Guide to NIST SP 800-56A]:
    https://github.com/briansmith/ring/blob/main/doc/ecdh.pdf

```rust
static ECDH_P384: agreement::Algorithm
```



