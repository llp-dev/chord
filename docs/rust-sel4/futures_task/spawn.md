**futures_task > spawn**

# Module: spawn

## Contents

**Structs**

- [`SpawnError`](#spawnerror) - An error that occurred during spawning.

**Traits**

- [`LocalSpawn`](#localspawn) - The `LocalSpawn` is similar to [`Spawn`], but allows spawning futures
- [`Spawn`](#spawn) - The `Spawn` trait allows for pushing futures onto an executor that will

---

## futures_task::spawn::LocalSpawn

*Trait*

The `LocalSpawn` is similar to [`Spawn`], but allows spawning futures
that don't implement `Send`.

**Methods:**

- `spawn_local_obj`: Spawns a future that will be run to completion.
- `status_local`: Determines whether the executor is able to spawn new tasks.



## futures_task::spawn::Spawn

*Trait*

The `Spawn` trait allows for pushing futures onto an executor that will
run them to completion.

**Methods:**

- `spawn_obj`: Spawns a future that will be run to completion.
- `status`: Determines whether the executor is able to spawn new tasks.



## futures_task::spawn::SpawnError

*Struct*

An error that occurred during spawning.

**Methods:**

- `fn shutdown() -> Self` - Spawning failed because the executor has been shut down.
- `fn is_shutdown(self: &Self) -> bool` - Check whether spawning failed to the executor being shut down.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



