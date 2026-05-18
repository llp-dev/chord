**rustls > server > server_conn**

# Module: server::server_conn

## Contents

**Structs**

- [`Accepted`](#accepted) - Represents a `ClientHello` message received through the [`Acceptor`].
- [`ClientHello`](#clienthello) - A struct representing the received Client Hello
- [`ServerConfig`](#serverconfig) - Common configuration for a set of server sessions.
- [`ServerConnectionData`](#serverconnectiondata) - State associated with a server connection.
- [`UnbufferedServerConnection`](#unbufferedserverconnection) - Unbuffered version of `ServerConnection`

**Traits**

- [`ProducesTickets`](#producestickets) - A trait for the ability to encrypt and decrypt tickets.
- [`ResolvesServerCert`](#resolvesservercert) - How to choose a certificate chain and signing key for use
- [`StoresServerSessions`](#storesserversessions) - A trait for the ability to store server session data.

---

## rustls::server::server_conn::Accepted

*Struct*

Represents a `ClientHello` message received through the [`Acceptor`].

Contains the state required to resume the connection through [`Accepted::into_connection()`].

**Methods:**

- `fn client_hello(self: &Self) -> ClientHello` - Get the [`ClientHello`] for this connection.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`



## rustls::server::server_conn::ClientHello

*Struct*

A struct representing the received Client Hello

**Generic Parameters:**
- 'a

**Methods:**

- `fn server_name(self: &Self) -> Option<&str>` - Get the server name indicator.
- `fn signature_schemes(self: &Self) -> &[SignatureScheme]` - Get the compatible signature schemes.
- `fn alpn(self: &Self) -> Option<impl Trait>` - Get the ALPN protocol identifiers submitted by the client.
- `fn cipher_suites(self: &Self) -> &[CipherSuite]` - Get cipher suites.
- `fn server_cert_types(self: &Self) -> Option<&'a [CertificateType]>` - Get the server certificate types offered in the ClientHello.
- `fn client_cert_types(self: &Self) -> Option<&'a [CertificateType]>` - Get the client certificate types offered in the ClientHello.
- `fn certificate_authorities(self: &Self) -> Option<&'a [DistinguishedName]>` - Get the [certificate_authorities] extension sent by the client.
- `fn named_groups(self: &Self) -> Option<&'a [NamedGroup]>` - Get the [`named_groups`] extension sent by the client.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::server::server_conn::ProducesTickets

*Trait*

A trait for the ability to encrypt and decrypt tickets.

**Methods:**

- `enabled`: Returns true if this implementation will encrypt/decrypt
- `lifetime`: Returns the lifetime in seconds of tickets produced now.
- `encrypt`: Encrypt and authenticate `plain`, returning the resulting
- `decrypt`: Decrypt `cipher`, validating its authenticity protection



## rustls::server::server_conn::ResolvesServerCert

*Trait*

How to choose a certificate chain and signing key for use
in server authentication.

This is suitable when selecting a certificate does not require
I/O or when the application is using blocking I/O anyhow.

For applications that use async I/O and need to do I/O to choose
a certificate (for instance, fetching a certificate from a data store),
the [`Acceptor`] interface is more suitable.

**Methods:**

- `resolve`: Choose a certificate chain and matching key given simplified
- `only_raw_public_keys`: Return true when the server only supports raw public keys.



## rustls::server::server_conn::ServerConfig

*Struct*

Common configuration for a set of server sessions.

Making one of these is cheap, though one of the inputs may be expensive: gathering trust roots
from the operating system to add to the [`RootCertStore`] passed to a `ClientCertVerifier`
builder may take on the order of a few hundred milliseconds.

These must be created via the [`ServerConfig::builder()`] or [`ServerConfig::builder_with_provider()`]
function.

# Defaults

* [`ServerConfig::max_fragment_size`]: the default is `None` (meaning 16kB).
* [`ServerConfig::session_storage`]: if the `std` feature is enabled, the default stores 256
  sessions in memory. If the `std` feature is not enabled, the default is to not store any
  sessions. In a no-std context, by enabling the `hashbrown` feature you may provide your
  own `session_storage` using [`ServerSessionMemoryCache`] and a `crate::lock::MakeMutex`
  implementation.
* [`ServerConfig::alpn_protocols`]: the default is empty -- no ALPN protocol is negotiated.
* [`ServerConfig::key_log`]: key material is not logged.
* [`ServerConfig::send_tls13_tickets`]: 2 tickets are sent.
* [`ServerConfig::cert_compressors`]: depends on the crate features, see [`compress::default_cert_compressors()`].
* [`ServerConfig::cert_compression_cache`]: caches the most recently used 4 compressions
* [`ServerConfig::cert_decompressors`]: depends on the crate features, see [`compress::default_cert_decompressors()`].

# Sharing resumption storage between `ServerConfig`s

In a program using many `ServerConfig`s it may improve resumption rates
(which has a significant impact on connection performance) if those
configs share [`ServerConfig::session_storage`] or [`ServerConfig::ticketer`].

However, caution is needed: other fields influence the security of a session
and resumption between them can be surprising.  If sharing
[`ServerConfig::session_storage`] or [`ServerConfig::ticketer`] between two
`ServerConfig`s, you should also evaluate the following fields and ensure
they are equivalent:

* `ServerConfig::verifier` -- client authentication requirements,
* [`ServerConfig::cert_resolver`] -- server identities.

To illustrate, imagine two `ServerConfig`s `A` and `B`.  `A` requires
client authentication, `B` does not.  If `A` and `B` shared a resumption store,
it would be possible for a session originated by `B` (that is, an unauthenticated client)
to be inserted into the store, and then resumed by `A`.  This would give a false
impression to the user of `A` that the client was authenticated.  This is possible
whether the resumption is performed statefully (via [`ServerConfig::session_storage`])
or statelessly (via [`ServerConfig::ticketer`]).

_Unlike_ `ClientConfig`, rustls does not enforce any policy here.

[`RootCertStore`]: crate::RootCertStore
[`ServerSessionMemoryCache`]: crate::server::handy::ServerSessionMemoryCache

**Fields:**
- `ignore_client_order: bool` - Ignore the client's ciphersuite order. Instead,
- `max_fragment_size: Option<usize>` - The maximum size of plaintext input to be emitted in a single TLS record.
- `session_storage: alloc::sync::Arc<dyn StoresServerSessions>` - How to store client sessions.
- `ticketer: alloc::sync::Arc<dyn ProducesTickets>` - How to produce tickets.
- `cert_resolver: alloc::sync::Arc<dyn ResolvesServerCert>` - How to choose a server cert and key. This is usually set by
- `alpn_protocols: alloc::vec::Vec<alloc::vec::Vec<u8>>` - Protocol names we support, most preferred first.
- `key_log: alloc::sync::Arc<dyn KeyLog>` - How to output key material for debugging.  The default
- `enable_secret_extraction: bool` - Allows traffic secrets to be extracted after the handshake,
- `max_early_data_size: u32` - Amount of early data to accept for sessions created by
- `send_half_rtt_data: bool` - Whether the server should send "0.5RTT" data.  This means the server
- `send_tls13_tickets: usize` - How many TLS1.3 tickets to send immediately after a successful
- `require_ems: bool` - If set to `true`, requires the client to support the extended
- `time_provider: alloc::sync::Arc<dyn TimeProvider>` - Provides the current system time
- `cert_compressors: alloc::vec::Vec<&'static dyn compress::CertCompressor>` - How to compress the server's certificate chain.
- `cert_compression_cache: alloc::sync::Arc<compress::CompressionCache>` - Caching for compressed certificates.
- `cert_decompressors: alloc::vec::Vec<&'static dyn compress::CertDecompressor>` - How to decompress the clients's certificate chain.

**Methods:**

- `fn builder_with_details(provider: alloc::sync::Arc<CryptoProvider>, time_provider: alloc::sync::Arc<dyn TimeProvider>) -> ConfigBuilder<Self, WantsVersions>` - Create a builder for a server configuration with no default implementation details.
- `fn fips(self: &Self) -> bool` - Return `true` if connections made with this `ServerConfig` will
- `fn crypto_provider(self: &Self) -> &alloc::sync::Arc<CryptoProvider>` - Return the crypto provider used to construct this client configuration.

**Traits:** ConfigSide

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ServerConfig`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::server::server_conn::ServerConnectionData

*Struct*

State associated with a server connection.

**Traits:** SideData

**Trait Implementations:**

- **Default**
  - `fn default() -> ServerConnectionData`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::server::server_conn::StoresServerSessions

*Trait*

A trait for the ability to store server session data.

The keys and values are opaque.

Inserted keys are randomly chosen by the library and have
no internal structure (in other words, you may rely on all
bits being uniformly random).  Queried keys are untrusted data.

Both the keys and values should be treated as
**highly sensitive data**, containing enough key material
to break all security of the corresponding sessions.

Implementations can be lossy (in other words, forgetting
key/value pairs) without any negative security consequences.

However, note that `take` **must** reliably delete a returned
value.  If it does not, there may be security consequences.

`put` and `take` are mutating operations; this isn't expressed
in the type system to allow implementations freedom in
how to achieve interior mutability.  `Mutex` is a common
choice.

**Methods:**

- `put`: Store session secrets encoded in `value` against `key`,
- `get`: Find a value with the given `key`.  Return it, or None
- `take`: Find a value with the given `key`.  Return it and delete it;
- `can_cache`: Whether the store can cache another session. This is used to indicate to clients



## rustls::server::server_conn::UnbufferedServerConnection

*Struct*

Unbuffered version of `ServerConnection`

See the [`crate::unbuffered`] module docs for more details

**Methods:**

- `fn new(config: alloc::sync::Arc<ServerConfig>) -> Result<Self, Error>` - Make a new ServerConnection. `config` controls how we behave in the TLS protocol.
- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>` - Extract secrets, so they can be used when configuring kTLS, for example.
- `fn dangerous_into_kernel_connection(self: Self) -> Result<(ExtractedSecrets, KernelConnection<ServerConnectionData>), Error>` - Extract secrets and an [`KernelConnection`] object.

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



