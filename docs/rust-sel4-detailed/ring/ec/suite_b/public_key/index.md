*[ring](../../../index.md) / [ec](../../index.md) / [suite_b](../index.md) / [public_key](index.md)*

---

# Module `public_key`

Functionality shared by operations on public keys (ECDSA verification and
ECDH agreement).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parse_uncompressed_point`](#parse-uncompressed-point) | fn | Parses a public key encoded in uncompressed form. |

## Functions

### `parse_uncompressed_point`

```rust
fn parse_uncompressed_point(ops: &PublicKeyOps, input: untrusted::Input<'_>) -> Result<(elem::Elem<Q, R>, elem::Elem<Q, R>), error::Unspecified>
```

Parses a public key encoded in uncompressed form. The key is validated
using the ECC Partial Public-Key Validation Routine from
[NIST SP 800-56A, revision 2] Section 5.6.2.3.3, the NSA's
"Suite B Implementer's Guide to NIST SP 800-56A," Appendix B.3, and the
NSA's "Suite B Implementer's Guide to FIPS 186-3 (ECDSA)," Appendix A.3.


