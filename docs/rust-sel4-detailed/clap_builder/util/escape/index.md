*[clap_builder](../../index.md) / [util](../index.md) / [escape](index.md)*

---

# Module `escape`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Escape`](#escape) | struct |  |

## Structs

### `Escape<'s>`

```rust
struct Escape<'s>(&'s str);
```

#### Implementations

- <span id="escape-needs-escaping"></span>`fn needs_escaping(&self) -> bool`

- <span id="escape-to-cow"></span>`fn to_cow(&self) -> Cow<'s, str>`

#### Trait Implementations

##### `impl Display for Escape<'_>`

- <span id="escape-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl ToString for Escape<'s>`

- <span id="escape-tostring-to-string"></span>`fn to_string(&self) -> String`

