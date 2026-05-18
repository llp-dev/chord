**rustls > client > builder > danger**

# Module: client::builder::danger

## Contents

**Structs**

- [`DangerousClientConfigBuilder`](#dangerousclientconfigbuilder) - Accessor for dangerous configuration options.

---

## rustls::client::builder::danger::DangerousClientConfigBuilder

*Struct*

Accessor for dangerous configuration options.

**Fields:**
- `cfg: crate::ConfigBuilder<crate::ClientConfig, crate::WantsVerifier>` - The underlying ClientConfigBuilder

**Methods:**

- `fn with_custom_certificate_verifier(self: Self, verifier: alloc::sync::Arc<dyn verify::ServerCertVerifier>) -> ConfigBuilder<ClientConfig, WantsClientCert>` - Set a custom certificate verifier.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



