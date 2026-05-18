**smoltcp > socket > dns**

# Module: socket::dns

## Contents

**Structs**

- [`DnsQuery`](#dnsquery) - State for an in-progress DNS query.
- [`QueryHandle`](#queryhandle) - A handle to an in-progress DNS query.
- [`Socket`](#socket) - A Domain Name System socket.

**Enums**

- [`GetQueryResultError`](#getqueryresulterror) - Error returned by [`Socket::get_query_result`]
- [`MulticastDns`](#multicastdns)
- [`StartQueryError`](#startqueryerror) - Error returned by [`Socket::start_query`]

---

## smoltcp::socket::dns::DnsQuery

*Struct*

State for an in-progress DNS query.

The only reason this struct is public is to allow the socket state
to be allocated externally.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::socket::dns::GetQueryResultError

*Enum*

Error returned by [`Socket::get_query_result`]

**Variants:**
- `Pending` - Query is not done yet.
- `Failed` - Query failed.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GetQueryResultError`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &GetQueryResultError) -> bool`



## smoltcp::socket::dns::MulticastDns

*Enum*

**Variants:**
- `Disabled`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::socket::dns::QueryHandle

*Struct*

A handle to an in-progress DNS query.

**Tuple Struct**: `()`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> QueryHandle`



## smoltcp::socket::dns::Socket

*Struct*

A Domain Name System socket.

A UDP socket is bound to a specific endpoint, and owns transmit and receive
packet buffers.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<Q>(servers: &[IpAddress], queries: Q) -> Socket<'a>` - Create a DNS socket.
- `fn update_servers(self: & mut Self, servers: &[IpAddress])` - Update the list of DNS servers, will replace all existing servers
- `fn hop_limit(self: &Self) -> Option<u8>` - Return the time-to-live (IPv4) or hop limit (IPv6) value used in outgoing packets.
- `fn set_hop_limit(self: & mut Self, hop_limit: Option<u8>)` - Set the time-to-live (IPv4) or hop limit (IPv6) value used in outgoing packets.
- `fn start_query(self: & mut Self, cx: & mut Context, name: &str, query_type: Type) -> Result<QueryHandle, StartQueryError>` - Start a query.
- `fn start_query_raw(self: & mut Self, cx: & mut Context, raw_name: &[u8], query_type: Type, mdns: MulticastDns) -> Result<QueryHandle, StartQueryError>` - Start a query with a raw (wire-format) DNS name.
- `fn get_query_result(self: & mut Self, handle: QueryHandle) -> Result<Vec<IpAddress, DNS_MAX_RESULT_COUNT>, GetQueryResultError>` - Get the result of a query.
- `fn cancel_query(self: & mut Self, handle: QueryHandle)` - Cancels a query, freeing the slot.
- `fn register_query_waker(self: & mut Self, handle: QueryHandle, waker: &Waker)` - Assign a waker to a query slot

**Trait Implementations:**

- **AnySocket**
  - `fn upcast(self: Self) -> Socket<'a>`
  - `fn downcast<'c>(socket: &'c Socket<'a>) -> Option<&'c Self>`
  - `fn downcast_mut<'c>(socket: &'c  mut Socket<'a>) -> Option<&'c  mut Self>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::socket::dns::StartQueryError

*Enum*

Error returned by [`Socket::start_query`]

**Variants:**
- `NoFreeSlot`
- `InvalidName`
- `NameTooLong`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &StartQueryError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> StartQueryError`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



