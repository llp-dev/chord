*[chrono](../../index.md) / [weekday](../index.md) / [weekday_serde](index.md)*

---

# Module `weekday_serde`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WeekdayVisitor`](#weekdayvisitor) | struct |  |

## Structs

### `WeekdayVisitor`

```rust
struct WeekdayVisitor;
```

#### Trait Implementations

##### `impl Expected for WeekdayVisitor`

- <span id="weekdayvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for WeekdayVisitor`

- <span id="weekdayvisitor-visitor-type-value"></span>`type Value = Weekday`

- <span id="weekdayvisitor-visitor-expecting"></span>`fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="weekdayvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, value: &str) -> Result<<Self as >::Value, E>`

