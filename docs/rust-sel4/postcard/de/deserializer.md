**postcard > de > deserializer**

# Module: de::deserializer

## Contents

**Structs**

- [`Deserializer`](#deserializer) - A `serde` compatible deserializer, generic over “Flavors” of deserializing plugins.

---

## postcard::de::deserializer::Deserializer

*Struct*

A `serde` compatible deserializer, generic over “Flavors” of deserializing plugins.

Please note that postcard messages are not self-describing and therefore incompatible with
[internally tagged enums](https://serde.rs/enum-representations.html#internally-tagged).

**Generic Parameters:**
- 'de
- F

**Methods:**

- `fn from_bytes(input: &'de [u8]) -> Self` - Obtain a Deserializer from a slice of bytes
- `fn from_flavor(flavor: F) -> Self` - Obtain a Deserializer from a slice of bytes
- `fn finalize(self: Self) -> Result<<F as >::Remainder>` - Return the remaining (unused) bytes in the Deserializer along with any



