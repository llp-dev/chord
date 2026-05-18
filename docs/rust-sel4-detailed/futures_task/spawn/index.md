*[futures_task](../index.md) / [spawn](index.md)*

---

# Module `spawn`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`if_alloc`](#if-alloc) | mod |  |
| [`SpawnError`](#spawnerror) | struct | An error that occurred during spawning. |
| [`Spawn`](#spawn) | trait | The `Spawn` trait allows for pushing futures onto an executor that will run them to completion. |
| [`LocalSpawn`](#localspawn) | trait | The `LocalSpawn` is similar to [`Spawn`], but allows spawning futures that don't implement `Send`. |

## Modules

- [`if_alloc`](if_alloc/index.md)

## Structs

### `SpawnError`

```rust
struct SpawnError {
    _priv: (),
}
```

An error that occurred during spawning.

#### Implementations

- <span id="spawnerror-shutdown"></span>`fn shutdown() -> Self`

  Spawning failed because the executor has been shut down.

- <span id="spawnerror-is-shutdown"></span>`fn is_shutdown(&self) -> bool`

  Check whether spawning failed to the executor being shut down.

#### Trait Implementations

##### `impl Debug for SpawnError`

- <span id="spawnerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SpawnError`

- <span id="spawnerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for SpawnError`

- <span id="spawnerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `Spawn`

```rust
trait Spawn { ... }
```

The `Spawn` trait allows for pushing futures onto an executor that will
run them to completion.

#### Required Methods

- `fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError>`

  Spawns a future that will be run to completion.

#### Provided Methods

- `fn status(&self) -> Result<(), SpawnError>`

  Determines whether the executor is able to spawn new tasks.

#### Implementors

- `&Sp`
- `&mut Sp`
- `alloc::boxed::Box<Sp>`
- `alloc::rc::Rc<Sp>`
- `alloc::sync::Arc<Sp>`

### `LocalSpawn`

```rust
trait LocalSpawn { ... }
```

The `LocalSpawn` is similar to [`Spawn`](#spawn), but allows spawning futures
that don't implement `Send`.

#### Required Methods

- `fn spawn_local_obj(&self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError>`

  Spawns a future that will be run to completion.

#### Provided Methods

- `fn status_local(&self) -> Result<(), SpawnError>`

  Determines whether the executor is able to spawn new tasks.

#### Implementors

- `&Sp`
- `&mut Sp`
- `alloc::boxed::Box<Sp>`
- `alloc::rc::Rc<Sp>`
- `alloc::sync::Arc<Sp>`

