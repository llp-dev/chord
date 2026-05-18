**serde_json > value > index**

# Module: value::index

## Contents

**Traits**

- [`Index`](#index) - A type that can be used to index into a `serde_json::Value`.

---

## serde_json::value::index::Index

*Trait*

A type that can be used to index into a `serde_json::Value`.

The [`get`] and [`get_mut`] methods of `Value` accept any type that
implements `Index`, as does the [square-bracket indexing operator]. This
trait is implemented for strings which are used as the index into a JSON
map, and for `usize` which is used as the index into a JSON array.

[`get`]: Value::get
[`get_mut`]: Value::get_mut
[square-bracket indexing operator]: Value#impl-Index%3CI%3E-for-Value

This trait is sealed and cannot be implemented for types outside of
`serde_json`.

# Examples

```
# use serde_json::json;
#
let data = json!({ "inner": [1, 2, 3] });

// Data is a JSON map so it can be indexed with a string.
let inner = &data["inner"];

// Inner is a JSON array so it can be indexed with an integer.
let first = &inner[0];

assert_eq!(first, 1);
```



