**sel4_pl011_driver**

# Module: sel4_pl011_driver

## Contents

**Structs**

- [`Driver`](#driver)

---

## sel4_pl011_driver::Driver

*Struct*

**Methods:**

- `fn new_uninit(ptr: *mut ()) -> Self`
- `fn new(ptr: *mut ()) -> Self`
- `fn init(self: & mut Self)`

**Traits:** Sync, ErrorType, Send

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, word: u8) -> nb::Result<(), <Self as >::Error>`
  - `fn flush(self: & mut Self) -> nb::Result<(), <Self as >::Error>`
- **HandleInterrupt**
  - `fn handle_interrupt(self: & mut Self)`
- **Read**
  - `fn read(self: & mut Self) -> nb::Result<u8, <Self as >::Error>`



