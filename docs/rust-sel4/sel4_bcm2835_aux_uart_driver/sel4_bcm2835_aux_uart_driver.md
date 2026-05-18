**sel4_bcm2835_aux_uart_driver**

# Module: sel4_bcm2835_aux_uart_driver

## Contents

**Structs**

- [`Driver`](#driver)

---

## sel4_bcm2835_aux_uart_driver::Driver

*Struct*

**Methods:**

- `fn new_uninit(ptr: *mut ()) -> Self`
- `fn new(ptr: *mut ()) -> Self`
- `fn init(self: & mut Self)`

**Traits:** Send, Sync, ErrorType

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, word: u8) -> nb::Result<(), <Self as >::Error>`
  - `fn flush(self: & mut Self) -> nb::Result<(), <Self as >::Error>`



