**rustls > msgs > message > inbound**

# Module: msgs::message::inbound

## Contents

**Structs**

- [`BorrowedPayload`](#borrowedpayload)
- [`InboundOpaqueMessage`](#inboundopaquemessage) - A TLS frame, named TLSPlaintext in the standard.
- [`InboundPlainMessage`](#inboundplainmessage) - A TLS frame, named `TLSPlaintext` in the standard.

---

## rustls::msgs::message::inbound::BorrowedPayload

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn truncate(self: & mut Self, len: usize)`

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## rustls::msgs::message::inbound::InboundOpaqueMessage

*Struct*

A TLS frame, named TLSPlaintext in the standard.

This inbound type borrows its encrypted payload from a buffer elsewhere.
It is used for joining and is consumed by decryption.

**Generic Parameters:**
- 'a

**Fields:**
- `typ: crate::enums::ContentType`
- `version: crate::enums::ProtocolVersion`
- `payload: BorrowedPayload<'a>`

**Methods:**

- `fn new(typ: ContentType, version: ProtocolVersion, payload: &'a  mut [u8]) -> Self` - Construct a new `InboundOpaqueMessage` from constituent fields.
- `fn into_plain_message(self: Self) -> InboundPlainMessage<'a>` - Force conversion into a plaintext message.
- `fn into_plain_message_range(self: Self, range: Range<usize>) -> InboundPlainMessage<'a>` - Force conversion into a plaintext message.
- `fn into_tls13_unpadded_message(self: Self) -> Result<InboundPlainMessage<'a>, Error>` - For TLS1.3 (only), checks the length msg.payload is valid and removes the padding.



## rustls::msgs::message::inbound::InboundPlainMessage

*Struct*

A TLS frame, named `TLSPlaintext` in the standard.

This inbound type borrows its decrypted payload from the original buffer.
It results from decryption.

**Generic Parameters:**
- 'a

**Fields:**
- `typ: crate::enums::ContentType`
- `version: crate::enums::ProtocolVersion`
- `payload: &'a [u8]`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



