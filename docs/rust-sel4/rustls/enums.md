**rustls > enums**

# Module: enums

## Contents

**Enums**

- [`AlertDescription`](#alertdescription) - The `AlertDescription` TLS protocol enum.  Values in this enum are taken
- [`CertificateCompressionAlgorithm`](#certificatecompressionalgorithm) - The "TLS Certificate Compression Algorithm IDs" TLS protocol enum.
- [`CertificateType`](#certificatetype) - The `CertificateType` enum sent in the cert_type extensions.
- [`CipherSuite`](#ciphersuite) - The `CipherSuite` TLS protocol enum.  Values in this enum are taken
- [`ContentType`](#contenttype) - The `ContentType` TLS protocol enum.  Values in this enum are taken
- [`EchClientHelloType`](#echclienthellotype) - The type of Encrypted Client Hello (`EchClientHelloType`).
- [`HandshakeType`](#handshaketype) - The `HandshakeType` TLS protocol enum.  Values in this enum are taken
- [`ProtocolVersion`](#protocolversion) - The `ProtocolVersion` TLS protocol enum.  Values in this enum are taken
- [`SignatureAlgorithm`](#signaturealgorithm) - The `SignatureAlgorithm` TLS protocol enum.  Values in this enum are taken
- [`SignatureScheme`](#signaturescheme) - The `SignatureScheme` TLS protocol enum.  Values in this enum are taken

---

## rustls::enums::AlertDescription

*Enum*

The `AlertDescription` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `CloseNotify`
- `UnexpectedMessage`
- `BadRecordMac`
- `DecryptionFailed`
- `RecordOverflow`
- `DecompressionFailure`
- `HandshakeFailure`
- `NoCertificate`
- `BadCertificate`
- `UnsupportedCertificate`
- `CertificateRevoked`
- `CertificateExpired`
- `CertificateUnknown`
- `IllegalParameter`
- `UnknownCA`
- `AccessDenied`
- `DecodeError`
- `DecryptError`
- `ExportRestriction`
- `ProtocolVersion`
- `InsufficientSecurity`
- `InternalError`
- `InappropriateFallback`
- `UserCanceled`
- `NoRenegotiation`
- `MissingExtension`
- `UnsupportedExtension`
- `CertificateUnobtainable`
- `UnrecognisedName`
- `BadCertificateStatusResponse`
- `BadCertificateHashValue`
- `UnknownPSKIdentity`
- `CertificateRequired`
- `NoApplicationProtocol`
- `EncryptedClientHelloRequired`
- `Unknown(u8)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 1]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(e: InvalidMessage) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &AlertDescription) -> bool`
- **From**
  - `fn from(e: CertificateError) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> AlertDescription`
- **From**
  - `fn from(x: u8) -> Self`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`



## rustls::enums::CertificateCompressionAlgorithm

*Enum*

The "TLS Certificate Compression Algorithm IDs" TLS protocol enum.
Values in this enum are taken from [RFC8879].

[RFC8879]: https://www.rfc-editor.org/rfc/rfc8879.html#section-7.3

**Variants:**
- `Zlib`
- `Brotli`
- `Zstd`
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
  - `fn eq(self: &Self, other: &CertificateCompressionAlgorithm) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CertificateCompressionAlgorithm`
- **From**
  - `fn from(x: u16) -> Self`



## rustls::enums::CertificateType

*Enum*

The `CertificateType` enum sent in the cert_type extensions.
Values in this enum are taken from the various RFCs covering TLS, and are listed by IANA.

[RFC 6091 Section 5]: <https://datatracker.ietf.org/doc/html/rfc6091#section-5>
[RFC 7250 Section 7]: <https://datatracker.ietf.org/doc/html/rfc7250#section-7>

**Variants:**
- `X509`
- `RawPublicKey`
- `Unknown(u8)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 1]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`
- **PartialEq**
  - `fn eq(self: &Self, other: &CertificateType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CertificateType`
- **Default**
  - `fn default() -> CertificateType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(x: u8) -> Self`



## rustls::enums::CipherSuite

*Enum*

The `CipherSuite` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `TLS_NULL_WITH_NULL_NULL`
- `TLS_PSK_WITH_AES_128_GCM_SHA256`
- `TLS_PSK_WITH_AES_256_GCM_SHA384`
- `TLS_EMPTY_RENEGOTIATION_INFO_SCSV`
- `TLS13_AES_128_GCM_SHA256`
- `TLS13_AES_256_GCM_SHA384`
- `TLS13_CHACHA20_POLY1305_SHA256`
- `TLS13_AES_128_CCM_SHA256`
- `TLS13_AES_128_CCM_8_SHA256`
- `TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA`
- `TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA`
- `TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA`
- `TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA`
- `TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256`
- `TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384`
- `TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256`
- `TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384`
- `TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256`
- `TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384`
- `TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256`
- `TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384`
- `TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256`
- `TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256`
- `TLS_RSA_WITH_NULL_MD5`
- `TLS_RSA_WITH_NULL_SHA`
- `TLS_RSA_EXPORT_WITH_RC4_40_MD5`
- `TLS_RSA_WITH_RC4_128_MD5`
- `TLS_RSA_WITH_RC4_128_SHA`
- `TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5`
- `TLS_RSA_WITH_IDEA_CBC_SHA`
- `TLS_RSA_EXPORT_WITH_DES40_CBC_SHA`
- `TLS_RSA_WITH_DES_CBC_SHA`
- `TLS_RSA_WITH_3DES_EDE_CBC_SHA`
- `TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA`
- `TLS_DH_DSS_WITH_DES_CBC_SHA`
- `TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA`
- `TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA`
- `TLS_DH_RSA_WITH_DES_CBC_SHA`
- `TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA`
- `TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA`
- `TLS_DHE_DSS_WITH_DES_CBC_SHA`
- `TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA`
- `TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA`
- `TLS_DHE_RSA_WITH_DES_CBC_SHA`
- `TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA`
- `TLS_DH_anon_EXPORT_WITH_RC4_40_MD5`
- `TLS_DH_anon_WITH_RC4_128_MD5`
- `TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA`
- `TLS_DH_anon_WITH_DES_CBC_SHA`
- `TLS_DH_anon_WITH_3DES_EDE_CBC_SHA`
- `SSL_FORTEZZA_KEA_WITH_NULL_SHA`
- `SSL_FORTEZZA_KEA_WITH_FORTEZZA_CBC_SHA`
- `TLS_KRB5_WITH_DES_CBC_SHA_or_SSL_FORTEZZA_KEA_WITH_RC4_128_SHA`
- `TLS_KRB5_WITH_3DES_EDE_CBC_SHA`
- `TLS_KRB5_WITH_RC4_128_SHA`
- `TLS_KRB5_WITH_IDEA_CBC_SHA`
- `TLS_KRB5_WITH_DES_CBC_MD5`
- `TLS_KRB5_WITH_3DES_EDE_CBC_MD5`
- `TLS_KRB5_WITH_RC4_128_MD5`
- `TLS_KRB5_WITH_IDEA_CBC_MD5`
- `TLS_KRB5_EXPORT_WITH_DES_CBC_40_SHA`
- `TLS_KRB5_EXPORT_WITH_RC2_CBC_40_SHA`
- `TLS_KRB5_EXPORT_WITH_RC4_40_SHA`
- `TLS_KRB5_EXPORT_WITH_DES_CBC_40_MD5`
- `TLS_KRB5_EXPORT_WITH_RC2_CBC_40_MD5`
- `TLS_KRB5_EXPORT_WITH_RC4_40_MD5`
- `TLS_PSK_WITH_NULL_SHA`
- `TLS_DHE_PSK_WITH_NULL_SHA`
- `TLS_RSA_PSK_WITH_NULL_SHA`
- `TLS_RSA_WITH_AES_128_CBC_SHA`
- `TLS_DH_DSS_WITH_AES_128_CBC_SHA`
- `TLS_DH_RSA_WITH_AES_128_CBC_SHA`
- `TLS_DHE_DSS_WITH_AES_128_CBC_SHA`
- `TLS_DHE_RSA_WITH_AES_128_CBC_SHA`
- `TLS_DH_anon_WITH_AES_128_CBC_SHA`
- `TLS_RSA_WITH_AES_256_CBC_SHA`
- `TLS_DH_DSS_WITH_AES_256_CBC_SHA`
- `TLS_DH_RSA_WITH_AES_256_CBC_SHA`
- `TLS_DHE_DSS_WITH_AES_256_CBC_SHA`
- `TLS_DHE_RSA_WITH_AES_256_CBC_SHA`
- `TLS_DH_anon_WITH_AES_256_CBC_SHA`
- `TLS_RSA_WITH_NULL_SHA256`
- `TLS_RSA_WITH_AES_128_CBC_SHA256`
- `TLS_RSA_WITH_AES_256_CBC_SHA256`
- `TLS_DH_DSS_WITH_AES_128_CBC_SHA256`
- `TLS_DH_RSA_WITH_AES_128_CBC_SHA256`
- `TLS_DHE_DSS_WITH_AES_128_CBC_SHA256`
- `TLS_RSA_WITH_CAMELLIA_128_CBC_SHA`
- `TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA`
- `TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA`
- `TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA`
- `TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA`
- `TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA`
- `TLS_ECDH_ECDSA_WITH_NULL_SHA_draft`
- `TLS_ECDH_ECDSA_WITH_RC4_128_SHA_draft`
- `TLS_ECDH_ECDSA_WITH_DES_CBC_SHA_draft`
- `TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA_draft`
- `TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA_draft`
- `TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA_draft`
- `TLS_ECDH_ECNRA_WITH_DES_CBC_SHA_draft`
- `TLS_ECDH_ECNRA_WITH_3DES_EDE_CBC_SHA_draft`
- `TLS_ECMQV_ECDSA_NULL_SHA_draft`
- `TLS_ECMQV_ECDSA_WITH_RC4_128_SHA_draft`
- `TLS_ECMQV_ECDSA_WITH_DES_CBC_SHA_draft`
- `TLS_ECMQV_ECDSA_WITH_3DES_EDE_CBC_SHA_draft`
- `TLS_ECMQV_ECNRA_NULL_SHA_draft`
- `TLS_ECMQV_ECNRA_WITH_RC4_128_SHA_draft`
- `TLS_ECMQV_ECNRA_WITH_DES_CBC_SHA_draft`
- `TLS_ECMQV_ECNRA_WITH_3DES_EDE_CBC_SHA_draft`
- `TLS_ECDH_anon_NULL_WITH_SHA_draft`
- `TLS_ECDH_anon_WITH_RC4_128_SHA_draft`
- `TLS_ECDH_anon_WITH_DES_CBC_SHA_draft`
- `TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA_draft`
- `TLS_ECDH_anon_EXPORT_WITH_DES40_CBC_SHA_draft`
- `TLS_ECDH_anon_EXPORT_WITH_RC4_40_SHA_draft`
- `TLS_RSA_EXPORT1024_WITH_RC4_56_MD5`
- `TLS_RSA_EXPORT1024_WITH_RC2_CBC_56_MD5`
- `TLS_RSA_EXPORT1024_WITH_DES_CBC_SHA`
- `TLS_DHE_DSS_EXPORT1024_WITH_DES_CBC_SHA`
- `TLS_RSA_EXPORT1024_WITH_RC4_56_SHA`
- `TLS_DHE_DSS_EXPORT1024_WITH_RC4_56_SHA`
- `TLS_DHE_DSS_WITH_RC4_128_SHA`
- `TLS_DHE_RSA_WITH_AES_128_CBC_SHA256`
- `TLS_DH_DSS_WITH_AES_256_CBC_SHA256`
- `TLS_DH_RSA_WITH_AES_256_CBC_SHA256`
- `TLS_DHE_DSS_WITH_AES_256_CBC_SHA256`
- `TLS_DHE_RSA_WITH_AES_256_CBC_SHA256`
- `TLS_DH_anon_WITH_AES_128_CBC_SHA256`
- `TLS_DH_anon_WITH_AES_256_CBC_SHA256`
- `TLS_DHE_DSS_WITH_3DES_EDE_CBC_RMD`
- `TLS_DHE_DSS_WITH_AES_128_CBC_RMD`
- `TLS_DHE_DSS_WITH_AES_256_CBC_RMD`
- `TLS_DHE_RSA_WITH_3DES_EDE_CBC_RMD`
- `TLS_DHE_RSA_WITH_AES_128_CBC_RMD`
- `TLS_DHE_RSA_WITH_AES_256_CBC_RMD`
- `TLS_RSA_WITH_3DES_EDE_CBC_RMD`
- `TLS_RSA_WITH_AES_128_CBC_RMD`
- `TLS_RSA_WITH_AES_256_CBC_RMD`
- `TLS_GOSTR341094_WITH_28147_CNT_IMIT`
- `TLS_GOSTR341001_WITH_28147_CNT_IMIT`
- `TLS_GOSTR341094_WITH_NULL_GOSTR3411`
- `TLS_GOSTR341001_WITH_NULL_GOSTR3411`
- `TLS_RSA_WITH_CAMELLIA_256_CBC_SHA`
- `TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA`
- `TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA`
- `TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA`
- `TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA`
- `TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA`
- `TLS_PSK_WITH_RC4_128_SHA`
- `TLS_PSK_WITH_3DES_EDE_CBC_SHA`
- `TLS_PSK_WITH_AES_128_CBC_SHA`
- `TLS_PSK_WITH_AES_256_CBC_SHA`
- `TLS_DHE_PSK_WITH_RC4_128_SHA`
- `TLS_DHE_PSK_WITH_3DES_EDE_CBC_SHA`
- `TLS_DHE_PSK_WITH_AES_128_CBC_SHA`
- `TLS_DHE_PSK_WITH_AES_256_CBC_SHA`
- `TLS_RSA_PSK_WITH_RC4_128_SHA`
- `TLS_RSA_PSK_WITH_3DES_EDE_CBC_SHA`
- `TLS_RSA_PSK_WITH_AES_128_CBC_SHA`
- `TLS_RSA_PSK_WITH_AES_256_CBC_SHA`
- `TLS_RSA_WITH_SEED_CBC_SHA`
- `TLS_DH_DSS_WITH_SEED_CBC_SHA`
- `TLS_DH_RSA_WITH_SEED_CBC_SHA`
- `TLS_DHE_DSS_WITH_SEED_CBC_SHA`
- `TLS_DHE_RSA_WITH_SEED_CBC_SHA`
- `TLS_DH_anon_WITH_SEED_CBC_SHA`
- `TLS_RSA_WITH_AES_128_GCM_SHA256`
- `TLS_RSA_WITH_AES_256_GCM_SHA384`
- `TLS_DHE_RSA_WITH_AES_128_GCM_SHA256`
- `TLS_DHE_RSA_WITH_AES_256_GCM_SHA384`
- `TLS_DH_RSA_WITH_AES_128_GCM_SHA256`
- `TLS_DH_RSA_WITH_AES_256_GCM_SHA384`
- `TLS_DHE_DSS_WITH_AES_128_GCM_SHA256`
- `TLS_DHE_DSS_WITH_AES_256_GCM_SHA384`
- `TLS_DH_DSS_WITH_AES_128_GCM_SHA256`
- `TLS_DH_DSS_WITH_AES_256_GCM_SHA384`
- `TLS_DH_anon_WITH_AES_128_GCM_SHA256`
- `TLS_DH_anon_WITH_AES_256_GCM_SHA384`
- `TLS_DHE_PSK_WITH_AES_128_GCM_SHA256`
- `TLS_DHE_PSK_WITH_AES_256_GCM_SHA384`
- `TLS_RSA_PSK_WITH_AES_128_GCM_SHA256`
- `TLS_RSA_PSK_WITH_AES_256_GCM_SHA384`
- `TLS_PSK_WITH_AES_128_CBC_SHA256`
- `TLS_PSK_WITH_AES_256_CBC_SHA384`
- `TLS_PSK_WITH_NULL_SHA256`
- `TLS_PSK_WITH_NULL_SHA384`
- `TLS_DHE_PSK_WITH_AES_128_CBC_SHA256`
- `TLS_DHE_PSK_WITH_AES_256_CBC_SHA384`
- `TLS_DHE_PSK_WITH_NULL_SHA256`
- `TLS_DHE_PSK_WITH_NULL_SHA384`
- `TLS_RSA_PSK_WITH_AES_128_CBC_SHA256`
- `TLS_RSA_PSK_WITH_AES_256_CBC_SHA384`
- `TLS_RSA_PSK_WITH_NULL_SHA256`
- `TLS_RSA_PSK_WITH_NULL_SHA384`
- `TLS_RSA_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_RSA_WITH_CAMELLIA_256_CBC_SHA256`
- `TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA256`
- `TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA256`
- `TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA256`
- `TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA256`
- `TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA256`
- `TLS_ECDH_ECDSA_WITH_NULL_SHA`
- `TLS_ECDH_ECDSA_WITH_RC4_128_SHA`
- `TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA`
- `TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA`
- `TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA`
- `TLS_ECDHE_ECDSA_WITH_NULL_SHA`
- `TLS_ECDHE_ECDSA_WITH_RC4_128_SHA`
- `TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA`
- `TLS_ECDH_RSA_WITH_NULL_SHA`
- `TLS_ECDH_RSA_WITH_RC4_128_SHA`
- `TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA`
- `TLS_ECDH_RSA_WITH_AES_128_CBC_SHA`
- `TLS_ECDH_RSA_WITH_AES_256_CBC_SHA`
- `TLS_ECDHE_RSA_WITH_NULL_SHA`
- `TLS_ECDHE_RSA_WITH_RC4_128_SHA`
- `TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA`
- `TLS_ECDH_anon_WITH_NULL_SHA`
- `TLS_ECDH_anon_WITH_RC4_128_SHA`
- `TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA`
- `TLS_ECDH_anon_WITH_AES_128_CBC_SHA`
- `TLS_ECDH_anon_WITH_AES_256_CBC_SHA`
- `TLS_SRP_SHA_WITH_3DES_EDE_CBC_SHA`
- `TLS_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA`
- `TLS_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA`
- `TLS_SRP_SHA_WITH_AES_128_CBC_SHA`
- `TLS_SRP_SHA_RSA_WITH_AES_128_CBC_SHA`
- `TLS_SRP_SHA_DSS_WITH_AES_128_CBC_SHA`
- `TLS_SRP_SHA_WITH_AES_256_CBC_SHA`
- `TLS_SRP_SHA_RSA_WITH_AES_256_CBC_SHA`
- `TLS_SRP_SHA_DSS_WITH_AES_256_CBC_SHA`
- `TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA256`
- `TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA384`
- `TLS_ECDH_RSA_WITH_AES_128_CBC_SHA256`
- `TLS_ECDH_RSA_WITH_AES_256_CBC_SHA384`
- `TLS_ECDH_ECDSA_WITH_AES_128_GCM_SHA256`
- `TLS_ECDH_ECDSA_WITH_AES_256_GCM_SHA384`
- `TLS_ECDH_RSA_WITH_AES_128_GCM_SHA256`
- `TLS_ECDH_RSA_WITH_AES_256_GCM_SHA384`
- `TLS_ECDHE_PSK_WITH_RC4_128_SHA`
- `TLS_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA`
- `TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA`
- `TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA`
- `TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA256`
- `TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA384`
- `TLS_ECDHE_PSK_WITH_NULL_SHA`
- `TLS_ECDHE_PSK_WITH_NULL_SHA256`
- `TLS_ECDHE_PSK_WITH_NULL_SHA384`
- `TLS_RSA_WITH_ARIA_128_CBC_SHA256`
- `TLS_RSA_WITH_ARIA_256_CBC_SHA384`
- `TLS_DH_DSS_WITH_ARIA_128_CBC_SHA256`
- `TLS_DH_DSS_WITH_ARIA_256_CBC_SHA384`
- `TLS_DH_RSA_WITH_ARIA_128_CBC_SHA256`
- `TLS_DH_RSA_WITH_ARIA_256_CBC_SHA384`
- `TLS_DHE_DSS_WITH_ARIA_128_CBC_SHA256`
- `TLS_DHE_DSS_WITH_ARIA_256_CBC_SHA384`
- `TLS_DHE_RSA_WITH_ARIA_128_CBC_SHA256`
- `TLS_DHE_RSA_WITH_ARIA_256_CBC_SHA384`
- `TLS_DH_anon_WITH_ARIA_128_CBC_SHA256`
- `TLS_DH_anon_WITH_ARIA_256_CBC_SHA384`
- `TLS_ECDHE_ECDSA_WITH_ARIA_128_CBC_SHA256`
- `TLS_ECDHE_ECDSA_WITH_ARIA_256_CBC_SHA384`
- `TLS_ECDH_ECDSA_WITH_ARIA_128_CBC_SHA256`
- `TLS_ECDH_ECDSA_WITH_ARIA_256_CBC_SHA384`
- `TLS_ECDHE_RSA_WITH_ARIA_128_CBC_SHA256`
- `TLS_ECDHE_RSA_WITH_ARIA_256_CBC_SHA384`
- `TLS_ECDH_RSA_WITH_ARIA_128_CBC_SHA256`
- `TLS_ECDH_RSA_WITH_ARIA_256_CBC_SHA384`
- `TLS_RSA_WITH_ARIA_128_GCM_SHA256`
- `TLS_RSA_WITH_ARIA_256_GCM_SHA384`
- `TLS_DHE_RSA_WITH_ARIA_128_GCM_SHA256`
- `TLS_DHE_RSA_WITH_ARIA_256_GCM_SHA384`
- `TLS_DH_RSA_WITH_ARIA_128_GCM_SHA256`
- `TLS_DH_RSA_WITH_ARIA_256_GCM_SHA384`
- `TLS_DHE_DSS_WITH_ARIA_128_GCM_SHA256`
- `TLS_DHE_DSS_WITH_ARIA_256_GCM_SHA384`
- `TLS_DH_DSS_WITH_ARIA_128_GCM_SHA256`
- `TLS_DH_DSS_WITH_ARIA_256_GCM_SHA384`
- `TLS_DH_anon_WITH_ARIA_128_GCM_SHA256`
- `TLS_DH_anon_WITH_ARIA_256_GCM_SHA384`
- `TLS_ECDHE_ECDSA_WITH_ARIA_128_GCM_SHA256`
- `TLS_ECDHE_ECDSA_WITH_ARIA_256_GCM_SHA384`
- `TLS_ECDH_ECDSA_WITH_ARIA_128_GCM_SHA256`
- `TLS_ECDH_ECDSA_WITH_ARIA_256_GCM_SHA384`
- `TLS_ECDHE_RSA_WITH_ARIA_128_GCM_SHA256`
- `TLS_ECDHE_RSA_WITH_ARIA_256_GCM_SHA384`
- `TLS_ECDH_RSA_WITH_ARIA_128_GCM_SHA256`
- `TLS_ECDH_RSA_WITH_ARIA_256_GCM_SHA384`
- `TLS_PSK_WITH_ARIA_128_CBC_SHA256`
- `TLS_PSK_WITH_ARIA_256_CBC_SHA384`
- `TLS_DHE_PSK_WITH_ARIA_128_CBC_SHA256`
- `TLS_DHE_PSK_WITH_ARIA_256_CBC_SHA384`
- `TLS_RSA_PSK_WITH_ARIA_128_CBC_SHA256`
- `TLS_RSA_PSK_WITH_ARIA_256_CBC_SHA384`
- `TLS_PSK_WITH_ARIA_128_GCM_SHA256`
- `TLS_PSK_WITH_ARIA_256_GCM_SHA384`
- `TLS_DHE_PSK_WITH_ARIA_128_GCM_SHA256`
- `TLS_DHE_PSK_WITH_ARIA_256_GCM_SHA384`
- `TLS_RSA_PSK_WITH_ARIA_128_GCM_SHA256`
- `TLS_RSA_PSK_WITH_ARIA_256_GCM_SHA384`
- `TLS_ECDHE_PSK_WITH_ARIA_128_CBC_SHA256`
- `TLS_ECDHE_PSK_WITH_ARIA_256_CBC_SHA384`
- `TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_CBC_SHA384`
- `TLS_ECDH_ECDSA_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_ECDH_ECDSA_WITH_CAMELLIA_256_CBC_SHA384`
- `TLS_ECDHE_RSA_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_ECDHE_RSA_WITH_CAMELLIA_256_CBC_SHA384`
- `TLS_ECDH_RSA_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_ECDH_RSA_WITH_CAMELLIA_256_CBC_SHA384`
- `TLS_RSA_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_RSA_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_DHE_RSA_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_DHE_RSA_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_DH_RSA_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_DH_RSA_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_DHE_DSS_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_DHE_DSS_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_DH_DSS_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_DH_DSS_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_DH_anon_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_DH_anon_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_ECDH_ECDSA_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_ECDH_ECDSA_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_ECDHE_RSA_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_ECDHE_RSA_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_ECDH_RSA_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_ECDH_RSA_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_PSK_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_PSK_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_DHE_PSK_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_DHE_PSK_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_RSA_PSK_WITH_CAMELLIA_128_GCM_SHA256`
- `TLS_RSA_PSK_WITH_CAMELLIA_256_GCM_SHA384`
- `TLS_PSK_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_PSK_WITH_CAMELLIA_256_CBC_SHA384`
- `TLS_DHE_PSK_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_DHE_PSK_WITH_CAMELLIA_256_CBC_SHA384`
- `TLS_RSA_PSK_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_RSA_PSK_WITH_CAMELLIA_256_CBC_SHA384`
- `TLS_ECDHE_PSK_WITH_CAMELLIA_128_CBC_SHA256`
- `TLS_ECDHE_PSK_WITH_CAMELLIA_256_CBC_SHA384`
- `TLS_RSA_WITH_AES_128_CCM`
- `TLS_RSA_WITH_AES_256_CCM`
- `TLS_DHE_RSA_WITH_AES_128_CCM`
- `TLS_DHE_RSA_WITH_AES_256_CCM`
- `TLS_RSA_WITH_AES_128_CCM_8`
- `TLS_RSA_WITH_AES_256_CCM_8`
- `TLS_DHE_RSA_WITH_AES_128_CCM_8`
- `TLS_DHE_RSA_WITH_AES_256_CCM_8`
- `TLS_PSK_WITH_AES_128_CCM`
- `TLS_PSK_WITH_AES_256_CCM`
- `TLS_DHE_PSK_WITH_AES_128_CCM`
- `TLS_DHE_PSK_WITH_AES_256_CCM`
- `TLS_PSK_WITH_AES_128_CCM_8`
- `TLS_PSK_WITH_AES_256_CCM_8`
- `TLS_PSK_DHE_WITH_AES_128_CCM_8`
- `TLS_PSK_DHE_WITH_AES_256_CCM_8`
- `TLS_ECDHE_ECDSA_WITH_AES_128_CCM`
- `TLS_ECDHE_ECDSA_WITH_AES_256_CCM`
- `TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8`
- `TLS_ECDHE_ECDSA_WITH_AES_256_CCM_8`
- `TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256`
- `TLS_PSK_WITH_CHACHA20_POLY1305_SHA256`
- `TLS_ECDHE_PSK_WITH_CHACHA20_POLY1305_SHA256`
- `TLS_DHE_PSK_WITH_CHACHA20_POLY1305_SHA256`
- `TLS_RSA_PSK_WITH_CHACHA20_POLY1305_SHA256`
- `SSL_RSA_FIPS_WITH_DES_CBC_SHA`
- `SSL_RSA_FIPS_WITH_3DES_EDE_CBC_SHA`
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
  - `fn eq(self: &Self, other: &CipherSuite) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CipherSuite`
- **From**
  - `fn from(x: u16) -> Self`



## rustls::enums::ContentType

*Enum*

The `ContentType` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `ChangeCipherSpec`
- `Alert`
- `Handshake`
- `ApplicationData`
- `Heartbeat`
- `Unknown(u8)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 1]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ContentType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ContentType`
- **From**
  - `fn from(x: u8) -> Self`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`



## rustls::enums::EchClientHelloType

*Enum*

The type of Encrypted Client Hello (`EchClientHelloType`).

Specified in [draft-ietf-tls-esni Section 5].

[draft-ietf-tls-esni Section 5]: <https://www.ietf.org/archive/id/draft-ietf-tls-esni-18.html#section-5>

**Variants:**
- `ClientHelloOuter`
- `ClientHelloInner`
- `Unknown(u8)`



## rustls::enums::HandshakeType

*Enum*

The `HandshakeType` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `HelloRequest`
- `ClientHello`
- `ServerHello`
- `HelloVerifyRequest`
- `NewSessionTicket`
- `EndOfEarlyData`
- `HelloRetryRequest`
- `EncryptedExtensions`
- `Certificate`
- `ServerKeyExchange`
- `CertificateRequest`
- `ServerHelloDone`
- `CertificateVerify`
- `ClientKeyExchange`
- `Finished`
- `CertificateURL`
- `CertificateStatus`
- `KeyUpdate`
- `CompressedCertificate`
- `MessageHash`
- `Unknown(u8)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 1]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &HandshakeType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> HandshakeType`
- **From**
  - `fn from(x: u8) -> Self`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`



## rustls::enums::ProtocolVersion

*Enum*

The `ProtocolVersion` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `SSLv2`
- `SSLv3`
- `TLSv1_0`
- `TLSv1_1`
- `TLSv1_2`
- `TLSv1_3`
- `DTLSv1_0`
- `DTLSv1_2`
- `DTLSv1_3`
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
  - `fn eq(self: &Self, other: &ProtocolVersion) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ProtocolVersion`
- **From**
  - `fn from(x: u16) -> Self`



## rustls::enums::SignatureAlgorithm

*Enum*

The `SignatureAlgorithm` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `Anonymous`
- `RSA`
- `DSA`
- `ECDSA`
- `ED25519`
- `ED448`
- `Unknown(u8)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 1]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SignatureAlgorithm) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SignatureAlgorithm`
- **From**
  - `fn from(x: u8) -> Self`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`



## rustls::enums::SignatureScheme

*Enum*

The `SignatureScheme` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

**Variants:**
- `RSA_PKCS1_SHA1`
- `ECDSA_SHA1_Legacy`
- `RSA_PKCS1_SHA256`
- `ECDSA_NISTP256_SHA256`
- `RSA_PKCS1_SHA384`
- `ECDSA_NISTP384_SHA384`
- `RSA_PKCS1_SHA512`
- `ECDSA_NISTP521_SHA512`
- `RSA_PSS_SHA256`
- `RSA_PSS_SHA384`
- `RSA_PSS_SHA512`
- `ED25519`
- `ED448`
- `ML_DSA_44`
- `ML_DSA_65`
- `ML_DSA_87`
- `Unknown(u16)`

**Methods:**

- `fn to_array(self: Self) -> [u8; 2]`
- `fn as_str(self: &Self) -> Option<&'static str>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(x: u16) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut alloc::vec::Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, crate::error::InvalidMessage>`
- **PartialEq**
  - `fn eq(self: &Self, other: &SignatureScheme) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SignatureScheme`



