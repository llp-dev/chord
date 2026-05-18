*[sel4](../../../../../index.md) / [arch](../../../../index.md) / [imp](../../../index.md) / [arch](../../index.md) / [imp](../index.md) / [user_context](index.md)*

---

# Module `user_context`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UserContext`](#usercontext) | struct |  |

## Structs

### `UserContext`

```rust
struct UserContext(sys::seL4_UserContext);
```

#### Implementations

- <span id="usercontext-from-inner"></span>`const fn from_inner(inner: sys::seL4_UserContext) -> Self`

- <span id="usercontext-into-inner"></span>`const fn into_inner(self) -> sys::seL4_UserContext`

- <span id="usercontext-inner"></span>`const fn inner(&self) -> &sys::seL4_UserContext`

- <span id="usercontext-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_UserContext`

- <span id="usercontext-pc"></span>`fn pc(&self) -> &Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-pc-mut"></span>`fn pc_mut(&mut self) -> &mut Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-sp"></span>`fn sp(&self) -> &Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-sp-mut"></span>`fn sp_mut(&mut self) -> &mut Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-c-param"></span>`fn c_param(&self, ix: usize) -> &Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-c-param-mut"></span>`fn c_param_mut(&mut self, ix: usize) -> &mut Word` — [`Word`](../../../../../index.md#word)

#### Trait Implementations

##### `impl Clone for UserContext`

- <span id="usercontext-clone"></span>`fn clone(&self) -> UserContext` — [`UserContext`](#usercontext)

##### `impl Debug for UserContext`

- <span id="usercontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for UserContext`

- <span id="usercontext-default"></span>`fn default() -> UserContext` — [`UserContext`](#usercontext)

##### `impl Eq for UserContext`

##### `impl PartialEq for UserContext`

- <span id="usercontext-partialeq-eq"></span>`fn eq(&self, other: &UserContext) -> bool` — [`UserContext`](#usercontext)

##### `impl StructuralPartialEq for UserContext`

