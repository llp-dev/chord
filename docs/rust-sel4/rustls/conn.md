**rustls > conn**

# Module: conn

## Contents

**Modules**

- [`kernel`](#kernel) - Kernel connection API.

**Structs**

- [`ConnectionCommon`](#connectioncommon) - Interface shared by client and server connections.
- [`UnbufferedConnectionCommon`](#unbufferedconnectioncommon) - Interface shared by unbuffered client and server connections.

**Traits**

- [`SideData`](#sidedata) - Data specific to the peer's side (client or server).

---

## rustls::conn::ConnectionCommon

*Struct*

Interface shared by client and server connections.

**Generic Parameters:**
- Data

**Methods:**

- `fn process_new_packets(self: & mut Self) -> Result<IoState, Error>` - Processes any new packets read by a previous call to
- `fn export_keying_material<T>(self: &Self, output: T, label: &[u8], context: Option<&[u8]>) -> Result<T, Error>` - Derives key material from the agreed connection secrets.
- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>` - Extract secrets, so they can be used when configuring kTLS, for example.
- `fn set_buffer_limit(self: & mut Self, limit: Option<usize>)` - Sets a limit on the internal buffers used to buffer
- `fn refresh_traffic_keys(self: & mut Self) -> Result<(), Error>` - Sends a TLS1.3 `key_update` message to refresh a connection's keys.

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## rustls::conn::SideData

*Trait*

Data specific to the peer's side (client or server).



## rustls::conn::UnbufferedConnectionCommon

*Struct*

Interface shared by unbuffered client and server connections.

**Generic Parameters:**
- Data

**Methods:**

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>` - Extract secrets, so they can be used when configuring kTLS, for example.
- `fn process_tls_records<'c, 'i>(self: &'c  mut Self, incoming_tls: &'i  mut [u8]) -> UnbufferedStatus<'c, 'i, ClientConnectionData>` - Processes the TLS records in `incoming_tls` buffer until a new [`UnbufferedStatus`] is
- `fn process_tls_records<'c, 'i>(self: &'c  mut Self, incoming_tls: &'i  mut [u8]) -> UnbufferedStatus<'c, 'i, ServerConnectionData>` - Processes the TLS records in `incoming_tls` buffer until a new [`UnbufferedStatus`] is

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



## Module: kernel

Kernel connection API.

This module gives you the bare minimum you need to implement a TLS connection
that does its own encryption and decryption while still using rustls to manage
connection secrets and session tickets. It is intended for use cases like kTLS
where you want to use rustls to establish the connection but want to use
something else to do the encryption/decryption after that.

There are only two things that [`KernelConnection`] is able to do:
1. Compute new traffic secrets when a key update occurs.
2. Save received session tickets sent by a server peer.

That's it. Everything else you will need to implement yourself.

# Entry Point
The entry points into this API are
[`UnbufferedClientConnection::dangerous_into_kernel_connection`][client-into]
and
[`UnbufferedServerConnection::dangerous_into_kernel_connection`][server-into].

In order to actually create an [`KernelConnection`] all of the following
must be true:
- the connection must have completed its handshake,
- the connection must have no buffered TLS data waiting to be sent, and,
- the config used to create the connection must have `enable_extract_secrets`
  set to true.

This sounds fairly complicated to achieve at first glance. However, if you
drive an unbuffered connection through the handshake until it returns
[`WriteTraffic`] then it will end up in an appropriate state to convert
into an external connection.

[client-into]: crate::client::UnbufferedClientConnection::dangerous_into_kernel_connection
[server-into]: crate::server::UnbufferedServerConnection::dangerous_into_kernel_connection
[`WriteTraffic`]: crate::unbuffered::ConnectionState::WriteTraffic

# Cipher Suite Confidentiality Limits
Some cipher suites (notably AES-GCM) have vulnerabilities where they are no
longer secure once a certain number of messages have been sent. Normally,
rustls tracks how many messages have been written or read and will
automatically either refresh keys or emit an error when approaching the
confidentiality limit of the cipher suite.

[`KernelConnection`] has no way to track this. It is the responsibility
of the user of the API to track approximately how many messages have been
sent and either refresh the traffic keys or abort the connection before the
confidentiality limit is reached.

You can find the current confidentiality limit by looking at
[`CipherSuiteCommon::confidentiality_limit`] for the cipher suite selected
by the connection.

[`CipherSuiteCommon::confidentiality_limit`]: crate::CipherSuiteCommon::confidentiality_limit
[`KernelConnection`]: crate::kernel::KernelConnection



