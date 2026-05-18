*[serde_derive](../../index.md) / [internals](../index.md) / [name](index.md)*

---

# Module `name`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiName`](#multiname) | struct |  |
| [`Name`](#name) | struct |  |

## Structs

### `MultiName`

```rust
struct MultiName {
    serialize: Name,
    serialize_renamed: bool,
    deserialize: Name,
    deserialize_renamed: bool,
    deserialize_aliases: std::collections::BTreeSet<Name>,
}
```

#### Implementations

- <span id="multiname-from-attrs"></span>`fn from_attrs(source_name: Name, ser_name: Attr<'_, Name>, de_name: Attr<'_, Name>, de_aliases: Option<VecAttr<'_, Name>>) -> Self` — [`Name`](#name), [`Attr`](../attr/index.md#attr), [`VecAttr`](../attr/index.md#vecattr)

- <span id="multiname-serialize-name"></span>`fn serialize_name(&self) -> &Name` — [`Name`](#name)

  Return the container name for the container when serializing.

- <span id="multiname-deserialize-name"></span>`fn deserialize_name(&self) -> &Name` — [`Name`](#name)

  Return the container name for the container when deserializing.

- <span id="multiname-deserialize-aliases"></span>`fn deserialize_aliases(&self) -> &BTreeSet<Name>` — [`Name`](#name)

### `Name`

```rust
struct Name {
    pub value: String,
    pub span: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl Clone for Name`

- <span id="name-clone"></span>`fn clone(&self) -> Name` — [`Name`](#name)

##### `impl Display for Name`

- <span id="name-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Name`

##### `impl Ord for Name`

- <span id="name-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl PartialEq for Name`

- <span id="name-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for Name`

- <span id="name-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl Spanned for Name`

- <span id="name-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToString for Name`

- <span id="name-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for Name`

- <span id="name-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

