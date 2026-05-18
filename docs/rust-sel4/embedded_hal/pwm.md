**embedded_hal > pwm**

# Module: pwm

## Contents

**Enums**

- [`ErrorKind`](#errorkind) - Error kind.

**Traits**

- [`Error`](#error) - Error
- [`ErrorType`](#errortype) - Error type trait.
- [`SetDutyCycle`](#setdutycycle) - Single PWM channel / pin.

---

## embedded_hal::pwm::Error

*Trait*

Error

**Methods:**

- `kind`: Convert error to a generic error kind.



## embedded_hal::pwm::ErrorKind

*Enum*

Error kind.

This represents a common set of operation errors. HAL implementations are
free to define more specific or additional error types. However, by providing
a mapping to these common errors, generic code can still react to them.

**Variants:**
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
- **Error**
  - `fn kind(self: &Self) -> ErrorKind`
- **PartialEq**
  - `fn eq(self: &Self, other: &ErrorKind) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ErrorKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## embedded_hal::pwm::ErrorType

*Trait*

Error type trait.

This just defines the error type, to be used by the other traits.

**Methods:**

- `Error`: Error type



## embedded_hal::pwm::SetDutyCycle

*Trait*

Single PWM channel / pin.

**Methods:**

- `max_duty_cycle`: Get the maximum duty cycle value.
- `set_duty_cycle`: Set the duty cycle to `duty / max_duty`.
- `set_duty_cycle_fully_off`: Set the duty cycle to 0%, or always inactive.
- `set_duty_cycle_fully_on`: Set the duty cycle to 100%, or always active.
- `set_duty_cycle_fraction`: Set the duty cycle to `num / denom`.
- `set_duty_cycle_percent`: Set the duty cycle to `percent / 100`



