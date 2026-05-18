*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range_to](index.md)*

---

# Module `range_to`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeToVisitor`](#rangetovisitor) | struct |  |
| [`Field`](#field) | enum |  |
| [`FIELDS`](#fields) | const |  |

## Structs

### `RangeToVisitor<Idx>`

```rust
struct RangeToVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
```

#### Trait Implementations

##### `impl Expected for RangeToVisitor<Idx>`

- <span id="rangetovisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Idx> Visitor for RangeToVisitor<Idx>`

- <span id="rangetovisitor-visitor-type-value"></span>`type Value = Idx`

- <span id="rangetovisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangetovisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="rangetovisitor-visitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

## Enums

### `Field`

```rust
enum Field {
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

