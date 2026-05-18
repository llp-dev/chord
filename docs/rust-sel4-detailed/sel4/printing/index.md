*[sel4](../index.md) / [printing](index.md)*

---

# Module `printing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugWrite`](#debugwrite) | struct | Implements `core::fmt::Write` using [`debug_put_char`]. |
| [`debug_put_char`](#debug-put-char) | fn | Corresponds to `seL4_DebugPutChar`. |

## Structs

### `DebugWrite`

```rust
struct DebugWrite;
```

Implements `core::fmt::Write` using [`debug_put_char`](#debug-put-char).

#### Trait Implementations

##### `impl Write for DebugWrite`

- <span id="debugwrite-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

## Functions

### `debug_put_char`

```rust
fn debug_put_char(c: u8)
```

Corresponds to `seL4_DebugPutChar`.

