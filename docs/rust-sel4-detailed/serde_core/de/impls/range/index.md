*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range](index.md)*

---

# Module `range`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeVisitor`](#rangevisitor) | struct |  |
| [`Field`](#field) | enum |  |
| [`FIELDS`](#fields) | const |  |

## Structs

### `RangeVisitor<Idx>`

```rust
struct RangeVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
```

#### Trait Implementations

##### `impl Expected for RangeVisitor<Idx>`

- <span id="rangevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Idx> Visitor for RangeVisitor<Idx>`

- <span id="rangevisitor-visitor-type-value"></span>`type Value = (Idx, Idx)`

- <span id="rangevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangevisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="rangevisitor-visitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

## Enums

### `Field`

```rust
enum Field {
    Start,
    End,
}
```

#### Trait Implementations

##### `impl Deserialize for Field`

- <span id="field-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../../index.md#deserializer)

##### `impl DeserializeOwned for Field`

## Constants

### `FIELDS`
```rust
const FIELDS: &[&str];
```

