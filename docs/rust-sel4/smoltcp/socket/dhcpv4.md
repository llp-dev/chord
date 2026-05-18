**smoltcp > socket > dhcpv4**

# Module: socket::dhcpv4

## Contents

**Structs**

- [`Config`](#config) - IPv4 configuration data provided by the DHCP server.
- [`RetryConfig`](#retryconfig) - Timeout and retry configuration.
- [`ServerInfo`](#serverinfo) - Information on how to reach a DHCP server.
- [`Socket`](#socket)

**Enums**

- [`Event`](#event) - Return value for the `Dhcpv4Socket::poll` function

---

## smoltcp::socket::dhcpv4::Config

*Struct*

IPv4 configuration data provided by the DHCP server.

**Generic Parameters:**
- 'a

**Fields:**
- `server: ServerInfo` - Information on how to reach the DHCP server that responded with DHCP
- `address: crate::wire::Ipv4Cidr` - IP address
- `router: Option<crate::wire::Ipv4Address>` - Router address, also known as default gateway. Does not necessarily
- `dns_servers: heapless::Vec<crate::wire::Ipv4Address, DHCP_MAX_DNS_SERVER_COUNT>` - DNS servers
- `packet: Option<crate::wire::DhcpPacket<&'a [u8]>>` - Received DHCP packet

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Config<'a>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Config<'a>`



## smoltcp::socket::dhcpv4::Event

*Enum*

Return value for the `Dhcpv4Socket::poll` function

**Generic Parameters:**
- 'a

**Variants:**
- `Deconfigured` - Configuration has been lost (for example, the lease has expired)
- `Configured(Config<'a>)` - Configuration has been newly acquired, or modified.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Event<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Event<'a>`



## smoltcp::socket::dhcpv4::RetryConfig

*Struct*

Timeout and retry configuration.

**Fields:**
- `discover_timeout: crate::time::Duration`
- `initial_request_timeout: crate::time::Duration` - The REQUEST timeout doubles every 2 tries.
- `request_retries: u16`
- `min_renew_timeout: crate::time::Duration`
- `max_renew_timeout: crate::time::Duration` - An upper bound on how long to wait between retrying a renew or rebind.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RetryConfig`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RetryConfig) -> bool`
- **Default**
  - `fn default() -> Self`



## smoltcp::socket::dhcpv4::ServerInfo

*Struct*

Information on how to reach a DHCP server.

**Fields:**
- `address: crate::wire::Ipv4Address` - IP address to use as destination in outgoing packets
- `identifier: crate::wire::Ipv4Address` - Server identifier to use in outgoing packets. Usually equal to server_address,

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ServerInfo) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ServerInfo`



## smoltcp::socket::dhcpv4::Socket

*Struct*

**Generic Parameters:**
- 'a

**Methods:**

- `fn new() -> Self` - Create a DHCPv4 socket
- `fn set_retry_config(self: & mut Self, config: RetryConfig)` - Set the retry/timeouts configuration.
- `fn get_retry_config(self: &Self) -> RetryConfig` - Gets the current retry/timeouts configuration
- `fn set_outgoing_options(self: & mut Self, options: &'a [DhcpOption<'a>])` - Set the outgoing options.
- `fn set_receive_packet_buffer(self: & mut Self, buffer: &'a  mut [u8])` - Set the buffer into which incoming DHCP packets are copied into.
- `fn set_parameter_request_list(self: & mut Self, parameter_request_list: &'a [u8])` - Set the parameter request list.
- `fn max_lease_duration(self: &Self) -> Option<Duration>` - Get the configured max lease duration.
- `fn set_max_lease_duration(self: & mut Self, max_lease_duration: Option<Duration>)` - Set the max lease duration.
- `fn ignore_naks(self: &Self) -> bool` - Get whether to ignore NAKs.
- `fn set_ignore_naks(self: & mut Self, ignore_naks: bool)` - Set whether to ignore NAKs.
- `fn set_ports(self: & mut Self, server_port: u16, client_port: u16)` - Set the server/client port
- `fn reset(self: & mut Self)` - Reset state and restart discovery phase.
- `fn poll(self: & mut Self) -> Option<Event>` - Query the socket for configuration changes.
- `fn register_waker(self: & mut Self, waker: &Waker)` - Register a waker.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AnySocket**
  - `fn upcast(self: Self) -> Socket<'a>`
  - `fn downcast<'c>(socket: &'c Socket<'a>) -> Option<&'c Self>`
  - `fn downcast_mut<'c>(socket: &'c  mut Socket<'a>) -> Option<&'c  mut Self>`



