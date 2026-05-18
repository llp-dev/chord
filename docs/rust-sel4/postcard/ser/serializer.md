**postcard > ser > serializer**

# Module: ser::serializer

## Contents

**Structs**

- [`Serializer`](#serializer) - A `serde` compatible serializer, generic over "Flavors" of serializing plugins.

---

## postcard::ser::serializer::Serializer

*Struct*

A `serde` compatible serializer, generic over "Flavors" of serializing plugins.

It should rarely be necessary to directly use this type unless you are implementing your
own [`SerFlavor`].

See the docs for [`SerFlavor`] for more information about "flavors" of serialization

[`SerFlavor`]: crate::ser_flavors::Flavor

**Generic Parameters:**
- F

**Fields:**
- `output: F` - This is the Flavor(s) that will be used to modify or store any bytes generated



