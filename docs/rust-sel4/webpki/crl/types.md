**webpki > crl > types**

# Module: crl::types

## Contents

**Structs**

- [`BorrowedCertRevocationList`](#borrowedcertrevocationlist) - Borrowed representation of a RFC 5280[^1] profile Certificate Revocation List (CRL).
- [`BorrowedRevokedCert`](#borrowedrevokedcert) - Borrowed representation of a RFC 5280[^1] profile Certificate Revocation List (CRL) revoked
- [`OwnedCertRevocationList`](#ownedcertrevocationlist) - Owned representation of a RFC 5280[^1] profile Certificate Revocation List (CRL).
- [`OwnedRevokedCert`](#ownedrevokedcert) - Owned representation of a RFC 5280[^1] profile Certificate Revocation List (CRL) revoked

**Enums**

- [`CertRevocationList`](#certrevocationlist) - A RFC 5280[^1] profile Certificate Revocation List (CRL).
- [`RevocationReason`](#revocationreason) - Identifies the reason a certificate was revoked.

---

## webpki::crl::types::BorrowedCertRevocationList

*Struct*

Borrowed representation of a RFC 5280[^1] profile Certificate Revocation List (CRL).

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

**Generic Parameters:**
- 'a

**Methods:**

- `fn from_der(crl_der: &'a [u8]) -> Result<Self, Error>` - Try to parse the given bytes as a RFC 5280[^1] profile Certificate Revocation List (CRL).
- `fn to_owned(self: &Self) -> Result<OwnedCertRevocationList, Error>` - Convert the CRL to an [`OwnedCertRevocationList`]. This may error if any of the revoked

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## webpki::crl::types::BorrowedRevokedCert

*Struct*

Borrowed representation of a RFC 5280[^1] profile Certificate Revocation List (CRL) revoked
certificate entry.

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

**Generic Parameters:**
- 'a

**Fields:**
- `serial_number: &'a [u8]` - Serial number of the revoked certificate.
- `revocation_date: pki_types::UnixTime` - The date at which the CA processed the revocation.
- `reason_code: Option<RevocationReason>` - Identifies the reason for the certificate revocation. When absent, the revocation reason
- `invalidity_date: Option<pki_types::UnixTime>` - Provides the date on which it is known or suspected that the private key was compromised or

**Methods:**

- `fn to_owned(self: &Self) -> OwnedRevokedCert` - Construct an owned representation of the revoked certificate.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## webpki::crl::types::CertRevocationList

*Enum*

A RFC 5280[^1] profile Certificate Revocation List (CRL).

May be either an owned, or a borrowed representation.

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

**Generic Parameters:**
- 'a

**Variants:**
- `Owned(OwnedCertRevocationList)` - An owned representation of a CRL.
- `Borrowed(BorrowedCertRevocationList<'a>)` - A borrowed representation of a CRL.

**Methods:**

- `fn issuer(self: &Self) -> &[u8]` - Return the DER encoded issuer of the CRL.
- `fn issuing_distribution_point(self: &Self) -> Option<&[u8]>` - Return the DER encoded issuing distribution point of the CRL, if any.
- `fn find_serial(self: &Self, serial: &[u8]) -> Result<Option<BorrowedRevokedCert>, Error>` - Try to find a revoked certificate in the CRL by DER encoded serial number. This

**Trait Implementations:**

- **From**
  - `fn from(crl: BorrowedCertRevocationList<'a>) -> Self`
- **From**
  - `fn from(crl: OwnedCertRevocationList) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## webpki::crl::types::OwnedCertRevocationList

*Struct*

Owned representation of a RFC 5280[^1] profile Certificate Revocation List (CRL).

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

**Methods:**

- `fn from_der(crl_der: &[u8]) -> Result<Self, Error>` - Try to parse the given bytes as a RFC 5280[^1] profile Certificate Revocation List (CRL).

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> OwnedCertRevocationList`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## webpki::crl::types::OwnedRevokedCert

*Struct*

Owned representation of a RFC 5280[^1] profile Certificate Revocation List (CRL) revoked
certificate entry.

Only available when the "alloc" feature is enabled.

[^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>

**Fields:**
- `serial_number: alloc::vec::Vec<u8>` - Serial number of the revoked certificate.
- `revocation_date: pki_types::UnixTime` - The date at which the CA processed the revocation.
- `reason_code: Option<RevocationReason>` - Identifies the reason for the certificate revocation. When absent, the revocation reason
- `invalidity_date: Option<pki_types::UnixTime>` - Provides the date on which it is known or suspected that the private key was compromised or

**Methods:**

- `fn borrow(self: &Self) -> BorrowedRevokedCert` - Convert the owned representation of this revoked cert to a borrowed version.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OwnedRevokedCert`



## webpki::crl::types::RevocationReason

*Enum*

Identifies the reason a certificate was revoked.
See [RFC 5280 §5.3.1][1]

[1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5.3.1>

**Variants:**
- `Unspecified` - Unspecified should not be used, and is instead assumed by the absence of a RevocationReason
- `KeyCompromise`
- `CaCompromise`
- `AffiliationChanged`
- `Superseded`
- `CessationOfOperation`
- `CertificateHold`
- `RemoveFromCrl` - RemoveFromCrl only appears in delta CRLs that are unsupported.
- `PrivilegeWithdrawn`
- `AaCompromise`

**Methods:**

- `fn iter() -> impl Trait` - Return an iterator over all possible [RevocationReason] variants.

**Traits:** Eq, Copy

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(value: u8) -> Result<Self, <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> RevocationReason`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RevocationReason) -> bool`



