*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range_from](index.md)*

---

# Module `range_from`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeFromVisitor`](#rangefromvisitor) | struct |  |
| [`Field`](#field) | enum |  |
| [`FIELDS`](#fields) | const |  |

## Structs

### `RangeFromVisitor<Idx>`

```rust
struct RangeFromVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
```

#### Trait Implementations

##### `impl Expected for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Idx> Visitor for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-visitor-type-value"></span>`type Value = Idx`

- <span id="rangefromvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangefromvisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="rangefromvisitor-visitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

## Enums

### `Field`

```rust
enum Field {
    Start,
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

