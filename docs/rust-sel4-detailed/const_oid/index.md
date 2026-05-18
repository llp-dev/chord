# Crate `const_oid`

# [RustCrypto]: Object Identifiers (OIDs)

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

Const-friendly implementation of the ISO/IEC Object Identifier (OID) standard
as defined in ITU [X.660], with support for BER/DER encoding/decoding as well
as heapless `no_std` (i.e. embedded) environments.

[Documentation][docs-link]

## About OIDs

Object Identifiers, a.k.a. OIDs, are an International Telecommunications
Union (ITU) and ISO/IEC standard for naming any object, concept, or "thing"
with a globally unambiguous persistent name.

The ITU's [X.660] standard provides the OID specification. Every OID is part of
a hierarchical namespace which begins with a *root OID*, which is either the
ITU's root OID (0), the ISO's root OID (1), or the joint ISO/ITU root OID (2).

The following is an example of an OID, in this case identifying the
`rsaEncryption` algorithm:

```text
1.2.840.113549.1.1.1
```

For more information, see: <https://en.wikipedia.org/wiki/Object_identifier>

## Implementation

This library supports parsing OIDs in const contexts, e.g.:

```rust
use const_oid::ObjectIdentifier;

pub const MY_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.1");
```

The OID parser is implemented entirely in terms of `const fn` and without the
use of proc macros.

Additionally, it also includes a `const fn` OID serializer, and stores the OIDs
parsed from const contexts encoded using the BER/DER serialization
(sans header).

This allows `ObjectIdentifier` to impl `AsRef<[u8]>` which can be used to
obtain the BER/DER serialization of an OID, even one declared `const`.

Additionally, it impls `FromStr` and `TryFrom<&[u8]>` and functions just as
well as a runtime OID library.

## Minimum Supported Rust Version

This crate requires **Rust 1.57** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a minor
version bump.

## License

Licensed under either of:

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)










