**sel4_config_types**

# Module: sel4_config_types

## Contents

**Structs**

- [`Configuration`](#configuration)

**Enums**

- [`Value`](#value)

**Type Aliases**

- [`Key`](#key)

---

## sel4_config_types::Configuration

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn new(map: BTreeMap<Key, Value>) -> Self`
- `fn empty() -> Self`
- `fn get(self: &Self, key: &str) -> Option<&Value>`
- `fn iter(self: &Self) -> impl Trait`
- `fn insert(self: & mut Self, key: Key, value: Value) -> Option<Value>`
- `fn append(self: & mut Self, other: Self)`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Configuration) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Configuration`



## sel4_config_types::Key

*Type Alias*: `alloc::string::String`



## sel4_config_types::Value

*Enum*

**Variants:**
- `Bool(bool)`
- `String(alloc::string::String)`

**Methods:**

- `fn as_bool(self: &Self) -> Option<bool>`
- `fn as_str(self: &Self) -> Option<&str>`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Value`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Value) -> bool`



