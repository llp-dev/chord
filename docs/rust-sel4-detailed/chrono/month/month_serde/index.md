*[chrono](../../index.md) / [month](../index.md) / [month_serde](index.md)*

---

# Module `month_serde`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MonthVisitor`](#monthvisitor) | struct |  |

## Structs

### `MonthVisitor`

```rust
struct MonthVisitor;
```

#### Trait Implementations

##### `impl Expected for MonthVisitor`

- <span id="monthvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for MonthVisitor`

- <span id="monthvisitor-visitor-type-value"></span>`type Value = Month`

- <span id="monthvisitor-visitor-expecting"></span>`fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="monthvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, value: &str) -> Result<<Self as >::Value, E>`

