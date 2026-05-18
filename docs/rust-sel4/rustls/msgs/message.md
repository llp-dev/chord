**rustls > msgs > message**

# Module: msgs::message

## Contents

**Structs**

- [`Message`](#message) - A message with decoded payload
- [`PlainMessage`](#plainmessage) - A decrypted TLS frame

**Enums**

- [`MessageError`](#messageerror)
- [`MessagePayload`](#messagepayload)

---

## rustls::msgs::message::Message

*Struct*

A message with decoded payload

**Generic Parameters:**
- 'a

**Fields:**
- `version: crate::enums::ProtocolVersion`
- `payload: MessagePayload<'a>`

**Methods:**

- `fn is_handshake_type(self: &Self, hstyp: HandshakeType) -> bool`
- `fn build_alert(level: AlertLevel, desc: AlertDescription) -> Self`
- `fn build_key_update_notify() -> Self`
- `fn build_key_update_request() -> Self`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryFrom**
  - `fn try_from(plain: InboundPlainMessage<'a>) -> Result<Self, <Self as >::Error>`
- **TryFrom**
  - `fn try_from(plain: PlainMessage) -> Result<Self, <Self as >::Error>`



## rustls::msgs::message::MessageError

*Enum*

**Variants:**
- `TooShortForHeader`
- `TooShortForLength`
- `InvalidEmptyPayload`
- `MessageTooLarge`
- `InvalidContentType`
- `UnknownProtocolVersion`



## rustls::msgs::message::MessagePayload

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `Alert(crate::msgs::alert::AlertMessagePayload)`
- `Handshake{ parsed: crate::msgs::handshake::HandshakeMessagePayload<'a>, encoded: crate::msgs::base::Payload<'a> }`
- `HandshakeFlight(crate::msgs::base::Payload<'a>)`
- `ChangeCipherSpec(crate::msgs::ccs::ChangeCipherSpecPayload)`
- `ApplicationData(crate::msgs::base::Payload<'a>)`

**Methods:**

- `fn encode(self: &Self, bytes: & mut Vec<u8>)`
- `fn handshake(parsed: HandshakeMessagePayload<'a>) -> Self`
- `fn new(typ: ContentType, vers: ProtocolVersion, payload: &'a [u8]) -> Result<Self, InvalidMessage>`
- `fn content_type(self: &Self) -> ContentType`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::msgs::message::PlainMessage

*Struct*

A decrypted TLS frame

This type owns all memory for its interior parts. It can be decrypted from an OpaqueMessage
or encrypted into an OpaqueMessage, and it is also used for joining and fragmenting.

**Fields:**
- `typ: crate::enums::ContentType`
- `version: crate::enums::ProtocolVersion`
- `payload: crate::msgs::base::Payload<'static>`

**Methods:**

- `fn into_unencrypted_opaque(self: Self) -> OutboundOpaqueMessage`
- `fn borrow_inbound(self: &Self) -> InboundPlainMessage`
- `fn borrow_outbound(self: &Self) -> OutboundPlainMessage`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(msg: Message) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> PlainMessage`



