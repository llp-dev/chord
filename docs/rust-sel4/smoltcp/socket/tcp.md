**smoltcp > socket > tcp**

# Module: socket::tcp

## Contents

**Structs**

- [`Socket`](#socket) - A Transmission Control Protocol socket.

**Enums**

- [`CongestionControl`](#congestioncontrol) - A congestion control algorithm.
- [`ConnectError`](#connecterror) - Error returned by [`Socket::connect`]
- [`ListenError`](#listenerror) - Error returned by [`Socket::listen`]
- [`RecvError`](#recverror) - Error returned by [`Socket::recv`]
- [`SendError`](#senderror) - Error returned by [`Socket::send`]
- [`State`](#state) - The state of a TCP socket, according to [RFC 793].

**Type Aliases**

- [`SocketBuffer`](#socketbuffer) - A TCP socket ring buffer.

---

## smoltcp::socket::tcp::CongestionControl

*Enum*

A congestion control algorithm.

**Variants:**
- `None`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &CongestionControl) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CongestionControl`



## smoltcp::socket::tcp::ConnectError

*Enum*

Error returned by [`Socket::connect`]

**Variants:**
- `InvalidState`
- `Unaddressable`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ConnectError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ConnectError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## smoltcp::socket::tcp::ListenError

*Enum*

Error returned by [`Socket::listen`]

**Variants:**
- `InvalidState`
- `Unaddressable`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ListenError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ListenError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## smoltcp::socket::tcp::RecvError

*Enum*

Error returned by [`Socket::recv`]

**Variants:**
- `InvalidState`
- `Finished`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RecvError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RecvError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## smoltcp::socket::tcp::SendError

*Enum*

Error returned by [`Socket::send`]

**Variants:**
- `InvalidState`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SendError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SendError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## smoltcp::socket::tcp::Socket

*Struct*

A Transmission Control Protocol socket.

A TCP socket may passively listen for connections or actively connect to another endpoint.
Note that, for listening sockets, there is no "backlog"; to be able to simultaneously
accept several connections, as many sockets must be allocated, or any new connection
attempts will be reset.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<T>(rx_buffer: T, tx_buffer: T) -> Socket<'a>` - Create a socket using the given buffers.
- `fn set_tsval_generator(self: & mut Self, generator: Option<TcpTimestampGenerator>)` - Enable or disable TCP Timestamp.
- `fn timestamp_enabled(self: &Self) -> bool` - Return whether TCP Timestamp is enabled.
- `fn set_congestion_control(self: & mut Self, congestion_control: CongestionControl)` - Set an algorithm for congestion control.
- `fn congestion_control(self: &Self) -> CongestionControl` - Return the current congestion control algorithm.
- `fn register_recv_waker(self: & mut Self, waker: &Waker)` - Register a waker for receive operations.
- `fn register_send_waker(self: & mut Self, waker: &Waker)` - Register a waker for send operations.
- `fn timeout(self: &Self) -> Option<Duration>` - Return the timeout duration.
- `fn ack_delay(self: &Self) -> Option<Duration>` - Return the ACK delay duration.
- `fn nagle_enabled(self: &Self) -> bool` - Return whether Nagle's Algorithm is enabled.
- `fn set_timeout(self: & mut Self, duration: Option<Duration>)` - Set the timeout duration.
- `fn set_ack_delay(self: & mut Self, duration: Option<Duration>)` - Set the ACK delay duration.
- `fn set_nagle_enabled(self: & mut Self, enabled: bool)` - Enable or disable Nagle's Algorithm.
- `fn keep_alive(self: &Self) -> Option<Duration>` - Return the keep-alive interval.
- `fn set_keep_alive(self: & mut Self, interval: Option<Duration>)` - Set the keep-alive interval.
- `fn hop_limit(self: &Self) -> Option<u8>` - Return the time-to-live (IPv4) or hop limit (IPv6) value used in outgoing packets.
- `fn set_hop_limit(self: & mut Self, hop_limit: Option<u8>)` - Set the time-to-live (IPv4) or hop limit (IPv6) value used in outgoing packets.
- `fn listen_endpoint(self: &Self) -> IpListenEndpoint` - Return the listen endpoint
- `fn local_endpoint(self: &Self) -> Option<IpEndpoint>` - Return the local endpoint, or None if not connected.
- `fn remote_endpoint(self: &Self) -> Option<IpEndpoint>` - Return the remote endpoint, or None if not connected.
- `fn state(self: &Self) -> State` - Return the connection state, in terms of the TCP state machine.
- `fn listen<T>(self: & mut Self, local_endpoint: T) -> Result<(), ListenError>` - Start listening on the given endpoint.
- `fn connect<T, U>(self: & mut Self, cx: & mut Context, remote_endpoint: T, local_endpoint: U) -> Result<(), ConnectError>` - Connect to a given endpoint.
- `fn close(self: & mut Self)` - Close the transmit half of the full-duplex connection.
- `fn abort(self: & mut Self)` - Aborts the connection, if any.
- `fn is_listening(self: &Self) -> bool` - Return whether the socket is passively listening for incoming connections.
- `fn is_open(self: &Self) -> bool` - Return whether the socket is open.
- `fn is_active(self: &Self) -> bool` - Return whether a connection is active.
- `fn may_send(self: &Self) -> bool` - Return whether the transmit half of the full-duplex connection is open.
- `fn may_recv(self: &Self) -> bool` - Return whether the receive half of the full-duplex connection is open.
- `fn can_send(self: &Self) -> bool` - Check whether the transmit half of the full-duplex connection is open
- `fn recv_capacity(self: &Self) -> usize` - Return the maximum number of bytes inside the recv buffer.
- `fn send_capacity(self: &Self) -> usize` - Return the maximum number of bytes inside the transmit buffer.
- `fn can_recv(self: &Self) -> bool` - Check whether the receive buffer is not empty.
- `fn send<'b, F, R>(self: &'b  mut Self, f: F) -> Result<R, SendError>` - Call `f` with the largest contiguous slice of octets in the transmit buffer,
- `fn send_slice(self: & mut Self, data: &[u8]) -> Result<usize, SendError>` - Enqueue a sequence of octets to be sent, and fill it from a slice.
- `fn recv<'b, F, R>(self: &'b  mut Self, f: F) -> Result<R, RecvError>` - Call `f` with the largest contiguous slice of octets in the receive buffer,
- `fn recv_slice(self: & mut Self, data: & mut [u8]) -> Result<usize, RecvError>` - Dequeue a sequence of received octets, and fill a slice from it.
- `fn peek(self: & mut Self, size: usize) -> Result<&[u8], RecvError>` - Peek at a sequence of received octets without removing them from
- `fn peek_slice(self: & mut Self, data: & mut [u8]) -> Result<usize, RecvError>` - Peek at a sequence of received octets without removing them from
- `fn send_queue(self: &Self) -> usize` - Return the amount of octets queued in the transmit buffer.
- `fn recv_queue(self: &Self) -> usize` - Return the amount of octets queued in the receive buffer. This value can be larger than

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Write**
  - `fn write_str(self: & mut Self, slice: &str) -> fmt::Result`
- **AnySocket**
  - `fn upcast(self: Self) -> Socket<'a>`
  - `fn downcast<'c>(socket: &'c Socket<'a>) -> Option<&'c Self>`
  - `fn downcast_mut<'c>(socket: &'c  mut Socket<'a>) -> Option<&'c  mut Self>`



## smoltcp::socket::tcp::SocketBuffer

*Type Alias*: `crate::storage::RingBuffer<'a, u8>`

A TCP socket ring buffer.



## smoltcp::socket::tcp::State

*Enum*

The state of a TCP socket, according to [RFC 793].

[RFC 793]: https://tools.ietf.org/html/rfc793

**Variants:**
- `Closed`
- `Listen`
- `SynSent`
- `SynReceived`
- `Established`
- `FinWait1`
- `FinWait2`
- `CloseWait`
- `Closing`
- `LastAck`
- `TimeWait`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &State) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> State`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



