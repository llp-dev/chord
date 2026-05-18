**ring**

# Module: ring

## Contents

**Modules**

- [`aead`](#aead) - Authenticated Encryption with Associated Data (AEAD).
- [`agreement`](#agreement) - Key Agreement: ECDH, including X25519.
- [`constant_time`](#constant_time) - Constant-time operations.
- [`digest`](#digest) - SHA-2 and the legacy SHA-1 digest algorithm.
- [`error`](#error) - Error reporting.
- [`hkdf`](#hkdf) - HMAC-based Extract-and-Expand Key Derivation Function.
- [`hmac`](#hmac) - HMAC is specified in [RFC 2104].
- [`io`](#io) - Serialization and deserialization.
- [`pbkdf2`](#pbkdf2) - PBKDF2 derivation and verification.
- [`pkcs8`](#pkcs8) - PKCS#8 is specified in [RFC 5958].
- [`rand`](#rand) - Cryptographic pseudo-random number generation.
- [`rsa`](#rsa) - RSA.
- [`signature`](#signature) - Public key signatures: signing and verification.
- [`test`](#test) - Testing framework.

**Macros**

- [`test_file`](#test_file) - References a test input file.

---

## Module: aead

Authenticated Encryption with Associated Data (AEAD).

See [Authenticated encryption: relations among notions and analysis of the
generic composition paradigm][AEAD] for an introduction to the concept of
AEADs.

[AEAD]: https://eprint.iacr.org/2000/025.pdf
[`crypto.cipher.AEAD`]: https://golang.org/pkg/crypto/cipher/#AEAD



## Module: agreement

Key Agreement: ECDH, including X25519.

# Example

Note that this example uses X25519, but ECDH using NIST P-256/P-384 is done
exactly the same way, just substituting
`agreement::ECDH_P256`/`agreement::ECDH_P384` for `agreement::X25519`.

```
use ring::{agreement, rand};

let rng = rand::SystemRandom::new();

let my_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;

// Make `my_public_key` a byte slice containing my public key. In a real
// application, this would be sent to the peer in an encoded protocol
// message.
let my_public_key = my_private_key.compute_public_key()?;

let peer_public_key_bytes = {
    // In a real application, the peer public key would be parsed out of a
    // protocol message. Here we just generate one.
    let peer_private_key =
        agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
    peer_private_key.compute_public_key()?
};

let peer_public_key = agreement::UnparsedPublicKey::new(
    &agreement::X25519,
    peer_public_key_bytes);

agreement::agree_ephemeral(
    my_private_key,
    &peer_public_key,
    |_key_material| {
        // In a real application, we'd apply a KDF to the key material and the
        // public keys (as recommended in RFC 7748) and then derive session
        // keys from the result. We omit all that here.
    },
)?;

# Ok::<(), ring::error::Unspecified>(())
```



## Module: constant_time

Constant-time operations.



## Module: digest

SHA-2 and the legacy SHA-1 digest algorithm.

If all the data is available in a single contiguous slice then the `digest`
function should be used. Otherwise, the digest can be calculated in
multiple steps using `Context`.



## Module: error

Error reporting.



## Module: hkdf

HMAC-based Extract-and-Expand Key Derivation Function.

HKDF is specified in [RFC 5869].

[RFC 5869]: https://tools.ietf.org/html/rfc5869



## Module: hmac

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

```
use ring::{hmac, rand};

let rng = rand::SystemRandom::new();
let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng)?;

let msg = "hello, world";

let tag = hmac::sign(&key, msg.as_bytes());

// [We give access to the message to an untrusted party, and they give it
// back to us. We need to verify they didn't tamper with it.]

hmac::verify(&key, msg.as_bytes(), tag.as_ref())?;

# Ok::<(), ring::error::Unspecified>(())
```

## Using the one-shot API:

```
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

# Ok::<(), ring::error::Unspecified>(())
```

## Using the multi-part API:
```
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

# Ok::<(), ring::error::Unspecified>(())
```

[RFC 2104]: https://tools.ietf.org/html/rfc2104
[code for `ring::pbkdf2`]:
    https://github.com/briansmith/ring/blob/main/src/pbkdf2.rs
[code for `ring::hkdf`]:
    https://github.com/briansmith/ring/blob/main/src/hkdf.rs



## Module: io

Serialization and deserialization.



## Module: pbkdf2

PBKDF2 derivation and verification.

Use `derive` to derive PBKDF2 outputs. Use `verify` to verify secret
against previously-derived outputs.

PBKDF2 is specified in [RFC 2898 Section 5.2] with test vectors given in
[RFC 6070]. See also [NIST Special Publication 800-132].

[RFC 2898 Section 5.2]: https://tools.ietf.org/html/rfc2898#section-5.2
[RFC 6070]: https://tools.ietf.org/html/rfc6070
[NIST Special Publication 800-132]:
   http://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-132.pdf

# Examples

## Password Database Example

```
use ring::{digest, pbkdf2};
use std::{collections::HashMap, num::NonZeroU32};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

enum Error {
    WrongUsernameOrPassword
}

struct PasswordDatabase {
    pbkdf2_iterations: NonZeroU32,
    db_salt_component: [u8; 16],

    // Normally this would be a persistent database.
    storage: HashMap<String, Credential>,
}

impl PasswordDatabase {
    pub fn store_password(&mut self, username: &str, password: &str) {
        let salt = self.salt(username);
        let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(PBKDF2_ALG, self.pbkdf2_iterations, &salt,
                       password.as_bytes(), &mut to_store);
        self.storage.insert(String::from(username), to_store);
    }

    pub fn verify_password(&self, username: &str, attempted_password: &str)
                           -> Result<(), Error> {
        match self.storage.get(username) {
           Some(actual_password) => {
               let salt = self.salt(username);
               pbkdf2::verify(PBKDF2_ALG, self.pbkdf2_iterations, &salt,
                              attempted_password.as_bytes(),
                              actual_password)
                    .map_err(|_| Error::WrongUsernameOrPassword)
           },

           None => Err(Error::WrongUsernameOrPassword)
        }
    }

    // The salt should have a user-specific component so that an attacker
    // cannot crack one password for multiple users in the database. It
    // should have a database-unique component so that an attacker cannot
    // crack the same user's password across databases in the unfortunate
    // but common case that the user has used the same password for
    // multiple systems.
    fn salt(&self, username: &str) -> Vec<u8> {
        let mut salt = Vec::with_capacity(self.db_salt_component.len() +
                                          username.as_bytes().len());
        salt.extend(self.db_salt_component.as_ref());
        salt.extend(username.as_bytes());
        salt
    }
}

fn main() {
    // Normally these parameters would be loaded from a configuration file.
    let mut db = PasswordDatabase {
        pbkdf2_iterations: NonZeroU32::new(100_000).unwrap(),
        db_salt_component: [
            // This value was generated from a secure PRNG.
            0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52,
            0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a
        ],
        storage: HashMap::new(),
    };

    db.store_password("alice", "@74d7]404j|W}6u");

    // An attempt to log in with the wrong password fails.
    assert!(db.verify_password("alice", "wrong password").is_err());

    // Normally there should be an expoentially-increasing delay between
    // attempts to further protect against online attacks.

    // An attempt to log in with the right password succeeds.
    assert!(db.verify_password("alice", "@74d7]404j|W}6u").is_ok());
}



## Module: pkcs8

PKCS#8 is specified in [RFC 5958].

[RFC 5958]: https://tools.ietf.org/html/rfc5958



## Module: rand

Cryptographic pseudo-random number generation.

*ring* functions that generate random bytes take a `&dyn SecureRandom`
parameter to make it clear which functions are non-deterministic.



## Module: rsa

RSA.



## Module: signature

Public key signatures: signing and verification.

Use the `verify` function to verify signatures, passing a reference to the
algorithm that identifies the algorithm. See the documentation for `verify`
for examples.

For signature verification, this API treats each combination of parameters
as a separate algorithm. For example, instead of having a single "RSA"
algorithm with a verification function that takes a bunch of parameters,
there are `RSA_PKCS1_2048_8192_SHA256`, `RSA_PKCS1_2048_8192_SHA384`, etc.,
which encode sets of parameter choices into objects. This is designed to
reduce the risks of algorithm agility and to provide consistency with ECDSA
and EdDSA.

Currently this module does not support digesting the message to be signed
separately from the public key operation, as it is currently being
optimized for Ed25519 and for the implementation of protocols that do not
requiring signing large messages. An interface for efficiently supporting
larger messages may be added later.


# Algorithm Details

## `ECDSA_*_ASN1` Details: ASN.1-encoded ECDSA Signatures

The signature is a ASN.1 DER-encoded `Ecdsa-Sig-Value` as described in
[RFC 3279 Section 2.2.3]. This is the form of ECDSA signature used in
X.509-related structures and in TLS's `ServerKeyExchange` messages.

The public key is encoding in uncompressed form using the
Octet-String-to-Elliptic-Curve-Point algorithm in
[SEC 1: Elliptic Curve Cryptography, Version 2.0].

During verification, the public key is validated using the ECC Partial
Public-Key Validation Routine from Section 5.6.2.3.3 of
[NIST Special Publication 800-56A, revision 2] and Appendix A.3 of the
NSA's [Suite B implementer's guide to FIPS 186-3]. Note that, as explained
in the NSA guide, ECC Partial Public-Key Validation is equivalent to ECC
Full Public-Key Validation for prime-order curves like this one.

## `ECDSA_*_FIXED` Details: Fixed-length (PKCS#11-style) ECDSA Signatures

The signature is *r*||*s*, where || denotes concatenation, and where both
*r* and *s* are both big-endian-encoded values that are left-padded to the
maximum length. A P-256 signature will be 64 bytes long (two 32-byte
components) and a P-384 signature will be 96 bytes long (two 48-byte
components). This is the form of ECDSA signature used PKCS#11 and DNSSEC.

The public key is encoding in uncompressed form using the
Octet-String-to-Elliptic-Curve-Point algorithm in
[SEC 1: Elliptic Curve Cryptography, Version 2.0].

During verification, the public key is validated using the ECC Partial
Public-Key Validation Routine from Section 5.6.2.3.3 of
[NIST Special Publication 800-56A, revision 2] and Appendix A.3 of the
NSA's [Suite B implementer's guide to FIPS 186-3]. Note that, as explained
in the NSA guide, ECC Partial Public-Key Validation is equivalent to ECC
Full Public-Key Validation for prime-order curves like this one.

## `RSA_PKCS1_*` Details: RSA PKCS#1 1.5 Signatures

The signature is an RSASSA-PKCS1-v1_5 signature as described in
[RFC 3447 Section 8.2].

The public key is encoded as an ASN.1 `RSAPublicKey` as described in
[RFC 3447 Appendix-A.1.1]. The public key modulus length, rounded *up* to
the nearest (larger) multiple of 8 bits, must be in the range given in the
name of the algorithm. The public exponent must be an odd integer of 2-33
bits, inclusive.


## `RSA_PSS_*` Details: RSA PSS Signatures

The signature is an RSASSA-PSS signature as described in
[RFC 3447 Section 8.1].

The public key is encoded as an ASN.1 `RSAPublicKey` as described in
[RFC 3447 Appendix-A.1.1]. The public key modulus length, rounded *up* to
the nearest (larger) multiple of 8 bits, must be in the range given in the
name of the algorithm. The public exponent must be an odd integer of 2-33
bits, inclusive.

During verification, signatures will only be accepted if the MGF1 digest
algorithm is the same as the message digest algorithm and if the salt
length is the same length as the message digest. This matches the
requirements in TLS 1.3 and other recent specifications.

During signing, the message digest algorithm will be used as the MGF1
digest algorithm. The salt will be the same length as the message digest.
This matches the requirements in TLS 1.3 and other recent specifications.
Additionally, the entire salt is randomly generated separately for each
signature using the secure random number generator passed to `sign()`.


[SEC 1: Elliptic Curve Cryptography, Version 2.0]:
    http://www.secg.org/sec1-v2.pdf
[NIST Special Publication 800-56A, revision 2]:
    http://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Ar2.pdf
[Suite B implementer's guide to FIPS 186-3]:
    https://github.com/briansmith/ring/blob/main/doc/ecdsa.pdf
[RFC 3279 Section 2.2.3]:
    https://tools.ietf.org/html/rfc3279#section-2.2.3
[RFC 3447 Section 8.2]:
    https://tools.ietf.org/html/rfc3447#section-7.2
[RFC 3447 Section 8.1]:
    https://tools.ietf.org/html/rfc3447#section-8.1
[RFC 3447 Appendix-A.1.1]:
    https://tools.ietf.org/html/rfc3447#appendix-A.1.1


# Examples

## Signing and verifying with Ed25519

```
use ring::{
    rand,
    signature::{self, KeyPair},
};

# fn main() -> Result<(), ring::error::Unspecified> {
// Generate a key pair in PKCS#8 (v2) format.
let rng = rand::SystemRandom::new();
let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

// Normally the application would store the PKCS#8 file persistently. Later
// it would read the PKCS#8 file from persistent storage to use it.

let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;

// Sign the message "hello, world".
const MESSAGE: &[u8] = b"hello, world";
let sig = key_pair.sign(MESSAGE);

// Normally an application would extract the bytes of the signature and
// send them in a protocol message to the peer(s). Here we just get the
// public key key directly from the key pair.
let peer_public_key_bytes = key_pair.public_key().as_ref();

// Verify the signature of the message using the public key. Normally the
// verifier of the message would parse the inputs to this code out of the
// protocol message(s) sent by the signer.
let peer_public_key =
    signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
peer_public_key.verify(MESSAGE, sig.as_ref())?;

# Ok(())
# }
```

## Signing and verifying with RSA (PKCS#1 1.5 padding)

By default OpenSSL writes RSA public keys in SubjectPublicKeyInfo format,
not RSAPublicKey format, and Base64-encodes them (“PEM” format).

To convert the PEM SubjectPublicKeyInfo format (“BEGIN PUBLIC KEY”) to the
binary RSAPublicKey format needed by `verify()`, use:

```sh
openssl rsa -pubin \
            -in public_key.pem \
            -inform PEM \
            -RSAPublicKey_out \
            -outform DER \
            -out public_key.der
```

To extract the RSAPublicKey-formatted public key from an ASN.1 (binary)
DER-encoded RSAPrivateKey format private key file, use:

```sh
openssl rsa -in private_key.der \
            -inform DER \
            -RSAPublicKey_out \
            -outform DER \
            -out public_key.der
```

```
# #[cfg(feature = "std")]
use ring::{rand, rsa, signature};

# #[cfg(feature = "std")]
fn sign_and_verify_rsa(private_key_path: &std::path::Path,
                       public_key_path: &std::path::Path)
                       -> Result<(), MyError> {
// Create an RSA keypair from the DER-encoded bytes. This example uses
// a 2048-bit key, but larger keys are also supported.
let private_key_der = read_file(private_key_path)?;
let key_pair = rsa::KeyPair::from_der(&private_key_der)
    .map_err(|_| MyError::BadPrivateKey)?;

// Sign the message "hello, world", using PKCS#1 v1.5 padding and the
// SHA256 digest algorithm.
const MESSAGE: &'static [u8] = b"hello, world";
let rng = rand::SystemRandom::new();
let mut signature = vec![0; key_pair.public().modulus_len()];
key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature)
    .map_err(|_| MyError::OOM)?;

// Verify the signature.
let public_key =
    signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256,
                                      read_file(public_key_path)?);
public_key.verify(MESSAGE, &signature)
    .map_err(|_| MyError::BadSignature)
}

#[derive(Debug)]
enum MyError {
#  #[cfg(feature = "std")]
   IO(std::io::Error),
   BadPrivateKey,
   OOM,
   BadSignature,
}

# #[cfg(feature = "std")]
fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).map_err(|e| MyError::IO(e))?;
    Ok(contents)
}
#
# #[cfg(not(feature = "std"))]
# fn sign_and_verify_rsa(_private_key_path: &std::path::Path,
#                        _public_key_path: &std::path::Path)
#                        -> Result<(), ()> {
#     Ok(())
# }
#
# fn main() {
#     let private_key_path =
#         std::path::Path::new("src/rsa/signature_rsa_example_private_key.der");
#     let public_key_path =
#         std::path::Path::new("src/rsa/signature_rsa_example_public_key.der");
#     sign_and_verify_rsa(&private_key_path, &public_key_path).unwrap()
# }
```



## Module: test

Testing framework.

Unlike the rest of *ring*, this testing framework uses panics pretty
liberally. It was originally designed for internal use--it drives most of
*ring*'s internal tests, and so it is optimized for getting *ring*'s tests
written quickly at the expense of some usability. The documentation is
lacking. The best way to learn it is to look at some examples. The digest
tests are the most complicated because they use named sections. Other tests
avoid named sections and so are easier to understand.

# Examples

## Writing Tests

Input files look like this:

```text
# This is a comment.

HMAC = SHA1
Input = "My test data"
Key = ""
Output = 61afdecb95429ef494d61fdee15990cabf0826fc

HMAC = SHA256
Input = "Sample message for keylen<blocklen"
Key = 000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F
Output = A28CF43130EE696A98F14A37678B56BCFCBDD9E5CF69717FECF5480F0EBDF790
```

Test cases are separated with blank lines. Note how the bytes of the `Key`
attribute are specified as a quoted string in the first test case and as
hex in the second test case; you can use whichever form is more convenient
and you can mix and match within the same file. The empty sequence of bytes
can only be represented with the quoted string form (`""`).

Here's how you would consume the test data:

```ignore
use ring::test;

test::run(test::test_file!("hmac_tests.txt"), |section, test_case| {
    assert_eq!(section, ""); // This test doesn't use named sections.

    let digest_alg = test_case.consume_digest_alg("HMAC");
    let input = test_case.consume_bytes("Input");
    let key = test_case.consume_bytes("Key");
    let output = test_case.consume_bytes("Output");

    // Do the actual testing here
});
```

Note that `consume_digest_alg` automatically maps the string "SHA1" to a
reference to `digest::SHA1_FOR_LEGACY_USE_ONLY`, "SHA256" to
`digest::SHA256`, etc.

## Output When a Test Fails

When a test case fails, the framework automatically prints out the test
case. If the test case failed with a panic, then the backtrace of the panic
will be printed too. For example, let's say the failing test case looks
like this:

```text
Curve = P-256
a = 2b11cb945c8cf152ffa4c9c2b1c965b019b35d0b7626919ef0ae6cb9d232f8af
b = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
r = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
```
If the test fails, this will be printed (if `$RUST_BACKTRACE` is `1`):

```text
src/example_tests.txt: Test panicked.
Curve = P-256
a = 2b11cb945c8cf152ffa4c9c2b1c965b019b35d0b7626919ef0ae6cb9d232f8af
b = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
r = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
thread 'example_test' panicked at 'Test failed.', src\test.rs:206
stack backtrace:
   0:     0x7ff654a05c7c - std::rt::lang_start::h61f4934e780b4dfc
   1:     0x7ff654a04f32 - std::rt::lang_start::h61f4934e780b4dfc
   2:     0x7ff6549f505d - std::panicking::rust_panic_with_hook::hfe203e3083c2b544
   3:     0x7ff654a0825b - rust_begin_unwind
   4:     0x7ff6549f63af - std::panicking::begin_panic_fmt::h484cd47786497f03
   5:     0x7ff654a07e9b - rust_begin_unwind
   6:     0x7ff654a0ae95 - core::panicking::panic_fmt::h257ceb0aa351d801
   7:     0x7ff654a0b190 - core::panicking::panic::h4bb1497076d04ab9
   8:     0x7ff65496dc41 - from_file<closure>
                        at C:\Users\Example\example\<core macros>:4
   9:     0x7ff65496d49c - example_test
                        at C:\Users\Example\example\src\example.rs:652
  10:     0x7ff6549d192a - test::stats::Summary::new::ha139494ed2e4e01f
  11:     0x7ff6549d51a2 - test::stats::Summary::new::ha139494ed2e4e01f
  12:     0x7ff654a0a911 - _rust_maybe_catch_panic
  13:     0x7ff6549d56dd - test::stats::Summary::new::ha139494ed2e4e01f
  14:     0x7ff654a03783 - std::sys::thread::Thread::new::h2b08da6cd2517f79
  15:     0x7ff968518101 - BaseThreadInitThunk
```

Notice that the output shows the name of the data file
(`src/example_tests.txt`), the test inputs that led to the failure, and the
stack trace to the line in the test code that panicked: entry 9 in the
stack trace pointing to line 652 of the file `example.rs`.



## ring::test_file

*Declarative Macro*

References a test input file.

```rust
macro_rules! test_file {
    ($file_name:expr) => { ... };
}
```



