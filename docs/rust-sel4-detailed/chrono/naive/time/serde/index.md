*[chrono](../../../index.md) / [naive](../../index.md) / [time](../index.md) / [serde](index.md)*

---

# Module `serde`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NaiveTimeVisitor`](#naivetimevisitor) | struct |  |

## Structs

### `NaiveTimeVisitor`

```rust
struct NaiveTimeVisitor;
```

#### Trait Implementations

##### `impl Expected for NaiveTimeVisitor`

- <span id="naivetimevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for NaiveTimeVisitor`

- <span id="naivetimevisitor-visitor-type-value"></span>`type Value = NaiveTime`

- <span id="naivetimevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="naivetimevisitor-visitor-visit-str"></span>`fn visit_str<E>(self, value: &str) -> Result<<Self as >::Value, E>`

