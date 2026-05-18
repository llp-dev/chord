**rustls**

# Module: rustls

## Contents

**Modules**

- [`client`](#client) - Items for use in a client.
- [`compress`](#compress) - Certificate compression and decompression support
- [`crypto`](#crypto) - Crypto provider interface.
- [`lock`](#lock) - APIs abstracting over locking primitives.
- [`manual`](#manual) -  This is the rustls manual.
- [`pki_types`](#pki_types) - Re-exports the contents of the [rustls-pki-types](https://docs.rs/rustls-pki-types) crate for easy access
- [`quic`](#quic) - APIs for implementing QUIC TLS
- [`server`](#server) - Items for use in a server.
- [`sign`](#sign) - Message signing interfaces.
- [`time_provider`](#time_provider) - The library's source of time.
- [`unbuffered`](#unbuffered) - Unbuffered connection API
- [`version`](#version) - All defined protocol versions appear in this module.

---

## Module: client

Items for use in a client.



## Module: compress

Certificate compression and decompression support

This crate supports compression and decompression everywhere
certificates are used, in accordance with [RFC8879][rfc8879].

Note that this is only supported for TLS1.3 connections.

# Getting started

Build this crate with the `brotli` and/or `zlib` crate features.  This
adds dependencies on these crates.  They are used by default if enabled.

We especially recommend `brotli` as it has the widest deployment so far.

# Custom compression/decompression implementations

1. Implement the [`CertCompressor`] and/or [`CertDecompressor`] traits
2. Provide those to:
  - [`ClientConfig::cert_compressors`][cc_cc] or [`ServerConfig::cert_compressors`][sc_cc].
  - [`ClientConfig::cert_decompressors`][cc_cd] or [`ServerConfig::cert_decompressors`][sc_cd].

These are used in these circumstances:

| Peer | Client authentication | Server authentication |
| ---- | --------------------- | --------------------- |
| *Client* | [`ClientConfig::cert_compressors`][cc_cc] | [`ClientConfig::cert_decompressors`][cc_cd] |
| *Server* | [`ServerConfig::cert_decompressors`][sc_cd] | [`ServerConfig::cert_compressors`][sc_cc] |

[rfc8879]: https://datatracker.ietf.org/doc/html/rfc8879
[cc_cc]: crate::ClientConfig::cert_compressors
[sc_cc]: crate::ServerConfig::cert_compressors
[cc_cd]: crate::ClientConfig::cert_decompressors
[sc_cd]: crate::ServerConfig::cert_decompressors



## Module: crypto

Crypto provider interface.



## Module: lock

APIs abstracting over locking primitives.



## Module: manual

 This is the rustls manual.

This documentation primarily aims to explain design decisions taken in rustls.

It does this from a few aspects: how rustls attempts to avoid construction errors
that occurred in other TLS libraries, how rustls attempts to avoid past TLS
protocol vulnerabilities, and assorted advice for achieving common tasks with rustls.



## Module: pki_types

Re-exports the contents of the [rustls-pki-types](https://docs.rs/rustls-pki-types) crate for easy access



## Module: quic

APIs for implementing QUIC TLS



## Module: server

Items for use in a server.



## Module: sign

Message signing interfaces.



## Module: time_provider

The library's source of time.



## Module: unbuffered

Unbuffered connection API

This is an alternative to the [`crate::ConnectionCommon`] API that does not internally buffer
TLS nor plaintext data. Instead those buffers are managed by the API user so they have
control over when and how to allocate, resize and dispose of them.

This API is lower level than the `ConnectionCommon` API and is built around a state machine
interface where the API user must handle each state to advance and complete the
handshake process.

Like the `ConnectionCommon` API, no IO happens internally so all IO must be handled by the API
user. Unlike the `ConnectionCommon` API, this API does not make use of the [`std::io::Read`] and
[`std::io::Write`] traits so it's usable in no-std context.

The entry points into this API are [`crate::client::UnbufferedClientConnection::new`],
[`crate::server::UnbufferedServerConnection::new`] and
[`unbuffered::UnbufferedConnectionCommon::process_tls_records`]. The state machine API is
documented in [`unbuffered::ConnectionState`].

# Examples

[`unbuffered-client`] and [`unbuffered-server`] are examples that fully exercise the API in
std, non-async context.

[`unbuffered-client`]: https://github.com/rustls/rustls/blob/main/examples/src/bin/unbuffered-client.rs
[`unbuffered-server`]: https://github.com/rustls/rustls/blob/main/examples/src/bin/unbuffered-server.rs



## Module: version

All defined protocol versions appear in this module.

ALL_VERSIONS is a provided as an array of all of these values.



