**serde_core**

# Module: serde_core

## Contents

**Modules**

- [`de`](#de) - Generic data structure deserialization framework.
- [`ser`](#ser) - Generic data structure serialization framework.

**Macros**

- [`forward_to_deserialize_any`](#forward_to_deserialize_any) - Helper macro when implementing the `Deserializer` part of a new data format

---

## Module: de

Generic data structure deserialization framework.

The two most important traits in this module are [`Deserialize`] and
[`Deserializer`].

 - **A type that implements `Deserialize` is a data structure** that can be
   deserialized from any data format supported by Serde, and conversely
 - **A type that implements `Deserializer` is a data format** that can
   deserialize any data structure supported by Serde.

# The Deserialize trait

Serde provides [`Deserialize`] implementations for many Rust primitive and
standard library types. The complete list is below. All of these can be
deserialized using Serde out of the box.

Additionally, Serde provides a procedural macro called [`serde_derive`] to
automatically generate [`Deserialize`] implementations for structs and enums
in your program. See the [derive section of the manual] for how to use this.

In rare cases it may be necessary to implement [`Deserialize`] manually for
some type in your program. See the [Implementing `Deserialize`] section of
the manual for more about this.

Third-party crates may provide [`Deserialize`] implementations for types
that they expose. For example the [`linked-hash-map`] crate provides a
[`LinkedHashMap<K, V>`] type that is deserializable by Serde because the
crate provides an implementation of [`Deserialize`] for it.

# The Deserializer trait

[`Deserializer`] implementations are provided by third-party crates, for
example [`serde_json`], [`serde_yaml`] and [`postcard`].

A partial list of well-maintained formats is given on the [Serde
website][data formats].

# Implementations of Deserialize provided by Serde

This is a slightly different set of types than what is supported for
serialization. Some types can be serialized by Serde but not deserialized.
One example is `OsStr`.

 - **Primitive types**:
   - bool
   - i8, i16, i32, i64, i128, isize
   - u8, u16, u32, u64, u128, usize
   - f32, f64
   - char
 - **Compound types**:
   - \[T; 0\] through \[T; 32\]
   - tuples up to size 16
 - **Common standard library types**:
   - String
   - Option\<T\>
   - Result\<T, E\>
   - PhantomData\<T\>
 - **Wrapper types**:
   - Box\<T\>
   - Box\<\[T\]\>
   - Box\<str\>
   - Cow\<'a, T\>
   - Cell\<T\>
   - RefCell\<T\>
   - Mutex\<T\>
   - RwLock\<T\>
   - Rc\<T\>&emsp;*(if* features = \["rc"\] *is enabled)*
   - Arc\<T\>&emsp;*(if* features = \["rc"\] *is enabled)*
 - **Collection types**:
   - BTreeMap\<K, V\>
   - BTreeSet\<T\>
   - BinaryHeap\<T\>
   - HashMap\<K, V, H\>
   - HashSet\<T, H\>
   - LinkedList\<T\>
   - VecDeque\<T\>
   - Vec\<T\>
 - **Zero-copy types**:
   - &str
   - &\[u8\]
 - **FFI types**:
   - CString
   - Box\<CStr\>
   - OsString
 - **Miscellaneous standard library types**:
   - Duration
   - SystemTime
   - Path
   - PathBuf
   - Range\<T\>
   - RangeInclusive\<T\>
   - Bound\<T\>
   - num::NonZero*
   - `!` *(unstable)*
 - **Net types**:
   - IpAddr
   - Ipv4Addr
   - Ipv6Addr
   - SocketAddr
   - SocketAddrV4
   - SocketAddrV6

[Implementing `Deserialize`]: https://serde.rs/impl-deserialize.html
[`Deserialize`]: crate::Deserialize
[`Deserializer`]: crate::Deserializer
[`LinkedHashMap<K, V>`]: https://docs.rs/linked-hash-map/*/linked_hash_map/struct.LinkedHashMap.html
[`postcard`]: https://github.com/jamesmunns/postcard
[`linked-hash-map`]: https://crates.io/crates/linked-hash-map
[`serde_derive`]: https://crates.io/crates/serde_derive
[`serde_json`]: https://github.com/serde-rs/json
[`serde_yaml`]: https://github.com/dtolnay/serde-yaml
[derive section of the manual]: https://serde.rs/derive.html
[data formats]: https://serde.rs/#data-formats



## serde_core::forward_to_deserialize_any

*Declarative Macro*

Helper macro when implementing the `Deserializer` part of a new data format
for Serde.

Some [`Deserializer`] implementations for self-describing formats do not
care what hint the [`Visitor`] gives them, they just want to blindly call
the [`Visitor`] method corresponding to the data they can tell is in the
input. This requires repetitive implementations of all the [`Deserializer`]
trait methods.

```edition2021
# use serde::forward_to_deserialize_any;
# use serde::de::{value, Deserializer, Visitor};
#
# struct MyDeserializer;
#
# impl<'de> Deserializer<'de> for MyDeserializer {
#     type Error = value::Error;
#
#     fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
#     where
#         V: Visitor<'de>,
#     {
#         unimplemented!()
#     }
#
#[inline]
fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
where
    V: Visitor<'de>,
{
    self.deserialize_any(visitor)
}
#
#     forward_to_deserialize_any! {
#         i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
#         bytes byte_buf option unit unit_struct newtype_struct seq tuple
#         tuple_struct map struct enum identifier ignored_any
#     }
# }
```

The `forward_to_deserialize_any!` macro implements these simple forwarding
methods so that they forward directly to [`Deserializer::deserialize_any`].
You can choose which methods to forward.

```edition2021
# use serde::forward_to_deserialize_any;
# use serde::de::{value, Deserializer, Visitor};
#
# struct MyDeserializer;
#
impl<'de> Deserializer<'de> for MyDeserializer {
#   type Error = value::Error;
#
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        /* ... */
#       let _ = visitor;
#       unimplemented!()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
```

The macro assumes the convention that your `Deserializer` lifetime parameter
is called `'de` and that the `Visitor` type parameters on each method are
called `V`. A different type parameter and a different lifetime can be
specified explicitly if necessary.

```edition2021
# use serde::forward_to_deserialize_any;
# use serde::de::{value, Deserializer, Visitor};
# use std::marker::PhantomData;
#
# struct MyDeserializer<V>(PhantomData<V>);
#
# impl<'q, V> Deserializer<'q> for MyDeserializer<V> {
#     type Error = value::Error;
#
#     fn deserialize_any<W>(self, visitor: W) -> Result<W::Value, Self::Error>
#     where
#         W: Visitor<'q>,
#     {
#         unimplemented!()
#     }
#
forward_to_deserialize_any! {
    <W: Visitor<'q>>
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
    bytes byte_buf option unit unit_struct newtype_struct seq tuple
    tuple_struct map struct enum identifier ignored_any
}
# }
```

[`Deserializer`]: crate::Deserializer
[`Visitor`]: crate::de::Visitor
[`Deserializer::deserialize_any`]: crate::Deserializer::deserialize_any

```rust
macro_rules! forward_to_deserialize_any {
    (<$visitor:ident: Visitor<$lifetime:tt>> $($func:ident)*) => { ... };
    ($($func:ident)*) => { ... };
}
```



## Module: ser

Generic data structure serialization framework.

The two most important traits in this module are [`Serialize`] and
[`Serializer`].

 - **A type that implements `Serialize` is a data structure** that can be
   serialized to any data format supported by Serde, and conversely
 - **A type that implements `Serializer` is a data format** that can
   serialize any data structure supported by Serde.

# The Serialize trait

Serde provides [`Serialize`] implementations for many Rust primitive and
standard library types. The complete list is below. All of these can be
serialized using Serde out of the box.

Additionally, Serde provides a procedural macro called [`serde_derive`] to
automatically generate [`Serialize`] implementations for structs and enums
in your program. See the [derive section of the manual] for how to use this.

In rare cases it may be necessary to implement [`Serialize`] manually for
some type in your program. See the [Implementing `Serialize`] section of the
manual for more about this.

Third-party crates may provide [`Serialize`] implementations for types that
they expose. For example the [`linked-hash-map`] crate provides a
[`LinkedHashMap<K, V>`] type that is serializable by Serde because the crate
provides an implementation of [`Serialize`] for it.

# The Serializer trait

[`Serializer`] implementations are provided by third-party crates, for
example [`serde_json`], [`serde_yaml`] and [`postcard`].

A partial list of well-maintained formats is given on the [Serde
website][data formats].

# Implementations of Serialize provided by Serde

 - **Primitive types**:
   - bool
   - i8, i16, i32, i64, i128, isize
   - u8, u16, u32, u64, u128, usize
   - f32, f64
   - char
   - str
   - &T and &mut T
 - **Compound types**:
   - \[T\]
   - \[T; 0\] through \[T; 32\]
   - tuples up to size 16
 - **Common standard library types**:
   - String
   - Option\<T\>
   - Result\<T, E\>
   - PhantomData\<T\>
 - **Wrapper types**:
   - Box\<T\>
   - Cow\<'a, T\>
   - Cell\<T\>
   - RefCell\<T\>
   - Mutex\<T\>
   - RwLock\<T\>
   - Rc\<T\>&emsp;*(if* features = \["rc"\] *is enabled)*
   - Arc\<T\>&emsp;*(if* features = \["rc"\] *is enabled)*
 - **Collection types**:
   - BTreeMap\<K, V\>
   - BTreeSet\<T\>
   - BinaryHeap\<T\>
   - HashMap\<K, V, H\>
   - HashSet\<T, H\>
   - LinkedList\<T\>
   - VecDeque\<T\>
   - Vec\<T\>
 - **FFI types**:
   - CStr
   - CString
   - OsStr
   - OsString
 - **Miscellaneous standard library types**:
   - Duration
   - SystemTime
   - Path
   - PathBuf
   - Range\<T\>
   - RangeInclusive\<T\>
   - Bound\<T\>
   - num::NonZero*
   - `!` *(unstable)*
 - **Net types**:
   - IpAddr
   - Ipv4Addr
   - Ipv6Addr
   - SocketAddr
   - SocketAddrV4
   - SocketAddrV6

[Implementing `Serialize`]: https://serde.rs/impl-serialize.html
[`LinkedHashMap<K, V>`]: https://docs.rs/linked-hash-map/*/linked_hash_map/struct.LinkedHashMap.html
[`Serialize`]: crate::Serialize
[`Serializer`]: crate::Serializer
[`postcard`]: https://github.com/jamesmunns/postcard
[`linked-hash-map`]: https://crates.io/crates/linked-hash-map
[`serde_derive`]: https://crates.io/crates/serde_derive
[`serde_json`]: https://github.com/serde-rs/json
[`serde_yaml`]: https://github.com/dtolnay/serde-yaml
[derive section of the manual]: https://serde.rs/derive.html
[data formats]: https://serde.rs/#data-formats



