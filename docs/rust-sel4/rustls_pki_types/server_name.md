**rustls_pki_types > server_name**

# Module: server_name

## Contents

**Structs**

- [`AddrParseError`](#addrparseerror) - Failure to parse an IP address
- [`DnsName`](#dnsname) - A type which encapsulates a string (borrowed or owned) that is a syntactically valid DNS name.
- [`InvalidDnsNameError`](#invaliddnsnameerror) - The provided input could not be parsed because
- [`Ipv4Addr`](#ipv4addr) - `no_std` implementation of `std::net::Ipv4Addr`.
- [`Ipv6Addr`](#ipv6addr) - `no_std` implementation of `std::net::Ipv6Addr`.

**Enums**

- [`IpAddr`](#ipaddr) - `no_std` implementation of `std::net::IpAddr`.
- [`ServerName`](#servername) - Encodes ways a client can know the expected name of the server.

---

## rustls_pki_types::server_name::AddrParseError

*Struct*

Failure to parse an IP address

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &AddrParseError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AddrParseError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## rustls_pki_types::server_name::DnsName

*Struct*

A type which encapsulates a string (borrowed or owned) that is a syntactically valid DNS name.

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn borrow(self: &'a Self) -> Self` - Produce a borrowed `DnsName` from this owned `DnsName`.
- `fn to_lowercase_owned(self: &Self) -> DnsName<'static>` - Copy this object to produce an owned `DnsName`, smashing the case to lowercase
- `fn to_owned(self: &Self) -> DnsName<'static>` - Produce an owned `DnsName` from this (potentially borrowed) `DnsName`.
- `fn try_from_str(s: &str) -> Result<DnsName, InvalidDnsNameError>` - Produces a borrowed [`DnsName`] from a borrowed [`str`].

**Traits:** Eq

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(value: &'a [u8]) -> Result<Self, <Self as >::Error>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &DnsName<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DnsName<'a>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &str`
- **TryFrom**
  - `fn try_from(value: String) -> Result<Self, <Self as >::Error>`
- **TryFrom**
  - `fn try_from(value: &'a str) -> Result<Self, <Self as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls_pki_types::server_name::InvalidDnsNameError

*Struct*

The provided input could not be parsed because
it is not a syntactically-valid DNS Name.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## rustls_pki_types::server_name::IpAddr

*Enum*

`no_std` implementation of `std::net::IpAddr`.

Note: because we intend to replace this type with `core::net::IpAddr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::IpAddr`.

**Variants:**
- `V4(Ipv4Addr)` - An Ipv4 address.
- `V6(Ipv6Addr)` - An Ipv6 address.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> IpAddr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **TryFrom**
  - `fn try_from(value: &str) -> Result<Self, <Self as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &IpAddr) -> bool`



## rustls_pki_types::server_name::Ipv4Addr

*Struct*

`no_std` implementation of `std::net::Ipv4Addr`.

Note: because we intend to replace this type with `core::net::Ipv4Addr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::Ipv4Addr`.

**Tuple Struct**: `()`

**Traits:** Copy, Eq

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(value: &str) -> Result<Self, <Self as >::Error>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Ipv4Addr) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8; 4]`
- **Clone**
  - `fn clone(self: &Self) -> Ipv4Addr`
- **From**
  - `fn from(value: [u8; 4]) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls_pki_types::server_name::Ipv6Addr

*Struct*

`no_std` implementation of `std::net::Ipv6Addr`.

Note: because we intend to replace this type with `core::net::Ipv6Addr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::Ipv6Addr`.

**Tuple Struct**: `()`

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: [u16; 8]) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Ipv6Addr) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8; 16]`
- **TryFrom**
  - `fn try_from(value: &str) -> Result<Self, <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Ipv6Addr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls_pki_types::server_name::ServerName

*Enum*

Encodes ways a client can know the expected name of the server.

This currently covers knowing the DNS name of the server, but
will be extended in the future to supporting privacy-preserving names
for the server ("ECH").  For this reason this enum is `non_exhaustive`.

# Making one

If you have a DNS name as a `&str`, this type implements `TryFrom<&str>`,
so you can do:

```
# use rustls_pki_types::ServerName;
ServerName::try_from("example.com").expect("invalid DNS name");
```

If you have an owned `String`, you can use `TryFrom` directly:

```
# use rustls_pki_types::ServerName;
let name = "example.com".to_string();
#[cfg(feature = "alloc")]
ServerName::try_from(name).expect("invalid DNS name");
```

which will yield a `ServerName<'static>` if successful.

or, alternatively...

```
# use rustls_pki_types::ServerName;
let x: ServerName = "example.com".try_into().expect("invalid DNS name");
```

**Generic Parameters:**
- 'a

**Variants:**
- `DnsName(DnsName<'a>)` - The server is identified by a DNS name.  The name
- `IpAddress(IpAddr)` - The server is identified by an IP address. SNI is not

**Methods:**

- `fn to_owned(self: &Self) -> ServerName<'static>` - Produce an owned `ServerName` from this (potentially borrowed) `ServerName`.

**Traits:** Eq

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(value: &'a [u8]) -> Result<Self, <Self as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(v6: Ipv6Addr) -> Self`
- **TryFrom**
  - `fn try_from(s: &'a str) -> Result<Self, <Self as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ServerName<'a>) -> bool`
- **From**
  - `fn from(dns_name: DnsName<'a>) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> ServerName<'a>`
- **From**
  - `fn from(addr: IpAddr) -> Self`
- **TryFrom**
  - `fn try_from(value: String) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(v4: Ipv4Addr) -> Self`



