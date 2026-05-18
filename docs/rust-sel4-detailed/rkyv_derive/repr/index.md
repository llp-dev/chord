*[rkyv_derive](../index.md) / [repr](index.md)*

---

# Module `repr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Primitive`](#primitive) | enum |  |
| [`Modifier`](#modifier) | enum |  |
| [`Repr`](#repr) | enum |  |

## Enums

### `Primitive`

```rust
enum Primitive {
    I8,
    I16,
    I32,
    I64,
    Isize,
    U8,
    U16,
    U32,
    U64,
    Usize,
}
```

#### Implementations

- <span id="primitive-const-all"></span>`const ALL: [Self; 10]`

- <span id="primitive-as-str"></span>`const fn as_str(&self) -> &'static str`

- <span id="primitive-is-well-defined"></span>`const fn is_well_defined(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Primitive`

- <span id="primitive-clone"></span>`fn clone(&self) -> Primitive` — [`Primitive`](#primitive)

##### `impl Copy for Primitive`

### `Modifier`

```rust
enum Modifier {
    Packed(usize),
    Align(usize),
}
```

### `Repr`

```rust
enum Repr {
    Transparent,
    Primitive(Primitive),
    C {
        primitive: Option<Primitive>,
        modifier: Option<Modifier>,
    },
    Rust {
        modifier: Option<Modifier>,
    },
}
```

#### Implementations

- <span id="repr-from-attrs"></span>`fn from_attrs(attrs: &[Attribute]) -> Result<Self, Error>`

- <span id="repr-is-struct-well-defined"></span>`fn is_struct_well_defined(&self) -> bool`

- <span id="repr-is-enum-well-defined"></span>`fn is_enum_well_defined(&self) -> bool`

