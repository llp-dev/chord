**smoltcp > iface > interface**

# Module: iface::interface

## Contents

**Structs**

- [`Config`](#config) - Configuration structure used for creating a network interface.
- [`Interface`](#interface) - A  network interface.
- [`InterfaceInner`](#interfaceinner) - The device independent part of an Ethernet network interface.

**Enums**

- [`PollIngressSingleResult`](#pollingresssingleresult) - Result returned by [`Interface::poll_ingress_single`].
- [`PollResult`](#pollresult) - Result returned by [`Interface::poll`].

---

## smoltcp::iface::interface::Config

*Struct*

Configuration structure used for creating a network interface.

**Fields:**
- `random_seed: u64` - Random seed.
- `hardware_addr: HardwareAddress` - Set the Hardware address the interface will use.

**Methods:**

- `fn new(hardware_addr: HardwareAddress) -> Self`



## smoltcp::iface::interface::Interface

*Struct*

A  network interface.

The network interface logically owns a number of other data structures; to avoid
a dependency on heap allocation, it instead owns a `BorrowMut<[T]>`, which can be
a `&mut [T]`, or `Vec<T>` if a heap is available.

**Methods:**

- `fn new<impl Device + ?Sized>(config: Config, device: & mut impl Trait, now: Instant) -> Self` - Create a network interface using the previously provided configuration.
- `fn context(self: & mut Self) -> & mut InterfaceInner` - Get the socket context.
- `fn hardware_addr(self: &Self) -> HardwareAddress` - Get the HardwareAddress address of the interface.
- `fn set_hardware_addr(self: & mut Self, addr: HardwareAddress)` - Set the HardwareAddress address of the interface.
- `fn ip_addrs(self: &Self) -> &[IpCidr]` - Get the IP addresses of the interface.
- `fn ipv4_addr(self: &Self) -> Option<Ipv4Address>` - Get the first IPv4 address if present.
- `fn get_source_address(self: &Self, dst_addr: &IpAddress) -> Option<IpAddress>` - Get an address from the interface that could be used as source address.
- `fn get_source_address_ipv4(self: &Self, dst_addr: &Ipv4Address) -> Option<Ipv4Address>` - Get an IPv4 source address based on a destination address. This function tries
- `fn update_ip_addrs<F>(self: & mut Self, f: F)` - Update the IP addresses of the interface.
- `fn has_ip_addr<T>(self: &Self, addr: T) -> bool` - Check whether the interface has the given IP address assigned.
- `fn routes(self: &Self) -> &Routes`
- `fn routes_mut(self: & mut Self) -> & mut Routes`
- `fn set_any_ip(self: & mut Self, any_ip: bool)` - Enable or disable the AnyIP capability.
- `fn any_ip(self: &Self) -> bool` - Get whether AnyIP is enabled.
- `fn poll<impl Device + ?Sized>(self: & mut Self, timestamp: Instant, device: & mut impl Trait, sockets: & mut SocketSet) -> PollResult` - Transmit packets queued in the sockets, and receive packets queued
- `fn poll_egress<impl Device + ?Sized>(self: & mut Self, timestamp: Instant, device: & mut impl Trait, sockets: & mut SocketSet) -> PollResult` - Transmit packets queued in the sockets.
- `fn poll_ingress_single<impl Device + ?Sized>(self: & mut Self, timestamp: Instant, device: & mut impl Trait, sockets: & mut SocketSet) -> PollIngressSingleResult` - Process one incoming packet queued in the device.
- `fn poll_maintenance(self: & mut Self, timestamp: Instant)` - Maintain stateful processing on the device.
- `fn poll_at(self: & mut Self, timestamp: Instant, sockets: &SocketSet) -> Option<Instant>` - Return a _soft deadline_ for calling [poll] the next time.
- `fn poll_delay(self: & mut Self, timestamp: Instant, sockets: &SocketSet) -> Option<Duration>` - Return an _advisory wait time_ for calling [poll] the next time.



## smoltcp::iface::interface::InterfaceInner

*Struct*

The device independent part of an Ethernet network interface.

Separating the device from the data required for processing and dispatching makes
it possible to borrow them independently. For example, the tx and rx tokens borrow
the `device` mutably until they're used, which makes it impossible to call other
methods on the `Interface` in this time (since its `device` field is borrowed
exclusively). However, it is still possible to call methods on its `inner` field.

**Methods:**

- `fn ipv4_addr(self: &Self) -> Option<Ipv4Address>` - Get the first IPv4 address of the interface.



## smoltcp::iface::interface::PollIngressSingleResult

*Enum*

Result returned by [`Interface::poll_ingress_single`].

This contains information on whether a packet was processed or not,
and whether it might've affected socket states.

**Variants:**
- `None` - No packet was processed. You don't need to call [`Interface::poll_ingress_single`]
- `PacketProcessed` - A packet was processed.
- `SocketStateChanged` - A packet was processed, which might have caused socket state to change.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PollIngressSingleResult`
- **PartialEq**
  - `fn eq(self: &Self, other: &PollIngressSingleResult) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::iface::interface::PollResult

*Enum*

Result returned by [`Interface::poll`].

This contains information on whether socket states might have changed.

**Variants:**
- `None` - Socket state is guaranteed to not have changed.
- `SocketStateChanged` - You should check the state of sockets again for received data or completion of operations.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PollResult`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PollResult) -> bool`



