*[const_oid](../index.md) / [arcs](index.md)*

---

# Module `arcs`

Arcs are integer values which exist within an OID's hierarchy.

## Contents

- [Structs](#structs)
  - [`Arcs`](#arcs)
  - [`RootArcs`](#rootarcs)
- [Type Aliases](#type-aliases)
  - [`Arc`](#arc)
- [Constants](#constants)
  - [`ARC_MAX_FIRST`](#arc-max-first)
  - [`ARC_MAX_SECOND`](#arc-max-second)
  - [`ARC_MAX_BYTES`](#arc-max-bytes)
  - [`ARC_MAX_LAST_OCTET`](#arc-max-last-octet)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Arcs`](#arcs) | struct | [`Iterator`] over [`Arc`] values (a.k.a. |
| [`RootArcs`](#rootarcs) | struct | Byte containing the first and second arcs of an OID. |
| [`Arc`](#arc) | type | Type alias used to represent an "arc" (i.e. integer identifier value). |
| [`ARC_MAX_FIRST`](#arc-max-first) | const | Maximum value of the first arc in an OID. |
| [`ARC_MAX_SECOND`](#arc-max-second) | const | Maximum value of the second arc in an OID. |
| [`ARC_MAX_BYTES`](#arc-max-bytes) | const | Maximum number of bytes supported in an arc. |
| [`ARC_MAX_LAST_OCTET`](#arc-max-last-octet) | const | Maximum value of the last byte in an arc. |

## Structs

### `Arcs<'a>`

```rust
struct Arcs<'a> {
    oid: &'a crate::ObjectIdentifier,
    cursor: Option<usize>,
}
```

[`Iterator`](../../fallible_iterator/index.md) over [`Arc`](#arc) values (a.k.a. nodes) in an [`ObjectIdentifier`](../index.md).

This iterates over all arcs in an OID, including the root.

#### Fields

- **`oid`**: `&'a crate::ObjectIdentifier`

  OID we're iterating over

- **`cursor`**: `Option<usize>`

  Current position within the serialized DER bytes of this OID

#### Implementations

- <span id="arcs-new"></span>`fn new(oid: &'a ObjectIdentifier) -> Self` — [`ObjectIdentifier`](../index.md#objectidentifier)

  Create a new iterator over the arcs of this OID

- <span id="arcs-try-next"></span>`fn try_next(&mut self) -> Result<Option<Arc>>` — [`Result`](../error/index.md#result), [`Arc`](#arc)

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

- <span id="arcs-iterator-next"></span>`fn next(&mut self) -> Option<Arc>` — [`Arc`](#arc)

### `RootArcs`

```rust
struct RootArcs(u8);
```

Byte containing the first and second arcs of an OID.

This is represented this way in order to reduce the overall size of the
[`ObjectIdentifier`](../index.md) struct.

#### Implementations

- <span id="rootarcs-new"></span>`const fn new(first_arc: Arc, second_arc: Arc) -> Result<Self>` — [`Arc`](#arc), [`Result`](../error/index.md#result)

  Create [`RootArcs`](#rootarcs) from the first and second arc values represented

  as `Arc` integers.

- <span id="rootarcs-first-arc"></span>`const fn first_arc(self) -> Arc` — [`Arc`](#arc)

  Get the value of the first arc

- <span id="rootarcs-second-arc"></span>`const fn second_arc(self) -> Arc` — [`Arc`](#arc)

  Get the value of the second arc

#### Trait Implementations

##### `impl Clone for RootArcs`

- <span id="rootarcs-clone"></span>`fn clone(&self) -> RootArcs` — [`RootArcs`](#rootarcs)

##### `impl Copy for RootArcs`

##### `impl Debug for RootArcs`

- <span id="rootarcs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RootArcs`

##### `impl PartialEq for RootArcs`

- <span id="rootarcs-partialeq-eq"></span>`fn eq(&self, other: &RootArcs) -> bool` — [`RootArcs`](#rootarcs)

##### `impl StructuralPartialEq for RootArcs`

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

## Constants

### `ARC_MAX_FIRST`
```rust
const ARC_MAX_FIRST: Arc = 2u32;
```

Maximum value of the first arc in an OID.

### `ARC_MAX_SECOND`
```rust
const ARC_MAX_SECOND: Arc = 39u32;
```

Maximum value of the second arc in an OID.

### `ARC_MAX_BYTES`
```rust
const ARC_MAX_BYTES: usize = 4usize;
```

Maximum number of bytes supported in an arc.

### `ARC_MAX_LAST_OCTET`
```rust
const ARC_MAX_LAST_OCTET: u8 = 240u8;
```

Maximum value of the last byte in an arc.

