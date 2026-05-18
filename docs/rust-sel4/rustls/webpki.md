**rustls > webpki**

# Module: webpki

## Contents

**Enums**

- [`VerifierBuilderError`](#verifierbuildererror) - An error that can occur when building a certificate verifier.

---

## rustls::webpki::VerifierBuilderError

*Enum*

An error that can occur when building a certificate verifier.

**Variants:**
- `NoRootAnchors` - No root trust anchors were provided.
- `InvalidCrl(crate::error::CertRevocationListError)` - A provided CRL could not be parsed.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VerifierBuilderError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(value: CertRevocationListError) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



