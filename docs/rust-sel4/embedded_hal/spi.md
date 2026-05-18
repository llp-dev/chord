**embedded_hal > spi**

# Module: spi

## Contents

**Structs**

- [`Mode`](#mode) - SPI mode.

**Enums**

- [`ErrorKind`](#errorkind) - SPI error kind.
- [`Operation`](#operation) - SPI transaction operation.
- [`Phase`](#phase) - Clock phase.
- [`Polarity`](#polarity) - Clock polarity.

**Traits**

- [`Error`](#error) - SPI error.
- [`ErrorType`](#errortype) - SPI error type trait.
- [`SpiBus`](#spibus) - SPI bus.
- [`SpiDevice`](#spidevice) - SPI device trait.

**Constants**

- [`MODE_0`](#mode_0) - Helper for CPOL = 0, CPHA = 0.
- [`MODE_1`](#mode_1) - Helper for CPOL = 0, CPHA = 1.
- [`MODE_2`](#mode_2) - Helper for CPOL = 1, CPHA = 0.
- [`MODE_3`](#mode_3) - Helper for CPOL = 1, CPHA = 1.

---

## embedded_hal::spi::Error

*Trait*

SPI error.

**Methods:**

- `kind`: Convert error to a generic SPI error kind.



## embedded_hal::spi::ErrorKind

*Enum*

SPI error kind.

This represents a common set of SPI operation errors. HAL implementations are
free to define more specific or additional error types. However, by providing
a mapping to these common SPI errors, generic code can still react to them.

**Variants:**
- `Overrun` - The peripheral receive buffer was overrun.
- `ModeFault` - Multiple devices on the SPI bus are trying to drive the slave select pin, e.g. in a multi-master setup.
- `FrameFormat` - Received data does not conform to the peripheral configuration.
- `ChipSelectFault` - An error occurred while asserting or deasserting the Chip Select pin.
- `Other` - A different error occurred. The original error may contain more information.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ErrorKind) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &ErrorKind) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ErrorKind) -> bool`
- **Error**
  - `fn kind(self: &Self) -> ErrorKind`
- **Clone**
  - `fn clone(self: &Self) -> ErrorKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## embedded_hal::spi::ErrorType

*Trait*

SPI error type trait.

This just defines the error type, to be used by the other SPI traits.

**Methods:**

- `Error`: Error type.



## embedded_hal::spi::MODE_0

*Constant*: `Mode`

Helper for CPOL = 0, CPHA = 0.



## embedded_hal::spi::MODE_1

*Constant*: `Mode`

Helper for CPOL = 0, CPHA = 1.



## embedded_hal::spi::MODE_2

*Constant*: `Mode`

Helper for CPOL = 1, CPHA = 0.



## embedded_hal::spi::MODE_3

*Constant*: `Mode`

Helper for CPOL = 1, CPHA = 1.



## embedded_hal::spi::Mode

*Struct*

SPI mode.

**Fields:**
- `polarity: Polarity` - Clock polarity.
- `phase: Phase` - Clock phase.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Mode) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Mode`



## embedded_hal::spi::Operation

*Enum*

SPI transaction operation.

This allows composition of SPI operations into a single bus transaction.

**Generic Parameters:**
- 'a
- Word

**Variants:**
- `Read(&'a  mut [Word])` - Read data into the provided buffer.
- `Write(&'a [Word])` - Write data from the provided buffer, discarding read data.
- `Transfer(&'a  mut [Word], &'a [Word])` - Read data into the first buffer, while writing data from the second buffer.
- `TransferInPlace(&'a  mut [Word])` - Write data out while reading data into the provided buffer.
- `DelayNs(u32)` - Delay for at least the specified number of nanoseconds.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Operation<'a, Word>) -> bool`



## embedded_hal::spi::Phase

*Enum*

Clock phase.

**Variants:**
- `CaptureOnFirstTransition` - Data in "captured" on the first clock transition.
- `CaptureOnSecondTransition` - Data in "captured" on the second clock transition.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Phase) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Phase`



## embedded_hal::spi::Polarity

*Enum*

Clock polarity.

**Variants:**
- `IdleLow` - Clock signal low when idle.
- `IdleHigh` - Clock signal high when idle.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Polarity) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Polarity`



## embedded_hal::spi::SpiBus

*Trait*

SPI bus.

`SpiBus` represents **exclusive ownership** over the whole SPI bus, with SCK, MOSI and MISO pins.

See the [module-level documentation](self) for important information on SPI Bus vs Device traits.

**Methods:**

- `read`: Read `words` from the slave.
- `write`: Write `words` to the slave, ignoring all the incoming words.
- `transfer`: Write and read simultaneously. `write` is written to the slave on MOSI and
- `transfer_in_place`: Write and read simultaneously. The contents of `words` are
- `flush`: Wait until all operations have completed and the bus is idle.



## embedded_hal::spi::SpiDevice

*Trait*

SPI device trait.

`SpiDevice` represents ownership over a single SPI device on a (possibly shared) bus, selected
with a CS (Chip Select) pin.

See the [module-level documentation](self) for important usage information.

**Methods:**

- `transaction`: Perform a transaction against the device.
- `read`: Do a read within a transaction.
- `write`: Do a write within a transaction.
- `transfer`: Do a transfer within a transaction.
- `transfer_in_place`: Do an in-place transfer within a transaction.



