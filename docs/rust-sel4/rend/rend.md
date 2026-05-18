**rend**

# Module: rend

## Contents

**Modules**

- [`unaligned`](#unaligned) - Cross-platform primitives with unaligned representations.

**Structs**

- [`AtomicI16_be`](#atomici16_be) - A big-endian `AtomicI16` with a guaranteed size and alignment of `2`.
- [`AtomicI16_le`](#atomici16_le) - A little-endian `AtomicI16` with a guaranteed size and alignment of `2`.
- [`AtomicI32_be`](#atomici32_be) - A big-endian `AtomicI32` with a guaranteed size and alignment of `4`.
- [`AtomicI32_le`](#atomici32_le) - A little-endian `AtomicI32` with a guaranteed size and alignment of `4`.
- [`AtomicI64_be`](#atomici64_be) - A big-endian `AtomicI64` with a guaranteed size and alignment of `8`.
- [`AtomicI64_le`](#atomici64_le) - A little-endian `AtomicI64` with a guaranteed size and alignment of `8`.
- [`AtomicU16_be`](#atomicu16_be) - A big-endian `AtomicU16` with a guaranteed size and alignment of `2`.
- [`AtomicU16_le`](#atomicu16_le) - A little-endian `AtomicU16` with a guaranteed size and alignment of `2`.
- [`AtomicU32_be`](#atomicu32_be) - A big-endian `AtomicU32` with a guaranteed size and alignment of `4`.
- [`AtomicU32_le`](#atomicu32_le) - A little-endian `AtomicU32` with a guaranteed size and alignment of `4`.
- [`AtomicU64_be`](#atomicu64_be) - A big-endian `AtomicU64` with a guaranteed size and alignment of `8`.
- [`AtomicU64_le`](#atomicu64_le) - A little-endian `AtomicU64` with a guaranteed size and alignment of `8`.
- [`NonZeroI128_be`](#nonzeroi128_be) - A big-endian `NonZeroI128` with a guaranteed size and alignment of `16`.
- [`NonZeroI128_le`](#nonzeroi128_le) - A little-endian `NonZeroI128` with a guaranteed size and alignment of `16`.
- [`NonZeroI16_be`](#nonzeroi16_be) - A big-endian `NonZeroI16` with a guaranteed size and alignment of `2`.
- [`NonZeroI16_le`](#nonzeroi16_le) - A little-endian `NonZeroI16` with a guaranteed size and alignment of `2`.
- [`NonZeroI32_be`](#nonzeroi32_be) - A big-endian `NonZeroI32` with a guaranteed size and alignment of `4`.
- [`NonZeroI32_le`](#nonzeroi32_le) - A little-endian `NonZeroI32` with a guaranteed size and alignment of `4`.
- [`NonZeroI64_be`](#nonzeroi64_be) - A big-endian `NonZeroI64` with a guaranteed size and alignment of `8`.
- [`NonZeroI64_le`](#nonzeroi64_le) - A little-endian `NonZeroI64` with a guaranteed size and alignment of `8`.
- [`NonZeroU128_be`](#nonzerou128_be) - A big-endian `NonZeroU128` with a guaranteed size and alignment of `16`.
- [`NonZeroU128_le`](#nonzerou128_le) - A little-endian `NonZeroU128` with a guaranteed size and alignment of `16`.
- [`NonZeroU16_be`](#nonzerou16_be) - A big-endian `NonZeroU16` with a guaranteed size and alignment of `2`.
- [`NonZeroU16_le`](#nonzerou16_le) - A little-endian `NonZeroU16` with a guaranteed size and alignment of `2`.
- [`NonZeroU32_be`](#nonzerou32_be) - A big-endian `NonZeroU32` with a guaranteed size and alignment of `4`.
- [`NonZeroU32_le`](#nonzerou32_le) - A little-endian `NonZeroU32` with a guaranteed size and alignment of `4`.
- [`NonZeroU64_be`](#nonzerou64_be) - A big-endian `NonZeroU64` with a guaranteed size and alignment of `8`.
- [`NonZeroU64_le`](#nonzerou64_le) - A little-endian `NonZeroU64` with a guaranteed size and alignment of `8`.
- [`char_be`](#char_be) - A big-endian `u32` with a guaranteed size and alignment of `4`.
- [`char_le`](#char_le) - A little-endian `u32` with a guaranteed size and alignment of `4`.
- [`f32_be`](#f32_be) - A big-endian `f32` with a guaranteed size and alignment of `4`.
- [`f32_le`](#f32_le) - A little-endian `f32` with a guaranteed size and alignment of `4`.
- [`f64_be`](#f64_be) - A big-endian `f64` with a guaranteed size and alignment of `8`.
- [`f64_le`](#f64_le) - A little-endian `f64` with a guaranteed size and alignment of `8`.
- [`i128_be`](#i128_be) - A big-endian `i128` with a guaranteed size and alignment of `16`.
- [`i128_le`](#i128_le) - A little-endian `i128` with a guaranteed size and alignment of `16`.
- [`i16_be`](#i16_be) - A big-endian `i16` with a guaranteed size and alignment of `2`.
- [`i16_le`](#i16_le) - A little-endian `i16` with a guaranteed size and alignment of `2`.
- [`i32_be`](#i32_be) - A big-endian `i32` with a guaranteed size and alignment of `4`.
- [`i32_le`](#i32_le) - A little-endian `i32` with a guaranteed size and alignment of `4`.
- [`i64_be`](#i64_be) - A big-endian `i64` with a guaranteed size and alignment of `8`.
- [`i64_le`](#i64_le) - A little-endian `i64` with a guaranteed size and alignment of `8`.
- [`u128_be`](#u128_be) - A big-endian `u128` with a guaranteed size and alignment of `16`.
- [`u128_le`](#u128_le) - A little-endian `u128` with a guaranteed size and alignment of `16`.
- [`u16_be`](#u16_be) - A big-endian `u16` with a guaranteed size and alignment of `2`.
- [`u16_le`](#u16_le) - A little-endian `u16` with a guaranteed size and alignment of `2`.
- [`u32_be`](#u32_be) - A big-endian `u32` with a guaranteed size and alignment of `4`.
- [`u32_le`](#u32_le) - A little-endian `u32` with a guaranteed size and alignment of `4`.
- [`u64_be`](#u64_be) - A big-endian `u64` with a guaranteed size and alignment of `8`.
- [`u64_le`](#u64_le) - A little-endian `u64` with a guaranteed size and alignment of `8`.

---

## rend::AtomicI16_be

*Struct*

A big-endian `AtomicI16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn compare_exchange(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: i16, order: Ordering) -> i16` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: i16, order: Ordering) -> i16` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: i16, order: Ordering) -> i16` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: i16, order: Ordering) -> i16` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: i16, order: Ordering) -> i16` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: i16, order: Ordering) -> i16` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: i16, order: Ordering) -> i16` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i16, i16>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: i16, order: Ordering) -> i16` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> i16` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> i16` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: i16, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: i16, order: Ordering) -> i16` - Stores a value into the atomic integer, returning the previous
- `fn new(value: i16) -> Self` - Returns a `AtomicI16_be` containing `value`.

**Trait Implementations:**

- **From**
  - `fn from(value: i16) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`



## rend::AtomicI16_le

*Struct*

A little-endian `AtomicI16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i16) -> Self` - Returns a `AtomicI16_le` containing `value`.
- `fn compare_exchange(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: i16, order: Ordering) -> i16` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: i16, order: Ordering) -> i16` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: i16, order: Ordering) -> i16` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: i16, order: Ordering) -> i16` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: i16, order: Ordering) -> i16` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: i16, order: Ordering) -> i16` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: i16, order: Ordering) -> i16` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i16, i16>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: i16, order: Ordering) -> i16` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> i16` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> i16` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: i16, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: i16, order: Ordering) -> i16` - Stores a value into the atomic integer, returning the previous

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: i16) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`



## rend::AtomicI32_be

*Struct*

A big-endian `AtomicI32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn compare_exchange(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: i32, order: Ordering) -> i32` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: i32, order: Ordering) -> i32` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: i32, order: Ordering) -> i32` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: i32, order: Ordering) -> i32` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: i32, order: Ordering) -> i32` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: i32, order: Ordering) -> i32` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: i32, order: Ordering) -> i32` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i32, i32>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: i32, order: Ordering) -> i32` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> i32` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> i32` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: i32, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: i32, order: Ordering) -> i32` - Stores a value into the atomic integer, returning the previous
- `fn new(value: i32) -> Self` - Returns a `AtomicI32_be` containing `value`.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: i32) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## rend::AtomicI32_le

*Struct*

A little-endian `AtomicI32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn compare_exchange(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: i32, order: Ordering) -> i32` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: i32, order: Ordering) -> i32` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: i32, order: Ordering) -> i32` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: i32, order: Ordering) -> i32` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: i32, order: Ordering) -> i32` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: i32, order: Ordering) -> i32` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: i32, order: Ordering) -> i32` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i32, i32>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: i32, order: Ordering) -> i32` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> i32` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> i32` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: i32, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: i32, order: Ordering) -> i32` - Stores a value into the atomic integer, returning the previous
- `fn new(value: i32) -> Self` - Returns a `AtomicI32_le` containing `value`.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: i32) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## rend::AtomicI64_be

*Struct*

A big-endian `AtomicI64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn compare_exchange(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: i64, order: Ordering) -> i64` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: i64, order: Ordering) -> i64` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: i64, order: Ordering) -> i64` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: i64, order: Ordering) -> i64` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: i64, order: Ordering) -> i64` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: i64, order: Ordering) -> i64` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: i64, order: Ordering) -> i64` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i64, i64>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: i64, order: Ordering) -> i64` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> i64` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> i64` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: i64, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: i64, order: Ordering) -> i64` - Stores a value into the atomic integer, returning the previous
- `fn new(value: i64) -> Self` - Returns a `AtomicI64_be` containing `value`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: i64) -> Self`



## rend::AtomicI64_le

*Struct*

A little-endian `AtomicI64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn compare_exchange(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: i64, order: Ordering) -> i64` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: i64, order: Ordering) -> i64` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: i64, order: Ordering) -> i64` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: i64, order: Ordering) -> i64` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: i64, order: Ordering) -> i64` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: i64, order: Ordering) -> i64` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: i64, order: Ordering) -> i64` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i64, i64>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: i64, order: Ordering) -> i64` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> i64` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> i64` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: i64, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: i64, order: Ordering) -> i64` - Stores a value into the atomic integer, returning the previous
- `fn new(value: i64) -> Self` - Returns a `AtomicI64_le` containing `value`.

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: i64) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`



## rend::AtomicU16_be

*Struct*

A big-endian `AtomicU16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn compare_exchange(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: u16, order: Ordering) -> u16` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: u16, order: Ordering) -> u16` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: u16, order: Ordering) -> u16` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: u16, order: Ordering) -> u16` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: u16, order: Ordering) -> u16` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: u16, order: Ordering) -> u16` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: u16, order: Ordering) -> u16` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u16, u16>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: u16, order: Ordering) -> u16` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> u16` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> u16` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: u16, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: u16, order: Ordering) -> u16` - Stores a value into the atomic integer, returning the previous
- `fn new(value: u16) -> Self` - Returns a `AtomicU16_be` containing `value`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: u16) -> Self`



## rend::AtomicU16_le

*Struct*

A little-endian `AtomicU16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn compare_exchange(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: u16, order: Ordering) -> u16` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: u16, order: Ordering) -> u16` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: u16, order: Ordering) -> u16` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: u16, order: Ordering) -> u16` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: u16, order: Ordering) -> u16` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: u16, order: Ordering) -> u16` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: u16, order: Ordering) -> u16` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u16, u16>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: u16, order: Ordering) -> u16` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> u16` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> u16` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: u16, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: u16, order: Ordering) -> u16` - Stores a value into the atomic integer, returning the previous
- `fn new(value: u16) -> Self` - Returns a `AtomicU16_le` containing `value`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: u16) -> Self`



## rend::AtomicU32_be

*Struct*

A big-endian `AtomicU32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u32) -> Self` - Returns a `AtomicU32_be` containing `value`.
- `fn compare_exchange(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: u32, order: Ordering) -> u32` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: u32, order: Ordering) -> u32` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: u32, order: Ordering) -> u32` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: u32, order: Ordering) -> u32` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: u32, order: Ordering) -> u32` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: u32, order: Ordering) -> u32` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: u32, order: Ordering) -> u32` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u32, u32>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: u32, order: Ordering) -> u32` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> u32` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> u32` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: u32, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: u32, order: Ordering) -> u32` - Stores a value into the atomic integer, returning the previous

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: u32) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`



## rend::AtomicU32_le

*Struct*

A little-endian `AtomicU32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u32) -> Self` - Returns a `AtomicU32_le` containing `value`.
- `fn compare_exchange(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: u32, order: Ordering) -> u32` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: u32, order: Ordering) -> u32` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: u32, order: Ordering) -> u32` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: u32, order: Ordering) -> u32` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: u32, order: Ordering) -> u32` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: u32, order: Ordering) -> u32` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: u32, order: Ordering) -> u32` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u32, u32>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: u32, order: Ordering) -> u32` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> u32` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> u32` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: u32, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: u32, order: Ordering) -> u32` - Stores a value into the atomic integer, returning the previous

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: u32) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`



## rend::AtomicU64_be

*Struct*

A big-endian `AtomicU64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn compare_exchange(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: u64, order: Ordering) -> u64` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: u64, order: Ordering) -> u64` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: u64, order: Ordering) -> u64` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: u64, order: Ordering) -> u64` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: u64, order: Ordering) -> u64` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: u64, order: Ordering) -> u64` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: u64, order: Ordering) -> u64` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u64, u64>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: u64, order: Ordering) -> u64` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> u64` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> u64` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: u64, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: u64, order: Ordering) -> u64` - Stores a value into the atomic integer, returning the previous
- `fn new(value: u64) -> Self` - Returns a `AtomicU64_be` containing `value`.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: u64) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## rend::AtomicU64_le

*Struct*

A little-endian `AtomicU64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn compare_exchange(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>` - Stores a value into the atomic integer if the current value is
- `fn compare_exchange_weak(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>` - Stores a value into the atomic integer if the current value is
- `fn fetch_add(self: &Self, val: u64, order: Ordering) -> u64` - Adds to the current value, returning the previous value.
- `fn fetch_and(self: &Self, val: u64, order: Ordering) -> u64` - Bitwise "and" with the current value.
- `fn fetch_max(self: &Self, val: u64, order: Ordering) -> u64` - Maximum with the current value.
- `fn fetch_min(self: &Self, val: u64, order: Ordering) -> u64` - Minimum with the current value.
- `fn fetch_nand(self: &Self, val: u64, order: Ordering) -> u64` - Bitwise "nand" with the current value.
- `fn fetch_or(self: &Self, val: u64, order: Ordering) -> u64` - Bitwise "or" with the current value.
- `fn fetch_sub(self: &Self, val: u64, order: Ordering) -> u64` - Subtracts from the current value, returning the previous value.
- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u64, u64>` - Fetches the value, and applies a function to it that returns an
- `fn fetch_xor(self: &Self, val: u64, order: Ordering) -> u64` - Bitwise "xor" with the current value.
- `fn into_inner(self: Self) -> u64` - Consumes the atomic and returns the contained value.
- `fn load(self: &Self, order: Ordering) -> u64` - Loads a value from the atomic integer.
- `fn store(self: &Self, val: u64, order: Ordering)` - Stores a value into the atomic integer.
- `fn swap(self: &Self, val: u64, order: Ordering) -> u64` - Stores a value into the atomic integer, returning the previous
- `fn new(value: u64) -> Self` - Returns a `AtomicU64_le` containing `value`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **From**
  - `fn from(value: u64) -> Self`



## rend::NonZeroI128_be

*Struct*

A big-endian `NonZeroI128` with a guaranteed size and alignment of `16`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i128) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i128) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i128` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI128) -> Self` - Returns a `NonZeroI128_be` containing `value`.
- `fn to_native(self: Self) -> NonZeroI128` - Returns a `NonZeroI128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI128) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI128) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI128_be)`
- **From**
  - `fn from(value: NonZeroI128) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI128_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI128_be) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI128) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`



## rend::NonZeroI128_le

*Struct*

A little-endian `NonZeroI128` with a guaranteed size and alignment of `16`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i128) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i128) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i128` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI128) -> Self` - Returns a `NonZeroI128_le` containing `value`.
- `fn to_native(self: Self) -> NonZeroI128` - Returns a `NonZeroI128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI128) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI128_le)`
- **From**
  - `fn from(value: NonZeroI128) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI128_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI128_le) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI128) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI128) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`



