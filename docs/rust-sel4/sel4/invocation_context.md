**sel4 > invocation_context**

# Module: invocation_context

## Contents

**Structs**

- [`NoInvocationContext`](#noinvocationcontext) - The absence of a strategy for discovering the current thread's IPC buffer.

**Traits**

- [`InvocationContext`](#invocationcontext) - A strategy for discovering the current thread's IPC buffer.

**Type Aliases**

- [`NoExplicitInvocationContext`](#noexplicitinvocationcontext) - The default strategy for discovering the current thread's IPC buffer.

---

## sel4::invocation_context::InvocationContext

*Trait*

A strategy for discovering the current thread's IPC buffer.

**Methods:**

- `with_context`



## sel4::invocation_context::NoExplicitInvocationContext

*Type Alias*: `crate::ImplicitInvocationContext`

The default strategy for discovering the current thread's IPC buffer.

When the `"state"` feature is enabled, [`NoExplicitInvocationContext`] is an alias for
[`ImplicitInvocationContext`](crate::ImplicitInvocationContext), which uses the [`IpcBuffer`]
set by [`set_ipc_buffer`](crate::set_ipc_buffer). Otherwise, it is an alias for
[`NoInvocationContext`](crate::NoInvocationContext), which does not implement
[`InvocationContext`].



## sel4::invocation_context::NoInvocationContext

*Struct*

The absence of a strategy for discovering the current thread's IPC buffer.

**Unit Struct**

**Methods:**

- `fn new() -> Self`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> NoInvocationContext`
- **PartialEq**
  - `fn eq(self: &Self, other: &NoInvocationContext) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> NoInvocationContext`



