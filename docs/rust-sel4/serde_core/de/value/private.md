**serde_core > de > value > private**

# Module: de::value::private

## Contents

**Structs**

- [`MapAsEnum`](#mapasenum)
- [`UnitOnly`](#unitonly)

**Traits**

- [`Pair`](#pair) - Avoid having to restate the generic types on `MapDeserializer`. The

---

## serde_core::de::value::private::MapAsEnum

*Struct*

**Generic Parameters:**
- A



## serde_core::de::value::private::Pair

*Trait*

Avoid having to restate the generic types on `MapDeserializer`. The
`Iterator::Item` contains enough information to figure out K and V.

**Methods:**

- `First`
- `Second`
- `split`



## serde_core::de::value::private::UnitOnly

*Struct*

**Generic Parameters:**
- E



