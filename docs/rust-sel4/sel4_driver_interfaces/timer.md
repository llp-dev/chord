**sel4_driver_interfaces > timer**

# Module: timer

## Contents

**Structs**

- [`DefaultTimer`](#defaulttimer)
- [`NumTimers`](#numtimers)
- [`SingleTimer`](#singletimer)
- [`TrivialTimers`](#trivialtimers)

**Traits**

- [`Clock`](#clock)
- [`ErrorType`](#errortype)
- [`Timer`](#timer)
- [`Timers`](#timers)

---

## sel4_driver_interfaces::timer::Clock

*Trait*

**Methods:**

- `get_time`



## sel4_driver_interfaces::timer::DefaultTimer

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** ErrorType

**Trait Implementations:**

- **Clock**
  - `fn get_time(self: & mut Self) -> Result<Duration, <Self as >::Error>`
- **Timer**
  - `fn set_timeout(self: & mut Self, relative: Duration) -> Result<(), <Self as >::Error>`
  - `fn clear_timeout(self: & mut Self) -> Result<(), <Self as >::Error>`



## sel4_driver_interfaces::timer::ErrorType

*Trait*

**Methods:**

- `Error`



## sel4_driver_interfaces::timer::NumTimers

*Struct*

**Tuple Struct**: `(usize)`



## sel4_driver_interfaces::timer::SingleTimer

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** ErrorType

**Trait Implementations:**

- **HandleInterrupt**
  - `fn handle_interrupt(self: & mut Self)`
- **Clock**
  - `fn get_time(self: & mut Self) -> Result<Duration, <Self as >::Error>`
- **Timers**
  - `fn timer_layout(self: & mut Self) -> Result<<Self as >::TimerLayout, <Self as >::Error>`
  - `fn set_timeout_on(self: & mut Self, _timer: <Self as >::Timer, relative: Duration) -> Result<(), <Self as >::Error>`
  - `fn clear_timeout_on(self: & mut Self, _timer: <Self as >::Timer) -> Result<(), <Self as >::Error>`



## sel4_driver_interfaces::timer::Timer

*Trait*

**Methods:**

- `set_timeout`
- `clear_timeout`



## sel4_driver_interfaces::timer::Timers

*Trait*

**Methods:**

- `TimerLayout`
- `Timer`
- `timer_layout`
- `set_timeout_on`
- `clear_timeout_on`



## sel4_driver_interfaces::timer::TrivialTimers

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** ErrorType

**Trait Implementations:**

- **Clock**
  - `fn get_time(self: & mut Self) -> Result<Duration, <Self as >::Error>`
- **Timers**
  - `fn timer_layout(self: & mut Self) -> Result<<Self as >::TimerLayout, <Self as >::Error>`
  - `fn set_timeout_on(self: & mut Self, _timer: <Self as >::Timer, relative: Duration) -> Result<(), <Self as >::Error>`
  - `fn clear_timeout_on(self: & mut Self, _timer: <Self as >::Timer) -> Result<(), <Self as >::Error>`



