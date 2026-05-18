**smoltcp > phy > fault_injector**

# Module: phy::fault_injector

## Contents

**Structs**

- [`FaultInjector`](#faultinjector) - A fault injector device.

---

## smoltcp::phy::fault_injector::FaultInjector

*Struct*

A fault injector device.

A fault injector is a device that alters packets traversing through it to simulate
adverse network conditions (such as random packet loss or corruption), or software
or hardware limitations (such as a limited number or size of usable network buffers).

**Generic Parameters:**
- D

**Methods:**

- `fn new(inner: D, seed: u32) -> FaultInjector<D>` - Create a fault injector device, using the given random number generator seed.
- `fn into_inner(self: Self) -> D` - Return the underlying device, consuming the fault injector.
- `fn corrupt_chance(self: &Self) -> u8` - Return the probability of corrupting a packet, in percents.
- `fn drop_chance(self: &Self) -> u8` - Return the probability of dropping a packet, in percents.
- `fn max_packet_size(self: &Self) -> usize` - Return the maximum packet size, in octets.
- `fn max_tx_rate(self: &Self) -> u64` - Return the maximum packet transmission rate, in packets per second.
- `fn max_rx_rate(self: &Self) -> u64` - Return the maximum packet reception rate, in packets per second.
- `fn bucket_interval(self: &Self) -> Duration` - Return the interval for packet rate limiting, in milliseconds.
- `fn set_corrupt_chance(self: & mut Self, pct: u8)` - Set the probability of corrupting a packet, in percents.
- `fn set_drop_chance(self: & mut Self, pct: u8)` - Set the probability of dropping a packet, in percents.
- `fn set_max_packet_size(self: & mut Self, size: usize)` - Set the maximum packet size, in octets.
- `fn set_max_tx_rate(self: & mut Self, rate: u64)` - Set the maximum packet transmission rate, in packets per interval.
- `fn set_max_rx_rate(self: & mut Self, rate: u64)` - Set the maximum packet reception rate, in packets per interval.
- `fn set_bucket_interval(self: & mut Self, interval: Duration)` - Set the interval for packet rate limiting, in milliseconds.

**Trait Implementations:**

- **Device**
  - `fn capabilities(self: &Self) -> DeviceCapabilities`
  - `fn receive(self: & mut Self, timestamp: Instant) -> Option<(<Self as >::RxToken, <Self as >::TxToken)>`
  - `fn transmit(self: & mut Self, timestamp: Instant) -> Option<<Self as >::TxToken>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



