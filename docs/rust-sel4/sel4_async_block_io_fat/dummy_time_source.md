**sel4_async_block_io_fat > dummy_time_source**

# Module: dummy_time_source

## Contents

**Structs**

- [`DummyTimeSource`](#dummytimesource)

---

## sel4_async_block_io_fat::dummy_time_source::DummyTimeSource

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self`

**Trait Implementations:**

- **Default**
  - `fn default() -> DummyTimeSource`
- **TimeSource**
  - `fn get_timestamp(self: &Self) -> fat::Timestamp`



