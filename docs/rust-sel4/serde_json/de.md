**serde_json > de**

# Module: de

## Contents

**Structs**

- [`Deserializer`](#deserializer) - A structure that deserializes JSON into Rust values.
- [`StreamDeserializer`](#streamdeserializer) - Iterator that deserializes a stream into multiple JSON values.

**Functions**

- [`from_reader`](#from_reader) - Deserialize an instance of type `T` from an I/O stream of JSON.
- [`from_slice`](#from_slice) - Deserialize an instance of type `T` from bytes of JSON text.
- [`from_str`](#from_str) - Deserialize an instance of type `T` from a string of JSON text.

---

## serde_json::de::Deserializer

*Struct*

A structure that deserializes JSON into Rust values.

**Generic Parameters:**
- R

**Methods:**

- `fn from_reader(reader: R) -> Self` - Creates a JSON deserializer from an `io::Read`.
- `fn end(self: & mut Self) -> Result<()>` - The `Deserializer::end` method should be called after a value has been fully deserialized.
- `fn into_iter<T>(self: Self) -> StreamDeserializer<'de, R, T>` - Turn a JSON deserializer into an iterator over values of type T.
- `fn from_slice(bytes: &'a [u8]) -> Self` - Creates a JSON deserializer from a `&[u8]`.
- `fn new(read: R) -> Self` - Create a JSON deserializer from one of the possible serde_json input
- `fn from_str(s: &'a str) -> Self` - Creates a JSON deserializer from a `&str`.



## serde_json::de::StreamDeserializer

*Struct*

Iterator that deserializes a stream into multiple JSON values.

A stream deserializer can be created from any JSON deserializer using the
`Deserializer::into_iter` method.

The data can consist of any JSON value. Values need to be a self-delineating value e.g.
arrays, objects, or strings, or be followed by whitespace or a self-delineating value.

```
use serde_json::{Deserializer, Value};

fn main() {
    let data = "{\"k\": 3}1\"cool\"\"stuff\" 3{}  [0, 1, 2]";

    let stream = Deserializer::from_str(data).into_iter::<Value>();

    for value in stream {
        println!("{}", value.unwrap());
    }
}
```

**Generic Parameters:**
- 'de
- R
- T

**Methods:**

- `fn new(read: R) -> Self` - Create a JSON stream deserializer from one of the possible serde_json
- `fn byte_offset(self: &Self) -> usize` - Returns the number of bytes so far deserialized into a successful `T`.

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<Result<T>>`



## serde_json::de::from_reader

*Function*

Deserialize an instance of type `T` from an I/O stream of JSON.

The content of the I/O stream is deserialized directly from the stream
without being buffered in memory by serde_json.

When reading from a source against which short reads are not efficient, such
as a [`File`], you will want to apply your own buffering because serde_json
will not buffer the input. See [`std::io::BufReader`].

It is expected that the input stream ends after the deserialized object.
If the stream does not end, such as in the case of a persistent socket connection,
this function will not return. It is possible instead to deserialize from a prefix of an input
stream without looking for EOF by managing your own [`Deserializer`].

Note that counter to intuition, this function is usually slower than
reading a file completely into memory and then applying [`from_str`]
or [`from_slice`] on it. See [issue #160].

[`File`]: std::fs::File
[issue #160]: https://github.com/serde-rs/json/issues/160

# Example

Reading the contents of a file.

```
use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
# }
# fn fake_main() {
    let u = read_user_from_file("test.json").unwrap();
    println!("{:#?}", u);
}
```

Reading from a persistent socket connection.

```
use serde::Deserialize;

use std::error::Error;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_stream(stream: &mut BufReader<TcpStream>) -> Result<User, Box<dyn Error>> {
    let mut de = serde_json::Deserializer::from_reader(stream);
    let u = User::deserialize(&mut de)?;

    Ok(u)
}

fn main() {
# }
# fn fake_main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    for tcp_stream in listener.incoming() {
        let mut buffered = BufReader::new(tcp_stream.unwrap());
        println!("{:#?}", read_user_from_stream(&mut buffered));
    }
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
fn from_reader<R, T>(rdr: R) -> crate::error::Result<T>
```



## serde_json::de::from_slice

*Function*

Deserialize an instance of type `T` from bytes of JSON text.

# Example

```
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&[u8]`
    let j = b"
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_slice(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
fn from_slice<'a, T>(v: &'a [u8]) -> crate::error::Result<T>
```



## serde_json::de::from_str

*Function*

Deserialize an instance of type `T` from a string of JSON text.

# Example

```
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&str`
    let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_str(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
fn from_str<'a, T>(s: &'a str) -> crate::error::Result<T>
```



