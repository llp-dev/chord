**smoltcp > iface > route**

# Module: iface::route

## Contents

**Structs**

- [`Route`](#route) - A prefix of addresses that should be routed via a router
- [`RouteTableFull`](#routetablefull)
- [`Routes`](#routes) - A routing table.

---

## smoltcp::iface::route::Route

*Struct*

A prefix of addresses that should be routed via a router

**Fields:**
- `cidr: crate::wire::IpCidr`
- `via_router: crate::wire::IpAddress`
- `preferred_until: Option<crate::time::Instant>` - `None` means "forever".
- `expires_at: Option<crate::time::Instant>` - `None` means "forever".

**Methods:**

- `fn new_ipv4_gateway(gateway: Ipv4Address) -> Route` - Returns a route to 0.0.0.0/0 via the `gateway`, with no expiry.
- `fn is_ipv4_gateway(self: &Self) -> bool` - Returns `true` if the route is a default route for IPv4.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Route`



## smoltcp::iface::route::RouteTableFull

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RouteTableFull) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RouteTableFull`



## smoltcp::iface::route::Routes

*Struct*

A routing table.

**Methods:**

- `fn new() -> Self` - Creates a new empty routing table.
- `fn update<F>(self: & mut Self, f: F)` - Update the routes of this node.
- `fn add_default_ipv4_route(self: & mut Self, gateway: Ipv4Address) -> Result<Option<Route>, RouteTableFull>` - Add a default ipv4 gateway (ie. "ip route add 0.0.0.0/0 via `gateway`").
- `fn get_default_ipv4_route(self: &Self) -> Option<Route>` - Returns the ipv4 default route if there is one in the route table.
- `fn remove_default_ipv4_route(self: & mut Self) -> Option<Route>` - Remove the default ipv4 gateway

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



