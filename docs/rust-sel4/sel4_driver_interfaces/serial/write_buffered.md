**sel4_driver_interfaces > serial > write_buffered**

# Module: serial::write_buffered

## Contents

**Structs**

- [`WriteBuffered`](#writebuffered)

---

## sel4_driver_interfaces::serial::write_buffered::WriteBuffered

*Struct*

**Generic Parameters:**
- T
- const WRITE_BUF_SIZE

**Methods:**

- `fn new(unbuffered: T) -> Self`

**Traits:** ErrorType

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, v: u8) -> nb::Result<(), <Self as >::Error>`
  - `fn flush(self: & mut Self) -> nb::Result<(), <Self as >::Error>`
- **Read**
  - `fn read(self: & mut Self) -> nb::Result<u8, <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> WriteBuffered<T, WRITE_BUF_SIZE>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



