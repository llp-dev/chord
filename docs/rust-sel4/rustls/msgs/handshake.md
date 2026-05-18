**rustls > msgs > handshake**

# Module: msgs::handshake

## Contents

**Structs**

- [`DistinguishedName`](#distinguishedname) - A `DistinguishedName` is a `Vec<u8>` wrapped in internal types.
- [`EchConfigContents`](#echconfigcontents)
- [`HandshakeMessagePayload`](#handshakemessagepayload)
- [`HpkeKeyConfig`](#hpkekeyconfig)
- [`HpkeSymmetricCipherSuite`](#hpkesymmetricciphersuite)
- [`UnknownExtension`](#unknownextension)

**Enums**

- [`EchConfigExtension`](#echconfigextension)
- [`EchConfigPayload`](#echconfigpayload) - An encrypted client hello (ECH) config.
- [`KeyExchangeAlgorithm`](#keyexchangealgorithm) - Describes supported key exchange mechanisms.

---

## rustls::msgs::handshake::DistinguishedName

*Struct*

A `DistinguishedName` is a `Vec<u8>` wrapped in internal types.

It contains the DER or BER encoded [`Subject` field from RFC 5280](https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.6)
for a single certificate. The Subject field is [encoded as an RFC 5280 `Name`](https://datatracker.ietf.org/doc/html/rfc5280#page-116).
It can be decoded using [x509-parser's FromDer trait](https://docs.rs/x509-parser/latest/x509_parser/prelude/trait.FromDer.html).

```ignore
for name in distinguished_names {
    use x509_parser::prelude::FromDer;
    println!("{}", x509_parser::x509::X509Name::from_der(&name.0)?.1);
}
```

The TLS encoding is defined in RFC5246: `opaque DistinguishedName<1..2^16-1>;`

**Tuple Struct**: `()`

**Methods:**

- `fn in_sequence(bytes: &[u8]) -> Self` - Create a [`DistinguishedName`] after prepending its outer SEQUENCE encoding.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(v: Vec<u8>) -> Self`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, InvalidMessage>`
- **Clone**
  - `fn clone(self: &Self) -> DistinguishedName`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## rustls::msgs::handshake::EchConfigContents

*Struct*

**Fields:**
- `key_config: HpkeKeyConfig`
- `maximum_name_length: u8`
- `public_name: pki_types::DnsName<'static>`
- `extensions: alloc::vec::Vec<EchConfigExtension>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &EchConfigContents) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> EchConfigContents`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, InvalidMessage>`



## rustls::msgs::handshake::EchConfigExtension

*Enum*

**Variants:**
- `Unknown(UnknownExtension)`



## rustls::msgs::handshake::EchConfigPayload

*Enum*

An encrypted client hello (ECH) config.

**Variants:**
- `V18(EchConfigContents)` - A recognized V18 ECH configuration.
- `Unknown{ version: crate::msgs::enums::EchVersion, contents: crate::msgs::base::PayloadU16 }` - An unknown version ECH configuration.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &EchConfigPayload) -> bool`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, InvalidMessage>`
- **Clone**
  - `fn clone(self: &Self) -> EchConfigPayload`



## rustls::msgs::handshake::HandshakeMessagePayload

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`



## rustls::msgs::handshake::HpkeKeyConfig

*Struct*

**Fields:**
- `config_id: u8`
- `kem_id: crate::msgs::enums::HpkeKem`
- `public_key: crate::msgs::base::PayloadU16<crate::msgs::base::NonEmpty>` - draft-ietf-tls-esni-24: `opaque HpkePublicKey<1..2^16-1>;`
- `symmetric_cipher_suites: alloc::vec::Vec<HpkeSymmetricCipherSuite>`

**Trait Implementations:**

- **Codec**
  - `fn encode(self: &Self, bytes: & mut Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, InvalidMessage>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &HpkeKeyConfig) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> HpkeKeyConfig`



## rustls::msgs::handshake::HpkeSymmetricCipherSuite

*Struct*

**Fields:**
- `kdf_id: crate::msgs::enums::HpkeKdf`
- `aead_id: crate::msgs::enums::HpkeAead`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Codec**
  - `fn encode(self: &Self, bytes: & mut Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, InvalidMessage>`
- **Clone**
  - `fn clone(self: &Self) -> HpkeSymmetricCipherSuite`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &HpkeSymmetricCipherSuite) -> bool`
- **Default**
  - `fn default() -> HpkeSymmetricCipherSuite`



## rustls::msgs::handshake::KeyExchangeAlgorithm

*Enum*

Describes supported key exchange mechanisms.

**Variants:**
- `DHE` - Diffie-Hellman Key exchange (with only known parameters as defined in [RFC 7919]).
- `ECDHE` - Key exchange performed via elliptic curve Diffie-Hellman.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> KeyExchangeAlgorithm`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &KeyExchangeAlgorithm) -> bool`



## rustls::msgs::handshake::UnknownExtension

*Struct*



