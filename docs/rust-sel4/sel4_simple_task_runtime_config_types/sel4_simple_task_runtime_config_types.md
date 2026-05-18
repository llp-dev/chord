**sel4_simple_task_runtime_config_types**

# Module: sel4_simple_task_runtime_config_types

## Contents

**Structs**

- [`RuntimeConfig`](#runtimeconfig)
- [`RuntimeThreadConfig`](#runtimethreadconfig)

**Type Aliases**

- [`Address`](#address)
- [`CPtrBits`](#cptrbits)

---

## sel4_simple_task_runtime_config_types::Address

*Type Alias*: `NativeWord`



## sel4_simple_task_runtime_config_types::CPtrBits

*Type Alias*: `NativeWord`



## sel4_simple_task_runtime_config_types::RuntimeConfig

*Struct*

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(bytes: &'a [u8]) -> Self`
- `fn static_heap(self: &Self) -> Option<Range<Address>>`
- `fn static_heap_mutex_notification(self: &Self) -> Option<CPtrBits>`
- `fn idle_notification(self: &Self) -> Option<CPtrBits>`
- `fn threads(self: &Self) -> &[RuntimeThreadConfig]`
- `fn image_identifier(self: &Self) -> Option<&str>`
- `fn arg(self: &Self) -> &[u8]`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RuntimeConfig<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RuntimeConfig<'a>`



## sel4_simple_task_runtime_config_types::RuntimeThreadConfig

*Struct*

**Methods:**

- `fn ipc_buffer_addr(self: &Self) -> Address`
- `fn endpoint(self: &Self) -> Option<CPtrBits>`
- `fn reply_authority(self: &Self) -> Option<CPtrBits>`

**Traits:** IntoBytes, FromBytes, FromZeros, TryFromBytes, Eq, Immutable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &RuntimeThreadConfig) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RuntimeThreadConfig`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



