**rustls > client > ech**

# Module: client::ech

## Contents

**Structs**

- [`EchConfig`](#echconfig) - Configuration for performing encrypted client hello.
- [`EchGreaseConfig`](#echgreaseconfig) - Configuration for GREASE Encrypted Client Hello.

**Enums**

- [`EchMode`](#echmode) - Controls how Encrypted Client Hello (ECH) is used in a client handshake.
- [`EchStatus`](#echstatus) - An enum representing ECH offer status.

---

## rustls::client::ech::EchConfig

*Struct*

Configuration for performing encrypted client hello.

Note: differs from the protocol-encoded EchConfig (`EchConfigMsg`).

**Methods:**

- `fn new(ech_config_list: EchConfigListBytes, hpke_suites: &[&'static dyn Hpke]) -> Result<Self, Error>` - Construct an EchConfig by selecting a ECH config from the provided bytes that is compatible

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> EchConfig`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::client::ech::EchGreaseConfig

*Struct*

Configuration for GREASE Encrypted Client Hello.

**Methods:**

- `fn new(suite: &'static dyn Hpke, placeholder_key: HpkePublicKey) -> Self` - Construct a GREASE ECH configuration.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> EchGreaseConfig`



## rustls::client::ech::EchMode

*Enum*

Controls how Encrypted Client Hello (ECH) is used in a client handshake.

**Variants:**
- `Enable(EchConfig)` - ECH is enabled and the ClientHello will be encrypted based on the provided
- `Grease(EchGreaseConfig)` - No ECH configuration is available but the client should act as though it were.

**Methods:**

- `fn fips(self: &Self) -> bool` - Returns true if the ECH mode will use a FIPS approved HPKE suite.

**Trait Implementations:**

- **From**
  - `fn from(config: EchConfig) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> EchMode`
- **From**
  - `fn from(config: EchGreaseConfig) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::client::ech::EchStatus

*Enum*

An enum representing ECH offer status.

**Variants:**
- `NotOffered` - ECH was not offered - it is a normal TLS handshake.
- `Grease` - GREASE ECH was sent. This is not considered offering ECH.
- `Offered` - ECH was offered but we do not yet know whether the offer was accepted or rejected.
- `Accepted` - ECH was offered and the server accepted.
- `Rejected` - ECH was offered and the server rejected.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &EchStatus) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> EchStatus`