## rend::NonZeroI16_be

*Struct*

A big-endian `NonZeroI16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i16) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i16) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i16` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI16) -> Self` - Returns a `NonZeroI16_be` containing `value`.
- `fn to_native(self: Self) -> NonZeroI16` - Returns a `NonZeroI16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI16_be)`
- **From**
  - `fn from(value: NonZeroI16) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI16_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI16_be) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI16) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI16) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI16) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::NonZeroI16_le

*Struct*

A little-endian `NonZeroI16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i16) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i16) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i16` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI16) -> Self` - Returns a `NonZeroI16_le` containing `value`.
- `fn to_native(self: Self) -> NonZeroI16` - Returns a `NonZeroI16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI16_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI16_le) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI16) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI16) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI16) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI16_le)`
- **From**
  - `fn from(value: NonZeroI16) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::NonZeroI32_be

*Struct*

A big-endian `NonZeroI32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i32) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i32) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i32` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI32) -> Self` - Returns a `NonZeroI32_be` containing `value`.
- `fn to_native(self: Self) -> NonZeroI32` - Returns a `NonZeroI32` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI32_be)`
- **From**
  - `fn from(value: NonZeroI32) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI32_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI32_be) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI32) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI32) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI32) -> bool`



## rend::NonZeroI32_le

