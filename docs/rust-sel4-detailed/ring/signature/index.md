*[ring](../index.md) / [signature](index.md)*

---

# Module `signature`

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







# Examples

## Signing and verifying with Ed25519

```rust
use ring::{
    rand,
    signature::{self, KeyPair},
};

fn main() -> Result<(), ring::error::Unspecified> {
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

Ok(())
}
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

```rust
#[cfg(feature = "std")]
use ring::{rand, rsa, signature};

#[cfg(feature = "std")]
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
 #[cfg(feature = "std")]
   IO(std::io::Error),
   BadPrivateKey,
   OOM,
   BadSignature,
}

#[cfg(feature = "std")]
fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).map_err(|e| MyError::IO(e))?;
    Ok(contents)
}

#[cfg(not(feature = "std"))]
fn sign_and_verify_rsa(_private_key_path: &std::path::Path,
                       _public_key_path: &std::path::Path)
                       -> Result<(), ()> {
    Ok(())
}

fn main() {
    let private_key_path =
        std::path::Path::new("src/rsa/signature_rsa_example_private_key.der");
    let public_key_path =
        std::path::Path::new("src/rsa/signature_rsa_example_public_key.der");
    sign_and_verify_rsa(&private_key_path, &public_key_path).unwrap()
}
```

## Contents

- [Structs](#structs)
  - [`Ed25519KeyPair`](#ed25519keypair)
  - [`EdDSAParameters`](#eddsaparameters)
  - [`EcdsaKeyPair`](#ecdsakeypair)
  - [`EcdsaSigningAlgorithm`](#ecdsasigningalgorithm)
  - [`EcdsaVerificationAlgorithm`](#ecdsaverificationalgorithm)
  - [`RsaPublicKeyComponents`](#rsapublickeycomponents)
  - [`RsaParameters`](#rsaparameters)
  - [`Signature`](#signature)
  - [`UnparsedPublicKey`](#unparsedpublickey)
- [Traits](#traits)
  - [`RsaEncoding`](#rsaencoding)
  - [`KeyPair`](#keypair)
  - [`VerificationAlgorithm`](#verificationalgorithm)
- [Type Aliases](#type-aliases)
  - [`RsaKeyPair`](#rsakeypair)
- [Constants](#constants)
  - [`ED25519_PUBLIC_KEY_LEN`](#ed25519-public-key-len)
  - [`MAX_LEN`](#max-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ed25519KeyPair`](#ed25519keypair) | struct |  |
| [`EdDSAParameters`](#eddsaparameters) | struct |  |
| [`EcdsaKeyPair`](#ecdsakeypair) | struct |  |
| [`EcdsaSigningAlgorithm`](#ecdsasigningalgorithm) | struct |  |
| [`EcdsaVerificationAlgorithm`](#ecdsaverificationalgorithm) | struct |  |
| [`RsaPublicKeyComponents`](#rsapublickeycomponents) | struct |  |
| [`RsaParameters`](#rsaparameters) | struct |  |
| [`Signature`](#signature) | struct | A public key signature returned from a signing operation. |
| [`UnparsedPublicKey`](#unparsedpublickey) | struct | An unparsed, possibly malformed, public key for signature verification. |
| [`RsaEncoding`](#rsaencoding) | trait |  |
| [`KeyPair`](#keypair) | trait | Key pairs for signing messages (private key and public key). |
| [`VerificationAlgorithm`](#verificationalgorithm) | trait | A signature verification algorithm. |
| [`RsaKeyPair`](#rsakeypair) | type | An RSA key pair, used for signing. |
| [`ED25519_PUBLIC_KEY_LEN`](#ed25519-public-key-len) | const |  |
| [`MAX_LEN`](#max-len) | const | The longest signature is an ASN.1 P-384 signature where *r* and *s* are of maximum length with the leading high bit set on each. |

## Structs

### `Ed25519KeyPair`

```rust
struct Ed25519KeyPair {
    private_scalar: Scalar,
    private_prefix: [u8; 32],
    public_key: PublicKey,
}
```

An Ed25519 key pair, for signing.

#### Implementations

- <span id="ed25519keypair-generate-pkcs8"></span>`fn generate_pkcs8(rng: &dyn rand::SecureRandom) -> Result<pkcs8::Document, error::Unspecified>` — [`SecureRandom`](../rand/index.md#securerandom), [`Document`](../pkcs8/index.md#document), [`Unspecified`](../error/index.md#unspecified)

  Generates a new key pair and returns the key pair serialized as a

  PKCS#8 document.

  

  The PKCS#8 document will be a v2 `OneAsymmetricKey` with the public key,

  as described in [RFC 5958 Section 2]; see [RFC 8410 Section 10.3] for an

  example.

  

- <span id="ed25519keypair-from-pkcs8"></span>`fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../error/index.md#keyrejected)

  Constructs an Ed25519 key pair by parsing an unencrypted PKCS#8 v2

  Ed25519 private key.

  

  `openssl genpkey -algorithm ED25519` generates PKCS# v1 keys, which

  require the use of `Ed25519KeyPair::from_pkcs8_maybe_unchecked()`

  instead of `Ed25519KeyPair::from_pkcs8()`.

  

  The input must be in PKCS#8 v2 format, and in particular it must contain

  the public key in addition to the private key. `from_pkcs8()` will

  verify that the public key and the private key are consistent with each

  other.

  

  Some early implementations of PKCS#8 v2, including earlier versions of

  *ring* and other implementations, wrapped the public key in the wrong

  ASN.1 tags. Both that incorrect form and the standardized form are

  accepted.

  

  If you need to parse PKCS#8 v1 files (without the public key) then use

  `Ed25519KeyPair::from_pkcs8_maybe_unchecked()` instead.

- <span id="ed25519keypair-from-pkcs8-maybe-unchecked"></span>`fn from_pkcs8_maybe_unchecked(pkcs8: &[u8]) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../error/index.md#keyrejected)

  Constructs an Ed25519 key pair by parsing an unencrypted PKCS#8 v1 or v2

  Ed25519 private key.

  

  `openssl genpkey -algorithm ED25519` generates PKCS# v1 keys.

  

  It is recommended to use `Ed25519KeyPair::from_pkcs8()`, which accepts

  only PKCS#8 v2 files that contain the public key.

  `from_pkcs8_maybe_unchecked()` parses PKCS#2 files exactly like

  `from_pkcs8()`. It also accepts v1 files. PKCS#8 v1 files do not contain

  the public key, so when a v1 file is parsed the public key will be

  computed from the private key, and there will be no consistency check

  between the public key and the private key.

  

  Some early implementations of PKCS#8 v2, including earlier versions of

  *ring* and other implementations, wrapped the public key in the wrong

  ASN.1 tags. Both that incorrect form and the standardized form are

  accepted.

  

  PKCS#8 v2 files are parsed exactly like `Ed25519KeyPair::from_pkcs8()`.

- <span id="ed25519keypair-from-seed-and-public-key"></span>`fn from_seed_and_public_key(seed: &[u8], public_key: &[u8]) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../error/index.md#keyrejected)

  Constructs an Ed25519 key pair from the private key seed `seed` and its

  public key `public_key`.

  

  It is recommended to use `Ed25519KeyPair::from_pkcs8()` instead.

  

  The private and public keys will be verified to be consistent with each

  other. This helps avoid misuse of the key (e.g. accidentally swapping

  the private key and public key, or using the wrong private key for the

  public key). This also detects any corruption of the public or private

  key.

- <span id="ed25519keypair-from-seed-unchecked"></span>`fn from_seed_unchecked(seed: &[u8]) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../error/index.md#keyrejected)

  Constructs a Ed25519 key pair from the private key seed `seed`.

  

  It is recommended to use `Ed25519KeyPair::from_pkcs8()` instead. When

  that is not practical, it is recommended to use

  `Ed25519KeyPair::from_seed_and_public_key()` instead.

  

  Since the public key is not given, the public key will be computed from

  the private key. It is not possible to detect misuse or corruption of

  the private key since the public key isn't given as input.

- <span id="ed25519keypair-from-seed"></span>`fn from_seed_(seed: &[u8; 32]) -> Self`

- <span id="ed25519keypair-sign"></span>`fn sign(&self, msg: &[u8]) -> signature::Signature` — [`Signature`](#signature)

  Returns the signature of the message `msg`.

#### Trait Implementations

##### `impl Debug for Ed25519KeyPair`

- <span id="ed25519keypair-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl KeyPair for Ed25519KeyPair`

- <span id="ed25519keypair-keypair-type-publickey"></span>`type PublicKey = PublicKey`

- <span id="ed25519keypair-keypair-public-key"></span>`fn public_key(&self) -> &<Self as >::PublicKey` — [`KeyPair`](#keypair)

### `EdDSAParameters`

```rust
struct EdDSAParameters;
```

Parameters for EdDSA signing and verification.

#### Trait Implementations

##### `impl Debug for EdDSAParameters`

- <span id="eddsaparameters-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

##### `impl Sealed for EdDSAParameters`

##### `impl VerificationAlgorithm for EdDSAParameters`

- <span id="eddsaparameters-verificationalgorithm-verify"></span>`fn verify(&self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>` — [`Unspecified`](../error/index.md#unspecified)

### `EcdsaKeyPair`

```rust
struct EcdsaKeyPair {
    d: elem::Elem<N, R>,
    nonce_key: NonceRandomKey,
    alg: &'static EcdsaSigningAlgorithm,
    public_key: PublicKey,
}
```

An ECDSA key pair, used for signing.

#### Implementations

- <span id="ecdsakeypair-generate-pkcs8"></span>`fn generate_pkcs8(alg: &'static EcdsaSigningAlgorithm, rng: &dyn rand::SecureRandom) -> Result<pkcs8::Document, error::Unspecified>` — [`EcdsaSigningAlgorithm`](../ec/suite_b/ecdsa/signing/index.md#ecdsasigningalgorithm), [`SecureRandom`](../rand/index.md#securerandom), [`Document`](../pkcs8/index.md#document), [`Unspecified`](../error/index.md#unspecified)

  Generates a new key pair and returns the key pair serialized as a

  PKCS#8 document.

  

  The PKCS#8 document will be a v1 `OneAsymmetricKey` with the public key

  included in the `ECPrivateKey` structure, as described in

  [RFC 5958 Section 2] and [RFC 5915]. The `ECPrivateKey` structure will

  not have a `parameters` field so the generated key is compatible with

  PKCS#11.

  

- <span id="ecdsakeypair-from-pkcs8"></span>`fn from_pkcs8(alg: &'static EcdsaSigningAlgorithm, pkcs8: &[u8], rng: &dyn rand::SecureRandom) -> Result<Self, error::KeyRejected>` — [`EcdsaSigningAlgorithm`](../ec/suite_b/ecdsa/signing/index.md#ecdsasigningalgorithm), [`SecureRandom`](../rand/index.md#securerandom), [`KeyRejected`](../error/index.md#keyrejected)

  Constructs an ECDSA key pair by parsing an unencrypted PKCS#8 v1

  id-ecPublicKey `ECPrivateKey` key.

  

  The input must be in PKCS#8 v1 format. It must contain the public key in

  the `ECPrivateKey` structure; `from_pkcs8()` will verify that the public

  key and the private key are consistent with each other. The algorithm

  identifier must identify the curve by name; it must not use an

  "explicit" encoding of the curve. The `parameters` field of the

  `ECPrivateKey`, if present, must be the same named curve that is in the

  algorithm identifier in the PKCS#8 header.

- <span id="ecdsakeypair-from-private-key-and-public-key"></span>`fn from_private_key_and_public_key(alg: &'static EcdsaSigningAlgorithm, private_key: &[u8], public_key: &[u8], rng: &dyn rand::SecureRandom) -> Result<Self, error::KeyRejected>` — [`EcdsaSigningAlgorithm`](../ec/suite_b/ecdsa/signing/index.md#ecdsasigningalgorithm), [`SecureRandom`](../rand/index.md#securerandom), [`KeyRejected`](../error/index.md#keyrejected)

  Constructs an ECDSA key pair from the private key and public key bytes

  

  The private key must encoded as a big-endian fixed-length integer. For

  example, a P-256 private key must be 32 bytes prefixed with leading

  zeros as needed.

  

  The public key is encoding in uncompressed form using the

  Octet-String-to-Elliptic-Curve-Point algorithm in

  [SEC 1: Elliptic Curve Cryptography, Version 2.0].

  

  This is intended for use by code that deserializes key pairs. It is

  recommended to use `EcdsaKeyPair::from_pkcs8()` (with a PKCS#8-encoded

  key) instead.

- <span id="ecdsakeypair-new"></span>`fn new(alg: &'static EcdsaSigningAlgorithm, key_pair: ec::KeyPair, rng: &dyn rand::SecureRandom) -> Result<Self, error::KeyRejected>` — [`EcdsaSigningAlgorithm`](../ec/suite_b/ecdsa/signing/index.md#ecdsasigningalgorithm), [`KeyPair`](../ec/keys/index.md#keypair), [`SecureRandom`](../rand/index.md#securerandom), [`KeyRejected`](../error/index.md#keyrejected)

- <span id="ecdsakeypair-sign"></span>`fn sign(&self, rng: &dyn rand::SecureRandom, message: &[u8]) -> Result<signature::Signature, error::Unspecified>` — [`SecureRandom`](../rand/index.md#securerandom), [`Signature`](#signature), [`Unspecified`](../error/index.md#unspecified)

  Returns the signature of the `message` using a random nonce generated by `rng`.

- <span id="ecdsakeypair-sign-digest"></span>`fn sign_digest(&self, h: digest::Digest, rng: &dyn rand::SecureRandom) -> Result<signature::Signature, error::Unspecified>` — [`Digest`](../digest/index.md#digest), [`SecureRandom`](../rand/index.md#securerandom), [`Signature`](#signature), [`Unspecified`](../error/index.md#unspecified)

  Returns the signature of message digest `h` using a "random" nonce

  generated by `rng`.

#### Trait Implementations

##### `impl Debug for EcdsaKeyPair`

- <span id="ecdsakeypair-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl KeyPair for EcdsaKeyPair`

- <span id="ecdsakeypair-keypair-type-publickey"></span>`type PublicKey = PublicKey`

- <span id="ecdsakeypair-keypair-public-key"></span>`fn public_key(&self) -> &<Self as >::PublicKey` — [`KeyPair`](#keypair)

### `EcdsaSigningAlgorithm`

```rust
struct EcdsaSigningAlgorithm {
    curve: &'static ec::Curve,
    private_scalar_ops: &'static PrivateScalarOps,
    private_key_ops: &'static PrivateKeyOps,
    digest_alg: &'static digest::Algorithm,
    pkcs8_template: &'static pkcs8::Template,
    format_rs: fn(&'static ScalarOps, &elem::Elem<N, Unencoded>, &elem::Elem<N, Unencoded>, &mut [u8]) -> usize,
    id: AlgorithmID,
}
```

An ECDSA signing algorithm.

#### Trait Implementations

##### `impl Debug for EcdsaSigningAlgorithm`

- <span id="ecdsasigningalgorithm-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl Eq for EcdsaSigningAlgorithm`

##### `impl PartialEq for EcdsaSigningAlgorithm`

- <span id="ecdsasigningalgorithm-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for EcdsaSigningAlgorithm`

### `EcdsaVerificationAlgorithm`

```rust
struct EcdsaVerificationAlgorithm {
    ops: &'static PublicScalarOps,
    digest_alg: &'static digest::Algorithm,
    split_rs: fn(&'static ScalarOps, &mut untrusted::Reader<'a>) -> Result<(untrusted::Input<'a>, untrusted::Input<'a>), error::Unspecified>,
    id: AlgorithmID,
}
```

An ECDSA verification algorithm.

#### Implementations

- <span id="ecdsaverificationalgorithm-verify-digest"></span>`fn verify_digest(&self, public_key: untrusted::Input<'_>, e: elem::Elem<N, Unencoded>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>` — [`Elem`](../ec/suite_b/ops/index.md#elem), [`N`](../ec/suite_b/ops/index.md#n), [`Unencoded`](../arithmetic/montgomery/index.md#unencoded), [`Unspecified`](../error/index.md#unspecified)

  This is intentionally not public.

#### Trait Implementations

##### `impl Debug for EcdsaVerificationAlgorithm`

- <span id="ecdsaverificationalgorithm-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl Sealed for EcdsaVerificationAlgorithm`

##### `impl VerificationAlgorithm for EcdsaVerificationAlgorithm`

- <span id="ecdsaverificationalgorithm-verificationalgorithm-verify"></span>`fn verify(&self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>` — [`Unspecified`](../error/index.md#unspecified)

### `RsaPublicKeyComponents<B>`

```rust
struct RsaPublicKeyComponents<B> {
    pub n: B,
    pub e: B,
}
```

RSA public key components.

`B` must implement `AsRef<[u8]>` like `&[u8]` or `Vec<u8>`.

#### Fields

- **`n`**: `B`

  The public modulus, encoded in big-endian bytes without leading zeros.

- **`e`**: `B`

  The public exponent, encoded in big-endian bytes without leading zeros.

#### Implementations

- <span id="superpublickeycomponents-verify"></span>`fn verify(&self, params: &RsaParameters, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>` — [`RsaParameters`](../rsa/index.md#rsaparameters), [`Unspecified`](../error/index.md#unspecified)

  Verifies that `signature` is a valid signature of `message` using `self`

  as the public key. `params` determine what algorithm parameters

  (padding, digest algorithm, key length range, etc.) are used in the

  verification.

  

  When the public key is in DER-encoded PKCS#1 ASN.1 format, it is

  recommended to use `ring::signature::verify()` with

  `ring::signature::RSA_PKCS1_*`, because `ring::signature::verify()`

  will handle the parsing in that case. Otherwise, this function can be used

  to pass in the raw bytes for the public key components as

  `untrusted::Input` arguments.

#### Trait Implementations

##### `impl<B: clone::Clone> Clone for PublicKeyComponents<B>`

- <span id="publickeycomponents-clone"></span>`fn clone(&self) -> PublicKeyComponents<B>` — [`PublicKeyComponents`](../rsa/public_key_components/index.md#publickeycomponents)

##### `impl<B: marker::Copy> Copy for PublicKeyComponents<B>`

##### `impl<B> Debug for PublicKeyComponents<B>`

- <span id="publickeycomponents-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `RsaParameters`

```rust
struct RsaParameters {
    padding_alg: &'static dyn padding::Verification,
    min_bits: bits::BitLength,
}
```

Parameters for RSA verification.

#### Trait Implementations

##### `impl Debug for RsaParameters`

- <span id="rsaparameters-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Sealed for super::RsaParameters`

##### `impl VerificationAlgorithm for super::RsaParameters`

- <span id="superrsaparameters-verificationalgorithm-verify"></span>`fn verify(&self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>` — [`Unspecified`](../error/index.md#unspecified)

### `Signature`

```rust
struct Signature {
    value: [u8; 105],
    len: usize,
}
```

A public key signature returned from a signing operation.

#### Implementations

- <span id="signature-new"></span>`fn new<F>(fill: F) -> Self`

#### Trait Implementations

##### `impl AsRef for Signature`

- <span id="signature-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for Signature`

- <span id="signature-clone"></span>`fn clone(&self) -> Signature` — [`Signature`](#signature)

##### `impl Copy for Signature`

### `UnparsedPublicKey<B>`

```rust
struct UnparsedPublicKey<B> {
    algorithm: &'static dyn VerificationAlgorithm,
    bytes: B,
}
```

An unparsed, possibly malformed, public key for signature verification.

#### Implementations

- <span id="unparsedpublickey-new"></span>`fn new(algorithm: &'static dyn VerificationAlgorithm, bytes: B) -> Self` — [`VerificationAlgorithm`](#verificationalgorithm)

  Construct a new `UnparsedPublicKey`.

  

  No validation of `bytes` is done until `verify()` is called.

- <span id="unparsedpublickey-verify"></span>`fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>` — [`Unspecified`](../error/index.md#unspecified)

  Parses the public key and verifies `signature` is a valid signature of

  `message` using it.

  

  See the [crate::signature] module-level documentation for examples.

#### Trait Implementations

##### `impl<B> AsRef for UnparsedPublicKey<B>`

- <span id="unparsedpublickey-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl<B: clone::Clone> Clone for UnparsedPublicKey<B>`

- <span id="unparsedpublickey-clone"></span>`fn clone(&self) -> UnparsedPublicKey<B>` — [`UnparsedPublicKey`](#unparsedpublickey)

##### `impl<B: marker::Copy> Copy for UnparsedPublicKey<B>`

##### `impl<B> Debug for UnparsedPublicKey<B>`

- <span id="unparsedpublickey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

## Traits

### `RsaEncoding`

```rust
trait RsaEncoding: Padding { ... }
```

An RSA signature encoding as described in [RFC 3447 Section 8].


#### Implementors

- [`PKCS1`](../rsa/padding/pkcs1/index.md#pkcs1)
- [`PSS`](../rsa/padding/pss/index.md#pss)

### `KeyPair`

```rust
trait KeyPair: core::fmt::Debug + Send + Sized + Sync { ... }
```

Key pairs for signing messages (private key and public key).

#### Associated Types

- `type PublicKey: 6`

#### Required Methods

- `fn public_key(&self) -> &<Self as >::PublicKey`

  The public key for the key pair.

#### Implementors

- [`EcdsaKeyPair`](../ec/suite_b/ecdsa/signing/index.md#ecdsakeypair)
- [`Ed25519KeyPair`](../ec/curve25519/ed25519/signing/index.md#ed25519keypair)
- [`KeyPair`](../rsa/keypair/index.md#keypair)

### `VerificationAlgorithm`

```rust
trait VerificationAlgorithm: core::fmt::Debug + Sync + sealed::Sealed { ... }
```

A signature verification algorithm.

#### Required Methods

- `fn verify(&self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>`

  Verify the signature `signature` of message `msg` with the public key

#### Implementors

- [`EcdsaVerificationAlgorithm`](../ec/suite_b/ecdsa/verification/index.md#ecdsaverificationalgorithm)
- [`EdDSAParameters`](../ec/curve25519/ed25519/verification/index.md#eddsaparameters)
- [`RsaParameters`](../rsa/index.md#rsaparameters)

## Type Aliases

### `RsaKeyPair`

```rust
type RsaKeyPair = crate::rsa::KeyPair;
```

An RSA key pair, used for signing.

## Constants

### `ED25519_PUBLIC_KEY_LEN`
```rust
const ED25519_PUBLIC_KEY_LEN: usize = 32usize;
```

The length of an Ed25519 public key.

### `MAX_LEN`
```rust
const MAX_LEN: usize = 105usize;
```

The longest signature is an ASN.1 P-384 signature where *r* and *s* are of
maximum length with the leading high bit set on each. Then each component
will have a tag, a one-byte length, and a one-byte “I'm not negative”
prefix, and the outer sequence will have a two-byte length.

