*[rustls_pki_types](../../index.md) / [server_name](../index.md) / [parser](index.md)*

---

# Module `parser`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parser`](#parser) | struct |  |
| [`AddrKind`](#addrkind) | enum |  |
| [`ReadNumberHelper`](#readnumberhelper) | trait |  |
| [`impl_helper!`](#impl-helper) | macro |  |

## Structs

### `Parser<'a>`

```rust
struct Parser<'a> {
    state: &'a [u8],
}
```

#### Implementations

- <span id="parser-new"></span>`const fn new(input: &'a [u8]) -> Self`

- <span id="parser-read-atomically"></span>`fn read_atomically<T, F>(&mut self, inner: F) -> Option<T>`

  Run a parser, and restore the pre-parse state if it fails.

- <span id="parser-parse-with"></span>`fn parse_with<T, F>(&mut self, inner: F, kind: AddrKind) -> Result<T, AddrParseError>` — [`AddrKind`](#addrkind), [`AddrParseError`](../index.md#addrparseerror)

  Run a parser, but fail if the entire input wasn't consumed.

  Doesn't run atomically.

- <span id="parser-peek-char"></span>`fn peek_char(&self) -> Option<char>`

  Peek the next character from the input

- <span id="parser-read-char"></span>`fn read_char(&mut self) -> Option<char>`

  Read the next character from the input

- <span id="parser-read-given-char"></span>`fn read_given_char(&mut self, target: char) -> Option<()>`

  Read the next character from the input if it matches the target.

- <span id="parser-read-separator"></span>`fn read_separator<T, F>(&mut self, sep: char, index: usize, inner: F) -> Option<T>`

  Helper for reading separators in an indexed loop. Reads the separator

  character iff index > 0, then runs the parser. When used in a loop,

  the separator character will only be read on index > 0 (see

  read_ipv4_addr for an example)

- <span id="parser-read-number"></span>`fn read_number<T: ReadNumberHelper>(&mut self, radix: u32, max_digits: Option<usize>, allow_zero_prefix: bool) -> Option<T>`

- <span id="parser-read-ipv4-addr"></span>`fn read_ipv4_addr(&mut self) -> Option<Ipv4Addr>` — [`Ipv4Addr`](../index.md#ipv4addr)

  Read an IPv4 address.

- <span id="parser-read-ipv6-addr"></span>`fn read_ipv6_addr(&mut self) -> Option<Ipv6Addr>` — [`Ipv6Addr`](../index.md#ipv6addr)

  Read an IPv6 Address.

## Enums

### `AddrKind`

```rust
enum AddrKind {
    Ipv4,
    Ipv6,
}
```

#### Trait Implementations

##### `impl Clone for AddrKind`

- <span id="addrkind-clone"></span>`fn clone(&self) -> AddrKind` — [`AddrKind`](#addrkind)

##### `impl Copy for AddrKind`

##### `impl Debug for AddrKind`

- <span id="addrkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AddrKind`

##### `impl PartialEq for AddrKind`

- <span id="addrkind-partialeq-eq"></span>`fn eq(&self, other: &AddrKind) -> bool` — [`AddrKind`](#addrkind)

##### `impl StructuralPartialEq for AddrKind`

## Traits

### `ReadNumberHelper`

```rust
trait ReadNumberHelper: Sized { ... }
```

#### Associated Constants

- `const ZERO: Self`

#### Required Methods

- `fn checked_mul(&self, other: u32) -> Option<Self>`

- `fn checked_add(&self, other: u32) -> Option<Self>`

#### Implementors

- `u16`
- `u32`
- `u8`

## Macros

### `impl_helper!`

