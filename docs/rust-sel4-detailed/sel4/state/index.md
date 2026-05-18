*[sel4](../index.md) / [state](index.md)*

---

# Module `state`

## Contents

- [Modules](#modules)
  - [`token`](#token)
- [Structs](#structs)
  - [`SyncUnsafeCell`](#syncunsafecell)
  - [`TokenCellWrapper`](#tokencellwrapper)
  - [`IpcBufferAccessor`](#ipcbufferaccessor)
  - [`ImplicitInvocationContext`](#implicitinvocationcontext)
- [Functions](#functions)
  - [`try_with_ipc_buffer_slot`](#try-with-ipc-buffer-slot)
  - [`try_with_ipc_buffer_slot_mut`](#try-with-ipc-buffer-slot-mut)
  - [`with_ipc_buffer`](#with-ipc-buffer)
  - [`with_ipc_buffer_mut`](#with-ipc-buffer-mut)
  - [`set_ipc_buffer`](#set-ipc-buffer)
  - [`ipc_buffer_is_thread_local`](#ipc-buffer-is-thread-local)
- [Type Aliases](#type-aliases)
  - [`TokenImpl`](#tokenimpl)
- [Constants](#constants)
  - [`STATE_IS_THREAD_LOCAL`](#state-is-thread-local)
- [Macros](#macros)
  - [`maybe_add_thread_local_attr!`](#maybe-add-thread-local-attr)
  - [`maybe_extern!`](#maybe-extern)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`token`](#token) | mod |  |
| [`SyncUnsafeCell`](#syncunsafecell) | struct |  |
| [`TokenCellWrapper`](#tokencellwrapper) | struct |  |
| [`IpcBufferAccessor`](#ipcbufferaccessor) | struct |  |
| [`ImplicitInvocationContext`](#implicitinvocationcontext) | struct | The strategy for discovering the current thread's IPC buffer which uses thread-local state. |
| [`try_with_ipc_buffer_slot`](#try-with-ipc-buffer-slot) | fn | Provides low-level access to this thread's IPC buffer. |
| [`try_with_ipc_buffer_slot_mut`](#try-with-ipc-buffer-slot-mut) | fn | Provides low-level mutable access to this thread's IPC buffer. |
| [`with_ipc_buffer`](#with-ipc-buffer) | fn | Provides access to this thread's IPC buffer. |
| [`with_ipc_buffer_mut`](#with-ipc-buffer-mut) | fn | Provides mutable access to this thread's IPC buffer. |
| [`set_ipc_buffer`](#set-ipc-buffer) | fn | Sets the IPC buffer that this crate will use for this thread. |
| [`ipc_buffer_is_thread_local`](#ipc-buffer-is-thread-local) | fn | Returns whether this crate's IPC buffer slot is thread-local. |
| [`TokenImpl`](#tokenimpl) | type |  |
| [`STATE_IS_THREAD_LOCAL`](#state-is-thread-local) | const |  |
| [`maybe_add_thread_local_attr!`](#maybe-add-thread-local-attr) | macro |  |
| [`maybe_extern!`](#maybe-extern) | macro |  |

## Modules

- [`token`](token/index.md)

## Structs

### `SyncUnsafeCell<T>`

```rust
struct SyncUnsafeCell<T>(core::cell::UnsafeCell<T>);
```

#### Trait Implementations

##### `impl<T: Sync> Sync for SyncUnsafeCell<T>`

### `TokenCellWrapper<A>`

```rust
struct TokenCellWrapper<A>(token::TokenCell<token::UnsyncToken, A>);
```

### `IpcBufferAccessor`

```rust
struct IpcBufferAccessor;
```

#### Trait Implementations

##### `impl Accessor for IpcBufferAccessor`

- <span id="ipcbufferaccessor-accessor-with"></span>`fn with<F, U>(&self, f: F) -> U`

### `ImplicitInvocationContext`

```rust
struct ImplicitInvocationContext;
```

The strategy for discovering the current thread's IPC buffer which uses thread-local state.

This thread-local state can be modified using [`with_ipc_buffer`](#with-ipc-buffer) and [`set_ipc_buffer`](#set-ipc-buffer).

Requires the `"state"` feature to be enabled.

#### Implementations

- <span id="implicitinvocationcontext-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-clone"></span>`fn clone(&self) -> ImplicitInvocationContext` — [`ImplicitInvocationContext`](#implicitinvocationcontext)

##### `impl Copy for ImplicitInvocationContext`

##### `impl Debug for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-default"></span>`fn default() -> ImplicitInvocationContext` — [`ImplicitInvocationContext`](#implicitinvocationcontext)

##### `impl Eq for ImplicitInvocationContext`

##### `impl Hash for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl InvocationContext for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-invocationcontext-with-context"></span>`fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T` — [`IpcBuffer`](../ipc_buffer/index.md#ipcbuffer)

##### `impl PartialEq for ImplicitInvocationContext`

- <span id="implicitinvocationcontext-partialeq-eq"></span>`fn eq(&self, other: &ImplicitInvocationContext) -> bool` — [`ImplicitInvocationContext`](#implicitinvocationcontext)

##### `impl StructuralPartialEq for ImplicitInvocationContext`

## Functions

### `try_with_ipc_buffer_slot`

```rust
fn try_with_ipc_buffer_slot<F, T>(f: F) -> T
where
    F: FnOnce(Result<&Option<&'static mut crate::IpcBuffer>, token::BorrowError>) -> T
```

Provides low-level access to this thread's IPC buffer.

This function does not modify kernel state. It only affects this crate's thread-local state.

Requires the `"state"` feature to be enabled.

### `try_with_ipc_buffer_slot_mut`

```rust
fn try_with_ipc_buffer_slot_mut<F, T>(f: F) -> T
where
    F: FnOnce(Result<&mut Option<&'static mut crate::IpcBuffer>, token::BorrowMutError>) -> T
```

Provides low-level mutable access to this thread's IPC buffer.

This function does not modify kernel state. It only affects this crate's thread-local state.

Requires the `"state"` feature to be enabled.

### `with_ipc_buffer`

```rust
fn with_ipc_buffer<F, T>(f: F) -> T
where
    F: FnOnce(&crate::IpcBuffer) -> T
```

Provides access to this thread's IPC buffer.

Requires the `"state"` feature to be enabled.

### `with_ipc_buffer_mut`

```rust
fn with_ipc_buffer_mut<F, T>(f: F) -> T
where
    F: FnOnce(&mut crate::IpcBuffer) -> T
```

Provides mutable access to this thread's IPC buffer.

Requires the `"state"` feature to be enabled.

### `set_ipc_buffer`

```rust
fn set_ipc_buffer(ipc_buffer: &'static mut crate::IpcBuffer)
```

Sets the IPC buffer that this crate will use for this thread.

This function does not modify kernel state. It only affects this crate's thread-local state.

Requires the `"state"` feature to be enabled.

### `ipc_buffer_is_thread_local`

```rust
const fn ipc_buffer_is_thread_local() -> bool
```

Returns whether this crate's IPC buffer slot is thread-local.

Requires the `"state"` feature to be enabled.

## Type Aliases

### `TokenImpl`

```rust
type TokenImpl = token::UnsyncToken;
```

## Constants

### `STATE_IS_THREAD_LOCAL`
```rust
const STATE_IS_THREAD_LOCAL: bool = true;
```

## Macros

### `maybe_add_thread_local_attr!`

### `maybe_extern!`

