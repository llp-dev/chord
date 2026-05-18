**sel4_simple_task_threading**

# Module: sel4_simple_task_threading

## Contents

**Structs**

- [`StaticThread`](#staticthread)

**Type Aliases**

- [`StaticThreadEntryFn`](#staticthreadentryfn)

---

## sel4_simple_task_threading::StaticThread

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn new(endpoint: sel4::cap::Endpoint) -> Self`
- `fn recv_and_run(endpoint: sel4::cap::Endpoint, reply_authority: sel4::ReplyAuthority)`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(endpoint: sel4::cap::Endpoint) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> StaticThread`



## sel4_simple_task_threading::StaticThreadEntryFn

*Type Alias*: `fn(...)`



