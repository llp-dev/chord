**sel4 > state**

# Module: state

## Contents

**Structs**

- [`ImplicitInvocationContext`](#implicitinvocationcontext) - The strategy for discovering the current thread's IPC buffer which uses thread-local state.

**Functions**

- [`ipc_buffer_is_thread_local`](#ipc_buffer_is_thread_local) - Returns whether this crate's IPC buffer slot is thread-local.
- [`set_ipc_buffer`](#set_ipc_buffer) - Sets the IPC buffer that this crate will use for this thread.
- [`try_with_ipc_buffer_slot`](#try_with_ipc_buffer_slot) - Provides low-level access to this thread's IPC buffer.
- [`try_with_ipc_buffer_slot_mut`](#try_with_ipc_buffer_slot_mut) - Provides low-level mutable access to this thread's IPC buffer.
- [`with_ipc_buffer`](#with_ipc_buffer) - Provides access to this thread's IPC buffer.
- [`with_ipc_buffer_mut`](#with_ipc_buffer_mut) - Provides mutable access to this thread's IPC buffer.

---

## sel4::state::ImplicitInvocationContext

*Struct*

The strategy for discovering the current thread's IPC buffer which uses thread-local state.

This thread-local state can be modified using [`with_ipc_buffer`] and [`set_ipc_buffer`].

Requires the `"state"` feature to be enabled.

**Unit Struct**

**Methods:**

- `fn new() -> Self`

**Traits:** Eq, Copy

**Trait Implementations:**

- **InvocationContext**
  - `fn with_context<T, impl FnOnce(&mut IpcBuffer) -> T>(self: & mut Self, f: impl Trait) -> T`
- **Default**
  - `fn default() -> ImplicitInvocationContext`
- **PartialEq**
  - `fn eq(self: &Self, other: &ImplicitInvocationContext) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> ImplicitInvocationContext`



## sel4::state::ipc_buffer_is_thread_local

*Function*

Returns whether this crate's IPC buffer slot is thread-local.

Requires the `"state"` feature to be enabled.

```rust
fn ipc_buffer_is_thread_local() -> bool
```



## sel4::state::set_ipc_buffer

*Function*

Sets the IPC buffer that this crate will use for this thread.

This function does not modify kernel state. It only affects this crate's thread-local state.

Requires the `"state"` feature to be enabled.

```rust
fn set_ipc_buffer(ipc_buffer: &'static  mut crate::IpcBuffer)
```



## sel4::state::try_with_ipc_buffer_slot

*Function*

Provides low-level access to this thread's IPC buffer.

This function does not modify kernel state. It only affects this crate's thread-local state.

Requires the `"state"` feature to be enabled.

```rust
fn try_with_ipc_buffer_slot<F, T>(f: F) -> T
```



## sel4::state::try_with_ipc_buffer_slot_mut

*Function*

Provides low-level mutable access to this thread's IPC buffer.

This function does not modify kernel state. It only affects this crate's thread-local state.

Requires the `"state"` feature to be enabled.

```rust
fn try_with_ipc_buffer_slot_mut<F, T>(f: F) -> T
```



## sel4::state::with_ipc_buffer

*Function*

Provides access to this thread's IPC buffer.

Requires the `"state"` feature to be enabled.

```rust
fn with_ipc_buffer<F, T>(f: F) -> T
```



## sel4::state::with_ipc_buffer_mut

*Function*

Provides mutable access to this thread's IPC buffer.

Requires the `"state"` feature to be enabled.

```rust
fn with_ipc_buffer_mut<F, T>(f: F) -> T
```



