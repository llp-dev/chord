*[ring](../index.md) / [test](index.md)*

---

# Module `test`

Testing framework.

Unlike the rest of *ring*, this testing framework uses panics pretty
liberally. It was originally designed for internal use--it drives most of
*ring*'s internal tests, and so it is optimized for getting *ring*'s tests
written quickly at the expense of some usability. The documentation is
lacking. The best way to learn it is to look at some examples. The digest
tests are the most complicated because they use named sections. Other tests
avoid named sections and so are easier to understand.

# Examples

## Writing Tests

Input files look like this:

```text
This is a comment.

HMAC = SHA1
Input = "My test data"
Key = ""
Output = 61afdecb95429ef494d61fdee15990cabf0826fc

HMAC = SHA256
Input = "Sample message for keylen<blocklen"
Key = 000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F
Output = A28CF43130EE696A98F14A37678B56BCFCBDD9E5CF69717FECF5480F0EBDF790
```

Test cases are separated with blank lines. Note how the bytes of the `Key`
attribute are specified as a quoted string in the first test case and as
hex in the second test case; you can use whichever form is more convenient
and you can mix and match within the same file. The empty sequence of bytes
can only be represented with the quoted string form (`""`).

Here's how you would consume the test data:

```ignore
use ring::test;

test::run(test::test_file!("hmac_tests.txt"), |section, test_case| {
    assert_eq!(section, ""); // This test doesn't use named sections.

    let digest_alg = test_case.consume_digest_alg("HMAC");
    let input = test_case.consume_bytes("Input");
    let key = test_case.consume_bytes("Key");
    let output = test_case.consume_bytes("Output");

    // Do the actual testing here
});
```

Note that `consume_digest_alg` automatically maps the string "SHA1" to a
reference to `digest::SHA1_FOR_LEGACY_USE_ONLY`, "SHA256" to
`digest::SHA256`, etc.

## Output When a Test Fails

When a test case fails, the framework automatically prints out the test
case. If the test case failed with a panic, then the backtrace of the panic
will be printed too. For example, let's say the failing test case looks
like this:

```text
Curve = P-256
a = 2b11cb945c8cf152ffa4c9c2b1c965b019b35d0b7626919ef0ae6cb9d232f8af
b = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
r = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
```
If the test fails, this will be printed (if `$RUST_BACKTRACE` is `1`):

```text
src/example_tests.txt: Test panicked.
Curve = P-256
a = 2b11cb945c8cf152ffa4c9c2b1c965b019b35d0b7626919ef0ae6cb9d232f8af
b = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
r = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
thread 'example_test' panicked at 'Test failed.', src\test.rs:206
stack backtrace:
   0:     0x7ff654a05c7c - std::rt::lang_start::h61f4934e780b4dfc
   1:     0x7ff654a04f32 - std::rt::lang_start::h61f4934e780b4dfc
   2:     0x7ff6549f505d - std::panicking::rust_panic_with_hook::hfe203e3083c2b544
   3:     0x7ff654a0825b - rust_begin_unwind
   4:     0x7ff6549f63af - std::panicking::begin_panic_fmt::h484cd47786497f03
   5:     0x7ff654a07e9b - rust_begin_unwind
   6:     0x7ff654a0ae95 - core::panicking::panic_fmt::h257ceb0aa351d801
   7:     0x7ff654a0b190 - core::panicking::panic::h4bb1497076d04ab9
   8:     0x7ff65496dc41 - from_file<closure>
                        at C:\Users\Example\example\<core macros>:4
   9:     0x7ff65496d49c - example_test
                        at C:\Users\Example\example\src\example.rs:652
  10:     0x7ff6549d192a - test::stats::Summary::new::ha139494ed2e4e01f
  11:     0x7ff6549d51a2 - test::stats::Summary::new::ha139494ed2e4e01f
  12:     0x7ff654a0a911 - _rust_maybe_catch_panic
  13:     0x7ff6549d56dd - test::stats::Summary::new::ha139494ed2e4e01f
  14:     0x7ff654a03783 - std::sys::thread::Thread::new::h2b08da6cd2517f79
  15:     0x7ff968518101 - BaseThreadInitThunk
```

Notice that the output shows the name of the data file
(`src/example_tests.txt`), the test inputs that led to the failure, and the
stack trace to the line in the test code that panicked: entry 9 in the
stack trace pointing to line 652 of the file `example.rs`.

## Contents

