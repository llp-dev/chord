# rustls_pemfile

# rustls-pemfile
A basic parser for .pem files containing cryptographic keys and certificates.

The input to this crate is a .pem file containing potentially many sections,
and the output is those sections as alleged DER-encodings.  This crate does
not decode the actual DER-encoded keys/certificates.

## Quick start
Starting with an `io::BufRead` containing the file to be read:
- Use `read_all()` to ingest the whole file, then work through the contents in-memory, or,
- Use `read_one()` to stream through the file, processing the items as found, or,
- Use `certs()` to extract just the certificates (silently discarding other sections), and
  similarly for `rsa_private_keys()` and `pkcs8_private_keys()`.

# no-std support

The opt-out "std" Cargo feature can be disabled to put this crate in no-std mode.

In no-std mode, the `read_one_from_slice` API can be used to parse a .pem file that has already
been loaded into memory.

## Example code
```ignore
use std::iter;
use rustls_pemfile::{Item, read_one};
# let mut reader = std::io::BufReader::new(&b"junk\n-----BEGIN RSA PRIVATE KEY-----\nqw\n-----END RSA PRIVATE KEY-----\n"[..]);
// Assume `reader` is any std::io::BufRead implementor
for item in iter::from_fn(|| read_one(&mut reader).transpose()) {
    match item.unwrap() {
        Item::X509Certificate(cert) => println!("certificate {:?}", cert),
        Item::Crl(crl) => println!("certificate revocation list: {:?}", crl),
        Item::Csr(csr) => println!("certificate signing request: {:?}", csr),
        Item::Pkcs1Key(key) => println!("rsa pkcs1 key {:?}", key),
        Item::Pkcs8Key(key) => println!("pkcs8 key {:?}", key),
        Item::Sec1Key(key) => println!("sec1 ec key {:?}", key),
        _ => println!("unhandled item"),
    }
}
```

## Modules

### [`pemfile`](pemfile.md)

*1 function, 2 enums*

