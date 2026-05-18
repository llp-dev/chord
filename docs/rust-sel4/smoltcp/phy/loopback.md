**smoltcp > phy > loopback**

# Module: phy::loopback

## Contents

**Structs**

- [`Loopback`](#loopback) - A loopback device.

---

## smoltcp::phy::loopback::Loopback

*Struct*

A loopback device.

**Methods:**

- `fn new(medium: Medium) -> Loopback` - Creates a loopback device.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Device**
  - `fn capabilities(self: &Self) -> DeviceCapabilities`
  - `fn receive(self: & mut Self, _timestamp: Instant) -> Option<(<Self as >::RxToken, <Self as >::TxToken)>`
  - `fn transmit(self: & mut Self, _timestamp: Instant) -> Option<<Self as >::TxToken>`