*Struct*

A little-endian `NonZeroI32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i32) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i32) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i32` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI32) -> Self` - Returns a `NonZeroI32_le` containing `value`.
- `fn to_native(self: Self) -> NonZeroI32` - Returns a `NonZeroI32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI32_le)`
- **From**
  - `fn from(value: NonZeroI32) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI32_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI32_le) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI32) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI32) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI32) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::NonZeroI64_be

*Struct*

A big-endian `NonZeroI64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i64) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i64) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i64` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI64) -> Self` - Returns a `NonZeroI64_be` containing `value`.
- `fn to_native(self: Self) -> NonZeroI64` - Returns a `NonZeroI64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI64) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI64_be)`
- **From**
  - `fn from(value: NonZeroI64) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI64_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI64_be) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI64) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI64) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`



## rend::NonZeroI64_le

*Struct*

A little-endian `NonZeroI64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i64) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i64) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i64` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI64) -> Self` - Returns a `NonZeroI64_le` containing `value`.
- `fn to_native(self: Self) -> NonZeroI64` - Returns a `NonZeroI64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI64) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI64_le)`
- **From**
  - `fn from(value: NonZeroI64) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI64_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI64_le) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI64) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI64) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::NonZeroU128_be

*Struct*

A big-endian `NonZeroU128` with a guaranteed size and alignment of `16`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u128) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u128) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u128` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU128) -> Self` - Returns a `NonZeroU128_be` containing `value`.
- `fn to_native(self: Self) -> NonZeroU128` - Returns a `NonZeroU128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU128) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU128_be)`
- **From**
  - `fn from(value: NonZeroU128) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU128_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU128_be) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU128) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU128) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`



## rend::NonZeroU128_le

*Struct*

A little-endian `NonZeroU128` with a guaranteed size and alignment of `16`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u128) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u128) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u128` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU128) -> Self` - Returns a `NonZeroU128_le` containing `value`.
- `fn to_native(self: Self) -> NonZeroU128` - Returns a `NonZeroU128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU128) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU128_le)`
- **From**
  - `fn from(value: NonZeroU128) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU128_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU128_le) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU128) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU128) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::NonZeroU16_be

*Struct*

A big-endian `NonZeroU16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u16) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u16) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u16` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU16) -> Self` - Returns a `NonZeroU16_be` containing `value`.
- `fn to_native(self: Self) -> NonZeroU16` - Returns a `NonZeroU16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU16_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU16_be) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU16) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU16) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU16) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU16_be)`
- **From**
  - `fn from(value: NonZeroU16) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::NonZeroU16_le

*Struct*

A little-endian `NonZeroU16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u16) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u16) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u16` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU16) -> Self` - Returns a `NonZeroU16_le` containing `value`.
- `fn to_native(self: Self) -> NonZeroU16` - Returns a `NonZeroU16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU16) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU16) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU16) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU16_le)`
- **From**
  - `fn from(value: NonZeroU16) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU16_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU16_le) -> <Self as >::Output`



## rend::NonZeroU32_be

*Struct*

A big-endian `NonZeroU32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u32) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u32) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u32` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU32) -> Self` - Returns a `NonZeroU32_be` containing `value`.
- `fn to_native(self: Self) -> NonZeroU32` - Returns a `NonZeroU32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(value: NonZeroU32) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU32_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU32_be) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU32) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU32) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU32) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU32_be)`



## rend::NonZeroU32_le

*Struct*

A little-endian `NonZeroU32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u32) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u32) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u32` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU32) -> Self` - Returns a `NonZeroU32_le` containing `value`.
- `fn to_native(self: Self) -> NonZeroU32` - Returns a `NonZeroU32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU32_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU32_le) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU32) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU32) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU32) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU32_le)`
- **From**
  - `fn from(value: NonZeroU32) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::NonZeroU64_be

*Struct*

A big-endian `NonZeroU64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u64) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u64) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u64` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU64) -> Self` - Returns a `NonZeroU64_be` containing `value`.
- `fn to_native(self: Self) -> NonZeroU64` - Returns a `NonZeroU64` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU64_be)`
- **From**
  - `fn from(value: NonZeroU64) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU64_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU64_be) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU64) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU64) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU64) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::NonZeroU64_le

*Struct*

A little-endian `NonZeroU64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u64) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u64) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u64` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU64) -> Self` - Returns a `NonZeroU64_le` containing `value`.
- `fn to_native(self: Self) -> NonZeroU64` - Returns a `NonZeroU64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU64_le)`
- **From**
  - `fn from(value: NonZeroU64) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU64_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU64_le) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU64) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU64) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU64) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::char_be

*Struct*

A big-endian `u32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: char) -> Self` - Returns a `char_be` containing `value`.
- `fn to_native(self: Self) -> char` - Returns a `$prim` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(value: &'a char) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &char) -> Option<::core::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: char) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &char) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::char_le

*Struct*

A little-endian `u32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: char) -> Self` - Returns a `char_le` containing `value`.
- `fn to_native(self: Self) -> char` - Returns a `$prim` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &char) -> Option<::core::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: char) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &char) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(value: &'a char) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`



## rend::f32_be

*Struct*

A big-endian `f32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: f32) -> Self` - Returns a `f32_be` containing `value`.
- `fn to_native(self: Self) -> f32` - Returns a `f32` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f32_be)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f32_be)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **From**
  - `fn from(value: f32) -> Self`
- **Rem**
  - `fn rem(self: Self, other: f32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f32)`
- **Rem**
  - `fn rem(self: Self, other: &f32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f32_be)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f32_be)`
- **Rem**
  - `fn rem(self: Self, other: f32_be) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Div**
  - `fn div(self: Self, other: f32) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: &f32_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f32)`
- **Div**
  - `fn div(self: Self, other: &f32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f32_be)`
- **Div**
  - `fn div(self: Self, other: f32_be) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Div**
  - `fn div(self: Self, other: &f32_be) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &f32) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f32)`
- **From**
  - `fn from(value: &'a f32) -> Self`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f32_be)`
- **Mul**
  - `fn mul(self: Self, other: f32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f32_be)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &f32) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: f32_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f32)`
- **Mul**
  - `fn mul(self: Self, other: &f32_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f32_be)`
- **Sub**
  - `fn sub(self: Self, other: f32) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: f32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f32)`
- **Sub**
  - `fn sub(self: Self, other: &f32) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &f32) -> bool`
- **Add**
  - `fn add(self: Self, other: &f32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f32)`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Sub**
  - `fn sub(self: Self, other: f32_be) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f32_be)`
- **Add**
  - `fn add(self: Self, other: f32_be) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &f32_be) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &f32_be) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f32)`
- **Default**
  - `fn default() -> Self`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f32_be)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f32)`



## rend::f32_le

*Struct*

