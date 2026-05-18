**smoltcp > phy > fuzz_injector**

# Module: phy::fuzz_injector

## Contents

**Structs**

- [`FuzzInjector`](#fuzzinjector) - A fuzz injector device.

**Traits**

- [`Fuzzer`](#fuzzer) - Represents a fuzzer. It is expected to replace bytes in the packet with fuzzed data.

---

## smoltcp::phy::fuzz_injector::FuzzInjector

*Struct*

A fuzz injector device.

A fuzz injector is a device that alters packets traversing through it according to the
directions of a guided fuzzer. It is designed to support fuzzing internal state machines inside
smoltcp, and is not for production use.

**Generic Parameters:**
- D
- FTx
- FRx

**Methods:**

- `fn new(inner: D, fuzz_tx: FTx, fuzz_rx: FRx) -> FuzzInjector<D, FTx, FRx>` - Create a fuzz injector device.
- `fn into_inner(self: Self) -> D` - Return the underlying device, consuming the fuzz injector.

**Trait Implementations:**

- **Device**
  - `fn capabilities(self: &Self) -> DeviceCapabilities`
  - `fn receive(self: & mut Self, timestamp: Instant) -> Option<(<Self as >::RxToken, <Self as >::TxToken)>`
  - `fn transmit(self: & mut Self, timestamp: Instant) -> Option<<Self as >::TxToken>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::phy::fuzz_injector::Fuzzer

*Trait*

Represents a fuzzer. It is expected to replace bytes in the packet with fuzzed data.

**Methods:**

- `fuzz_packet`: Modify a single packet with fuzzed data.