[//]: # (links)



## Contents

- [Modules](#modules)
  - [`checked`](#checked)
  - [`arcs`](#arcs)
  - [`encoder`](#encoder)
  - [`error`](#error)
  - [`parser`](#parser)
- [Structs](#structs)
  - [`Arcs`](#arcs)
  - [`ObjectIdentifier`](#objectidentifier)
- [Enums](#enums)
  - [`Error`](#error)
- [Traits](#traits)
  - [`AssociatedOid`](#associatedoid)
  - [`DynAssociatedOid`](#dynassociatedoid)
- [Type Aliases](#type-aliases)
  - [`Arc`](#arc)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`checked`](#checked) | mod | Checked arithmetic helpers. |
| [`arcs`](#arcs) | mod | Arcs are integer values which exist within an OID's hierarchy. |
| [`encoder`](#encoder) | mod | OID encoder with `const` support. |
| [`error`](#error) | mod | Error types |
| [`parser`](#parser) | mod | OID string parser with `const` support. |
| [`Arcs`](#arcs) | struct |  |
| [`ObjectIdentifier`](#objectidentifier) | struct | Object identifier (OID). |
| [`Error`](#error) | enum |  |
| [`AssociatedOid`](#associatedoid) | trait | A trait which associates an OID with a type. |
| [`DynAssociatedOid`](#dynassociatedoid) | trait | A trait which associates a dynamic, `&self`-dependent OID with a type, which may change depending on the type's value. |
| [`Arc`](#arc) | type |  |
| [`Result`](#result) | type |  |

## Modules

- [`checked`](checked/index.md) — Checked arithmetic helpers.
- [`arcs`](arcs/index.md) — Arcs are integer values which exist within an OID's hierarchy.
- [`encoder`](encoder/index.md) — OID encoder with `const` support.
- [`error`](error/index.md) — Error types
- [`parser`](parser/index.md) — OID string parser with `const` support.

## Structs

### `Arcs<'a>`

```rust
struct Arcs<'a> {
    oid: &'a crate::ObjectIdentifier,
    cursor: Option<usize>,
}
```

[`Iterator`](../fallible_iterator/index.md) over [`Arc`](arcs/index.md) values (a.k.a. nodes) in an [`ObjectIdentifier`](#objectidentifier).

This iterates over all arcs in an OID, including the root.

#### Fields

- **`oid`**: `&'a crate::ObjectIdentifier`

  OID we're iterating over

- **`cursor`**: `Option<usize>`

  Current position within the serialized DER bytes of this OID

#### Implementations

- <span id="arcs-new"></span>`fn new(oid: &'a ObjectIdentifier) -> Self` — [`ObjectIdentifier`](#objectidentifier)

  Create a new iterator over the arcs of this OID

- <span id="arcs-try-next"></span>`fn try_next(&mut self) -> Result<Option<Arc>>` — [`Result`](error/index.md#result), [`Arc`](arcs/index.md#arc)

  Try to parse the next arc in this OID.

  

  This method is fallible so it can be used as a first pass to determine

  that the arcs in the OID are well-formed.

#### Trait Implementations

##### `impl IntoIterator for Arcs<'a>`

- <span id="arcs-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arcs-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arcs-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Arcs<'a>`

- <span id="arcs-iterator-type-item"></span>`type Item = u32`

- <span id="arcs-iterator-next"></span>`fn next(&mut self) -> Option<Arc>` — [`Arc`](arcs/index.md#arc)

### `ObjectIdentifier`

```rust
struct ObjectIdentifier {
    length: u8,
    bytes: [u8; 39],
}
```

Object identifier (OID).

OIDs are hierarchical structures consisting of "arcs", i.e. integer
identifiers.

# Validity

In order for an OID to be considered valid by this library, it must meet
the following criteria:

- The OID MUST have at least 3 arcs
- The first arc MUST be within the range 0-2
- The second arc MUST be within the range 0-39
- The BER/DER encoding of the OID MUST be shorter than
  `ObjectIdentifier::MAX_SIZE`

#### Fields

- **`length`**: `u8`

  Length in bytes

- **`bytes`**: `[u8; 39]`

  Array containing BER/DER-serialized bytes (no header)

#### Implementations

- <span id="objectidentifier-const-max-size"></span>`const MAX_SIZE: usize`

- <span id="objectidentifier-new-unwrap"></span>`const fn new_unwrap(s: &str) -> Self`

  Parse an [`ObjectIdentifier`](#objectidentifier) from the dot-delimited string form,

  panicking on parse errors.

  

  This function exists as a workaround for `unwrap` not yet being

  stable in `const fn` contexts, and is intended to allow the result to

  be bound to a constant value:

  

  ```rust

  use const_oid::ObjectIdentifier;

  

  pub const MY_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.1");

  ```

  

  In future versions of Rust it should be possible to replace this with

  `ObjectIdentifier::new(...).unwrap()`.

  

  Use `ObjectIdentifier::new` for fallible parsing.

- <span id="objectidentifier-new"></span>`const fn new(s: &str) -> Result<Self>` — [`Result`](error/index.md#result)

  Parse an [`ObjectIdentifier`](#objectidentifier) from the dot-delimited string form.

- <span id="objectidentifier-from-arcs"></span>`fn from_arcs(arcs: impl IntoIterator<Item = Arc>) -> Result<Self>` — [`Arc`](arcs/index.md#arc), [`Result`](error/index.md#result)

  Parse an OID from a slice of [`Arc`](arcs/index.md) values (i.e. integers).

- <span id="objectidentifier-from-bytes"></span>`fn from_bytes(ber_bytes: &[u8]) -> Result<Self>` — [`Result`](error/index.md#result)

  Parse an OID from from its BER/DER encoding.

- <span id="objectidentifier-as-bytes"></span>`fn as_bytes(&self) -> &[u8]`

  Get the BER/DER serialization of this OID as bytes.

  

  Note that this encoding omits the tag/length, and only contains the

  value portion of the encoded OID.

- <span id="objectidentifier-arc"></span>`fn arc(&self, index: usize) -> Option<Arc>` — [`Arc`](arcs/index.md#arc)

  Return the arc with the given index, if it exists.

- <span id="objectidentifier-arcs"></span>`fn arcs(&self) -> Arcs<'_>` — [`Arcs`](arcs/index.md#arcs)

  Iterate over the arcs (a.k.a. nodes) of an [`ObjectIdentifier`](#objectidentifier).

  

  Returns [`Arcs`](arcs/index.md), an iterator over [`Arc`](arcs/index.md) values.

- <span id="objectidentifier-len"></span>`fn len(&self) -> usize`

  Get the length of this [`ObjectIdentifier`](#objectidentifier) in arcs.

- <span id="objectidentifier-parent"></span>`fn parent(&self) -> Option<Self>`

  Get the parent OID of this one (if applicable).

- <span id="objectidentifier-push-arc"></span>`const fn push_arc(self, arc: Arc) -> Result<Self>` — [`Arc`](arcs/index.md#arc), [`Result`](error/index.md#result)

  Push an additional arc onto this OID, returning the child OID.

#### Trait Implementations

##### `impl AsRef for ObjectIdentifier`

- <span id="objectidentifier-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for ObjectIdentifier`

- <span id="objectidentifier-clone"></span>`fn clone(&self) -> ObjectIdentifier` — [`ObjectIdentifier`](#objectidentifier)

##### `impl Copy for ObjectIdentifier`

##### `impl Debug for ObjectIdentifier`

- <span id="objectidentifier-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ObjectIdentifier`

- <span id="objectidentifier-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectIdentifier`

##### `impl FromStr for ObjectIdentifier`

- <span id="objectidentifier-fromstr-type-err"></span>`type Err = Error`

- <span id="objectidentifier-fromstr-from-str"></span>`fn from_str(string: &str) -> Result<Self>` — [`Result`](error/index.md#result)

##### `impl Hash for ObjectIdentifier`

- <span id="objectidentifier-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ObjectIdentifier`

- <span id="objectidentifier-ord-cmp"></span>`fn cmp(&self, other: &ObjectIdentifier) -> cmp::Ordering` — [`ObjectIdentifier`](#objectidentifier)

##### `impl PartialEq for ObjectIdentifier`

- <span id="objectidentifier-partialeq-eq"></span>`fn eq(&self, other: &ObjectIdentifier) -> bool` — [`ObjectIdentifier`](#objectidentifier)

##### `impl PartialOrd for ObjectIdentifier`

- <span id="objectidentifier-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ObjectIdentifier) -> option::Option<cmp::Ordering>` — [`ObjectIdentifier`](#objectidentifier)

##### `impl StructuralPartialEq for ObjectIdentifier`

## Enums

### `Error`

```rust
enum Error {
    ArcInvalid {
        arc: crate::Arc,
    },
    ArcTooBig,
    Base128,
    DigitExpected {
        actual: u8,
    },
    Empty,
    Length,
    NotEnoughArcs,
    TrailingDot,
}
```

OID errors.

#### Variants

- **`ArcInvalid`**

  Arc exceeds allowed range (i.e. for first or second OID)

- **`ArcTooBig`**

  Arc is too big (exceeds 32-bit limits of this library).
  
  Technically the size of an arc is not constrained by X.660, however
  this library has elected to use `u32` as the arc representation as
  sufficient for PKIX/PKCS usages.

- **`Base128`**

  Base 128 encoding error (used in BER/DER serialization of arcs).

- **`DigitExpected`**

  Expected a digit, but was provided something else.

- **`Empty`**

  Input data is empty.

- **`Length`**

  OID length is invalid (too short or too long).

- **`NotEnoughArcs`**

  Minimum 3 arcs required.

- **`TrailingDot`**

  Trailing `.` character at end of input.

#### Implementations

- <span id="error-panic"></span>`const fn panic(self) -> never`

  Escalate this error into a panic.

  

  This is a workaround until `Result::unwrap` is allowed in `const fn`.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](error/index.md#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl Ord for Error`

- <span id="error-ord-cmp"></span>`fn cmp(&self, other: &Error) -> cmp::Ordering` — [`Error`](error/index.md#error)

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](error/index.md#error)

##### `impl PartialOrd for Error`

- <span id="error-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Error) -> option::Option<cmp::Ordering>` — [`Error`](error/index.md#error)

##### `impl StructuralPartialEq for Error`

## Traits

### `AssociatedOid`

```rust
trait AssociatedOid { ... }
```

A trait which associates an OID with a type.

#### Associated Constants

- `const OID: ObjectIdentifier`

### `DynAssociatedOid`

```rust
trait DynAssociatedOid { ... }
```

A trait which associates a dynamic, `&self`-dependent OID with a type,
which may change depending on the type's value.

This trait is object safe and auto-impl'd for any types which impl
[`AssociatedOid`](#associatedoid).

#### Required Methods

- `fn oid(&self) -> ObjectIdentifier`

  Get the OID associated with this value.

#### Implementors

- `T`

## Type Aliases

### `Arc`

```rust
type Arc = u32;
```

Type alias used to represent an "arc" (i.e. integer identifier value).

X.660 does not define a maximum size of an arc.

The current representation is `u32`, which has been selected as being
sufficient to cover the current PKCS/PKIX use cases this library has been
used in conjunction with.

Future versions may potentially make it larger if a sufficiently important
use case is discovered.

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, Error>;
```

Result type

