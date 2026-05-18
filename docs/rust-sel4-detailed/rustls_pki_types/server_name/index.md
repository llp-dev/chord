*[rustls_pki_types](../index.md) / [server_name](index.md)*

---

# Module `server_name`

DNS name validation according to RFC1035, but with underscores allowed.

## Contents

- [Modules](#modules)
  - [`parser`](#parser)
- [Structs](#structs)
  - [`DnsName`](#dnsname)
  - [`InvalidDnsNameError`](#invaliddnsnameerror)
  - [`Ipv4Addr`](#ipv4addr)
  - [`Ipv6Addr`](#ipv6addr)
  - [`AddrParseError`](#addrparseerror)
- [Enums](#enums)
  - [`ServerName`](#servername)
  - [`DnsNameInner`](#dnsnameinner)
  - [`IpAddr`](#ipaddr)
- [Functions](#functions)
  - [`validate`](#validate)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parser`](#parser) | mod |  |
| [`DnsName`](#dnsname) | struct | A type which encapsulates a string (borrowed or owned) that is a syntactically valid DNS name. |
| [`InvalidDnsNameError`](#invaliddnsnameerror) | struct | The provided input could not be parsed because it is not a syntactically-valid DNS Name. |
| [`Ipv4Addr`](#ipv4addr) | struct | `no_std` implementation of `std::net::Ipv4Addr`. |
| [`Ipv6Addr`](#ipv6addr) | struct | `no_std` implementation of `std::net::Ipv6Addr`. |
| [`AddrParseError`](#addrparseerror) | struct | Failure to parse an IP address |
| [`ServerName`](#servername) | enum | Encodes ways a client can know the expected name of the server. |
| [`DnsNameInner`](#dnsnameinner) | enum |  |
| [`IpAddr`](#ipaddr) | enum | `no_std` implementation of `std::net::IpAddr`. |
| [`validate`](#validate) | fn |  |

## Modules

- [`parser`](parser/index.md)

## Structs

### `DnsName<'a>`

```rust
struct DnsName<'a>(DnsNameInner<'a>);
```

A type which encapsulates a string (borrowed or owned) that is a syntactically valid DNS name.

#### Implementations

- <span id="dnsname-borrow"></span>`fn borrow(self: &'a Self) -> Self`

  Produce a borrowed `DnsName` from this owned `DnsName`.

- <span id="dnsname-to-lowercase-owned"></span>`fn to_lowercase_owned(&self) -> DnsName<'static>` — [`DnsName`](#dnsname)

  Copy this object to produce an owned `DnsName`, smashing the case to lowercase

  in one operation.

- <span id="dnsname-to-owned"></span>`fn to_owned(&self) -> DnsName<'static>` — [`DnsName`](#dnsname)

  Produce an owned `DnsName` from this (potentially borrowed) `DnsName`.

- <span id="dnsname-try-from-string"></span>`fn try_from_string(s: String) -> Result<Self, String>`

- <span id="dnsname-try-from-str"></span>`const fn try_from_str(s: &str) -> Result<DnsName<'_>, InvalidDnsNameError>` — [`DnsName`](#dnsname), [`InvalidDnsNameError`](#invaliddnsnameerror)

  Produces a borrowed [`DnsName`](#dnsname) from a borrowed [`str`](#str).

#### Trait Implementations

##### `impl AsRef for DnsName<'_>`

- <span id="dnsname-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl Clone for DnsName<'a>`

- <span id="dnsname-clone"></span>`fn clone(&self) -> DnsName<'a>` — [`DnsName`](#dnsname)

##### `impl Debug for DnsName<'a>`

- <span id="dnsname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DnsName<'a>`

##### `impl Hash for DnsName<'a>`

- <span id="dnsname-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DnsName<'a>`

- <span id="dnsname-partialeq-eq"></span>`fn eq(&self, other: &DnsName<'a>) -> bool` — [`DnsName`](#dnsname)

##### `impl StructuralPartialEq for DnsName<'a>`

### `InvalidDnsNameError`

```rust
struct InvalidDnsNameError;
```

The provided input could not be parsed because
it is not a syntactically-valid DNS Name.

#### Trait Implementations

##### `impl Debug for InvalidDnsNameError`

- <span id="invaliddnsnameerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for InvalidDnsNameError`

- <span id="invaliddnsnameerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for InvalidDnsNameError`

- <span id="invaliddnsnameerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Ipv4Addr`

```rust
struct Ipv4Addr([u8; 4]);
```

`no_std` implementation of `std::net::Ipv4Addr`.

Note: because we intend to replace this type with `core::net::Ipv4Addr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::Ipv4Addr`.

#### Trait Implementations

##### `impl AsRef for Ipv4Addr`

- <span id="ipv4addr-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 4]`

##### `impl Clone for Ipv4Addr`

- <span id="ipv4addr-clone"></span>`fn clone(&self) -> Ipv4Addr` — [`Ipv4Addr`](#ipv4addr)

##### `impl Copy for Ipv4Addr`

##### `impl Debug for Ipv4Addr`

- <span id="ipv4addr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ipv4Addr`

##### `impl Hash for Ipv4Addr`

- <span id="ipv4addr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ipv4Addr`

- <span id="ipv4addr-partialeq-eq"></span>`fn eq(&self, other: &Ipv4Addr) -> bool` — [`Ipv4Addr`](#ipv4addr)

##### `impl StructuralPartialEq for Ipv4Addr`

### `Ipv6Addr`

```rust
struct Ipv6Addr([u8; 16]);
```

`no_std` implementation of `std::net::Ipv6Addr`.

Note: because we intend to replace this type with `core::net::Ipv6Addr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::Ipv6Addr`.

#### Trait Implementations

##### `impl AsRef for Ipv6Addr`

- <span id="ipv6addr-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 16]`

##### `impl Clone for Ipv6Addr`

- <span id="ipv6addr-clone"></span>`fn clone(&self) -> Ipv6Addr` — [`Ipv6Addr`](#ipv6addr)

##### `impl Copy for Ipv6Addr`

##### `impl Debug for Ipv6Addr`

- <span id="ipv6addr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ipv6Addr`

##### `impl Hash for Ipv6Addr`

- <span id="ipv6addr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ipv6Addr`

- <span id="ipv6addr-partialeq-eq"></span>`fn eq(&self, other: &Ipv6Addr) -> bool` — [`Ipv6Addr`](#ipv6addr)

##### `impl StructuralPartialEq for Ipv6Addr`

### `AddrParseError`

```rust
struct AddrParseError(parser::AddrKind);
```

Failure to parse an IP address

#### Trait Implementations

##### `impl Clone for AddrParseError`

- <span id="addrparseerror-clone"></span>`fn clone(&self) -> AddrParseError` — [`AddrParseError`](#addrparseerror)

##### `impl Copy for AddrParseError`

##### `impl Debug for AddrParseError`

- <span id="addrparseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for AddrParseError`

- <span id="addrparseerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AddrParseError`

##### `impl PartialEq for AddrParseError`

- <span id="addrparseerror-partialeq-eq"></span>`fn eq(&self, other: &AddrParseError) -> bool` — [`AddrParseError`](#addrparseerror)

##### `impl StructuralPartialEq for AddrParseError`

##### `impl ToString for AddrParseError`

- <span id="addrparseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `ServerName<'a>`

```rust
enum ServerName<'a> {
    DnsName(DnsName<'a>),
    IpAddress(IpAddr),
}
```

Encodes ways a client can know the expected name of the server.

This currently covers knowing the DNS name of the server, but
will be extended in the future to supporting privacy-preserving names
for the server ("ECH").  For this reason this enum is `non_exhaustive`.

# Making one

If you have a DNS name as a `&str`, this type implements `TryFrom<&str>`,
so you can do:

```rust
use rustls_pki_types::ServerName;
ServerName::try_from("example.com").expect("invalid DNS name");
```

If you have an owned `String`, you can use `TryFrom` directly:

```rust
use rustls_pki_types::ServerName;
let name = "example.com".to_string();
#[cfg(feature = "alloc")]
ServerName::try_from(name).expect("invalid DNS name");
```

which will yield a `ServerName<'static>` if successful.

or, alternatively...

```rust
use rustls_pki_types::ServerName;
let x: ServerName = "example.com".try_into().expect("invalid DNS name");
```

#### Variants

- **`DnsName`**

  The server is identified by a DNS name.  The name
  is sent in the TLS Server Name Indication (SNI)
  extension.

- **`IpAddress`**

  The server is identified by an IP address. SNI is not
  done.

#### Implementations

- <span id="servername-to-owned"></span>`fn to_owned(&self) -> ServerName<'static>` — [`ServerName`](#servername)

  Produce an owned `ServerName` from this (potentially borrowed) `ServerName`.

#### Trait Implementations

##### `impl Clone for ServerName<'a>`

- <span id="servername-clone"></span>`fn clone(&self) -> ServerName<'a>` — [`ServerName`](#servername)

##### `impl Debug for ServerName<'_>`

- <span id="servername-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ServerName<'a>`

##### `impl Hash for ServerName<'a>`

- <span id="servername-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ServerName<'a>`

- <span id="servername-partialeq-eq"></span>`fn eq(&self, other: &ServerName<'a>) -> bool` — [`ServerName`](#servername)

##### `impl StructuralPartialEq for ServerName<'a>`

### `DnsNameInner<'a>`

```rust
enum DnsNameInner<'a> {
    Borrowed(&'a str),
    Owned(alloc::string::String),
}
```

#### Trait Implementations

##### `impl Clone for DnsNameInner<'a>`

- <span id="dnsnameinner-clone"></span>`fn clone(&self) -> DnsNameInner<'a>` — [`DnsNameInner`](#dnsnameinner)

##### `impl Debug for DnsNameInner<'_>`

- <span id="dnsnameinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DnsNameInner<'a>`

##### `impl Hash for DnsNameInner<'_>`

- <span id="dnsnameinner-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl PartialEq for DnsNameInner<'_>`

- <span id="dnsnameinner-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `IpAddr`

```rust
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

`no_std` implementation of `std::net::IpAddr`.

Note: because we intend to replace this type with `core::net::IpAddr` as soon as it is
stabilized, the identity of this type should not be considered semver-stable. However, the
attached interfaces are stable; they form a subset of those provided by `core::net::IpAddr`.

#### Variants

- **`V4`**

  An Ipv4 address.

- **`V6`**

  An Ipv6 address.

#### Trait Implementations

##### `impl Clone for IpAddr`

- <span id="ipaddr-clone"></span>`fn clone(&self) -> IpAddr` — [`IpAddr`](#ipaddr)

##### `impl Copy for IpAddr`

##### `impl Debug for IpAddr`

- <span id="ipaddr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IpAddr`

##### `impl Hash for IpAddr`

- <span id="ipaddr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for IpAddr`

- <span id="ipaddr-partialeq-eq"></span>`fn eq(&self, other: &IpAddr) -> bool` — [`IpAddr`](#ipaddr)

##### `impl StructuralPartialEq for IpAddr`

## Functions

### `validate`

```rust
const fn validate(input: &[u8]) -> Result<(), InvalidDnsNameError>
```

