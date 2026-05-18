*[sel4](../index.md) / [invocation_context](index.md)*

---

# Module `invocation_context`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoInvocationContext`](#noinvocationcontext) | struct | The absence of a strategy for discovering the current thread's IPC buffer. |
| [`InvocationContext`](#invocationcontext) | trait | A strategy for discovering the current thread's IPC buffer. |
| [`NoExplicitInvocationContextInternal`](#noexplicitinvocationcontextinternal) | type |  |
| [`NoExplicitInvocationContext`](#noexplicitinvocationcontext) | type | The default strategy for discovering the current thread's IPC buffer. |

## Structs

### `NoInvocationContext`

```rust
struct NoInvocationContext;
```

The absence of a strategy for discovering the current thread's IPC buffer.

#### Implementations

- <span id="noinvocationcontext-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for NoInvocationContext`

- <span id="noinvocationcontext-clone"></span>`fn clone(&self) -> NoInvocationContext` — [`NoInvocationContext`](#noinvocationcontext)

##### `impl Copy for NoInvocationContext`

##### `impl Debug for NoInvocationContext`

- <span id="noinvocationcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NoInvocationContext`

- <span id="noinvocationcontext-default"></span>`fn default() -> NoInvocationContext` — [`NoInvocationContext`](#noinvocationcontext)

##### `impl Eq for NoInvocationContext`

##### `impl Hash for NoInvocationContext`

- <span id="noinvocationcontext-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for NoInvocationContext`

- <span id="noinvocationcontext-partialeq-eq"></span>`fn eq(&self, other: &NoInvocationContext) -> bool` — [`NoInvocationContext`](#noinvocationcontext)

##### `impl StructuralPartialEq for NoInvocationContext`

## Traits

### `InvocationContext`

```rust
trait InvocationContext { ... }
```

A strategy for discovering the current thread's IPC buffer.

#### Required Methods

- `fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T`

#### Implementors

- [`ImplicitInvocationContext`](../state/index.md#implicitinvocationcontext)
- `&core::cell::RefCell<U>`
- `&mut U`
- `&mut crate::IpcBuffer`

## Type Aliases

### `NoExplicitInvocationContextInternal`

```rust
type NoExplicitInvocationContextInternal = crate::ImplicitInvocationContext;
```

### `NoExplicitInvocationContext`

```rust
type NoExplicitInvocationContext = crate::ImplicitInvocationContext;
```

The default strategy for discovering the current thread's IPC buffer.

When the `"state"` feature is enabled, [`NoExplicitInvocationContext`](#noexplicitinvocationcontext) is an alias for
[`ImplicitInvocationContext`](crate::ImplicitInvocationContext), which uses the [`IpcBuffer`](../ipc_buffer/index.md)
set by [`set_ipc_buffer`](crate::set_ipc_buffer). Otherwise, it is an alias for
[`NoInvocationContext`](crate::NoInvocationContext), which does not implement
[`InvocationContext`](#invocationcontext).

