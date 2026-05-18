**rkyv > net**

# Module: net

## Contents

**Structs**

- [`ArchivedIpv4Addr`](#archivedipv4addr) - An archived [`Ipv4Addr`].
- [`ArchivedIpv6Addr`](#archivedipv6addr) - An archived [`Ipv6Addr`].
- [`ArchivedSocketAddrV4`](#archivedsocketaddrv4) - An archived [`SocketAddrV4`].
- [`ArchivedSocketAddrV6`](#archivedsocketaddrv6) - An archived [`SocketAddrV6`].

**Enums**

- [`ArchivedIpAddr`](#archivedipaddr) - An archived [`IpAddr`].
- [`ArchivedSocketAddr`](#archivedsocketaddr) - An archived [`SocketAddr`].

---

## rkyv::net::ArchivedIpAddr

*Enum*

An archived [`IpAddr`].

**Variants:**
- `V4(ArchivedIpv4Addr)` - An IPv4 address.
- `V6(ArchivedIpv6Addr)` - An IPv6 address.

**Methods:**

- `fn is_ipv4(self: &Self) -> bool` - Returns `true` if this address is an [`IPv4`
- `fn is_ipv6(self: &Self) -> bool` - Returns `true` if this address is an [`IPv6`
- `fn as_ipaddr(self: &Self) -> IpAddr` - Returns an [`IpAddr`] with the same value.
- `fn is_loopback(self: &Self) -> bool` - Returns `true` if this is a loopback address.
- `fn is_multicast(self: &Self) -> bool` - Returns `true` if this is a multicast address.
- `fn is_unspecified(self: &Self) -> bool` - Returns `true` for the special 'unspecified' address.

**Traits:** Portable, Copy, Eq

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<IpAddr, <D as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &IpAddr) -> Option<cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedIpAddr) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedIpAddr) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedIpAddr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &IpAddr) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedIpAddr) -> $crate::option::Option<$crate::cmp::Ordering>`



## rkyv::net::ArchivedIpv4Addr

*Struct*

An archived [`Ipv4Addr`].

**Methods:**

- `fn octets(self: &Self) -> [u8; 4]` - Returns the four eight-bit integers that make up this address.
- `fn as_ipv4(self: &Self) -> Ipv4Addr` - Returns an [`Ipv4Addr`] with the same value.
- `fn is_broadcast(self: &Self) -> bool` - Returns `true` if this is a broadcast address (255.255.255.255).
- `fn is_documentation(self: &Self) -> bool` - Returns `true` if this address is in a range designated for
- `fn is_link_local(self: &Self) -> bool` - Returns `true` if the address is link-local (169.254.0.0/16).
- `fn is_loopback(self: &Self) -> bool` - Returns `true` if this is a loopback address (127.0.0.0/8).
- `fn is_multicast(self: &Self) -> bool` - Returns `true` if this is a multicast address (224.0.0.0/4).
- `fn is_private(self: &Self) -> bool` - Returns `true` if this is a private address.
- `fn is_unspecified(self: &Self) -> bool` - Returns `true` for the special 'unspecified' address (0.0.0.0).
- `fn to_ipv6_compatible(self: &Self) -> Ipv6Addr` - Converts this address to an IPv4-compatible
- `fn to_ipv6_mapped(self: &Self) -> Ipv6Addr` - Converts this address to an IPv4-mapped
- `fn emplace(octets: [u8; 4], out: Place<Self>)` - Emplaces an `ArchivedIpv4Addr` with the given octets into a place.

**Traits:** Eq, Portable, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedIpv4Addr) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedIpv4Addr) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, _: & mut D) -> Result<Ipv4Addr, <D as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Ipv4Addr) -> Option<cmp::Ordering>`
- **Default**
  - `fn default() -> ArchivedIpv4Addr`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedIpv4Addr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedIpv4Addr) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Ipv4Addr) -> bool`



## rkyv::net::ArchivedIpv6Addr

*Struct*

An archived [`Ipv6Addr`].

**Methods:**

- `fn segments(self: &Self) -> [u16; 8]` - Returns the eight 16-bit segments that make up this address.
- `fn as_ipv6(self: &Self) -> Ipv6Addr` - Returns an [`Ipv6Addr`] with the same value.
- `fn is_loopback(self: &Self) -> bool` - Returns `true` if this is a loopback address (::1).
- `fn is_multicast(self: &Self) -> bool` - Returns `true` if this is a multicast address (ff00::/8).
- `fn is_unspecified(self: &Self) -> bool` - Returns `true` for the special 'unspecified' address (::).
- `fn octets(self: &Self) -> [u8; 16]` - Returns the sixteen eight-bit integers the IPv6 address consists of.
- `fn to_ipv4(self: &Self) -> Option<Ipv4Addr>` - Converts this address to an [`IPv4` address](std::net::Ipv4Addr).
- `fn emplace(octets: [u8; 16], out: Place<Self>)` - Emplaces an `ArchivedIpv6Addr` with the given octets into a place.

**Traits:** Eq, Portable, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedIpv6Addr) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, _: & mut D) -> Result<Ipv6Addr, <D as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Ipv6Addr) -> Option<cmp::Ordering>`
- **Default**
  - `fn default() -> ArchivedIpv6Addr`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedIpv6Addr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedIpv6Addr) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Ipv6Addr) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedIpv6Addr) -> $crate::cmp::Ordering`



## rkyv::net::ArchivedSocketAddr

*Enum*

An archived [`SocketAddr`].

**Variants:**
- `V4(ArchivedSocketAddrV4)` - An IPv4 socket address.
- `V6(ArchivedSocketAddrV6)` - An IPv6 socket address.

**Methods:**

- `fn port(self: &Self) -> u16` - Returns the port number associated with this socket address.
- `fn is_ipv4(self: &Self) -> bool` - Returns `true` if the [IP address](std::net::IpAddr) in this
- `fn is_ipv6(self: &Self) -> bool` - Returns `true` if the [IP address](std::net::IpAddr) in this
- `fn as_socket_addr(self: &Self) -> SocketAddr` - Returns a [`SocketAddr`] with the same value.
- `fn ip(self: &Self) -> IpAddr` - Returns the IP address associated with this socket address.

**Traits:** Copy, Eq, Portable

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ArchivedSocketAddr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &SocketAddr) -> bool`
- **ToSocketAddrs**
  - `fn to_socket_addrs(self: &Self) -> io::Result<<Self as >::Iter>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedSocketAddr) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<SocketAddr, <D as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SocketAddr) -> Option<cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedSocketAddr) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedSocketAddr) -> bool`



