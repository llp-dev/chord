**embedded_hal_nb > serial**

# Module: serial

## Contents

**Enums**

- [`ErrorKind`](#errorkind) - Serial error kind.

**Traits**

- [`Error`](#error) - Serial error.
- [`ErrorType`](#errortype) - Serial error type trait.
- [`Read`](#read) - Read half of a serial interface.
- [`Write`](#write) - Write half of a serial interface.

---

## embedded_hal_nb::serial::Error

*Trait*

Serial error.

**Methods:**

- `kind`: Convert error to a generic serial error kind



## embedded_hal_nb::serial::ErrorKind

*Enum*

Serial error kind.

This represents a common set of serial operation errors. HAL implementations are
free to define more specific or additional error types. However, by providing
a mapping to these common serial errors, generic code can still react to them.

**Variants:**
- `Overrun` - The peripheral receive buffer was overrun.
- `FrameFormat` - Received data does not conform to the peripheral configuration.
- `Parity` - Parity check failed.
- `Noise` - Serial line is too noisy to read valid data.
- `Other` - A different error occurred. The original error may contain more information.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &ErrorKind) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ErrorKind) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> ErrorKind`
- **PartialEq**
  - `fn eq(self: &Self, other: &ErrorKind) -> bool`
- **Error**
  - `fn kind(self: &Self) -> ErrorKind`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## embedded_hal_nb::serial::ErrorType

*Trait*

Serial error type trait.

This just defines the error type, to be used by the other traits.

**Methods:**

- `Error`: Error type



## embedded_hal_nb::serial::Read

*Trait*

Read half of a serial interface.

Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);
This can be encoded in this trait via the `Word` type parameter.

**Methods:**

- `read`: Reads a single word from the serial interface



## embedded_hal_nb::serial::Write

*Trait*

Write half of a serial interface.

**Methods:**

- `write`: Writes a single word to the serial interface.
- `flush`: Ensures that none of the previously written words are still buffered.



