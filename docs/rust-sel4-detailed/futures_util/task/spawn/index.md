*[futures_util](../../index.md) / [task](../index.md) / [spawn](index.md)*

---

# Module `spawn`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SpawnExt`](#spawnext) | trait | Extension trait for `Spawn`. |
| [`LocalSpawnExt`](#localspawnext) | trait | Extension trait for `LocalSpawn`. |

## Traits

### `SpawnExt`

```rust
trait SpawnExt: Spawn { ... }
```

Extension trait for `Spawn`.

#### Provided Methods

- `fn spawn<Fut>(&self, future: Fut) -> Result<(), SpawnError>`

  Spawns a task that polls the given future with output `()` to

#### Implementors

- `Sp`

### `LocalSpawnExt`

```rust
trait LocalSpawnExt: LocalSpawn { ... }
```

Extension trait for `LocalSpawn`.

#### Provided Methods

- `fn spawn_local<Fut>(&self, future: Fut) -> Result<(), SpawnError>`

  Spawns a task that polls the given future with output `()` to

#### Implementors

- `Sp`