A little-endian `f32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: f32) -> Self` - Returns a `f32_le` containing `value`.
- `fn to_native(self: Self) -> f32` - Returns a `f32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Sub**
  - `fn sub(self: Self, other: f32) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: f32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f32)`
- **Sub**
  - `fn sub(self: Self, other: &f32) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &f32) -> bool`
- **Add**
  - `fn add(self: Self, other: &f32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f32)`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Sub**
  - `fn sub(self: Self, other: f32_le) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f32_le)`
- **Add**
  - `fn add(self: Self, other: f32_le) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &f32_le) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &f32_le) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f32)`
- **Default**
  - `fn default() -> Self`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f32_le)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f32)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f32_le)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f32_le)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **From**
  - `fn from(value: f32) -> Self`
- **Rem**
  - `fn rem(self: Self, other: f32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f32)`
- **Rem**
  - `fn rem(self: Self, other: &f32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f32_le)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f32_le)`
- **Rem**
  - `fn rem(self: Self, other: f32_le) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Div**
  - `fn div(self: Self, other: f32) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: &f32_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f32)`
- **Div**
  - `fn div(self: Self, other: &f32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f32_le)`
- **Div**
  - `fn div(self: Self, other: f32_le) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Div**
  - `fn div(self: Self, other: &f32_le) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &f32) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f32)`
- **From**
  - `fn from(value: &'a f32) -> Self`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f32_le)`
- **Mul**
  - `fn mul(self: Self, other: f32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f32_le)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &f32) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: f32_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f32)`
- **Mul**
  - `fn mul(self: Self, other: &f32_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f32_le)`



## rend::f64_be

*Struct*

A big-endian `f64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: f64) -> Self` - Returns a `f64_be` containing `value`.
- `fn to_native(self: Self) -> f64` - Returns a `f64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f64)`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Sub**
  - `fn sub(self: Self, other: f64_be) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f64_be)`
- **Add**
  - `fn add(self: Self, other: f64_be) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &f64_be) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &f64_be) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f64)`
- **Default**
  - `fn default() -> Self`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f64_be)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f64)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f64_be)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f64_be)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **From**
  - `fn from(value: f64) -> Self`
- **Rem**
  - `fn rem(self: Self, other: f64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f64)`
- **Rem**
  - `fn rem(self: Self, other: &f64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f64_be)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f64_be)`
- **Rem**
  - `fn rem(self: Self, other: f64_be) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Div**
  - `fn div(self: Self, other: f64) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: &f64_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f64)`
- **Div**
  - `fn div(self: Self, other: &f64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f64_be)`
- **Div**
  - `fn div(self: Self, other: f64_be) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Div**
  - `fn div(self: Self, other: &f64_be) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &f64) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f64)`
- **From**
  - `fn from(value: &'a f64) -> Self`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f64_be)`
- **Mul**
  - `fn mul(self: Self, other: f64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f64_be)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &f64) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: f64_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f64)`
- **Mul**
  - `fn mul(self: Self, other: &f64_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f64_be)`
- **Sub**
  - `fn sub(self: Self, other: f64) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: f64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f64)`
- **Sub**
  - `fn sub(self: Self, other: &f64) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &f64) -> bool`
- **Add**
  - `fn add(self: Self, other: &f64) -> <Self as >::Output`



## rend::f64_le

*Struct*

A little-endian `f64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: f64) -> Self` - Returns a `f64_le` containing `value`.
- `fn to_native(self: Self) -> f64` - Returns a `f64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Div**
  - `fn div(self: Self, other: &f64_le) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &f64) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f64)`
- **From**
  - `fn from(value: &'a f64) -> Self`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f64_le)`
- **Mul**
  - `fn mul(self: Self, other: f64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f64_le)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &f64) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: f64_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f64)`
- **Mul**
  - `fn mul(self: Self, other: &f64_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f64_le)`
- **Sub**
  - `fn sub(self: Self, other: f64) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: f64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f64)`
- **Sub**
  - `fn sub(self: Self, other: &f64) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &f64) -> bool`
- **Add**
  - `fn add(self: Self, other: &f64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f64)`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Sub**
  - `fn sub(self: Self, other: f64_le) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f64_le)`
- **Add**
  - `fn add(self: Self, other: f64_le) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &f64_le) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &f64_le) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f64)`
- **Default**
  - `fn default() -> Self`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f64_le)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f64)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f64_le)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f64_le)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **From**
  - `fn from(value: f64) -> Self`
- **Rem**
  - `fn rem(self: Self, other: f64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f64)`
- **Rem**
  - `fn rem(self: Self, other: &f64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f64_le)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f64_le)`
- **Rem**
  - `fn rem(self: Self, other: f64_le) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Div**
  - `fn div(self: Self, other: f64) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: &f64_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f64)`
- **Div**
  - `fn div(self: Self, other: &f64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f64_le)`
- **Div**
  - `fn div(self: Self, other: f64_le) -> <Self as >::Output`



## rend::i128_be

*Struct*

A big-endian `i128` with a guaranteed size and alignment of `16`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i128) -> Self` - Returns a `i128_be` containing `value`.
- `fn to_native(self: Self) -> i128` - Returns a `i128` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i128_be)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i128_be)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i128)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i128_be)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i128_be)`
- **Shl**
  - `fn shl(self: Self, other: i128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i128_be)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i128)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i128)`
- **From**
  - `fn from(value: i128) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i128_be)`
- **Sub**
  - `fn sub(self: Self, other: i128) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i128_be)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i128) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i128_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i128)`
- **Sub**
  - `fn sub(self: Self, other: &i128) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i128_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i128_be)`
- **Mul**
  - `fn mul(self: Self, other: i128) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i128)`
- **Sub**
  - `fn sub(self: Self, other: i128_be) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i128_be)`
- **Div**
  - `fn div(self: Self, other: i128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i128_be) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i128) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i128_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i128_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i128)`
- **Mul**
  - `fn mul(self: Self, other: i128_be) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i128)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i128_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i128_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i128_be)`
- **Div**
  - `fn div(self: Self, other: &i128_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i128)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i128_be)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i128_be)`
- **From**
  - `fn from(value: &'a i128) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i128) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i128_be)`
- **Rem**
  - `fn rem(self: Self, other: i128) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i128)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i128_be)`
- **Rem**
  - `fn rem(self: Self, other: &i128) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i128_be)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i128) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i128)`
- **Rem**
  - `fn rem(self: Self, other: i128_be) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i128)`
- **Shr**
  - `fn shr(self: Self, other: &i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i128_be)`
- **Rem**
  - `fn rem(self: Self, other: &i128_be) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i128) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i128_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: i128) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i128)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i128)`
- **Shr**
  - `fn shr(self: Self, other: i128_be) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i128) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i128_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i128_be)`
- **Shr**
  - `fn shr(self: Self, other: &i128_be) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i128)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i128_be) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i128_be)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i128)`
- **Add**
  - `fn add(self: Self, other: &i128) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i128) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i128)`
- **BitXor**
  - `fn bitxor(self: Self, other: i128_be) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i128_be) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i128_be) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i128_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i128)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i128_be)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i128)`



## rend::i128_le

*Struct*

