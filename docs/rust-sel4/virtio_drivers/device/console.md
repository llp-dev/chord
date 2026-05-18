**virtio_drivers > device > console**

# Module: device::console

## Contents

**Structs**

- [`Config`](#config)
- [`Features`](#features)
- [`Size`](#size) - The width and height of a console, in characters.
- [`VirtIOConsole`](#virtioconsole) - Driver for a VirtIO console device.

**Constants**

- [`QUEUE_RECEIVEQ_PORT_0`](#queue_receiveq_port_0)
- [`QUEUE_SIZE`](#queue_size)
- [`QUEUE_TRANSMITQ_PORT_0`](#queue_transmitq_port_0)
- [`SUPPORTED_FEATURES`](#supported_features)

---

## virtio_drivers::device::console::Config

*Struct*

**Fields:**
- `cols: crate::config::ReadOnly<u16>`
- `rows: crate::config::ReadOnly<u16>`
- `max_nr_ports: crate::config::ReadOnly<u32>`
- `emerg_wr: crate::config::WriteOnly<u32>`

**Traits:** IntoBytes, TryFromBytes, Immutable, FromBytes, FromZeros



## virtio_drivers::device::console::Features

*Struct*

**Tuple Struct**: `(<Features as $crate::__private::PublicFlags>::Internal)`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u64` - Get the underlying bits value.
- `fn from_bits(bits: u64) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u64) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u64) -> Self` - Convert from a bits value exactly.
- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>` - Get a flags value with the bits of a flag with the given name set.
- `fn is_empty(self: &Self) -> bool` - Whether all bits in this flags value are unset.
- `fn is_all(self: &Self) -> bool` - Whether all known bits in this flags value are set.
- `fn intersects(self: &Self, other: Self) -> bool` - Whether any set bits in a source flags value are also set in a target flags value.
- `fn contains(self: &Self, other: Self) -> bool` - Whether all set bits in a source flags value are also set in a target flags value.
- `fn insert(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- `fn remove(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags
- `fn toggle(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- `fn set(self: & mut Self, other: Self, value: bool)` - Call `insert` when `value` is `true` or `remove` when `value` is `false`.
- `fn intersection(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- `fn union(self: Self, other: Self) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- `fn difference(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags
- `fn symmetric_difference(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- `fn complement(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- `fn iter(self: &Self) -> $crate::iter::Iter<Features>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Features>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **Flags**
  - `fn bits(self: &Self) -> u64`
  - `fn from_bits_retain(bits: u64) -> Features`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> Features`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: Features) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in two flags values.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **PartialEq**
  - `fn eq(self: &Self, other: &Features) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Default**
  - `fn default() -> Features`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.



## virtio_drivers::device::console::QUEUE_RECEIVEQ_PORT_0

*Constant*: `u16`



## virtio_drivers::device::console::QUEUE_SIZE

*Constant*: `usize`



## virtio_drivers::device::console::QUEUE_TRANSMITQ_PORT_0

*Constant*: `u16`



## virtio_drivers::device::console::SUPPORTED_FEATURES

*Constant*: `Features`



## virtio_drivers::device::console::Size

*Struct*

The width and height of a console, in characters.

**Fields:**
- `columns: u16` - The console width in characters.
- `rows: u16` - The console height in characters.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Size`
- **PartialEq**
  - `fn eq(self: &Self, other: &Size) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`



## virtio_drivers::device::console::VirtIOConsole

*Struct*

Driver for a VirtIO console device.

Only a single port is supported.

# Example

```
# use virtio_drivers::{Error, Hal, transport::Transport};
use virtio_drivers::device::console::VirtIOConsole;
# fn example<HalImpl: Hal, T: Transport>(transport: T) -> Result<(), Error> {
let mut console = VirtIOConsole::<HalImpl, _>::new(transport)?;

let size = console.size().unwrap().unwrap();
println!("VirtIO console {}x{}", size.rows, size.columns);

for &c in b"Hello console!\n" {
  console.send(c)?;
}

let c = console.recv(true)?;
println!("Read {:?} from console.", c);
# Ok(())
# }
```

**Generic Parameters:**
- H
- T

**Fields:**
- `transport: T`
- `negotiated_features: Features`
- `receiveq: crate::queue::VirtQueue<H, QUEUE_SIZE>`
- `transmitq: crate::queue::VirtQueue<H, QUEUE_SIZE>`
- `queue_buf_rx: alloc::boxed::Box<[u8; 4096]>`
- `cursor: usize` - The index of the next byte in `queue_buf_rx` which `recv` should return.
- `pending_len: usize` - The number of bytes read into `queue_buf_rx`.
- `receive_token: Option<u16>` - The token of the outstanding receive request, if there is one.

**Methods:**

- `fn new(transport: T) -> Result<Self>` - Creates a new VirtIO console driver.
- `fn size(self: &Self) -> Result<Option<Size>>` - Returns the size of the console, if the device supports reporting this.
- `fn poll_retrieve(self: & mut Self) -> Result<()>` - Makes a request to the device to receive data, if there is not already an outstanding
- `fn ack_interrupt(self: & mut Self) -> Result<bool>` - Acknowledges a pending interrupt, if any, and completes the outstanding finished read
- `fn finish_receive(self: & mut Self) -> Result<bool>` - If there is an outstanding receive request and it has finished, completes it.
- `fn recv(self: & mut Self, pop: bool) -> Result<Option<u8>>` - Returns the next available character from the console, if any.
- `fn send(self: & mut Self, chr: u8) -> Result<()>` - Sends a character to the console.
- `fn send_bytes(self: & mut Self, buffer: &[u8]) -> Result` - Sends one or more bytes to the console.
- `fn wait_for_receive(self: & mut Self) -> Result` - Blocks until at least one character is available to read.
- `fn emergency_write(self: & mut Self, chr: u8) -> Result<()>` - Sends a character to the console using the emergency write feature.

**Traits:** Send, Sync

**Trait Implementations:**

- **Write**
  - `fn write_str(self: & mut Self, s: &str) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



