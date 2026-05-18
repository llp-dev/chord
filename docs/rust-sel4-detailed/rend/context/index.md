*[rend](../index.md) / [context](index.md)*

---

# Module `context`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ValueCheckContext`](#valuecheckcontext) | struct |  |

## Structs

### `ValueCheckContext`

```rust
struct ValueCheckContext {
    pub inner_name: &'static str,
    pub outer_name: &'static str,
}
```

#### Trait Implementations

##### `impl Debug for ValueCheckContext`

- <span id="valuecheckcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ValueCheckContext`

- <span id="valuecheckcontext-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointee for ValueCheckContext`

- <span id="valuecheckcontext-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for ValueCheckContext`

- <span id="valuecheckcontext-tostring-to-string"></span>`fn to_string(&self) -> String`