A little-endian `i128` with a guaranteed size and alignment of `16`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i128) -> Self` - Returns a `i128_le` containing `value`.
- `fn to_native(self: Self) -> i128` - Returns a `i128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i128)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i128_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i128_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i128_le)`
- **Div**
  - `fn div(self: Self, other: &i128_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i128)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i128_le)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i128_le)`
- **From**
  - `fn from(value: &'a i128) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i128) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i128_le)`
- **Rem**
  - `fn rem(self: Self, other: i128) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i128)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i128_le)`
- **Rem**
  - `fn rem(self: Self, other: &i128) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i128_le)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i128) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i128)`
- **Rem**
  - `fn rem(self: Self, other: i128_le) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i128)`
- **Shr**
  - `fn shr(self: Self, other: &i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i128_le)`
- **Rem**
  - `fn rem(self: Self, other: &i128_le) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i128) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i128_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: i128) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i128)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i128)`
- **Shr**
  - `fn shr(self: Self, other: i128_le) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i128) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i128_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i128_le)`
- **Shr**
  - `fn shr(self: Self, other: &i128_le) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i128)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i128_le) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i128_le)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i128)`
- **Add**
  - `fn add(self: Self, other: &i128) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i128) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i128)`
- **BitXor**
  - `fn bitxor(self: Self, other: i128_le) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i128_le) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i128_le) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i128_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i128)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i128_le)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i128)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i128_le)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i128_le)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i128)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i128_le)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i128_le)`
- **Shl**
  - `fn shl(self: Self, other: i128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i128_le)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i128)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i128)`
- **From**
  - `fn from(value: i128) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i128_le)`
- **Sub**
  - `fn sub(self: Self, other: i128) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i128_le)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i128) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i128_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i128)`
- **Sub**
  - `fn sub(self: Self, other: &i128) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i128_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i128_le)`
- **Mul**
  - `fn mul(self: Self, other: i128) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i128)`
- **Sub**
  - `fn sub(self: Self, other: i128_le) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i128_le)`
- **Div**
  - `fn div(self: Self, other: i128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i128_le) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i128) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i128_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i128_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i128)`
- **Mul**
  - `fn mul(self: Self, other: i128_le) -> <Self as >::Output`



## rend::i16_be

*Struct*

A big-endian `i16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i16) -> Self` - Returns a `i16_be` containing `value`.
- `fn to_native(self: Self) -> i16` - Returns a `i16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i16)`
- **Add**
  - `fn add(self: Self, other: i16) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i16_be) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i16) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i16)`
- **BitAnd**
  - `fn bitand(self: Self, other: &i16_be) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i16_be)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i16_be)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Add**
  - `fn add(self: Self, other: &i16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i16)`
- **BitXor**
  - `fn bitxor(self: Self, other: i16_be) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i16) -> Option<::core::cmp::Ordering>`
- **BitXor**
  - `fn bitxor(self: Self, other: &i16_be) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: i16_be) -> <Self as >::Output`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Rem**
  - `fn rem(self: Self, other: i16) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &i16_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i16)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i16)`
- **Rem**
  - `fn rem(self: Self, other: &i16) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i16_be)`
- **Shr**
  - `fn shr(self: Self, other: i16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i16)`
- **Rem**
  - `fn rem(self: Self, other: i16_be) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &i16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i16_be)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i16_be)`
- **Rem**
  - `fn rem(self: Self, other: &i16_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i16)`
- **Shr**
  - `fn shr(self: Self, other: i16_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i16_be)`
- **Shr**
  - `fn shr(self: Self, other: &i16_be) -> <Self as >::Output`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i16_be)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i16)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i16)`
- **From**
  - `fn from(value: i16) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &i16) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i16_be)`
- **BitOr**
  - `fn bitor(self: Self, other: i16) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i16)`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &i16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i16_be)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i16_be)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i16_be)`
- **Div**
  - `fn div(self: Self, other: i16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i16_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i16_be) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i16_be)`
- **Div**
  - `fn div(self: Self, other: &i16) -> <Self as >::Output`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i16)`
- **Div**
  - `fn div(self: Self, other: i16_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i16_be)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i16_be)`
- **Div**
  - `fn div(self: Self, other: &i16_be) -> <Self as >::Output`
- **Shl**
  - `fn shl(self: Self, other: i16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i16)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i16)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i16_be)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i16_be)`
- **From**
  - `fn from(value: &'a i16) -> Self`
- **Sub**
  - `fn sub(self: Self, other: i16) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i16_be) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i16_be)`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i16_be) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: i16) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i16_be)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i16)`
- **Sub**
  - `fn sub(self: Self, other: i16_be) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Mul**
  - `fn mul(self: Self, other: &i16) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i16_be) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i16)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i16)`
- **Mul**
  - `fn mul(self: Self, other: i16_be) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: &i16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i16_be)`
- **Mul**
  - `fn mul(self: Self, other: &i16_be) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: i16) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i16_be)`



## rend::i16_le

*Struct*

A little-endian `i16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i16) -> Self` - Returns a `i16_le` containing `value`.
- `fn to_native(self: Self) -> i16` - Returns a `i16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i16_le)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i16_le)`
- **From**
  - `fn from(value: &'a i16) -> Self`
- **Sub**
  - `fn sub(self: Self, other: i16) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i16_le) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i16_le)`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i16_le) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: i16) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i16_le)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i16)`
- **Sub**
  - `fn sub(self: Self, other: i16_le) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Mul**
  - `fn mul(self: Self, other: &i16) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i16_le) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i16)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i16)`
- **Mul**
  - `fn mul(self: Self, other: i16_le) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: &i16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i16_le)`
- **Mul**
  - `fn mul(self: Self, other: &i16_le) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: i16) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i16_le)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i16)`
- **Add**
  - `fn add(self: Self, other: i16) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i16_le) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i16) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i16)`
- **BitAnd**
  - `fn bitand(self: Self, other: &i16_le) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i16_le)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i16_le)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Add**
  - `fn add(self: Self, other: &i16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i16)`
- **BitXor**
  - `fn bitxor(self: Self, other: i16_le) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i16) -> Option<::core::cmp::Ordering>`
- **BitXor**
  - `fn bitxor(self: Self, other: &i16_le) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: i16_le) -> <Self as >::Output`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Rem**
  - `fn rem(self: Self, other: i16) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &i16_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i16)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i16)`
- **Rem**
  - `fn rem(self: Self, other: &i16) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i16_le)`
- **Shr**
  - `fn shr(self: Self, other: i16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i16)`
- **Rem**
  - `fn rem(self: Self, other: i16_le) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &i16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i16_le)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i16_le)`
- **Rem**
  - `fn rem(self: Self, other: &i16_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i16)`
- **Shr**
  - `fn shr(self: Self, other: i16_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i16_le)`
- **Shr**
  - `fn shr(self: Self, other: &i16_le) -> <Self as >::Output`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i16_le)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i16)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i16)`
- **From**
  - `fn from(value: i16) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &i16) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i16_le)`
- **BitOr**
  - `fn bitor(self: Self, other: i16) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i16)`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &i16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i16_le)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i16_le)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i16_le)`
- **Div**
  - `fn div(self: Self, other: i16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i16_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i16_le) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i16_le)`
- **Div**
  - `fn div(self: Self, other: &i16) -> <Self as >::Output`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i16)`
- **Div**
  - `fn div(self: Self, other: i16_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i16_le)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i16_le)`
- **Div**
  - `fn div(self: Self, other: &i16_le) -> <Self as >::Output`
- **Shl**
  - `fn shl(self: Self, other: i16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i16)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i16)`



## rend::i32_be

*Struct*

A big-endian `i32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i32) -> Self` - Returns a `i32_be` containing `value`.
- `fn to_native(self: Self) -> i32` - Returns a `i32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i32_be)`
- **Rem**
  - `fn rem(self: Self, other: &i32) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i32_be)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i32) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i32)`
- **Rem**
  - `fn rem(self: Self, other: i32_be) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i32)`
- **Shr**
  - `fn shr(self: Self, other: &i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i32_be)`
- **Rem**
  - `fn rem(self: Self, other: &i32_be) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i32) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i32_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: i32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i32)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i32)`
- **Shr**
  - `fn shr(self: Self, other: i32_be) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i32) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i32_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i32_be)`
- **Shr**
  - `fn shr(self: Self, other: &i32_be) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i32)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i32_be) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i32_be)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i32)`
- **Add**
  - `fn add(self: Self, other: &i32) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i32) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i32)`
- **BitXor**
  - `fn bitxor(self: Self, other: i32_be) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i32_be) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i32_be) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i32_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i32)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i32_be)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i32)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i32_be)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i32_be)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i32)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i32_be)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i32_be)`
- **Shl**
  - `fn shl(self: Self, other: i32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i32_be)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i32)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i32)`
- **From**
  - `fn from(value: i32) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i32_be)`
- **Sub**
  - `fn sub(self: Self, other: i32) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i32_be)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i32) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i32_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i32)`
- **Sub**
  - `fn sub(self: Self, other: &i32) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i32_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i32_be)`
- **Mul**
  - `fn mul(self: Self, other: i32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i32)`
- **Sub**
  - `fn sub(self: Self, other: i32_be) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i32_be)`
- **Div**
  - `fn div(self: Self, other: i32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i32_be) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i32) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i32_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i32_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i32)`
- **Mul**
  - `fn mul(self: Self, other: i32_be) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i32)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i32_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i32_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i32_be)`
- **Div**
  - `fn div(self: Self, other: &i32_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i32)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i32_be)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i32_be)`
- **From**
  - `fn from(value: &'a i32) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i32) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i32_be)`
- **Rem**
  - `fn rem(self: Self, other: i32) -> <Self as >::Output`



## rend::i32_le

*Struct*

