**rustls > common_state**

# Module: common_state

## Contents

**Structs**

- [`CommonState`](#commonstate) - Connection state common to both client and server connections.
- [`IoState`](#iostate) - Values of this structure are returned from [`Connection::process_new_packets`]

**Enums**

- [`HandshakeKind`](#handshakekind) - Describes which sort of handshake happened.
- [`Side`](#side) - Side of the connection.

---

## rustls::common_state::CommonState

*Struct*

Connection state common to both client and server connections.

**Methods:**

- `fn wants_write(self: &Self) -> bool` - Returns true if the caller should call [`Connection::write_tls`] as soon as possible.
- `fn is_handshaking(self: &Self) -> bool` - Returns true if the connection is currently performing the TLS handshake.
- `fn peer_certificates(self: &Self) -> Option<&[CertificateDer<'static>]>` - Retrieves the certificate chain or the raw public key used by the peer to authenticate.
- `fn alpn_protocol(self: &Self) -> Option<&[u8]>` - Retrieves the protocol agreed with the peer via ALPN.
- `fn negotiated_cipher_suite(self: &Self) -> Option<SupportedCipherSuite>` - Retrieves the ciphersuite agreed with the peer.
- `fn negotiated_key_exchange_group(self: &Self) -> Option<&'static dyn SupportedKxGroup>` - Retrieves the key exchange group agreed with the peer.
- `fn protocol_version(self: &Self) -> Option<ProtocolVersion>` - Retrieves the protocol version agreed with the peer.
- `fn handshake_kind(self: &Self) -> Option<HandshakeKind>` - Which kind of handshake was performed.
- `fn send_close_notify(self: & mut Self)` - Queues a `close_notify` warning alert to be sent in the next
- `fn wants_read(self: &Self) -> bool` - Returns true if the caller should call [`Connection::read_tls`] as soon



## rustls::common_state::HandshakeKind

*Enum*

Describes which sort of handshake happened.

**Variants:**
- `Full` - A full handshake.
- `FullWithHelloRetryRequest` - A full TLS1.3 handshake, with an extra round-trip for a `HelloRetryRequest`.
- `Resumed` - A resumed handshake.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> HandshakeKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &HandshakeKind) -> bool`



## rustls::common_state::IoState

*Struct*

Values of this structure are returned from [`Connection::process_new_packets`]
and tell the caller the current I/O state of the TLS connection.

[`Connection::process_new_packets`]: crate::Connection::process_new_packets

**Methods:**

- `fn tls_bytes_to_write(self: &Self) -> usize` - How many bytes could be written by [`Connection::write_tls`] if called
- `fn plaintext_bytes_to_read(self: &Self) -> usize` - How many plaintext bytes could be obtained via [`std::io::Read`]
- `fn peer_has_closed(self: &Self) -> bool` - True if the peer has sent us a close_notify alert.  This is

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IoState) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::common_state::Side

*Enum*

Side of the connection.

**Variants:**
- `Client` - A client initiates the connection.
- `Server` - A server waits for a client to connect.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Side`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Side) -> bool`