- [Structs](#structs)
  - [`TestCase`](#testcase)
  - [`File`](#file)
- [Functions](#functions)
  - [`compile_time_assert_clone`](#compile-time-assert-clone)
  - [`compile_time_assert_copy`](#compile-time-assert-copy)
  - [`compile_time_assert_eq`](#compile-time-assert-eq)
  - [`compile_time_assert_send`](#compile-time-assert-send)
  - [`compile_time_assert_sync`](#compile-time-assert-sync)
  - [`run`](#run)
  - [`from_hex`](#from-hex)
  - [`from_hex_digit`](#from-hex-digit)
  - [`parse_test_case`](#parse-test-case)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TestCase`](#testcase) | struct | A test case. |
| [`File`](#file) | struct | A test input file. |
| [`compile_time_assert_clone`](#compile-time-assert-clone) | fn | `compile_time_assert_clone::<T>();` fails to compile if `T` doesn't implement `Clone`. |
| [`compile_time_assert_copy`](#compile-time-assert-copy) | fn | `compile_time_assert_copy::<T>();` fails to compile if `T` doesn't implement `Copy`. |
| [`compile_time_assert_eq`](#compile-time-assert-eq) | fn | `compile_time_assert_eq::<T>();` fails to compile if `T` doesn't implement `Eq`. |
| [`compile_time_assert_send`](#compile-time-assert-send) | fn | `compile_time_assert_send::<T>();` fails to compile if `T` doesn't implement `Send`. |
| [`compile_time_assert_sync`](#compile-time-assert-sync) | fn | `compile_time_assert_sync::<T>();` fails to compile if `T` doesn't implement `Sync`. |
| [`run`](#run) | fn | Parses test cases out of the given file, calling `f` on each vector until `f` fails or until all the test vectors have been read. |
| [`from_hex`](#from-hex) | fn | Decode an string of hex digits into a sequence of bytes. |
| [`from_hex_digit`](#from-hex-digit) | fn |  |
| [`parse_test_case`](#parse-test-case) | fn |  |

## Structs

### `TestCase`

```rust
struct TestCase {
    attributes: alloc::vec::Vec<(alloc::string::String, alloc::string::String, bool)>,
}
```

A test case. A test case consists of a set of named attributes. Every
attribute in the test case must be consumed exactly once; this helps catch
typos and omissions.

Requires the `alloc` default feature to be enabled.

#### Implementations

- <span id="testcase-consume-bool"></span>`fn consume_bool(&mut self, key: &str) -> bool`

  Maps the string "true" to true and the string "false" to false.

- <span id="testcase-consume-digest-alg"></span>`fn consume_digest_alg(&mut self, key: &str) -> Option<&'static digest::Algorithm>` — [`Algorithm`](../digest/index.md#algorithm)

  Maps the strings "SHA1", "SHA256", "SHA384", and "SHA512" to digest

  algorithms, maps "SHA224" to `None`, and panics on other (erroneous)

  inputs. "SHA224" is mapped to None because *ring* intentionally does

  not support SHA224, but we need to consume test vectors from NIST that

  have SHA224 vectors in them.

- <span id="testcase-consume-bytes"></span>`fn consume_bytes(&mut self, key: &str) -> Vec<u8>`

  Returns the value of an attribute that is encoded as a sequence of an

  even number of hex digits, or as a double-quoted UTF-8 string. The

  empty (zero-length) value is represented as "".

- <span id="testcase-consume-optional-bytes"></span>`fn consume_optional_bytes(&mut self, key: &str) -> Option<Vec<u8>>`

  Like `consume_bytes()` except it returns `None` if the test case

  doesn't have the attribute.

- <span id="testcase-consume-usize"></span>`fn consume_usize(&mut self, key: &str) -> usize`

  Returns the value of an attribute that is an integer, in decimal

  notation.

- <span id="testcase-consume-usize-bits"></span>`fn consume_usize_bits(&mut self, key: &str) -> bits::BitLength` — [`BitLength`](../bits/index.md#bitlength)

  Returns the value of an attribute that is an integer, in decimal

  notation, as a bit length.

- <span id="testcase-consume-string"></span>`fn consume_string(&mut self, key: &str) -> String`

  Returns the raw value of an attribute, without any unquoting or

  other interpretation.

- <span id="testcase-consume-optional-string"></span>`fn consume_optional_string(&mut self, key: &str) -> Option<String>`

  Like `consume_string()` except it returns `None` if the test case

  doesn't have the attribute.

#### Trait Implementations

##### `impl Debug for TestCase`

- <span id="testcase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `File<'a>`

```rust
struct File<'a> {
    pub file_name: &'a str,
    pub contents: &'a str,
}
```

A test input file.

#### Fields

- **`file_name`**: `&'a str`

  The name (path) of the file.

- **`contents`**: `&'a str`

  The contents of the file.

## Functions

### `compile_time_assert_clone`

```rust
fn compile_time_assert_clone<T: Clone>()
```

`compile_time_assert_clone::<T>();` fails to compile if `T` doesn't
implement `Clone`.

### `compile_time_assert_copy`

```rust
fn compile_time_assert_copy<T: Copy>()
```

`compile_time_assert_copy::<T>();` fails to compile if `T` doesn't
implement `Copy`.

### `compile_time_assert_eq`

```rust
fn compile_time_assert_eq<T: Eq>()
```

`compile_time_assert_eq::<T>();` fails to compile if `T` doesn't
implement `Eq`.

### `compile_time_assert_send`

```rust
fn compile_time_assert_send<T: Send>()
```

`compile_time_assert_send::<T>();` fails to compile if `T` doesn't
implement `Send`.

### `compile_time_assert_sync`

```rust
fn compile_time_assert_sync<T: Sync>()
```

`compile_time_assert_sync::<T>();` fails to compile if `T` doesn't
implement `Sync`.

### `run`

```rust
fn run<F>(test_file: File<'_>, f: F)
where
    F: FnMut(&str, &mut TestCase) -> Result<(), error::Unspecified>
```

Parses test cases out of the given file, calling `f` on each vector until
`f` fails or until all the test vectors have been read. `f` can indicate
failure either by returning `Err()` or by panicking.

### `from_hex`

```rust
fn from_hex(hex_str: &str) -> Result<alloc::vec::Vec<u8>, alloc::string::String>
```

Decode an string of hex digits into a sequence of bytes. The input must
have an even number of digits.

### `from_hex_digit`

```rust
fn from_hex_digit(d: u8) -> Result<u8, alloc::string::String>
```

### `parse_test_case`

```rust
fn parse_test_case(current_section: &mut alloc::string::String, lines: &mut dyn Iterator<Item = &str>) -> Option<TestCase>
```

