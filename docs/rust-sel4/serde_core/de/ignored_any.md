**serde_core > de > ignored_any**

# Module: de::ignored_any

## Contents

**Structs**

- [`IgnoredAny`](#ignoredany) - An efficient way of discarding data from a deserializer.

---

## serde_core::de::ignored_any::IgnoredAny

*Struct*

An efficient way of discarding data from a deserializer.

Think of this like `serde_json::Value` in that it can be deserialized from
any type, except that it does not store any information about the data that
gets deserialized.

```edition2021
use serde::de::{
    self, Deserialize, DeserializeSeed, Deserializer, IgnoredAny, SeqAccess, Visitor,
};
use std::fmt;
use std::marker::PhantomData;

/// A seed that can be used to deserialize only the `n`th element of a sequence
/// while efficiently discarding elements of any type before or after index `n`.
///
/// For example to deserialize only the element at index 3:
///
/// ```
/// NthElement::new(3).deserialize(deserializer)
/// ```
pub struct NthElement<T> {
    n: usize,
    marker: PhantomData<T>,
}

impl<T> NthElement<T> {
    pub fn new(n: usize) -> Self {
        NthElement {
            n: n,
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for NthElement<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a sequence in which we care about element {}",
            self.n
        )
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // Skip over the first `n` elements.
        for i in 0..self.n {
            // It is an error if the sequence ends before we get to element `n`.
            if seq.next_element::<IgnoredAny>()?.is_none() {
                return Err(de::Error::invalid_length(i, &self));
            }
        }

        // Deserialize the one we care about.
        let nth = match seq.next_element()? {
            Some(nth) => nth,
            None => {
                return Err(de::Error::invalid_length(self.n, &self));
            }
        };

        // Skip over any remaining elements in the sequence after `n`.
        while let Some(IgnoredAny) = seq.next_element()? {
            // ignore
        }

        Ok(nth)
    }
}

impl<'de, T> DeserializeSeed<'de> for NthElement<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

# fn example<'de, D>(deserializer: D) -> Result<(), D::Error>
# where
#     D: Deserializer<'de>,
# {
// Deserialize only the sequence element at index 3 from this deserializer.
// The element at index 3 is required to be a string. Elements before and
// after index 3 are allowed to be of any type.
let s: String = NthElement::new(3).deserialize(deserializer)?;
#     Ok(())
# }
```

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<IgnoredAny, <D as >::Error>`
- **Default**
  - `fn default() -> IgnoredAny`
- **Visitor**
  - `fn expecting(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
  - `fn visit_bool<E>(self: Self, x: bool) -> Result<<Self as >::Value, E>`
  - `fn visit_i64<E>(self: Self, x: i64) -> Result<<Self as >::Value, E>`
  - `fn visit_i128<E>(self: Self, x: i128) -> Result<<Self as >::Value, E>`
  - `fn visit_u64<E>(self: Self, x: u64) -> Result<<Self as >::Value, E>`
  - `fn visit_u128<E>(self: Self, x: u128) -> Result<<Self as >::Value, E>`
  - `fn visit_f64<E>(self: Self, x: f64) -> Result<<Self as >::Value, E>`
  - `fn visit_str<E>(self: Self, s: &str) -> Result<<Self as >::Value, E>`
  - `fn visit_none<E>(self: Self) -> Result<<Self as >::Value, E>`
  - `fn visit_some<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`
  - `fn visit_newtype_struct<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`
  - `fn visit_unit<E>(self: Self) -> Result<<Self as >::Value, E>`
  - `fn visit_seq<A>(self: Self, seq: A) -> Result<<Self as >::Value, <A as >::Error>`
  - `fn visit_map<A>(self: Self, map: A) -> Result<<Self as >::Value, <A as >::Error>`
  - `fn visit_bytes<E>(self: Self, bytes: &[u8]) -> Result<<Self as >::Value, E>`
  - `fn visit_enum<A>(self: Self, data: A) -> Result<<Self as >::Value, <A as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IgnoredAny) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> IgnoredAny`



