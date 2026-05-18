**sel4_test_harness > config > types**

# Module: config::types

## Contents

**Structs**

- [`Config`](#config)

**Enums**

- [`RunIgnored`](#runignored) - Whether ignored test should be run or not

---

## sel4_test_harness::config::types::Config

*Struct*

**Fields:**
- `run_ignored: RunIgnored`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Config`
- **PartialEq**
  - `fn eq(self: &Self, other: &Config) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Config`



## sel4_test_harness::config::types::RunIgnored

*Enum*

Whether ignored test should be run or not

**Variants:**
- `Yes`
- `No`
- `Only` - Run only ignored tests

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &RunIgnored) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RunIgnored`
- **Default**
  - `fn default() -> Self`



