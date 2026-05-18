*[ring](../../../../index.md) / [ec](../../../index.md) / [suite_b](../../index.md) / [ecdsa](../index.md) / [digest_scalar](index.md)*

---

# Module `digest_scalar`

ECDSA Signatures using the P-256 and P-384 curves.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`digest_scalar`](#digest-scalar) | fn | Calculate the digest of `msg` using the digest algorithm `digest_alg`. |
| [`digest_scalar_`](#digest-scalar) | fn |  |

## Functions

### `digest_scalar`

```rust
fn digest_scalar(ops: &ScalarOps, msg: digest::Digest) -> elem::Elem<N, Unencoded>
```

Calculate the digest of `msg` using the digest algorithm `digest_alg`. Then
convert the digest to a scalar in the range [0, n) as described in
NIST's FIPS 186-4 Section 4.2. Note that this is one of the few cases where
a `Scalar` is allowed to have the value zero.

NIST's FIPS 186-4 4.2 says "When the length of the output of the hash
function is greater than N (i.e., the bit length of q), then the leftmost N
bits of the hash function output block shall be used in any calculation
using the hash function output during the generation or verification of a
digital signature."

"Leftmost N bits" means "N most significant bits" because we interpret the
digest as a bit-endian encoded integer.

The NSA guide instead vaguely suggests that we should convert the digest
value to an integer and then reduce it mod `n`. However, real-world
implementations (e.g. `digest_to_bn` in OpenSSL and `hashToInt` in Go) do
what FIPS 186-4 says to do, not what the NSA guide suggests.

Why shifting the value right by at most one bit is sufficient: P-256's `n`
has its 256th bit set; i.e. 2**255 < n < 2**256. Once we've truncated the
digest to 256 bits and converted it to an integer, it will have a value
less than 2**256. If the value is larger than `n` then shifting it one bit
right will give a value less than 2**255, which is less than `n`. The
analogous argument applies for P-384. However, it does *not* apply in
general; for example, it doesn't apply to P-521.

### `digest_scalar_`

```rust
fn digest_scalar_(ops: &ScalarOps, digest: &[u8]) -> elem::Elem<N, Unencoded>
```