A little-endian `i32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i32) -> Self` - Returns a `i32_le` containing `value`.
- `fn to_native(self: Self) -> i32` - Returns a `i32` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i32_le)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i32_le)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i32)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i32_le)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i32_le)`
- **Shl**
  - `fn shl(self: Self, other: i32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i32_le)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i32)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i32)`
- **From**
  - `fn from(value: i32) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i32_le)`
- **Sub**
  - `fn sub(self: Self, other: i32) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i32_le)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i32) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i32_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i32)`
- **Sub**
  - `fn sub(self: Self, other: &i32) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i32_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i32_le)`
- **Mul**
  - `fn mul(self: Self, other: i32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i32)`
- **Sub**
  - `fn sub(self: Self, other: i32_le) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i32_le)`
- **Div**
  - `fn div(self: Self, other: i32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i32_le) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i32) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i32_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i32_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i32)`
- **Mul**
  - `fn mul(self: Self, other: i32_le) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i32)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i32_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i32_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i32_le)`
- **Div**
  - `fn div(self: Self, other: &i32_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i32)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i32_le)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i32_le)`
- **From**
  - `fn from(value: &'a i32) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i32) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i32_le)`
- **Rem**
  - `fn rem(self: Self, other: i32) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i32_le)`
- **Rem**
  - `fn rem(self: Self, other: &i32) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i32_le)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i32) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i32)`
- **Rem**
  - `fn rem(self: Self, other: i32_le) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i32)`
- **Shr**
  - `fn shr(self: Self, other: &i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i32_le)`
- **Rem**
  - `fn rem(self: Self, other: &i32_le) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i32) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i32_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: i32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i32)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i32)`
- **Shr**
  - `fn shr(self: Self, other: i32_le) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i32) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i32_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i32_le)`
- **Shr**
  - `fn shr(self: Self, other: &i32_le) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i32)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i32_le) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i32_le)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i32)`
- **Add**
  - `fn add(self: Self, other: &i32) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i32) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i32)`
- **BitXor**
  - `fn bitxor(self: Self, other: i32_le) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i32_le) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i32_le) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i32_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i32)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i32_le)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i32)`



## rend::i64_be

*Struct*

A big-endian `i64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i64) -> Self` - Returns a `i64_be` containing `value`.
- `fn to_native(self: Self) -> i64` - Returns a `i64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i64_be)`
- **Shr**
  - `fn shr(self: Self, other: &i64_be) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i64)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i64_be) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i64_be)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i64)`
- **Add**
  - `fn add(self: Self, other: &i64) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i64) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i64)`
- **BitXor**
  - `fn bitxor(self: Self, other: i64_be) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i64_be) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i64_be) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i64_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i64)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i64_be)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i64)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i64_be)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i64_be)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i64)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i64_be)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i64_be)`
- **Shl**
  - `fn shl(self: Self, other: i64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i64_be)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i64)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i64)`
- **From**
  - `fn from(value: i64) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i64_be)`
- **Sub**
  - `fn sub(self: Self, other: i64) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i64_be)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i64) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i64_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i64)`
- **Sub**
  - `fn sub(self: Self, other: &i64) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i64_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i64_be)`
- **Mul**
  - `fn mul(self: Self, other: i64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i64)`
- **Sub**
  - `fn sub(self: Self, other: i64_be) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i64_be)`
- **Div**
  - `fn div(self: Self, other: i64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i64_be) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i64) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i64_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i64_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i64)`
- **Mul**
  - `fn mul(self: Self, other: i64_be) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i64)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i64_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i64_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i64_be)`
- **Div**
  - `fn div(self: Self, other: &i64_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i64)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i64_be)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i64_be)`
- **From**
  - `fn from(value: &'a i64) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i64) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i64_be)`
- **Rem**
  - `fn rem(self: Self, other: i64) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i64_be)`
- **Rem**
  - `fn rem(self: Self, other: &i64) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i64_be)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i64) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i64)`
- **Rem**
  - `fn rem(self: Self, other: i64_be) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i64)`
- **Shr**
  - `fn shr(self: Self, other: &i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i64_be)`
- **Rem**
  - `fn rem(self: Self, other: &i64_be) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i64) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i64_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: i64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i64)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i64)`
- **Shr**
  - `fn shr(self: Self, other: i64_be) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i64) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i64_be) -> <Self as >::Output`



## rend::i64_le

*Struct*

A little-endian `i64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i64) -> Self` - Returns a `i64_le` containing `value`.
- `fn to_native(self: Self) -> i64` - Returns a `i64` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i64) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i64_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i64)`
- **Sub**
  - `fn sub(self: Self, other: &i64) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i64_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i64_le)`
- **Mul**
  - `fn mul(self: Self, other: i64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i64)`
- **Sub**
  - `fn sub(self: Self, other: i64_le) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i64_le)`
- **Div**
  - `fn div(self: Self, other: i64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i64_le) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i64) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i64_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i64_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i64)`
- **Mul**
  - `fn mul(self: Self, other: i64_le) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i64)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i64_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i64_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i64_le)`
- **Div**
  - `fn div(self: Self, other: &i64_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i64)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i64_le)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i64_le)`
- **From**
  - `fn from(value: &'a i64) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i64) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i64_le)`
- **Rem**
  - `fn rem(self: Self, other: i64) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i64_le)`
- **Rem**
  - `fn rem(self: Self, other: &i64) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i64_le)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i64) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i64)`
- **Rem**
  - `fn rem(self: Self, other: i64_le) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i64)`
- **Shr**
  - `fn shr(self: Self, other: &i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i64_le)`
- **Rem**
  - `fn rem(self: Self, other: &i64_le) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i64) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i64_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: i64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i64)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i64)`
- **Shr**
  - `fn shr(self: Self, other: i64_le) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i64) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i64_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i64_le)`
- **Shr**
  - `fn shr(self: Self, other: &i64_le) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i64)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i64_le) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i64_le)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i64)`
- **Add**
  - `fn add(self: Self, other: &i64) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i64) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i64)`
- **BitXor**
  - `fn bitxor(self: Self, other: i64_le) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i64_le) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i64_le) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i64_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i64)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i64_le)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i64)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i64_le)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i64_le)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i64)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i64_le)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i64_le)`
- **Shl**
  - `fn shl(self: Self, other: i64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i64_le)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i64)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i64)`
- **From**
  - `fn from(value: i64) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i64_le)`
- **Sub**
  - `fn sub(self: Self, other: i64) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i64_le)`



## rend::u128_be

*Struct*

A big-endian `u128` with a guaranteed size and alignment of `16`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u128) -> Self` - Returns a `u128_be` containing `value`.
- `fn to_native(self: Self) -> u128` - Returns a `u128` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u128_be)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u128_be)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u128)`
- **From**
  - `fn from(value: u128) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u128) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u128_be)`
- **BitOr**
  - `fn bitor(self: Self, other: u128) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u128_be) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u128)`
- **Shl**
  - `fn shl(self: Self, other: &u128_be) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u128_be)`
- **Mul**
  - `fn mul(self: Self, other: u128) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u128)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u128)`
- **Sub**
  - `fn sub(self: Self, other: u128_be) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u128_be)`
- **Div**
  - `fn div(self: Self, other: u128) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u128_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u128_be) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u128_be) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u128)`
- **Div**
  - `fn div(self: Self, other: &u128) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u128_be) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u128)`
- **Mul**
  - `fn mul(self: Self, other: &u128_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u128_be)`
- **Div**
  - `fn div(self: Self, other: u128_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u128_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u128)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u128_be)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u128)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u128_be)`
- **From**
  - `fn from(value: &'a u128) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u128) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u128_be)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u128)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u128_be)`
- **Rem**
  - `fn rem(self: Self, other: &u128) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u128_be)`
- **Shr**
  - `fn shr(self: Self, other: u128) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u128)`
- **BitAnd**
  - `fn bitand(self: Self, other: u128) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u128_be) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u128_be)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u128)`
- **Rem**
  - `fn rem(self: Self, other: &u128_be) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u128) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u128_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: u128) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u128)`
- **Shr**
  - `fn shr(self: Self, other: u128_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u128)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u128_be) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u128) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u128_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u128_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u128)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u128_be) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u128_be)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u128) -> bool`
- **Add**
  - `fn add(self: Self, other: &u128) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u128)`
- **BitXor**
  - `fn bitxor(self: Self, other: u128_be) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u128_be) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u128_be) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u128_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u128)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u128_be)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u128)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u128_be)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u128_be)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u128)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u128_be)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u128_be)`
- **Shl**
  - `fn shl(self: Self, other: u128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u128)`



