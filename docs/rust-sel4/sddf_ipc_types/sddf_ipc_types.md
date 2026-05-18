**sddf_ipc_types**

# Module: sddf_ipc_types

## Contents

**Structs**

- [`ImplicitMessageReader`](#implicitmessagereader)

**Enums**

- [`CallError`](#callerror)

**Traits**

- [`CallTarget`](#calltarget)
- [`MessageReader`](#messagereader)
- [`MessageWriter`](#messagewriter)
- [`ReadFromMessage`](#readfrommessage)

**Type Aliases**

- [`MessageLabel`](#messagelabel)
- [`MessageRegisterValue`](#messageregistervalue)

---

## sddf_ipc_types::CallError

*Enum*

**Generic Parameters:**
- W
- R

**Variants:**
- `WriteError(W)`
- `ReadError(R)`

**Methods:**

- `fn into_reader_error(self: Self) -> R`

**Traits:** Error, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CallError<W, R>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &CallError<W, R>) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &CallError<W, R>) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &CallError<W, R>) -> $crate::option::Option<$crate::cmp::Ordering>`



## sddf_ipc_types::CallTarget

*Trait*

**Methods:**

- `call`
- `call_with_implicit_reader`



## sddf_ipc_types::ImplicitMessageReader

*Struct*

**Unit Struct**

**Methods:**

- `fn new() -> Self`

**Trait Implementations:**

- **Default**
  - `fn default() -> ImplicitMessageReader`
- **MessageReader**
  - `fn read_message(self: &Self, label: MessageLabel, buf: &[MessageRegisterValue]) -> Result<T, <Self as >::Error>`



## sddf_ipc_types::MessageLabel

*Type Alias*: `u64`



## sddf_ipc_types::MessageReader

*Trait*

**Methods:**

- `Error`
- `read_message`



## sddf_ipc_types::MessageRegisterValue

*Type Alias*: `u64`



## sddf_ipc_types::MessageWriter

*Trait*

**Methods:**

- `Error`
- `write_message`



## sddf_ipc_types::ReadFromMessage

*Trait*

**Methods:**

- `Error`
- `read_from_message`



