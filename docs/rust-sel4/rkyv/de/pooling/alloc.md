**rkyv > de > pooling > alloc**

# Module: de::pooling::alloc

## Contents

**Structs**

- [`Pool`](#pool) - A shared pointer strategy that pools together deserializations of the same

---

## rkyv::de::pooling::alloc::Pool

*Struct*

A shared pointer strategy that pools together deserializations of the same
shared pointer.

**Methods:**

- `fn new() -> Self` - Creates a new shared pointer unifier.
- `fn with_capacity(capacity: usize) -> Self` - Creates a new shared pointer unifier with initial capacity.

**Trait Implementations:**

- **Pooling**
  - `fn start_pooling(self: & mut Self, address: usize) -> PoolingState`
  - `fn finish_pooling(self: & mut Self, address: usize, ptr: ErasedPtr, drop: fn(...)) -> Result<(), E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Pool`



