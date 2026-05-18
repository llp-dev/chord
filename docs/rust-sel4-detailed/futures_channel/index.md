# Crate `futures_channel`

Asynchronous channels.

Like threads, concurrent tasks sometimes need to communicate with each
other. This module contains two basic abstractions for doing so:

- [`oneshot`](oneshot/index.md), a way of sending a single value from one task to another.
- [mpsc], a multi-producer, single-consumer channel for sending values
  between tasks, analogous to the similarly-named structure in the standard
  library.

All items are only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`lock`](#lock) | mod | A "mutex" which only supports `try_lock` |
| [`oneshot`](#oneshot) | mod | A channel for sending a single message between asynchronous tasks. |

## Modules

- [`lock`](lock/index.md) — A "mutex" which only supports `try_lock`
- [`oneshot`](oneshot/index.md) — A channel for sending a single message between asynchronous tasks.

