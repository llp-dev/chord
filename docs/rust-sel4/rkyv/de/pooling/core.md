**rkyv > de > pooling > core**

# Module: de::pooling::core

## Contents

**Structs**

- [`Unpool`](#unpool) - A shared pointer strategy that duplicates deserializations of the same

---

## rkyv::de::pooling::core::Unpool

*Struct*

A shared pointer strategy that duplicates deserializations of the same
shared pointer.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Unpool`
- **Pooling**
  - `fn start_pooling(self: & mut Self, _: usize) -> PoolingState`
  - `fn finish_pooling(self: & mut Self, _: usize, _: ErasedPtr, _: fn(...)) -> Result<(), E>`



