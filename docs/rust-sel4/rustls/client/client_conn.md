**rustls > client > client_conn**

# Module: client::client_conn

## Contents

**Structs**

- [`ClientConfig`](#clientconfig) - Common configuration for (typically) all connections made by a program.
- [`ClientConnectionData`](#clientconnectiondata) - State associated with a client connection.
- [`MayEncryptEarlyData`](#mayencryptearlydata) - Allows encrypting early (RTT-0) data
- [`Resumption`](#resumption) - Configuration for how/when a client is allowed to resume a previous session.
- [`UnbufferedClientConnection`](#unbufferedclientconnection) - Unbuffered version of `ClientConnection`

**Enums**

- [`EarlyDataError`](#earlydataerror) - Errors that may arise when encrypting early (RTT-0) data
- [`Tls12Resumption`](#tls12resumption) - What mechanisms to support for resuming a TLS 1.2 session.

**Traits**

- [`ClientSessionStore`](#clientsessionstore) - A trait for the ability to store client session data, so that sessions
- [`ResolvesClientCert`](#resolvesclientcert) - A trait for the ability to choose a certificate chain and

---

## rustls::client::client_conn::ClientConfig

*Struct*

Common configuration for (typically) all connections made by a program.

Making one of these is cheap, though one of the inputs may be expensive: gathering trust roots
from the operating system to add to the [`RootCertStore`] passed to `with_root_certificates()`
(the rustls-native-certs crate is often used for this) may take on the order of a few hundred
milliseconds.

These must be created via the [`ClientConfig::builder()`] or [`ClientConfig::builder_with_provider()`]
function.

Note that using [`ConfigBuilder<ClientConfig, WantsVersions>::with_ech()`] will produce a common
configuration specific to the provided [`crate::client::EchConfig`] that may not be appropriate
for all connections made by the program. In this case the configuration should only be shared
by connections intended for domains that offer the provided [`crate::client::EchConfig`] in
their DNS zone.

# Defaults

* [`ClientConfig::max_fragment_size`]: the default is `None` (meaning 16kB).
* [`ClientConfig::resumption`]: supports resumption with up to 256 server names, using session
  ids or tickets, with a max of eight tickets per server.
* [`ClientConfig::alpn_protocols`]: the default is empty -- no ALPN protocol is negotiated.
* [`ClientConfig::key_log`]: key material is not logged.
* [`ClientConfig::cert_decompressors`]: depends on the crate features, see [`compress::default_cert_decompressors()`].
* [`ClientConfig::cert_compressors`]: depends on the crate features, see [`compress::default_cert_compressors()`].
* [`ClientConfig::cert_compression_cache`]: caches the most recently used 4 compressions

[`RootCertStore`]: crate::RootCertStore

**Fields:**
- `alpn_protocols: alloc::vec::Vec<alloc::vec::Vec<u8>>` - Which ALPN protocols we include in our client hello.
- `resumption: Resumption` - How and when the client can resume a previous session.
- `max_fragment_size: Option<usize>` - The maximum size of plaintext input to be emitted in a single TLS record.
- `client_auth_cert_resolver: alloc::sync::Arc<dyn ResolvesClientCert>` - How to decide what client auth certificate/keys to use.
- `enable_sni: bool` - Whether to send the Server Name Indication (SNI) extension
- `key_log: alloc::sync::Arc<dyn KeyLog>` - How to output key material for debugging.  The default
- `enable_secret_extraction: bool` - Allows traffic secrets to be extracted after the handshake,
- `enable_early_data: bool` - Whether to send data on the first flight ("early data") in
- `require_ems: bool` - If set to `true`, requires the server to support the extended
- `time_provider: alloc::sync::Arc<dyn TimeProvider>` - Provides the current system time
- `cert_decompressors: alloc::vec::Vec<&'static dyn compress::CertDecompressor>` - How to decompress the server's certificate chain.
- `cert_compressors: alloc::vec::Vec<&'static dyn compress::CertCompressor>` - How to compress the client's certificate chain.
- `cert_compression_cache: alloc::sync::Arc<compress::CompressionCache>` - Caching for compressed certificates.

**Methods:**

- `fn builder_with_details(provider: alloc::sync::Arc<CryptoProvider>, time_provider: alloc::sync::Arc<dyn TimeProvider>) -> ConfigBuilder<Self, WantsVersions>` - Create a builder for a client configuration with no default implementation details.
- `fn fips(self: &Self) -> bool` - Return true if connections made with this `ClientConfig` will
- `fn crypto_provider(self: &Self) -> &alloc::sync::Arc<CryptoProvider>` - Return the crypto provider used to construct this client configuration.
- `fn dangerous(self: & mut Self) -> danger::DangerousClientConfig` - Access configuration options whose use is dangerous and requires

**Traits:** ConfigSide

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ClientConfig`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::client::client_conn::ClientConnectionData

*Struct*

State associated with a client connection.

**Traits:** SideData

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::client::client_conn::ClientSessionStore

*Trait*

A trait for the ability to store client session data, so that sessions
can be resumed in future connections.

Generally all data in this interface should be treated as
**highly sensitive**, containing enough key material to break all security
of the corresponding session.

`set_`, `insert_`, `remove_` and `take_` operations are mutating; this isn't
expressed in the type system to allow implementations freedom in
how to achieve interior mutability.  `Mutex` is a common choice.

**Methods:**

- `set_kx_hint`: Remember what `NamedGroup` the given server chose.
- `kx_hint`: This should return the value most recently passed to `set_kx_hint`
- `set_tls12_session`: Remember a TLS1.2 session.
- `tls12_session`: Get the most recently saved TLS1.2 session for `server_name` provided to `set_tls12_session`.
- `remove_tls12_session`: Remove and forget any saved TLS1.2 session for `server_name`.
- `insert_tls13_ticket`: Remember a TLS1.3 ticket that might be retrieved later from `take_tls13_ticket`, allowing
- `take_tls13_ticket`: Return a TLS1.3 ticket previously provided to `add_tls13_ticket`.



## rustls::client::client_conn::EarlyDataError

*Enum*

Errors that may arise when encrypting early (RTT-0) data

**Variants:**
- `ExceededAllowedEarlyData` - Cannot encrypt more early data due to imposed limits
- `Encrypt(crate::unbuffered::EncryptError)` - Encryption error

**Trait Implementations:**

- **From**
  - `fn from(v: EncryptError) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## rustls::client::client_conn::MayEncryptEarlyData

*Struct*

Allows encrypting early (RTT-0) data

**Generic Parameters:**
- 'c



## rustls::client::client_conn::ResolvesClientCert

*Trait*

A trait for the ability to choose a certificate chain and
private key for the purposes of client authentication.

**Methods:**

- `resolve`: Resolve a client certificate chain/private key to use as the client's
- `only_raw_public_keys`: Return true if the client only supports raw public keys.
- `has_certs`: Return true if any certificates at all are available.



## rustls::client::client_conn::Resumption

*Struct*

Configuration for how/when a client is allowed to resume a previous session.

**Methods:**

- `fn store(store: alloc::sync::Arc<dyn ClientSessionStore>) -> Self` - Use a custom [`ClientSessionStore`] implementation to store sessions.
- `fn disabled() -> Self` - Disable all use of session resumption.
- `fn tls12_resumption(self: Self, tls12: Tls12Resumption) -> Self` - Configure whether TLS 1.2 sessions may be resumed, and by what mechanism.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self` - Create an in-memory session store resumption with up to 256 server names, allowing
- **Clone**
  - `fn clone(self: &Self) -> Resumption`



## rustls::client::client_conn::Tls12Resumption

*Enum*

What mechanisms to support for resuming a TLS 1.2 session.

**Variants:**
- `Disabled` - Disable 1.2 resumption.
- `SessionIdOnly` - Support 1.2 resumption using session ids only.
- `SessionIdOrTickets` - Support 1.2 resumption using session ids or RFC 5077 tickets.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Tls12Resumption`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Tls12Resumption) -> bool`



## rustls::client::client_conn::UnbufferedClientConnection

*Struct*

Unbuffered version of `ClientConnection`

See the [`crate::unbuffered`] module docs for more details

**Methods:**

- `fn new(config: alloc::sync::Arc<ClientConfig>, name: ServerName<'static>) -> Result<Self, Error>` - Make a new ClientConnection. `config` controls how we behave in the TLS protocol, `name` is
- `fn new_with_alpn(config: alloc::sync::Arc<ClientConfig>, name: ServerName<'static>, alpn_protocols: Vec<Vec<u8>>) -> Result<Self, Error>` - Make a new UnbufferedClientConnection with custom ALPN protocols.
- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>` - Extract secrets, so they can be used when configuring kTLS, for example.
- `fn dangerous_into_kernel_connection(self: Self) -> Result<(ExtractedSecrets, KernelConnection<ClientConnectionData>), Error>` - Extract secrets and a [`KernelConnection`] object.
- `fn tls13_tickets_received(self: &Self) -> u32` - Returns the number of TLS1.3 tickets that have been received.

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



