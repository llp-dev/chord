**rustls > quic**

# Module: quic

## Contents

**Structs**

- [`DirectionalKeys`](#directionalkeys) - Keys used to communicate in a single direction
- [`Keys`](#keys) - Complete set of keys used to communicate with the peer
- [`PacketKeySet`](#packetkeyset) - Packet protection keys for bidirectional 1-RTT communication
- [`Secrets`](#secrets) - Secrets used to encrypt/decrypt traffic
- [`Suite`](#suite) - Produces QUIC initial keys from a TLS 1.3 ciphersuite and a QUIC key generation algorithm.
- [`Tag`](#tag) - Authentication tag from an AEAD seal operation.

**Enums**

- [`KeyChange`](#keychange) - Key material for use in QUIC packet spaces
- [`Version`](#version) - QUIC protocol version

**Traits**

- [`Algorithm`](#algorithm) - How a `Tls13CipherSuite` generates `PacketKey`s and `HeaderProtectionKey`s.
- [`HeaderProtectionKey`](#headerprotectionkey) - A QUIC header protection key
- [`PacketKey`](#packetkey) - Keys to encrypt or decrypt the payload of a packet

---

## rustls::quic::Algorithm

*Trait*

How a `Tls13CipherSuite` generates `PacketKey`s and `HeaderProtectionKey`s.

**Methods:**

- `packet_key`: Produce a `PacketKey` encrypter/decrypter for this suite.
- `header_protection_key`: Produce a `HeaderProtectionKey` encrypter/decrypter for this suite.
- `aead_key_len`: The length in bytes of keys for this Algorithm.
- `fips`: Whether this algorithm is FIPS-approved.



## rustls::quic::DirectionalKeys

*Struct*

Keys used to communicate in a single direction

**Fields:**
- `header: alloc::boxed::Box<dyn HeaderProtectionKey>` - Encrypts or decrypts a packet's headers
- `packet: alloc::boxed::Box<dyn PacketKey>` - Encrypts or decrypts the payload of a packet



## rustls::quic::HeaderProtectionKey

*Trait*

A QUIC header protection key

**Methods:**

- `encrypt_in_place`: Adds QUIC Header Protection.
- `decrypt_in_place`: Removes QUIC Header Protection.
- `sample_len`: Expected sample length for the key's algorithm



## rustls::quic::KeyChange

*Enum*

Key material for use in QUIC packet spaces

QUIC uses 4 different sets of keys (and progressive key updates for long-running connections):

* Initial: these can be created from [`Keys::initial()`]
* 0-RTT keys: can be retrieved from [`ConnectionCommon::zero_rtt_keys()`]
* Handshake: these are returned from [`ConnectionCommon::write_hs()`] after `ClientHello` and
  `ServerHello` messages have been exchanged
* 1-RTT keys: these are returned from [`ConnectionCommon::write_hs()`] after the handshake is done

Once the 1-RTT keys have been exchanged, either side may initiate a key update. Progressive
update keys can be obtained from the [`Secrets`] returned in [`KeyChange::OneRtt`]. Note that
only packet keys are updated by key updates; header protection keys remain the same.

**Variants:**
- `Handshake{ keys: Keys }` - Keys for the handshake space
- `OneRtt{ keys: Keys, next: Secrets }` - Keys for 1-RTT data



## rustls::quic::Keys

*Struct*

Complete set of keys used to communicate with the peer

**Fields:**
- `local: DirectionalKeys` - Encrypts outgoing packets
- `remote: DirectionalKeys` - Decrypts incoming packets

**Methods:**

- `fn initial(version: Version, suite: &'static Tls13CipherSuite, quic: &'static dyn Algorithm, client_dst_connection_id: &[u8], side: Side) -> Self` - Construct keys for use with initial packets



## rustls::quic::PacketKey

*Trait*

Keys to encrypt or decrypt the payload of a packet

**Methods:**

- `encrypt_in_place`: Encrypt a QUIC packet
- `encrypt_in_place_for_path`: Encrypts a multipath QUIC packet
- `decrypt_in_place`: Decrypt a QUIC packet
- `decrypt_in_place_for_path`: Decrypt a multipath QUIC packet
- `tag_len`: Tag length for the underlying AEAD algorithm
- `confidentiality_limit`: Number of QUIC messages that can be safely encrypted with a single key of this type.
- `integrity_limit`: Number of QUIC messages that can be safely decrypted with a single key of this type



## rustls::quic::PacketKeySet

*Struct*

Packet protection keys for bidirectional 1-RTT communication

**Fields:**
- `local: alloc::boxed::Box<dyn PacketKey>` - Encrypts outgoing packets
- `remote: alloc::boxed::Box<dyn PacketKey>` - Decrypts incoming packets



## rustls::quic::Secrets

*Struct*

Secrets used to encrypt/decrypt traffic

**Methods:**

- `fn next_packet_keys(self: & mut Self) -> PacketKeySet` - Derive the next set of packet keys

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Secrets`



## rustls::quic::Suite

*Struct*

Produces QUIC initial keys from a TLS 1.3 ciphersuite and a QUIC key generation algorithm.

**Fields:**
- `suite: &'static crate::tls13::Tls13CipherSuite` - The TLS 1.3 ciphersuite used to derive keys.
- `quic: &'static dyn Algorithm` - The QUIC key generation algorithm used to derive keys.

**Methods:**

- `fn keys(self: &Self, client_dst_connection_id: &[u8], side: Side, version: Version) -> Keys` - Produce a set of initial keys given the connection ID, side and version

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Suite`



## rustls::quic::Tag

*Struct*

Authentication tag from an AEAD seal operation.

**Tuple Struct**: `()`

**Trait Implementations:**

- **From**
  - `fn from(value: &[u8]) -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## rustls::quic::Version

*Enum*

QUIC protocol version

Governs version-specific behavior in the TLS layer

**Variants:**
- `V1Draft` - Draft versions 29, 30, 31 and 32
- `V1` - First stable RFC
- `V2` - Anti-ossification variant of V1

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Version`
- **Clone**
  - `fn clone(self: &Self) -> Version`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



