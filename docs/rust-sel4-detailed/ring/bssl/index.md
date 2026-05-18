*[ring](../index.md) / [bssl](index.md)*

---

# Module `bssl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Result`](#result) | struct | An `int` returned from a foreign function containing **1** if the function was successful or **0** if an error occurred. |

## Structs

### `Result`

```rust
struct Result(i32);
```

An `int` returned from a foreign function containing **1** if the function
was successful or **0** if an error occurred. This is the convention used by
C code in `ring`.

#### Trait Implementations

##### `impl Clone for Result`

- <span id="result-clone"></span>`fn clone(&self) -> Result` — [`Result`](#result)

##### `impl Copy for Result`

##### `impl Debug for Result`

- <span id="result-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

