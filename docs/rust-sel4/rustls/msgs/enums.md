**rustls > msgs > enums**

# Module: msgs::enums

## Contents

**Enums**

- [`AlertLevel`](#alertlevel) - The `AlertLevel` TLS protocol enum.  Values in this enum are taken
- [`CertificateStatusType`](#certificatestatustype) - The `CertificateStatusType` TLS protocol enum.  Values in this enum are taken
- [`Compression`](#compression) - The `Compression` TLS protocol enum.  Values in this enum are taken
- [`ECPointFormat`](#ecpointformat) - The `ECPointFormat` TLS protocol enum.  Values in this enum are taken
- [`EchVersion`](#echversion) - The Encrypted Client Hello protocol version (`EchVersion`).
- [`ExtensionType`](#extensiontype) - The `ExtensionType` TLS protocol enum.  Values in this enum are taken
- [`HashAlgorithm`](#hashalgorithm) - The `HashAlgorithm` TLS protocol enum.  Values in this enum are taken
- [`HpkeAead`](#hpkeaead) - The Authenticated Encryption with Associated Data (`Aead`) type for HPKE operations.
- [`HpkeKdf`](#hpkekdf) - The Key Derivation Function (`Kdf`) type for HPKE operations.
- [`HpkeKem`](#hpkekem) - The Key Encapsulation Mechanism (`Kem`) type for HPKE operations.
- [`KeyUpdateRequest`](#keyupdaterequest) - The `KeyUpdateRequest` TLS protocol enum.  Values in this enum are taken
- [`NamedGroup`](#namedgroup) - The `NamedGroup` TLS protocol enum.  Values in this enum are taken
- [`PskKeyExchangeMode`](#pskkeyexchangemode) - The `PskKeyExchangeMode` TLS protocol enum.  Values in this enum are taken

---

## rustls::msgs::enums::AlertLevel

*Enum*

The `AlertLevel` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `Warning`
- `Fatal`
- `Unknown(u8)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 1]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`
- **PartialEq**
  - `fn eq(self: &Self, other: &AlertLevel) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AlertLevel`
- **From**
  - `fn from(x: u8) -> Self`



## rustls::msgs::enums::CertificateStatusType

*Enum*

The `CertificateStatusType` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `OCSP`
- `Unknown(u8)`



## rustls::msgs::enums::Compression

*Enum*

The `Compression` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `Null`
- `Deflate`
- `LSZ`
- `Unknown(u8)`



## rustls::msgs::enums::ECPointFormat

*Enum*

The `ECPointFormat` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `Uncompressed`
- `ANSIX962CompressedPrime`
- `ANSIX962CompressedChar2`
- `Unknown(u8)`



## rustls::msgs::enums::EchVersion

*Enum*

The Encrypted Client Hello protocol version (`EchVersion`).

Specified in [draft-ietf-tls-esni Section 4].
TODO(XXX): Update reference once RFC is published.

[draft-ietf-tls-esni Section 4]: <https://www.ietf.org/archive/id/draft-ietf-tls-esni-17.html#section-4>

**Variants:**
- `V18`
- `Unknown(u16)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 2]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &EchVersion) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> EchVersion`
- **From**
  - `fn from(x: u16) -> Self`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## rustls::msgs::enums::ExtensionType

*Enum*

The `ExtensionType` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `ServerName`
- `MaxFragmentLength`
- `ClientCertificateUrl`
- `TrustedCAKeys`
- `TruncatedHMAC`
- `StatusRequest`
- `UserMapping`
- `ClientAuthz`
- `ServerAuthz`
- `CertificateType`
- `EllipticCurves`
- `ECPointFormats`
- `SRP`
- `SignatureAlgorithms`
- `UseSRTP`
- `Heartbeat`
- `ALProtocolNegotiation`
- `SCT`
- `ClientCertificateType`
- `ServerCertificateType`
- `Padding`
- `ExtendedMasterSecret`
- `CompressCertificate`
- `SessionTicket`
- `PreSharedKey`
- `EarlyData`
- `SupportedVersions`
- `Cookie`
- `PSKKeyExchangeModes`
- `TicketEarlyDataInfo`
- `CertificateAuthorities`
- `OIDFilters`
- `PostHandshakeAuth`
- `SignatureAlgorithmsCert`
- `KeyShare`
- `TransportParameters`
- `NextProtocolNegotiation`
- `ChannelId`
- `RenegotiationInfo`
- `TransportParametersDraft`
- `EncryptedClientHello`
- `EncryptedClientHelloOuterExtensions`
- `Unknown(u16)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 2]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ExtensionType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ExtensionType`
- **From**
  - `fn from(x: u16) -> Self`



## rustls::msgs::enums::HashAlgorithm

*Enum*

The `HashAlgorithm` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `NONE`
- `MD5`
- `SHA1`
- `SHA224`
- `SHA256`
- `SHA384`
- `SHA512`
- `Unknown(u8)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 1]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`
- **From**
  - `fn from(x: u8) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &HashAlgorithm) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> HashAlgorithm`



## rustls::msgs::enums::HpkeAead

*Enum*

The Authenticated Encryption with Associated Data (`Aead`) type for HPKE operations.
Listed by IANA, as specified in [RFC 9180 Section 7.3]

[RFC 9180 Section 7.3]: <https://datatracker.ietf.org/doc/html/rfc9180#name-authenticated-encryption-wi>

**Variants:**
- `AES_128_GCM`
- `AES_256_GCM`
- `CHACHA20_POLY_1305`
- `EXPORT_ONLY`
- `Unknown(u16)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 2]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`
- **From**
  - `fn from(x: u16) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &HpkeAead) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> HpkeAead`
- **Default**
  - `fn default() -> HpkeAead`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## rustls::msgs::enums::HpkeKdf

*Enum*

The Key Derivation Function (`Kdf`) type for HPKE operations.
Listed by IANA, as specified in [RFC 9180 Section 7.2]

[RFC 9180 Section 7.2]: <https://datatracker.ietf.org/doc/html/rfc9180#name-key-derivation-functions-kd>

**Variants:**
- `HKDF_SHA256`
- `HKDF_SHA384`
- `HKDF_SHA512`
- `Unknown(u16)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 2]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &HpkeKdf) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> HpkeKdf`
- **Default**
  - `fn default() -> HpkeKdf`
- **From**
  - `fn from(x: u16) -> Self`



## rustls::msgs::enums::HpkeKem

*Enum*

The Key Encapsulation Mechanism (`Kem`) type for HPKE operations.
Listed by IANA, as specified in [RFC 9180 Section 7.1]

[RFC 9180 Section 7.1]: <https://datatracker.ietf.org/doc/html/rfc9180#kemid-values>

**Variants:**
- `DHKEM_P256_HKDF_SHA256`
- `DHKEM_P384_HKDF_SHA384`
- `DHKEM_P521_HKDF_SHA512`
- `DHKEM_X25519_HKDF_SHA256`
- `DHKEM_X448_HKDF_SHA512`
- `Unknown(u16)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 2]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &HpkeKem) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> HpkeKem`
- **From**
  - `fn from(x: u16) -> Self`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`



## rustls::msgs::enums::KeyUpdateRequest

*Enum*

The `KeyUpdateRequest` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `UpdateNotRequested`
- `UpdateRequested`
- `Unknown(u8)`



## rustls::msgs::enums::NamedGroup

*Enum*

The `NamedGroup` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `secp256r1`
- `secp384r1`
- `secp521r1`
- `X25519`
- `X448`
- `FFDHE2048`
- `FFDHE3072`
- `FFDHE4096`
- `FFDHE6144`
- `FFDHE8192`
- `MLKEM512`
- `MLKEM768`
- `MLKEM1024`
- `secp256r1MLKEM768`
- `X25519MLKEM768`
- `Unknown(u16)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 2]`
- `fn as_str(self: &Self) -> Option<&'static str>`
- `fn key_exchange_algorithm(self: Self) -> KeyExchangeAlgorithm` - Return the key exchange algorithm associated with this `NamedGroup`

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(x: u16) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NamedGroup) -> bool`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`
- **Clone**
  - `fn clone(self: &Self) -> NamedGroup`



## rustls::msgs::enums::PskKeyExchangeMode

*Enum*

The `PskKeyExchangeMode` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `PSK_KE`
- `PSK_DHE_KE`
- `Unknown(u8)`



