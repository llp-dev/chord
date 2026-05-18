**sel4_async_network**

# Module: sel4_async_network

## Contents

**Structs**

- [`DhcpOverrides`](#dhcpoverrides)
- [`ManagedInterface`](#managedinterface)
- [`Socket`](#socket)

**Enums**

- [`DnsError`](#dnserror)
- [`TcpSocketError`](#tcpsocketerror)

**Type Aliases**

- [`TcpSocket`](#tcpsocket)

---

## sel4_async_network::DhcpOverrides

*Struct*

**Fields:**
- `address: Option<smoltcp::wire::Ipv4Cidr>`
- `router: Option<Option<smoltcp::wire::Ipv4Address>>`
- `dns_servers: Option<alloc::vec::Vec<smoltcp::wire::Ipv4Address>>`

**Trait Implementations:**

- **Default**
  - `fn default() -> DhcpOverrides`



## sel4_async_network::DnsError

*Enum*

**Variants:**
- `StartQueryError(dns::StartQueryError)`
- `GetQueryResultError(dns::GetQueryResultError)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DnsError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DnsError`



## sel4_async_network::ManagedInterface

*Struct*

**Methods:**

- `fn new<D>(config: Config, dhcp_overrides: DhcpOverrides, device: & mut D, instant: Instant) -> Self`
- `fn new_tcp_socket(self: &Self) -> TcpSocket`
- `fn new_tcp_socket_with_buffer_sizes(self: &Self, rx_buffer_size: usize, tx_buffer_size: usize) -> TcpSocket`
- `fn new_socket<T>(self: &Self, socket: T) -> Socket<T>`
- `fn poll_at(self: &Self, timestamp: Instant) -> Option<Instant>`
- `fn poll_delay(self: &Self, timestamp: Instant) -> Option<Duration>`
- `fn poll<D>(self: &Self, timestamp: Instant, device: & mut D) -> bool`
- `fn dns_query(self: &Self, name: &str, query_type: DnsQueryType) -> Result<Vec<IpAddress>, DnsError>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ManagedInterface`



## sel4_async_network::Socket

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn with<R, impl FnOnce(&T) -> R>(self: &Self, f: impl Trait) -> R`
- `fn with_mut<R, impl FnOnce(&mut T) -> R>(self: & mut Self, f: impl Trait) -> R`
- `fn with_context_mut<R, impl FnOnce(&mut Context, &mut T) -> R>(self: & mut Self, f: impl Trait) -> R`
- `fn connect<T, U>(self: & mut Self, remote_endpoint: T, local_endpoint: U) -> Result<(), TcpSocketError>`
- `fn accept_with_keep_alive<impl Into<IpListenEndpoint>>(self: & mut Self, local_endpoint: impl Trait, keep_alive_interval: Option<Duration>) -> Result<(), TcpSocketError>`
- `fn accept<impl Into<IpListenEndpoint>>(self: & mut Self, local_endpoint: impl Trait) -> Result<(), TcpSocketError>`
- `fn close(self: & mut Self)`
- `fn abort(self: & mut Self)`

**Traits:** ErrorType

**Trait Implementations:**

- **Write**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut task::Context, buf: &[u8]) -> Poll<Result<usize, <Self as >::Error>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut task::Context) -> Poll<Result<(), <Self as >::Error>>`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Read**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut task::Context, buf: & mut [u8]) -> Poll<Result<usize, <Self as >::Error>>`



## sel4_async_network::TcpSocket

*Type Alias*: `Socket<tcp::Socket<'static>>`



## sel4_async_network::TcpSocketError

*Enum*

**Variants:**
- `InvalidState(tcp::State)`
- `RecvError(tcp::RecvError)`
- `SendError(tcp::SendError)`
- `ListenError(tcp::ListenError)`
- `ConnectError(tcp::ConnectError)`
- `ConnectionResetDuringConnect`

**Traits:** Error, Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TcpSocketError`
- **PartialEq**
  - `fn eq(self: &Self, other: &TcpSocketError) -> bool`
- **Error**
  - `fn kind(self: &Self) -> ErrorKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