## rend::u128_le

*Struct*

A little-endian `u128` with a guaranteed size and alignment of `16`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u128) -> Self` - Returns a `u128_le` containing `value`.
- `fn to_native(self: Self) -> u128` - Returns a `u128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u128_le)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u128)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u128_le)`
- **Rem**
  - `fn rem(self: Self, other: &u128) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u128_le)`
- **Shr**
  - `fn shr(self: Self, other: u128) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u128)`
- **BitAnd**
  - `fn bitand(self: Self, other: u128) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u128_le) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u128_le)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u128)`
- **Rem**
  - `fn rem(self: Self, other: &u128_le) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u128) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u128_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: u128) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u128)`
- **Shr**
  - `fn shr(self: Self, other: u128_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u128)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u128_le) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u128) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u128_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u128_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u128)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u128_le) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u128_le)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u128) -> bool`
- **Add**
  - `fn add(self: Self, other: &u128) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u128)`
- **BitXor**
  - `fn bitxor(self: Self, other: u128_le) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u128_le) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u128_le) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u128_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u128)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u128_le)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u128)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u128_le)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u128_le)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u128)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u128_le)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u128_le)`
- **Shl**
  - `fn shl(self: Self, other: u128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u128_le)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u128_le)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u128)`
- **From**
  - `fn from(value: u128) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u128) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u128_le)`
- **BitOr**
  - `fn bitor(self: Self, other: u128) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u128_le) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u128)`
- **Shl**
  - `fn shl(self: Self, other: &u128_le) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u128_le)`
- **Mul**
  - `fn mul(self: Self, other: u128) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u128)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u128)`
- **Sub**
  - `fn sub(self: Self, other: u128_le) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u128_le)`
- **Div**
  - `fn div(self: Self, other: u128) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u128_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u128_le) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u128_le) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u128)`
- **Div**
  - `fn div(self: Self, other: &u128) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u128_le) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u128)`
- **Mul**
  - `fn mul(self: Self, other: &u128_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u128_le)`
- **Div**
  - `fn div(self: Self, other: u128_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u128_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u128)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u128_le)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u128)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u128_le)`
- **From**
  - `fn from(value: &'a u128) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u128) -> Option<::core::cmp::Ordering>`



## rend::u16_be

*Struct*

A big-endian `u16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u16) -> Self` - Returns a `u16_be` containing `value`.
- `fn to_native(self: Self) -> u16` - Returns a `u16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u16)`
- **BitXor**
  - `fn bitxor(self: Self, other: u16_be) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u16) -> Option<::core::cmp::Ordering>`
- **BitXor**
  - `fn bitxor(self: Self, other: &u16_be) -> <Self as >::Output`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Add**
  - `fn add(self: Self, other: u16_be) -> <Self as >::Output`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u16) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &u16_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u16)`
- **Default**
  - `fn default() -> Self`
- **Rem**
  - `fn rem(self: Self, other: &u16) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u16_be)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u16)`
- **Shr**
  - `fn shr(self: Self, other: u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u16)`
- **Rem**
  - `fn rem(self: Self, other: u16_be) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u16_be)`
- **Rem**
  - `fn rem(self: Self, other: &u16_be) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u16_be)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u16)`
- **Shr**
  - `fn shr(self: Self, other: u16_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u16)`
- **Shr**
  - `fn shr(self: Self, other: &u16_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u16_be)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u16_be)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u16)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u16) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u16)`
- **From**
  - `fn from(value: u16) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u16_be)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOr**
  - `fn bitor(self: Self, other: u16) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u16)`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u16_be)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u16_be)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u16_be)`
- **Div**
  - `fn div(self: Self, other: u16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u16_be) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u16_be)`
- **BitOr**
  - `fn bitor(self: Self, other: &u16_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u16) -> <Self as >::Output`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u16)`
- **Div**
  - `fn div(self: Self, other: u16_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u16_be)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u16_be)`
- **Shl**
  - `fn shl(self: Self, other: u16) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u16_be) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u16)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u16)`
- **Shl**
  - `fn shl(self: Self, other: &u16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u16_be)`
- **Sub**
  - `fn sub(self: Self, other: u16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u16_be)`
- **From**
  - `fn from(value: &'a u16) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u16_be) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u16) -> <Self as >::Output`
- **Shl**
  - `fn shl(self: Self, other: &u16_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u16_be)`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: u16) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u16)`
- **Sub**
  - `fn sub(self: Self, other: u16_be) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u16_be)`
- **Sub**
  - `fn sub(self: Self, other: &u16_be) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Mul**
  - `fn mul(self: Self, other: &u16) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u16)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u16)`
- **Mul**
  - `fn mul(self: Self, other: u16_be) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: &u16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u16_be)`
- **Mul**
  - `fn mul(self: Self, other: &u16_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u16_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: u16) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u16)`
- **Add**
  - `fn add(self: Self, other: u16) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u16_be) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &u16) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u16)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u16_be)`
- **BitAnd**
  - `fn bitand(self: Self, other: &u16_be) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u16_be)`
- **Add**
  - `fn add(self: Self, other: &u16) -> <Self as >::Output`



## rend::u16_le

*Struct*

A little-endian `u16` with a guaranteed size and alignment of `2`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u16) -> Self` - Returns a `u16_le` containing `value`.
- `fn to_native(self: Self) -> u16` - Returns a `u16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Shl**
  - `fn shl(self: Self, other: &u16_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u16_le)`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: u16) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u16)`
- **Sub**
  - `fn sub(self: Self, other: u16_le) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u16_le)`
- **Sub**
  - `fn sub(self: Self, other: &u16_le) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Mul**
  - `fn mul(self: Self, other: &u16) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u16)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u16)`
- **Mul**
  - `fn mul(self: Self, other: u16_le) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: &u16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u16_le)`
- **Mul**
  - `fn mul(self: Self, other: &u16_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u16_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: u16) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u16)`
- **Add**
  - `fn add(self: Self, other: u16) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u16_le) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &u16) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u16)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u16_le)`
- **BitAnd**
  - `fn bitand(self: Self, other: &u16_le) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u16_le)`
- **Add**
  - `fn add(self: Self, other: &u16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u16)`
- **BitXor**
  - `fn bitxor(self: Self, other: u16_le) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u16) -> Option<::core::cmp::Ordering>`
- **BitXor**
  - `fn bitxor(self: Self, other: &u16_le) -> <Self as >::Output`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Add**
  - `fn add(self: Self, other: u16_le) -> <Self as >::Output`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u16) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &u16_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u16)`
- **Default**
  - `fn default() -> Self`
- **Rem**
  - `fn rem(self: Self, other: &u16) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u16_le)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u16)`
- **Shr**
  - `fn shr(self: Self, other: u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u16)`
- **Rem**
  - `fn rem(self: Self, other: u16_le) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u16_le)`
- **Rem**
  - `fn rem(self: Self, other: &u16_le) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u16_le)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u16)`
- **Shr**
  - `fn shr(self: Self, other: u16_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u16)`
- **Shr**
  - `fn shr(self: Self, other: &u16_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u16_le)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u16_le)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u16)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u16) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u16)`
- **From**
  - `fn from(value: u16) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u16_le)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOr**
  - `fn bitor(self: Self, other: u16) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u16)`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u16_le)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u16_le)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u16_le)`
- **Div**
  - `fn div(self: Self, other: u16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u16_le) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u16_le)`
- **BitOr**
  - `fn bitor(self: Self, other: &u16_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u16) -> <Self as >::Output`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u16)`
- **Div**
  - `fn div(self: Self, other: u16_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u16_le)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u16_le)`
- **Shl**
  - `fn shl(self: Self, other: u16) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u16_le) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u16)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u16)`
- **Shl**
  - `fn shl(self: Self, other: &u16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u16_le)`
- **Sub**
  - `fn sub(self: Self, other: u16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u16_le)`
- **From**
  - `fn from(value: &'a u16) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u16_le) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u16) -> <Self as >::Output`



## rend::u32_be

*Struct*

