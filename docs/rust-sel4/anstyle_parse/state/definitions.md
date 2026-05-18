**anstyle_parse > state > definitions**

# Module: state::definitions

## Contents

**Enums**

- [`Action`](#action)
- [`State`](#state)

---

## anstyle_parse::state::definitions::Action

*Enum*

**Variants:**
- `Nop`
- `Clear`
- `Collect`
- `CsiDispatch`
- `EscDispatch`
- `Execute`
- `Hook`
- `Ignore`
- `OscEnd`
- `OscPut`
- `OscStart`
- `Param`
- `Print`
- `Put`
- `Unhook`
- `BeginUtf8`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Action) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Action`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Action`
- **TryFrom**
  - `fn try_from(raw: u8) -> Result<Self, <Self as >::Error>`



## anstyle_parse::state::definitions::State

*Enum*

**Variants:**
- `Anywhere`
- `CsiEntry`
- `CsiIgnore`
- `CsiIntermediate`
- `CsiParam`
- `DcsEntry`
- `DcsIgnore`
- `DcsIntermediate`
- `DcsParam`
- `DcsPassthrough`
- `Escape`
- `EscapeIntermediate`
- `Ground`
- `OscString`
- `SosPmApcString`
- `Utf8`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> State`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> State`
- **TryFrom**
  - `fn try_from(raw: u8) -> Result<Self, <Self as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &State) -> bool`



