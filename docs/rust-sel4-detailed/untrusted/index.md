# Crate `untrusted`

untrusted.rs: Safe, fast, zero-panic, zero-crashing, zero-allocation
parsing of untrusted inputs in Rust.

<code>git clone https://github.com/briansmith/untrusted</code>

untrusted.rs goes beyond Rust's normal safety guarantees by  also
guaranteeing that parsing will be panic-free, as long as
`untrusted::Input::as_slice_less_safe()` is not used. It avoids copying
data and heap allocation and strives to prevent common pitfalls such as
accidentally parsing input bytes multiple times. In order to meet these
goals, untrusted.rs is limited in functionality such that it works best for
input languages with a small fixed amount of lookahead such as ASN.1, TLS,
TCP/IP, and many other networking, IPC, and related protocols. Languages
that require more lookahead and/or backtracking require some significant
contortions to parse using this framework. It would not be realistic to use
it for parsing programming language code, for example.

The overall pattern for using untrusted.rs is:

1. Write a recursive-descent-style parser for the input language, where the
   input data is given as a `&mut untrusted::Reader` parameter to each
   function. Each function should have a return type of `Result<V, E>` for
   some value type `V` and some error type `E`, either or both of which may
   be `()`. Functions for parsing the lowest-level language constructs
   should be defined. Those lowest-level functions will parse their inputs
   using `::read_byte()`, `Reader::peek()`, and similar functions.
   Higher-level language constructs are then parsed by calling the
   lower-level functions in sequence.

2. Wrap the top-most functions of your recursive-descent parser in
   functions that take their input data as an `untrusted::Input`. The
   wrapper functions should call the `Input`'s `read_all` (or a variant
   thereof) method. The wrapper functions are the only ones that should be
   exposed outside the parser's module.

3. After receiving the input data to parse, wrap it in an `untrusted::Input`
   using `untrusted::Input::from()` as early as possible. Pass the
   `untrusted::Input` to the wrapper functions when they need to be parsed.

In general parsers built using `untrusted::Reader` do not need to explicitly
check for end-of-input unless they are parsing optional constructs, because
`Reader::read_byte()` will return `Err(EndOfInput)` on end-of-input.
Similarly, parsers using `untrusted::Reader` generally don't need to check
for extra junk at the end of the input as long as the parser's API uses the
pattern described above, as `read_all` and its variants automatically check
for trailing junk. `Reader::skip_to_end()` must be used when any remaining
unread input should be ignored without triggering an error.

untrusted.rs works best when all processing of the input data is done
through the `untrusted::Input` and `untrusted::Reader` types. In
particular, avoid trying to parse input data using functions that take
byte slices. However, when you need to access a part of the input data as
a slice to use a function that isn't written using untrusted.rs,
`Input::as_slice_less_safe()` can be used.

It is recommend to use `use untrusted;` and then `untrusted::Input`,
`untrusted::Reader`, etc., instead of using `use untrusted::*`. Qualifying
the names with `untrusted` helps remind the reader of the code that it is
dealing with *untrusted* input.

# Examples

