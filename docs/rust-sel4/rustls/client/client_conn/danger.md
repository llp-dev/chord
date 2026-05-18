**rustls > client > client_conn > danger**

# Module: client::client_conn::danger

## Contents

**Structs**

- [`DangerousClientConfig`](#dangerousclientconfig) - Accessor for dangerous configuration options.

---

## rustls::client::client_conn::danger::DangerousClientConfig

*Struct*

Accessor for dangerous configuration options.

**Generic Parameters:**
- 'a

**Fields:**
- `cfg: &'a  mut super::ClientConfig` - The underlying ClientConfig

**Methods:**

- `fn set_certificate_verifier(self: & mut Self, verifier: alloc::sync::Arc<dyn ServerCertVerifier>)` - Overrides the default `ServerCertVerifier` with something else.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