A big-endian `u32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u32) -> Self` - Returns a `u32_be` containing `value`.
- `fn to_native(self: Self) -> u32` - Returns a `u32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitAnd**
  - `fn bitand(self: Self, other: &u32) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u32_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: u32) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u32)`
- **Shr**
  - `fn shr(self: Self, other: u32_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u32)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u32_be) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u32) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u32_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u32_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u32)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u32_be) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u32_be)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u32) -> bool`
- **Add**
  - `fn add(self: Self, other: &u32) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u32)`
- **BitXor**
  - `fn bitxor(self: Self, other: u32_be) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u32_be) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u32_be) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u32_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u32)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u32_be)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u32)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u32_be)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u32_be)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u32)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u32_be)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u32_be)`
- **Shl**
  - `fn shl(self: Self, other: u32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u32_be)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u32_be)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u32)`
- **From**
  - `fn from(value: u32) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u32) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u32_be)`
- **BitOr**
  - `fn bitor(self: Self, other: u32) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u32_be) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u32)`
- **Shl**
  - `fn shl(self: Self, other: &u32_be) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u32_be)`
- **Mul**
  - `fn mul(self: Self, other: u32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u32)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u32)`
- **Sub**
  - `fn sub(self: Self, other: u32_be) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u32_be)`
- **Div**
  - `fn div(self: Self, other: u32) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u32_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u32_be) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u32_be) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u32)`
- **Div**
  - `fn div(self: Self, other: &u32) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u32_be) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u32)`
- **Mul**
  - `fn mul(self: Self, other: &u32_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u32_be)`
- **Div**
  - `fn div(self: Self, other: u32_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u32_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u32)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u32_be)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u32)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u32_be)`
- **From**
  - `fn from(value: &'a u32) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u32) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u32_be)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u32)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u32_be)`
- **Rem**
  - `fn rem(self: Self, other: &u32) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u32_be)`
- **Shr**
  - `fn shr(self: Self, other: u32) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u32)`
- **BitAnd**
  - `fn bitand(self: Self, other: u32) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u32_be) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u32_be)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u32)`
- **Rem**
  - `fn rem(self: Self, other: &u32_be) -> <Self as >::Output`



## rend::u32_le

*Struct*

A little-endian `u32` with a guaranteed size and alignment of `4`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u32) -> Self` - Returns a `u32_le` containing `value`.
- `fn to_native(self: Self) -> u32` - Returns a `u32` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u32_le)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u32_le)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u32)`
- **From**
  - `fn from(value: u32) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u32) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u32_le)`
- **BitOr**
  - `fn bitor(self: Self, other: u32) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u32_le) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u32)`
- **Shl**
  - `fn shl(self: Self, other: &u32_le) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u32_le)`
- **Mul**
  - `fn mul(self: Self, other: u32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u32)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u32)`
- **Sub**
  - `fn sub(self: Self, other: u32_le) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u32_le)`
- **Div**
  - `fn div(self: Self, other: u32) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u32_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u32_le) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u32_le) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u32)`
- **Div**
  - `fn div(self: Self, other: &u32) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u32_le) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u32)`
- **Mul**
  - `fn mul(self: Self, other: &u32_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u32_le)`
- **Div**
  - `fn div(self: Self, other: u32_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u32_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u32)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u32_le)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u32)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u32_le)`
- **From**
  - `fn from(value: &'a u32) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u32) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u32_le)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u32)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u32_le)`
- **Rem**
  - `fn rem(self: Self, other: &u32) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u32_le)`
- **Shr**
  - `fn shr(self: Self, other: u32) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u32)`
- **BitAnd**
  - `fn bitand(self: Self, other: u32) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u32_le) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u32_le)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u32)`
- **Rem**
  - `fn rem(self: Self, other: &u32_le) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u32) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u32_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: u32) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u32)`
- **Shr**
  - `fn shr(self: Self, other: u32_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u32)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u32_le) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u32) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u32_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u32_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u32)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u32_le) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u32_le)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u32) -> bool`
- **Add**
  - `fn add(self: Self, other: &u32) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u32)`
- **BitXor**
  - `fn bitxor(self: Self, other: u32_le) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u32_le) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u32_le) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u32_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u32)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u32_le)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u32)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u32_le)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u32_le)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u32)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u32_le)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u32_le)`
- **Shl**
  - `fn shl(self: Self, other: u32) -> <Self as >::Output`



## rend::u64_be

*Struct*

A big-endian `u64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u64) -> Self` - Returns a `u64_be` containing `value`.
- `fn to_native(self: Self) -> u64` - Returns a `u64` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u64_be)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u64)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u64_be)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u64_be)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u64)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u64_be)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u64_be)`
- **Shl**
  - `fn shl(self: Self, other: u64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u64_be)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u64_be)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u64)`
- **From**
  - `fn from(value: u64) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u64) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u64_be)`
- **BitOr**
  - `fn bitor(self: Self, other: u64) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u64_be) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u64)`
- **Shl**
  - `fn shl(self: Self, other: &u64_be) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u64_be)`
- **Mul**
  - `fn mul(self: Self, other: u64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u64)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u64)`
- **Sub**
  - `fn sub(self: Self, other: u64_be) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u64_be)`
- **Div**
  - `fn div(self: Self, other: u64) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u64_be) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u64_be) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u64_be) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u64)`
- **Div**
  - `fn div(self: Self, other: &u64) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u64_be) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u64)`
- **Mul**
  - `fn mul(self: Self, other: &u64_be) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u64_be)`
- **Div**
  - `fn div(self: Self, other: u64_be) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u64_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u64)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u64_be)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u64)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u64_be)`
- **From**
  - `fn from(value: &'a u64) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u64) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u64_be)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u64)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u64_be)`
- **Rem**
  - `fn rem(self: Self, other: &u64) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u64_be)`
- **Shr**
  - `fn shr(self: Self, other: u64) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u64)`
- **BitAnd**
  - `fn bitand(self: Self, other: u64) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u64_be) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u64_be)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u64)`
- **Rem**
  - `fn rem(self: Self, other: &u64_be) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u64) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u64_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: u64) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u64)`
- **Shr**
  - `fn shr(self: Self, other: u64_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u64)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u64_be) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u64) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u64_be) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u64_be)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u64)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u64_be) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u64_be)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u64) -> bool`
- **Add**
  - `fn add(self: Self, other: &u64) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u64)`
- **BitXor**
  - `fn bitxor(self: Self, other: u64_be) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u64_be) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u64_be) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u64_be) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u64)`



## rend::u64_le

*Struct*

A little-endian `u64` with a guaranteed size and alignment of `8`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u64) -> Self` - Returns a `u64_le` containing `value`.
- `fn to_native(self: Self) -> u64` - Returns a `u64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOr**
  - `fn bitor(self: Self, other: &u64_le) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u64)`
- **Div**
  - `fn div(self: Self, other: &u64) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u64_le) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u64)`
- **Mul**
  - `fn mul(self: Self, other: &u64_le) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u64_le)`
- **Div**
  - `fn div(self: Self, other: u64_le) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u64_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u64)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u64_le)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u64)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u64_le)`
- **From**
  - `fn from(value: &'a u64) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u64) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u64_le)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u64)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u64_le)`
- **Rem**
  - `fn rem(self: Self, other: &u64) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u64_le)`
- **Shr**
  - `fn shr(self: Self, other: u64) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u64)`
- **BitAnd**
  - `fn bitand(self: Self, other: u64) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u64_le) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u64_le)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u64)`
- **Rem**
  - `fn rem(self: Self, other: &u64_le) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u64) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u64_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: u64) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u64)`
- **Shr**
  - `fn shr(self: Self, other: u64_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u64)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u64_le) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u64) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u64_le) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u64_le)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u64)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u64_le) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u64_le)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u64) -> bool`
- **Add**
  - `fn add(self: Self, other: &u64) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u64)`
- **BitXor**
  - `fn bitxor(self: Self, other: u64_le) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u64_le) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u64_le) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u64_le) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u64)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u64_le)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u64)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u64_le)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u64_le)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u64)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u64_le)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u64_le)`
- **Shl**
  - `fn shl(self: Self, other: u64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u64_le)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u64_le)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u64)`
- **From**
  - `fn from(value: u64) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u64) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u64_le)`
- **BitOr**
  - `fn bitor(self: Self, other: u64) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u64_le) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u64)`
- **Shl**
  - `fn shl(self: Self, other: &u64_le) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u64_le)`
- **Mul**
  - `fn mul(self: Self, other: u64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u64)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u64)`
- **Sub**
  - `fn sub(self: Self, other: u64_le) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u64_le)`
- **Div**
  - `fn div(self: Self, other: u64) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u64_le) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u64_le) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u64) -> <Self as >::Output`



## Module: unaligned

Cross-platform primitives with unaligned representations.



