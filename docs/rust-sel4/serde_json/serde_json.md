**serde_json**

# Module: serde_json

## Contents

**Modules**

- [`de`](#de) - Deserialize JSON data to a Rust data structure.
- [`error`](#error) - When serializing or deserializing JSON goes wrong.
- [`map`](#map) - A map of String to serde_json::Value.
- [`ser`](#ser) - Serialize a Rust data structure into JSON data.
- [`value`](#value) - The Value enum, a loosely typed way of representing any valid JSON value.

**Macros**

- [`json`](#json) - Construct a `serde_json::Value` from a JSON literal.

---

## Module: de

Deserialize JSON data to a Rust data structure.



## Module: error

When serializing or deserializing JSON goes wrong.



## serde_json::json

*Declarative Macro*

Construct a `serde_json::Value` from a JSON literal.

```
# use serde_json::json;
#
let value = json!({
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "serde",
            "json"
        ],
        "homepage": null
    }
});
```

Variables or expressions can be interpolated into the JSON literal. Any type
interpolated into an array element or object value must implement Serde's
`Serialize` trait, while any type interpolated into an object key must
implement `Into<String>`. If the `Serialize` implementation of the
interpolated type decides to fail, or if the interpolated type contains a
map with non-string keys, the `json!` macro will panic.

```
# use serde_json::json;
#
let code = 200;
let features = vec!["serde", "json"];

let value = json!({
    "code": code,
    "success": code == 200,
    "payload": {
        features[0]: features[1]
    }
});
```

Trailing commas are allowed inside both arrays and objects.

```
# use serde_json::json;
#
let value = json!([
    "notice",
    "the",
    "trailing",
    "comma -->",
]);
```

```rust
macro_rules! json {
    ($($json:tt)+) => { ... };
}
```



## Module: map

A map of String to serde_json::Value.

By default the map is backed by a [`BTreeMap`]. Enable the `preserve_order`
feature of serde_json to use [`IndexMap`] instead.

[`BTreeMap`]: std::collections::BTreeMap
[`IndexMap`]: indexmap::IndexMap



## Module: ser

Serialize a Rust data structure into JSON data.



## Module: value

The Value enum, a loosely typed way of representing any valid JSON value.

# Constructing JSON

Serde JSON provides a [`json!` macro][macro] to build `serde_json::Value`
objects with very natural JSON syntax.

```
use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
```

The `Value::to_string()` function converts a `serde_json::Value` into a
`String` of JSON text.

One neat thing about the `json!` macro is that variables and expressions can
be interpolated directly into the JSON value as you are building it. Serde
will check at compile time that the value you are interpolating is able to
be represented as JSON.

```
# use serde_json::json;
#
# fn random_phone() -> u16 { 0 }
#
let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

A string of JSON data can be parsed into a `serde_json::Value` by the
[`serde_json::from_str`][from_str] function. There is also
[`from_slice`][from_slice] for parsing from a byte slice `&[u8]` and
[`from_reader`][from_reader] for parsing from any `io::Read` like a File or
a TCP stream.

```
use serde_json::{json, Value, Error};

fn untyped_example() -> Result<(), Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
#
# untyped_example().unwrap();
```

[macro]: crate::json
[from_str]: crate::de::from_str
[from_slice]: crate::de::from_slice
[from_reader]: crate::de::from_reader



