**rustls > msgs > message > outbound**

# Module: msgs::message::outbound

## Contents

**Structs**

- [`OutboundOpaqueMessage`](#outboundopaquemessage) - A TLS frame, named `TLSPlaintext` in the standard.
- [`OutboundPlainMessage`](#outboundplainmessage) - A TLS frame, named `TLSPlaintext` in the standard.
- [`PrefixedPayload`](#prefixedpayload)

**Enums**

- [`OutboundChunks`](#outboundchunks) - A collection of borrowed plaintext slices.

---

## rustls::msgs::message::outbound::OutboundChunks

*Enum*

A collection of borrowed plaintext slices.

Warning: OutboundChunks does not guarantee that the simplest variant is used.
Multiple can hold non fragmented or empty payloads.

**Generic Parameters:**
- 'a

**Variants:**
- `Single(&'a [u8])` - A single byte slice. Contrary to `Multiple`, this uses a single pointer indirection
- `Multiple{ chunks: &'a [&'a [u8]], start: usize, end: usize }` - A collection of chunks (byte slices)

**Methods:**

- `fn new(chunks: &'a [&'a [u8]]) -> Self` - Create a payload from a slice of byte slices.
- `fn new_empty() -> Self` - Create a payload with a single empty slice
- `fn to_vec(self: &Self) -> Vec<u8>` - Flatten the slice of byte slices to an owned vector of bytes
- `fn copy_to_vec(self: &Self, vec: & mut Vec<u8>)` - Append all bytes to a vector
- `fn split_at(self: &Self, mid: usize) -> (Self, Self)` - Split self in two, around an index
- `fn is_empty(self: &Self) -> bool` - Returns true if the payload is empty
- `fn len(self: &Self) -> usize` - Returns the cumulative length of all chunks

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> OutboundChunks<'a>`
- **From**
  - `fn from(payload: &'a [u8]) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::msgs::message::outbound::OutboundOpaqueMessage

*Struct*

A TLS frame, named `TLSPlaintext` in the standard.

This outbound type owns all memory for its interior parts.
It results from encryption and is used for io write.

**Fields:**
- `typ: crate::enums::ContentType`
- `version: crate::enums::ProtocolVersion`
- `payload: PrefixedPayload`

**Methods:**

- `fn new(typ: ContentType, version: ProtocolVersion, payload: PrefixedPayload) -> Self` - Construct a new `OpaqueMessage` from constituent fields.
- `fn read(r: & mut Reader) -> Result<Self, MessageError>` - Construct by decoding from a [`Reader`].
- `fn encode(self: Self) -> Vec<u8>`
- `fn into_plain_message(self: Self) -> PlainMessage` - Force conversion into a plaintext message.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> OutboundOpaqueMessage`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::msgs::message::outbound::OutboundPlainMessage

*Struct*

A TLS frame, named `TLSPlaintext` in the standard.

This outbound type borrows its "to be encrypted" payload from the "user".
It is used for fragmenting and is consumed by encryption.

**Generic Parameters:**
- 'a

**Fields:**
- `typ: crate::enums::ContentType`
- `version: crate::enums::ProtocolVersion`
- `payload: OutboundChunks<'a>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::msgs::message::outbound::PrefixedPayload

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn with_capacity(capacity: usize) -> Self`
- `fn extend_from_slice(self: & mut Self, slice: &[u8])`
- `fn extend_from_chunks(self: & mut Self, chunks: &OutboundChunks)`
- `fn truncate(self: & mut Self, len: usize)`

**Trait Implementations:**

- **From**
  - `fn from(content: &[u8]) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> PrefixedPayload`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut [u8]`
- **From**
  - `fn from(content: &[u8; N]) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`



