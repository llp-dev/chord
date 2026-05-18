*[anstyle](../index.md) / [reset](index.md)*

---

# Module `reset`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Reset`](#reset) | struct | Reset terminal formatting |
| [`RESET`](#reset) | const |  |

## Structs

### `Reset`

```rust
struct Reset;
```

Reset terminal formatting

#### Implementations

- <span id="reset-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code

  

  `Reset` also implements `Display` directly, so calling this method is optional.

#### Trait Implementations

##### `impl Clone for Reset`

- <span id="reset-clone"></span>`fn clone(&self) -> Reset` — [`Reset`](../index.md#reset)

##### `impl Copy for Reset`

##### `impl Debug for Reset`

- <span id="reset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Reset`

- <span id="reset-default"></span>`fn default() -> Reset` — [`Reset`](../index.md#reset)

##### `impl Display for Reset`

- <span id="reset-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Reset`

##### `impl Hash for Reset`

- <span id="reset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Reset`

- <span id="reset-ord-cmp"></span>`fn cmp(&self, other: &Reset) -> cmp::Ordering` — [`Reset`](../index.md#reset)

##### `impl PartialEq for Reset`

- <span id="reset-partialeq-eq"></span>`fn eq(&self, other: &Reset) -> bool` — [`Reset`](../index.md#reset)

##### `impl PartialOrd for Reset`

- <span id="reset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Reset) -> option::Option<cmp::Ordering>` — [`Reset`](../index.md#reset)

##### `impl StructuralPartialEq for Reset`

##### `impl ToString for Reset`

- <span id="reset-tostring-to-string"></span>`fn to_string(&self) -> String`

## Constants

### `RESET`
```rust
const RESET: &str;
```

