**smoltcp > phy > tracer**

# Module: phy::tracer

## Contents

**Structs**

- [`Tracer`](#tracer) - A tracer device.
- [`TracerPacket`](#tracerpacket) - Packet which is being traced by [Tracer](struct.Tracer.html) device.

**Enums**

- [`TracerDirection`](#tracerdirection) - Direction on which packet is being traced

---

## smoltcp::phy::tracer::Tracer

*Struct*

A tracer device.

A tracer is a device that pretty prints all packets traversing it
using the provided writer function, and then passes them to another
device.

**Generic Parameters:**
- D

**Methods:**

- `fn new(inner: D, writer: fn(...)) -> Tracer<D>` - Create a tracer device.
- `fn get_ref(self: &Self) -> &D` - Get a reference to the underlying device.
- `fn get_mut(self: & mut Self) -> & mut D` - Get a mutable reference to the underlying device.
- `fn into_inner(self: Self) -> D` - Return the underlying device, consuming the tracer.

**Trait Implementations:**

- **Device**
  - `fn capabilities(self: &Self) -> DeviceCapabilities`
  - `fn receive(self: & mut Self, timestamp: Instant) -> Option<(<Self as >::RxToken, <Self as >::TxToken)>`
  - `fn transmit(self: & mut Self, timestamp: Instant) -> Option<<Self as >::TxToken>`



## smoltcp::phy::tracer::TracerDirection

*Enum*

Direction on which packet is being traced

**Variants:**
- `RX` - Packet is received by Smoltcp interface
- `TX` - Packet is transmitted by Smoltcp interface

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TracerDirection) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TracerDirection`



## smoltcp::phy::tracer::TracerPacket

*Struct*

Packet which is being traced by [Tracer](struct.Tracer.html) device.

**Generic Parameters:**
- 'a

**Fields:**
- `buffer: &'a [u8]` - Packet buffer
- `medium: crate::phy::Medium` - Packet medium
- `direction: TracerDirection` - Direction in which packet is being traced

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TracerPacket<'a>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



