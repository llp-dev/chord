**rustls > conn > kernel**

# Module: conn::kernel

## Contents

**Structs**

- [`KernelConnection`](#kernelconnection) - A kernel connection.

---

## rustls::conn::kernel::KernelConnection

*Struct*

A kernel connection.

This does not directly wrap a kernel connection, rather it gives you the
minimal interfaces you need to implement a well-behaved TLS connection on
top of kTLS.

See the [`crate::kernel`] module docs for more details.

**Generic Parameters:**
- Data

**Methods:**

- `fn handle_new_session_ticket(self: & mut Self, payload: &[u8]) -> Result<(), Error>` - Handle a `new_session_ticket` message from the peer.
- `fn negotiated_cipher_suite(self: &Self) -> SupportedCipherSuite` - Retrieves the ciphersuite agreed with the peer.
- `fn protocol_version(self: &Self) -> ProtocolVersion` - Retrieves the protocol version agreed with the peer.
- `fn update_tx_secret(self: & mut Self) -> Result<(u64, ConnectionTrafficSecrets), Error>` - Update the traffic secret used for encrypting messages sent to the peer.
- `fn update_rx_secret(self: & mut Self) -> Result<(u64, ConnectionTrafficSecrets), Error>` - Update the traffic secret used for decrypting messages received from the



