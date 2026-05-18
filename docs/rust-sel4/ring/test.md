**ring > test**

# Module: test

## Contents

**Structs**

- [`File`](#file) - A test input file.
- [`TestCase`](#testcase) - A test case. A test case consists of a set of named attributes. Every

**Functions**

- [`compile_time_assert_clone`](#compile_time_assert_clone) - `compile_time_assert_clone::<T>();` fails to compile if `T` doesn't
- [`compile_time_assert_copy`](#compile_time_assert_copy) - `compile_time_assert_copy::<T>();` fails to compile if `T` doesn't
- [`compile_time_assert_eq`](#compile_time_assert_eq) - `compile_time_assert_eq::<T>();` fails to compile if `T` doesn't
- [`compile_time_assert_send`](#compile_time_assert_send) - `compile_time_assert_send::<T>();` fails to compile if `T` doesn't
- [`compile_time_assert_sync`](#compile_time_assert_sync) - `compile_time_assert_sync::<T>();` fails to compile if `T` doesn't
- [`from_hex`](#from_hex) - Decode an string of hex digits into a sequence of bytes. The input must
- [`run`](#run) - Parses test cases out of the given file, calling `f` on each vector until

---

## ring::test::File

*Struct*

A test input file.

**Generic Parameters:**
- 'a

**Fields:**
- `file_name: &'a str` - The name (path) of the file.
- `contents: &'a str` - The contents of the file.



## ring::test::TestCase

*Struct*

A test case. A test case consists of a set of named attributes. Every
attribute in the test case must be consumed exactly once; this helps catch
typos and omissions.

Requires the `alloc` default feature to be enabled.

**Methods:**

- `fn consume_bool(self: & mut Self, key: &str) -> bool` - Maps the string "true" to true and the string "false" to false.
- `fn consume_digest_alg(self: & mut Self, key: &str) -> Option<&'static digest::Algorithm>` - Maps the strings "SHA1", "SHA256", "SHA384", and "SHA512" to digest
- `fn consume_bytes(self: & mut Self, key: &str) -> Vec<u8>` - Returns the value of an attribute that is encoded as a sequence of an
- `fn consume_optional_bytes(self: & mut Self, key: &str) -> Option<Vec<u8>>` - Like `consume_bytes()` except it returns `None` if the test case
- `fn consume_usize(self: & mut Self, key: &str) -> usize` - Returns the value of an attribute that is an integer, in decimal
- `fn consume_usize_bits(self: & mut Self, key: &str) -> bits::BitLength` - Returns the value of an attribute that is an integer, in decimal
- `fn consume_string(self: & mut Self, key: &str) -> String` - Returns the raw value of an attribute, without any unquoting or
- `fn consume_optional_string(self: & mut Self, key: &str) -> Option<String>` - Like `consume_string()` except it returns `None` if the test case

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ring::test::compile_time_assert_clone

*Function*

`compile_time_assert_clone::<T>();` fails to compile if `T` doesn't
implement `Clone`.

```rust
fn compile_time_assert_clone<T>()
```



## ring::test::compile_time_assert_copy

*Function*

`compile_time_assert_copy::<T>();` fails to compile if `T` doesn't
implement `Copy`.

```rust
fn compile_time_assert_copy<T>()
```



## ring::test::compile_time_assert_eq

*Function*

`compile_time_assert_eq::<T>();` fails to compile if `T` doesn't
implement `Eq`.

```rust
fn compile_time_assert_eq<T>()
```



## ring::test::compile_time_assert_send

*Function*

`compile_time_assert_send::<T>();` fails to compile if `T` doesn't
implement `Send`.

```rust
fn compile_time_assert_send<T>()
```



## ring::test::compile_time_assert_sync

*Function*

`compile_time_assert_sync::<T>();` fails to compile if `T` doesn't
implement `Sync`.

```rust
fn compile_time_assert_sync<T>()
```



## ring::test::from_hex

*Function*

Decode an string of hex digits into a sequence of bytes. The input must
have an even number of digits.

```rust
fn from_hex(hex_str: &str) -> Result<alloc::vec::Vec<u8>, alloc::string::String>
```



## ring::test::run

*Function*

Parses test cases out of the given file, calling `f` on each vector until
`f` fails or until all the test vectors have been read. `f` can indicate
failure either by returning `Err()` or by panicking.

```rust
fn run<F>(test_file: File, f: F)
```



