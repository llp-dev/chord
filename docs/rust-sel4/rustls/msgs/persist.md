**rustls > msgs > persist**

# Module: msgs::persist

## Contents

**Structs**

- [`ClientSessionCommon`](#clientsessioncommon)
- [`ServerSessionValue`](#serversessionvalue)
- [`Tls12ClientSessionValue`](#tls12clientsessionvalue)
- [`Tls13ClientSessionValue`](#tls13clientsessionvalue)

---

## rustls::msgs::persist::ClientSessionCommon

*Struct*



## rustls::msgs::persist::ServerSessionValue

*Struct*

**Fields:**
- `creation_time_sec: u64`

**Trait Implementations:**

- **Codec**
  - `fn encode(self: &Self, bytes: & mut Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, InvalidMessage>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::msgs::persist::Tls12ClientSessionValue

*Struct*

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Tls12ClientSessionValue`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::msgs::persist::Tls13ClientSessionValue

*Struct*

**Methods:**

- `fn max_early_data_size(self: &Self) -> u32`
- `fn suite(self: &Self) -> &'static Tls13CipherSuite`
- `fn set_quic_params(self: & mut Self, quic_params: &[u8])`
- `fn quic_params(self: &Self) -> Vec<u8>`

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



