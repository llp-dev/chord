**rustls > conn > unbuffered**

# Module: conn::unbuffered

## Contents

**Structs**

- [`AppDataRecord`](#appdatarecord) - A decrypted application-data record
- [`EncodeTlsData`](#encodetlsdata) - A handshake record must be encoded
- [`InsufficientSizeError`](#insufficientsizeerror) - Provided buffer was too small
- [`ReadEarlyData`](#readearlydata) - Early application-data is available.
- [`ReadTraffic`](#readtraffic) - Application data is available
- [`TransmitTlsData`](#transmittlsdata) - Previously encoded TLS data must be transmitted
- [`UnbufferedStatus`](#unbufferedstatus) - The current status of the `UnbufferedConnection*`
- [`WriteTraffic`](#writetraffic) - Allows encrypting app-data

**Enums**

- [`ConnectionState`](#connectionstate) - The state of the [`UnbufferedConnectionCommon`] object
- [`EncodeError`](#encodeerror) - Errors that may arise when encoding a handshake record
- [`EncryptError`](#encrypterror) - Errors that may arise when encrypting application data

---

## rustls::conn::unbuffered::AppDataRecord

*Struct*

A decrypted application-data record

**Generic Parameters:**
- 'i

**Fields:**
- `discard: usize` - Number of additional bytes to discard
- `payload: &'i [u8]` - The payload of the app-data record



## rustls::conn::unbuffered::ConnectionState

*Enum*

The state of the [`UnbufferedConnectionCommon`] object

**Generic Parameters:**
- 'c
- 'i
- Data

**Variants:**
- `ReadTraffic(ReadTraffic<'c, 'i, Data>)` - One, or more, application data records are available
- `PeerClosed` - Connection has been cleanly closed by the peer.
- `Closed` - Connection has been cleanly closed by both us and the peer.
- `ReadEarlyData(ReadEarlyData<'c, 'i, Data>)` - One, or more, early (RTT-0) data records are available
- `EncodeTlsData(EncodeTlsData<'c, Data>)` - A Handshake record is ready for encoding
- `TransmitTlsData(TransmitTlsData<'c, Data>)` - Previously encoded handshake records need to be transmitted
- `BlockedHandshake` - More TLS data is needed to continue with the handshake
- `WriteTraffic(WriteTraffic<'c, Data>)` - The handshake process has been completed.

**Trait Implementations:**

- **From**
  - `fn from(v: ReadTraffic<'c, 'i, Data>) -> Self`
- **From**
  - `fn from(v: TransmitTlsData<'c, Data>) -> Self`
- **From**
  - `fn from(v: ReadEarlyData<'c, 'i, Data>) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(v: EncodeTlsData<'c, Data>) -> Self`



## rustls::conn::unbuffered::EncodeError

*Enum*

Errors that may arise when encoding a handshake record

**Variants:**
- `InsufficientSize(InsufficientSizeError)` - Provided buffer was too small
- `AlreadyEncoded` - The handshake record has already been encoded; do not call `encode` again

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(v: InsufficientSizeError) -> Self`



## rustls::conn::unbuffered::EncodeTlsData

*Struct*

A handshake record must be encoded

**Generic Parameters:**
- 'c
- Data

**Methods:**

- `fn encode(self: & mut Self, outgoing_tls: & mut [u8]) -> Result<usize, EncodeError>` - Encodes a handshake record into the `outgoing_tls` buffer



## rustls::conn::unbuffered::EncryptError

*Enum*

Errors that may arise when encrypting application data

**Variants:**
- `InsufficientSize(InsufficientSizeError)` - Provided buffer was too small
- `EncryptExhausted` - Encrypter has been exhausted

**Trait Implementations:**

- **From**
  - `fn from(v: InsufficientSizeError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::conn::unbuffered::InsufficientSizeError

*Struct*

Provided buffer was too small

**Fields:**
- `required_size: usize` - buffer must be at least this size

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> InsufficientSizeError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::conn::unbuffered::ReadEarlyData

*Struct*

Early application-data is available.

**Generic Parameters:**
- 'c
- 'i
- Data

**Methods:**

- `fn next_record(self: & mut Self) -> Option<Result<AppDataRecord, Error>>` - decrypts and returns the next available app-data record
- `fn peek_len(self: &Self) -> Option<NonZeroUsize>` - returns the payload size of the next app-data record *without* decrypting it



## rustls::conn::unbuffered::ReadTraffic

*Struct*

Application data is available

**Generic Parameters:**
- 'c
- 'i
- Data

**Methods:**

- `fn next_record(self: & mut Self) -> Option<Result<AppDataRecord, Error>>` - Decrypts and returns the next available app-data record
- `fn peek_len(self: &Self) -> Option<NonZeroUsize>` - Returns the payload size of the next app-data record *without* decrypting it



## rustls::conn::unbuffered::TransmitTlsData

*Struct*

Previously encoded TLS data must be transmitted

**Generic Parameters:**
- 'c
- Data

**Methods:**

- `fn done(self: Self)` - Signals that the previously encoded TLS data has been transmitted
- `fn may_encrypt_app_data(self: & mut Self) -> Option<WriteTraffic<Data>>` - Returns an adapter that allows encrypting application data
- `fn may_encrypt_early_data(self: & mut Self) -> Option<MayEncryptEarlyData>` - returns an adapter that allows encrypting early (RTT-0) data before transmitting the



## rustls::conn::unbuffered::UnbufferedStatus

*Struct*

The current status of the `UnbufferedConnection*`

**Generic Parameters:**
- 'c
- 'i
- Data

**Fields:**
- `discard: usize` - Number of bytes to discard
- `state: Result<ConnectionState<'c, 'i, Data>, crate::Error>` - The current state of the handshake process

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::conn::unbuffered::WriteTraffic

*Struct*

Allows encrypting app-data

**Generic Parameters:**
- 'c
- Data

**Methods:**

- `fn encrypt(self: & mut Self, application_data: &[u8], outgoing_tls: & mut [u8]) -> Result<usize, EncryptError>` - Encrypts `application_data` into the `outgoing_tls` buffer
- `fn queue_close_notify(self: & mut Self, outgoing_tls: & mut [u8]) -> Result<usize, EncryptError>` - Encrypts a close_notify warning alert in `outgoing_tls`
- `fn refresh_traffic_keys(self: Self) -> Result<(), Error>` - Arranges for a TLS1.3 `key_update` to be sent.



