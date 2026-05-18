*[sel4_panicking](../../index.md) / [count](../index.md) / [with_tls](index.md)*

---

# Module `with_tls`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MustAbort`](#mustabort) | enum |  |
| [`count_panic`](#count-panic) | fn |  |
| [`count_panic_caught`](#count-panic-caught) | fn |  |
| [`MAX_PANIC_DEPTH`](#max-panic-depth) | const |  |

## Enums

### `MustAbort`

```rust
enum MustAbort {
    MaxDepthExceeded,
}
```

#### Trait Implementations

##### `impl Display for MustAbort`

- <span id="mustabort-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for MustAbort`

- <span id="mustabort-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `count_panic`

```rust
fn count_panic() -> Option<MustAbort>
```

### `count_panic_caught`

```rust
fn count_panic_caught()
```

## Constants

### `MAX_PANIC_DEPTH`
```rust
const MAX_PANIC_DEPTH: usize = 3usize;
```

