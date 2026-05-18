# Crate `one_shot_mutex`

One-shot locks that panic instead of (dead)locking on contention.

These locks allow no contention and panic instead of blocking on `lock` if they are already locked.
This is useful in situations where contention would be a bug,
such as in single-threaded programs that would deadlock on contention.

See the [`sync::RawOneShotMutex`](sync/mutex/index.md) and [`sync::RawOneShotRwLock`](sync/rwlock/index.md) types for more information.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sync`](#sync) | mod | One-shot lock variants that implement `Sync`. |
| [`unsync`](#unsync) | mod | One-shot lock variants that do not implement `Sync`. |

## Modules

- [`sync`](sync/index.md) — One-shot lock variants that implement `Sync`.
- [`unsync`](unsync/index.md) — One-shot lock variants that do not implement `Sync`.

