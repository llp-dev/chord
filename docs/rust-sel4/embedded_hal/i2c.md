**embedded_hal > i2c**

# Module: i2c

## Contents

**Enums**

- [`ErrorKind`](#errorkind) - I2C error kind.
- [`NoAcknowledgeSource`](#noacknowledgesource) - I2C no acknowledge error source.
- [`Operation`](#operation) - I2C operation.

**Traits**

- [`AddressMode`](#addressmode) - Address mode (7-bit / 10-bit).
- [`Error`](#error) - I2C error.
- [`ErrorType`](#errortype) - I2C error type trait.
- [`I2c`](#i2c) - Blocking I2C.

**Type Aliases**

- [`SevenBitAddress`](#sevenbitaddress) - 7-bit address mode type.
- [`TenBitAddress`](#tenbitaddress) - 10-bit address mode type.

---

## embedded_hal::i2c::AddressMode

*Trait*

Address mode (7-bit / 10-bit).

Note: This trait is sealed and should not be implemented outside of this crate.



## embedded_hal::i2c::Error

*Trait*

I2C error.

**Methods:**

- `kind`: Convert error to a generic I2C error kind.



## embedded_hal::i2c::ErrorKind

*Enum*

I2C error kind.

This represents a common set of I2C operation errors. HAL implementations are
free to define more specific or additional error types. However, by providing
a mapping to these common I2C errors, generic code can still react to them.

**Variants:**
- `Bus` - Bus error occurred. e.g. A START or a STOP condition is detected and is not
- `ArbitrationLoss` - The arbitration was lost, e.g. electrical problems with the clock signal.
- `NoAcknowledge(NoAcknowledgeSource)` - A bus operation was not acknowledged, e.g. due to the addressed device not
- `Overrun` - The peripheral receive buffer was overrun.
- `Other` - A different error occurred. The original error may contain more information.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ErrorKind) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &ErrorKind) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ErrorKind) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ErrorKind`
- **Error**
  - `fn kind(self: &Self) -> ErrorKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## embedded_hal::i2c::ErrorType

*Trait*

I2C error type trait.

This just defines the error type, to be used by the other traits.

**Methods:**

- `Error`: Error type



## embedded_hal::i2c::I2c

*Trait*

Blocking I2C.

**Methods:**

- `read`: Reads enough bytes from slave with `address` to fill `read`.
- `write`: Writes bytes to slave with address `address`.
- `write_read`: Writes bytes to slave with address `address` and then reads enough bytes to fill `read` *in a
- `transaction`: Execute the provided operations on the I2C bus.



## embedded_hal::i2c::NoAcknowledgeSource

*Enum*

I2C no acknowledge error source.

In cases where it is possible, a device should indicate if a no acknowledge
response was received to an address versus a no acknowledge to a data byte.
Where it is not possible to differentiate, `Unknown` should be indicated.

**Variants:**
- `Address` - The device did not acknowledge its address. The device may be missing.
- `Data` - The device did not acknowledge the data. It may not be ready to process
- `Unknown` - Either the device did not acknowledge its address or the data, but it is

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NoAcknowledgeSource`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NoAcknowledgeSource) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &NoAcknowledgeSource) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &NoAcknowledgeSource) -> bool`



## embedded_hal::i2c::Operation

*Enum*

I2C operation.

Several operations can be combined as part of a transaction.

**Generic Parameters:**
- 'a

**Variants:**
- `Read(&'a  mut [u8])` - Read data into the provided buffer.
- `Write(&'a [u8])` - Write data from the provided buffer.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Operation<'a>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## embedded_hal::i2c::SevenBitAddress

*Type Alias*: `u8`

7-bit address mode type.

Note that 7-bit addresses defined by drivers should be specified in **right-aligned** form,
e.g. in the range `0x00..=0x7F`.

For example, a device that has the seven bit address of `0b011_0010`, and therefore is addressed on the wire using:

* `0b0110010_0` or `0x64` for *writes*
* `0b0110010_1` or `0x65` for *reads*

Should be specified as `0b0011_0010` or `0x32`, NOT `0x64` or `0x65`. Care should be taken by both HAL and driver
crate writers to use this scheme consistently.



## embedded_hal::i2c::TenBitAddress

*Type Alias*: `u16`

10-bit address mode type.



