**sel4_sp804_driver**

# Module: sel4_sp804_driver

## Contents

**Structs**

- [`Driver`](#driver)

---

## sel4_sp804_driver::Driver

*Struct*

**Methods:**

- `fn new_uninit(ptr: *mut (), freq: u64) -> Self`
- `fn new(ptr: *mut (), freq: u64) -> Self`
- `fn init(self: & mut Self)`

**Traits:** ErrorType

**Trait Implementations:**

- **Timer**
  - `fn set_timeout(self: & mut Self, relative: Duration) -> Result<(), <Self as >::Error>`
  - `fn clear_timeout(self: & mut Self) -> Result<(), <Self as >::Error>`
- **Clock**
  - `fn get_time(self: & mut Self) -> Result<Duration, <Self as >::Error>`
- **HandleInterrupt**
  - `fn handle_interrupt(self: & mut Self)`



