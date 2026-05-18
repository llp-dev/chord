*[chrono](../../../index.md) / [naive](../../index.md) / [date](../index.md) / [serde](index.md)*

---

# Module `serde`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NaiveDateVisitor`](#naivedatevisitor) | struct |  |

## Structs

### `NaiveDateVisitor`

```rust
struct NaiveDateVisitor;
```

#### Trait Implementations

##### `impl Expected for NaiveDateVisitor`

- <span id="naivedatevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for NaiveDateVisitor`

- <span id="naivedatevisitor-visitor-type-value"></span>`type Value = NaiveDate`

- <span id="naivedatevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="naivedatevisitor-visitor-visit-str"></span>`fn visit_str<E>(self, value: &str) -> Result<<Self as >::Value, E>`