## rkyv::net::ArchivedSocketAddrV4

*Struct*

An archived [`SocketAddrV4`].

**Methods:**

- `fn ip(self: &Self) -> &ArchivedIpv4Addr` - Returns the IP address associated with this socket address.
- `fn port(self: &Self) -> u16` - Returns the port number associated with this socket address.
- `fn as_socket_addr_v4(self: &Self) -> SocketAddrV4` - Returns a [`SocketAddrV4`] with the same value.
- `fn emplace(value: &SocketAddrV4, out: Place<Self>)` - Emplaces an `ArchivedSocketAddrV4` of the given `value` into a place.

**Traits:** Portable, Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SocketAddrV4) -> bool`
- **Default**
  - `fn default() -> ArchivedSocketAddrV4`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<SocketAddrV4, <D as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SocketAddrV4) -> Option<cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedSocketAddrV4`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ToSocketAddrs**
  - `fn to_socket_addrs(self: &Self) -> io::Result<<Self as >::Iter>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedSocketAddrV4) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedSocketAddrV4) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedSocketAddrV4) -> $crate::option::Option<$crate::cmp::Ordering>`



## rkyv::net::ArchivedSocketAddrV6

*Struct*

An archived [`SocketAddrV6`].

**Methods:**

- `fn flowinfo(self: &Self) -> u32` - Returns the flow information associated with this address.
- `fn ip(self: &Self) -> &ArchivedIpv6Addr` - Returns the IP address associated with this socket address.
- `fn port(self: &Self) -> u16` - Returns the port number associated with this socket address.
- `fn scope_id(self: &Self) -> u32` - Returns the scope ID associated with this address.
- `fn as_socket_addr_v6(self: &Self) -> SocketAddrV6` - Returns a [`SocketAddrV6`] with the same value.
- `fn emplace(value: &SocketAddrV6, out: Place<Self>)` - Emplaces an `ArchivedSocketAddrV6` of the given `value` into a place.

**Traits:** Copy, Portable, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SocketAddrV6) -> bool`
- **ToSocketAddrs**
  - `fn to_socket_addrs(self: &Self) -> io::Result<<Self as >::Iter>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedSocketAddrV6) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedSocketAddrV6) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedSocketAddrV6) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<SocketAddrV6, <D as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SocketAddrV6) -> Option<cmp::Ordering>`
- **Default**
  - `fn default() -> ArchivedSocketAddrV6`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedSocketAddrV6`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



