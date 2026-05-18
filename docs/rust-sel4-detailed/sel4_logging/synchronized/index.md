*[sel4_logging](../index.md) / [synchronized](index.md)*

---

# Module `synchronized`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SynchronizedLogger`](#synchronizedlogger) | struct |  |

## Structs

### `SynchronizedLogger<R, T>`

```rust
struct SynchronizedLogger<R, T>(lock_api::Mutex<R, T>);
```

#### Implementations

- <span id="synchronizedlogger-new"></span>`const fn new(inner: T) -> Self`

#### Trait Implementations

##### `impl<R: RawMutex + Send + Sync, T: Log> Log for SynchronizedLogger<R, T>`

- <span id="synchronizedlogger-log-enabled"></span>`fn enabled(&self, metadata: &Metadata<'_>) -> bool`

- <span id="synchronizedlogger-log"></span>`fn log(&self, record: &Record<'_>)`

- <span id="synchronizedlogger-log-flush"></span>`fn flush(&self)`

