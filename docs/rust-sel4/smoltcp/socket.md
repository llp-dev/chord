**smoltcp > socket**

# Module: socket

## Contents

**Modules**

- [`dhcpv4`](#dhcpv4)
- [`dns`](#dns)
- [`tcp`](#tcp)

**Enums**

- [`Socket`](#socket) - A network socket.

**Traits**

- [`AnySocket`](#anysocket) - A conversion trait for network sockets.

---

## smoltcp::socket::AnySocket

*Trait*

A conversion trait for network sockets.

**Methods:**

- `upcast`
- `downcast`
- `downcast_mut`



## smoltcp::socket::Socket

*Enum*

A network socket.

This enumeration abstracts the various types of sockets based on the IP protocol.
To downcast a `Socket` value to a concrete socket, use the [AnySocket] trait,
e.g. to get `udp::Socket`, call `udp::Socket::downcast(socket)`.

It is usually more convenient to use [SocketSet::get] instead.

[AnySocket]: trait.AnySocket.html
[SocketSet::get]: struct.SocketSet.html#method.get

**Generic Parameters:**
- 'a

**Variants:**
- `Tcp(tcp::Socket<'a>)`
- `Dhcpv4(dhcpv4::Socket<'a>)`
- `Dns(dns::Socket<'a>)`

**Trait Implementations:**

- **AnySocket**
  - `fn upcast(self: Self) -> Socket<'a>`
  - `fn downcast<'c>(socket: &'c Socket<'a>) -> Option<&'c Self>`
  - `fn downcast_mut<'c>(socket: &'c  mut Socket<'a>) -> Option<&'c  mut Self>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: dhcpv4



## Module: dns



## Module: tcp



