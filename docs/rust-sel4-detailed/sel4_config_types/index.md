# Crate `sel4_config_types`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Configuration`](#configuration) | struct |  |
| [`Value`](#value) | enum |  |
| [`Key`](#key) | type |  |

## Structs

### `Configuration`

```rust
struct Configuration(alloc::collections::BTreeMap<Key, Value>);
```

#### Implementations

- <span id="configuration-new"></span>`fn new(map: BTreeMap<Key, Value>) -> Self` — [`Key`](#key), [`Value`](#value)

- <span id="configuration-empty"></span>`fn empty() -> Self`

- <span id="configuration-get"></span>`fn get(&self, key: &str) -> Option<&Value>` — [`Value`](#value)

- <span id="configuration-iter"></span>`fn iter(&self) -> impl Iterator<Item = (&String, &Value)>` — [`Value`](#value)

- <span id="configuration-insert"></span>`fn insert(&mut self, key: Key, value: Value) -> Option<Value>` — [`Key`](#key), [`Value`](#value)

- <span id="configuration-append"></span>`fn append(&mut self, other: Self)`

#### Trait Implementations

##### `impl Clone for Configuration`

- <span id="configuration-clone"></span>`fn clone(&self) -> Configuration` — [`Configuration`](#configuration)

##### `impl Debug for Configuration`

- <span id="configuration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Configuration`

- <span id="configuration-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Configuration`

##### `impl Eq for Configuration`

##### `impl PartialEq for Configuration`

- <span id="configuration-partialeq-eq"></span>`fn eq(&self, other: &Configuration) -> bool` — [`Configuration`](#configuration)

##### `impl Serialize for Configuration`

- <span id="configuration-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Configuration`

## Enums

### `Value`

```rust
enum Value {
    Bool(bool),
    String(alloc::string::String),
}
```

#### Implementations

- <span id="value-as-bool"></span>`fn as_bool(&self) -> Option<bool>`

- <span id="value-as-str"></span>`fn as_str(&self) -> Option<&str>`

#### Trait Implementations

##### `impl Clone for Value`

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](#value)

##### `impl Debug for Value`

- <span id="value-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Value`

- <span id="value-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Value`

##### `impl Eq for Value`

##### `impl PartialEq for Value`

- <span id="value-partialeq-eq"></span>`fn eq(&self, other: &Value) -> bool` — [`Value`](#value)

##### `impl Serialize for Value`

- <span id="value-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Value`

## Type Aliases

### `Key`

```rust
type Key = alloc::string::String;
```

