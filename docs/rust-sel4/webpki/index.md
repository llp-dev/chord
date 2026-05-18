# webpki

webpki: Web PKI X.509 Certificate Validation.

See `EndEntityCert`'s documentation for a description of the certificate
processing steps necessary for a TLS connection.

# Features

| Feature | Description |
| ------- | ----------- |
| `alloc` | Enable features that require use of the heap. Currently all RSA signature algorithms require this feature. |
| `std` | Enable features that require libstd. Implies `alloc`. |
| `ring` | Enable use of the *ring* crate for cryptography. |
| `aws-lc-rs` | Enable use of the aws-lc-rs crate for cryptography. Previously this feature was named `aws_lc_rs`. |

## Modules

### [`webpki`](webpki.md)

*1 module, 1 static*

### [`cert`](cert.md)

*1 struct*

### [`crl`](crl.md)

*3 enums, 3 structs*

### [`crl::types`](crl/types.md)

*2 enums, 4 structs*

### [`der`](der.md)

*1 struct*

### [`end_entity`](end_entity.md)

*1 struct*

### [`error`](error.md)

*2 enums, 3 structs*

### [`ring_algs`](ring_algs.md)

*15 statics*

### [`rpk_entity`](rpk_entity.md)

*1 struct*

### [`trust_anchor`](trust_anchor.md)

*1 function*

### [`verify_cert`](verify_cert.md)

*1 trait, 6 structs*