[*ring*](https://github.com/briansmith/ring)'s parser for the subset of
ASN.1 DER it needs to understand,
[`ring::der`](https://github.com/briansmith/ring/blob/main/src/io/der.rs),
is built on top of untrusted.rs. *ring* also uses untrusted.rs to parse ECC
public keys, RSA PKCS#1 1.5 padding, and for all other parsing it does.

All of [webpki](https://github.com/briansmith/webpki)'s parsing of X.509
certificates (also ASN.1 DER) is done using untrusted.rs.

## Contents

- [Modules](#modules)
  - [`input`](#input)
  - [`no_panic`](#no-panic)
  - [`reader`](#reader)
- [Structs](#structs)
  - [`Input`](#input)
  - [`EndOfInput`](#endofinput)
  - [`Reader`](#reader)
- [Functions](#functions)
  - [`read_all_optional`](#read-all-optional)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`input`](#input) | mod |  |
| [`no_panic`](#no-panic) | mod |  |
| [`reader`](#reader) | mod |  |
| [`Input`](#input) | struct |  |
| [`EndOfInput`](#endofinput) | struct |  |
| [`Reader`](#reader) | struct |  |
| [`read_all_optional`](#read-all-optional) | fn | Calls `read` with the given input as a `Reader`, ensuring that `read` consumed the entire input. |

## Modules

- [`input`](input/index.md)
- [`no_panic`](no_panic/index.md)
- [`reader`](reader/index.md)

## Structs

### `Input<'a>`

```rust
struct Input<'a> {
    value: no_panic::Slice<'a>,
}
```

A wrapper around `&'a [u8]` that helps in writing panic-free code.

No methods of `Input` will ever panic.

Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit
non-constant-time comparisons.

#### Implementations

- <span id="input-from"></span>`const fn from(bytes: &'a [u8]) -> Self`

  Construct a new `Input` for the given input `bytes`.

- <span id="input-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the input is empty and false otherwise.

- <span id="input-len"></span>`fn len(&self) -> usize`

  Returns the length of the `Input`.

- <span id="input-read-all"></span>`fn read_all<F, R, E>(&self, incomplete_read: E, read: F) -> Result<R, E>`

  Calls `read` with the given input as a `Reader`, ensuring that `read`

  consumed the entire input. If `read` does not consume the entire input,

  `incomplete_read` is returned.

- <span id="input-as-slice-less-safe"></span>`fn as_slice_less_safe(&self) -> &'a [u8]`

  Access the input as a slice so it can be processed by functions that

  are not written using the Input/Reader framework.

- <span id="input-into-value"></span>`fn into_value(self) -> no_panic::Slice<'a>` — [`Slice`](no_panic/index.md#slice)

#### Trait Implementations

##### `impl Clone for Input<'a>`

- <span id="input-clone"></span>`fn clone(&self) -> Input<'a>` — [`Input`](input/index.md#input)

##### `impl Copy for Input<'a>`

##### `impl Debug for Input<'_>`

- <span id="input-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `EndOfInput`

```rust
struct EndOfInput;
```

The error type used to indicate the end of the input was reached before the
operation could be completed.

#### Trait Implementations

##### `impl Clone for EndOfInput`

- <span id="endofinput-clone"></span>`fn clone(&self) -> EndOfInput` — [`EndOfInput`](reader/index.md#endofinput)

##### `impl Copy for EndOfInput`

##### `impl Debug for EndOfInput`

- <span id="endofinput-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EndOfInput`

##### `impl PartialEq for EndOfInput`

- <span id="endofinput-partialeq-eq"></span>`fn eq(&self, other: &EndOfInput) -> bool` — [`EndOfInput`](reader/index.md#endofinput)

##### `impl StructuralPartialEq for EndOfInput`

### `Reader<'a>`

```rust
struct Reader<'a> {
    input: no_panic::Slice<'a>,
    i: usize,
}
```

A read-only, forward-only cursor into the data in an `Input`.

Using `Reader` to parse input helps to ensure that no byte of the input
will be accidentally processed more than once. Using `Reader` in
conjunction with `read_all` and `read_all_optional` helps ensure that no
byte of the input is accidentally left unprocessed. The methods of `Reader`
never panic, so `Reader` also assists the writing of panic-free code.

Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit
non-constant-time comparisons.

#### Implementations

- <span id="reader-new"></span>`fn new(input: Input<'a>) -> Self` — [`Input`](input/index.md#input)

  Construct a new Reader for the given input. Use `read_all` or

  `read_all_optional` instead of `Reader::new` whenever possible.

- <span id="reader-at-end"></span>`fn at_end(&self) -> bool`

  Returns `true` if the reader is at the end of the input, and `false`

  otherwise.

- <span id="reader-peek"></span>`fn peek(&self, b: u8) -> bool`

  Returns `true` if there is at least one more byte in the input and that

  byte is equal to `b`, and false otherwise.

- <span id="reader-read-byte"></span>`fn read_byte(&mut self) -> Result<u8, EndOfInput>` — [`EndOfInput`](reader/index.md#endofinput)

  Reads the next input byte.

  

  Returns `Ok(b)` where `b` is the next input byte, or `Err(EndOfInput)`

  if the `Reader` is at the end of the input.

- <span id="reader-read-bytes"></span>`fn read_bytes(&mut self, num_bytes: usize) -> Result<Input<'a>, EndOfInput>` — [`Input`](input/index.md#input), [`EndOfInput`](reader/index.md#endofinput)

  Skips `num_bytes` of the input, returning the skipped input as an

  `Input`.

  

  Returns `Ok(i)` if there are at least `num_bytes` of input remaining,

  and `Err(EndOfInput)` otherwise.

- <span id="reader-read-bytes-to-end"></span>`fn read_bytes_to_end(&mut self) -> Input<'a>` — [`Input`](input/index.md#input)

  Skips the reader to the end of the input, returning the skipped input

  as an `Input`.

- <span id="reader-read-partial"></span>`fn read_partial<F, R, E>(&mut self, read: F) -> Result<(Input<'a>, R), E>` — [`Input`](input/index.md#input)

  Calls `read()` with the given input as a `Reader`. On success, returns a

  pair `(bytes_read, r)` where `bytes_read` is what `read()` consumed and

  `r` is `read()`'s return value.

- <span id="reader-skip"></span>`fn skip(&mut self, num_bytes: usize) -> Result<(), EndOfInput>` — [`EndOfInput`](reader/index.md#endofinput)

  Skips `num_bytes` of the input.

  

  Returns `Ok(i)` if there are at least `num_bytes` of input remaining,

  and `Err(EndOfInput)` otherwise.

- <span id="reader-skip-to-end"></span>`fn skip_to_end(&mut self)`

  Skips the reader to the end of the input.

#### Trait Implementations

##### `impl Debug for Reader<'_>`

- <span id="reader-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

## Functions

### `read_all_optional`

```rust
fn read_all_optional<'a, F, R, E>(input: Option<Input<'a>>, incomplete_read: E, read: F) -> Result<R, E>
where
    F: FnOnce(Option<&mut Reader<'a>>) -> Result<R, E>
```

Calls `read` with the given input as a `Reader`, ensuring that `read`
consumed the entire input. When `input` is `None`, `read` will be
called with `None`.

