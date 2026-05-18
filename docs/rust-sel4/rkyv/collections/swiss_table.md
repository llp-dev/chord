**rkyv > collections > swiss_table**

# Module: collections::swiss_table

## Contents

**Modules**

- [`index_map`](#index_map) - An archived index map implementation based on Google's high-performance
- [`index_set`](#index_set) - An archived index set implementation based on Google's high-performance
- [`map`](#map) - Archived hash map implementation using an archived SwissTable.
- [`set`](#set) - Archived hash set implementation using an archived SwissTable.
- [`table`](#table) - An archived hash table implementation based on Google's high-performance

---

## Module: index_map

An archived index map implementation based on Google's high-performance
SwissTable hash map.



## Module: index_set

An archived index set implementation based on Google's high-performance
SwissTable hash map.



## Module: map

Archived hash map implementation using an archived SwissTable.



## Module: set

Archived hash set implementation using an archived SwissTable.



## Module: table

An archived hash table implementation based on Google's high-performance
SwissTable hash map.

Notable differences from other implementations:

- The number of control bytes is rounded up to a maximum group width (16)
  instead of the next power of two. This reduces the number of empty buckets
  on the wire. Since this collection is immutable after writing, we'll never
  benefit from having more buckets than we need.
- Because the bucket count is not a power of two, the triangular probing
  sequence simply skips any indices larger than the actual size of the
  buckets array.
- Instead of the final control bytes always being marked EMPTY, the last
  control bytes repeat the first few. This helps reduce the number of
  lookups when probing at the end of the control bytes.
- Because the available SIMD group width may be less than the maximum group
  width, each probe reads N groups before striding where N is the maximum
  group width divided by the SIMD group width.



