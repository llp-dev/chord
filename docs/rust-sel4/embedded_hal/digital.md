**embedded_hal > digital**

# Module: digital

## Contents

**Enums**

- [`ErrorKind`](#errorkind) - Error kind.
- [`PinState`](#pinstate) - Digital output pin state.

**Traits**

- [`Error`](#error) - Error.
- [`ErrorType`](#errortype) - Error type trait.
- [`InputPin`](#inputpin) - Single digital input pin.
- [`OutputPin`](#outputpin) - Single digital push-pull output pin.
- [`StatefulOutputPin`](#statefuloutputpin) - Push-pull output pin that can read its output state.

---

## embedded_hal::digital::Error

*Trait*

Error.

**Methods:**

- `kind`: Convert error to a generic error kind



## embedded_hal::digital::ErrorKind

*Enum*

Error kind.

This represents a common set of operation errors. HAL implementations are
free to define more specific or additional error types. However, by providing
a mapping to these common errors, generic code can still react to them.

**Variants:**
- `Other` - A different error occurred. The original error may contain more information.

**Traits:** Eq, Copy

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



## embedded_hal::digital::ErrorType

*Trait*

Error type trait.

This just defines the error type, to be used by the other traits.

**Methods:**

- `Error`: Error type



## embedded_hal::digital::InputPin

*Trait*

Single digital input pin.

**Methods:**

- `is_high`: Is the input pin high?
- `is_low`: Is the input pin low?



## embedded_hal::digital::OutputPin

*Trait*

Single digital push-pull output pin.

**Methods:**

- `set_low`: Drives the pin low.
- `set_high`: Drives the pin high.
- `set_state`: Drives the pin high or low depending on the provided value.



## embedded_hal::digital::PinState

*Enum*

Digital output pin state.

Conversion from `bool` and logical negation are also implemented
for this type.
```rust
# use embedded_hal::digital::PinState;
let state = PinState::from(false);
assert_eq!(state, PinState::Low);
assert_eq!(!state, PinState::High);
```

**Variants:**
- `Low` - Low pin state.
- `High` - High pin state.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &PinState) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> PinState`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **From**
  - `fn from(value: bool) -> Self`



## embedded_hal::digital::StatefulOutputPin

*Trait*

Push-pull output pin that can read its output state.

**Methods:**

- `is_set_high`: Is the pin in drive high mode?
- `is_set_low`: Is the pin in drive low mode?
- `toggle`: Toggle pin output.



