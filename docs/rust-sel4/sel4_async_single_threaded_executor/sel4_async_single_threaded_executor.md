**sel4_async_single_threaded_executor**

# Module: sel4_async_single_threaded_executor

## Contents

**Structs**

- [`LocalPool`](#localpool)
- [`LocalSpawner`](#localspawner)

**Functions**

- [`run_until_stalled`](#run_until_stalled) - Run a future to completion on the current thread.

---

## sel4_async_single_threaded_executor::LocalPool

*Struct*

**Methods:**

- `fn new() -> Self` - Create a new, empty pool of tasks.
- `fn spawner(self: &Self) -> LocalSpawner` - Get a clonable handle to the pool as a [`Spawn`].
- `fn run_all_until_stalled(self: & mut Self) -> Poll<()>`
- `fn run_until_stalled<F>(self: & mut Self, future: Pin<& mut F>) -> Poll<<F as >::Output>`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_async_single_threaded_executor::LocalSpawner

*Struct*

**Trait Implementations:**

- **Spawn**
  - `fn spawn_obj(self: &Self, future: FutureObj<'static, ()>) -> Result<(), SpawnError>`
  - `fn status(self: &Self) -> Result<(), SpawnError>`
- **Clone**
  - `fn clone(self: &Self) -> LocalSpawner`
- **LocalSpawn**
  - `fn spawn_local_obj(self: &Self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError>`
  - `fn status_local(self: &Self) -> Result<(), SpawnError>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_async_single_threaded_executor::run_until_stalled

*Function*

Run a future to completion on the current thread.

This function will block the caller until the given future has completed.

Use a [`LocalPool`] if you need finer-grained control over spawned tasks.

```rust
fn run_until_stalled<F>(future: core::pin::Pin<& mut F>) -> futures::task::Poll<<F as >::Output>
```



