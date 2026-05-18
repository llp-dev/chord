**webpki > crl**

# Module: crl

## Contents

**Structs**

- [`CrlsRequired`](#crlsrequired) - An opaque error indicating the caller must provide at least one CRL when building a
- [`RevocationOptions`](#revocationoptions) - Describes how revocation checking is performed, if at all. Can be constructed with a
- [`RevocationOptionsBuilder`](#revocationoptionsbuilder) - Builds a RevocationOptions instance to control how revocation checking is performed.

**Enums**

- [`ExpirationPolicy`](#expirationpolicy) - Describes how to handle the nextUpdate field of the CRL (i.e. expiration).
- [`RevocationCheckDepth`](#revocationcheckdepth) - Describes how much of a certificate chain is checked for revocation status.
- [`UnknownStatusPolicy`](#unknownstatuspolicy) - Describes how to handle the case where a certificate's revocation status is unknown.

---

## webpki::crl::CrlsRequired

*Struct*

An opaque error indicating the caller must provide at least one CRL when building a
[RevocationOptions] instance.

**Tuple Struct**: `()`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CrlsRequired`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## webpki::crl::ExpirationPolicy

*Enum*

Describes how to handle the nextUpdate field of the CRL (i.e. expiration).

**Variants:**
- `Enforce` - Enforce the verification time is before the time in the nextUpdate field.
- `Ignore` - Ignore the CRL nextUpdate field.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ExpirationPolicy) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ExpirationPolicy`



## webpki::crl::RevocationCheckDepth

*Enum*

Describes how much of a certificate chain is checked for revocation status.

**Variants:**
- `EndEntity` - Only check the end entity (leaf) certificate's revocation status.
- `Chain` - Check the revocation status of the end entity (leaf) and all intermediates.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RevocationCheckDepth`
- **PartialEq**
  - `fn eq(self: &Self, other: &RevocationCheckDepth) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## webpki::crl::RevocationOptions

*Struct*

Describes how revocation checking is performed, if at all. Can be constructed with a
[RevocationOptionsBuilder] instance.

**Generic Parameters:**
- 'a

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RevocationOptions<'a>`



## webpki::crl::RevocationOptionsBuilder

*Struct*

Builds a RevocationOptions instance to control how revocation checking is performed.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(crls: &'a [&'a CertRevocationList<'a>]) -> Result<Self, CrlsRequired>` - Create a builder that will perform revocation checking using the provided certificate
- `fn with_depth(self: Self, depth: RevocationCheckDepth) -> Self` - Customize the depth at which revocation checking will be performed, controlling
- `fn with_status_policy(self: Self, policy: UnknownStatusPolicy) -> Self` - Customize whether unknown revocation status is an error, or permitted.
- `fn with_expiration_policy(self: Self, policy: ExpirationPolicy) -> Self` - Customize whether the CRL nextUpdate field (i.e. expiration) is enforced.
- `fn build(self: Self) -> RevocationOptions<'a>` - Construct a [RevocationOptions] instance based on the builder's configuration.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RevocationOptionsBuilder<'a>`



## webpki::crl::UnknownStatusPolicy

*Enum*

Describes how to handle the case where a certificate's revocation status is unknown.

**Variants:**
- `Allow` - Treat unknown revocation status permissively, acting as if the certificate were
- `Deny` - Treat unknown revocation status as an error condition, yielding

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &UnknownStatusPolicy) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> UnknownStatusPolicy`



