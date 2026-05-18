**sel4_async_network_rustls > conn**

# Module: conn

## Contents

**Structs**

- [`ClientConnector`](#clientconnector)
- [`Connect`](#connect)
- [`ServerConnector`](#serverconnector)
- [`TlsStream`](#tlsstream)

---

## sel4_async_network_rustls::conn::ClientConnector

*Struct*

**Methods:**

- `fn connect<IO>(self: &Self, domain: ServerName<'static>, stream: IO) -> Result<Connect<UnbufferedClientConnection, ClientConnectionData, IO>, Error<<IO as >::Error>>`

**Trait Implementations:**

- **From**
  - `fn from(config: Arc<ClientConfig>) -> Self`



## sel4_async_network_rustls::conn::Connect

*Struct*

**Generic Parameters:**
- T
- D
- IO



## sel4_async_network_rustls::conn::ServerConnector

*Struct*

**Methods:**

- `fn connect<IO>(self: &Self, stream: IO) -> Result<Connect<UnbufferedServerConnection, ServerConnectionData, IO>, Error<<IO as >::Error>>`

**Trait Implementations:**

- **From**
  - `fn from(config: Arc<ServerConfig>) -> Self`



## sel4_async_network_rustls::conn::TlsStream

*Struct*

**Generic Parameters:**
- T
- D
- IO

**Methods:**

- `fn into_io(self: Self) -> IO`

**Traits:** ErrorType

**Trait Implementations:**

- **Read**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut task::Context, buf: & mut [u8]) -> Poll<Result<usize, <Self as >::Error>>`
- **Write**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut task::Context, buf: &[u8]) -> Poll<Result<usize, <Self as >::Error>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut task::Context) -> Poll<Result<(), Error<<IO as >::Error>>>`



