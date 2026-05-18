# Crate `num_enum`

See <https://docs.rs/num_enum> for more info about this crate.

## Contents

- [Modules](#modules)
  - [`Default`](#default)
- [Structs](#structs)
  - [`UnsafeFromPrimitive`](#unsafefromprimitive)
  - [`TryFromPrimitiveError`](#tryfromprimitiveerror)
- [Traits](#traits)
  - [`FromPrimitive`](#fromprimitive)
  - [`TryFromPrimitive`](#tryfromprimitive)
  - [`UnsafeFromPrimitive`](#unsafefromprimitive)
- [Functions](#functions)
  - [`FromPrimitive`](#fromprimitive)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Default`](#default) | mod |  |
| [`UnsafeFromPrimitive`](#unsafefromprimitive) | struct |  |
| [`TryFromPrimitiveError`](#tryfromprimitiveerror) | struct |  |
| [`FromPrimitive`](#fromprimitive) | trait |  |
| [`TryFromPrimitive`](#tryfromprimitive) | trait |  |
| [`UnsafeFromPrimitive`](#unsafefromprimitive) | trait |  |
| [`FromPrimitive`](#fromprimitive) | fn |  |

## Modules

- [`Default`](Default/index.md)

## Structs

### `UnsafeFromPrimitive<'a>`

```rust
struct UnsafeFromPrimitive<'a> {
    pub file: Option<&'a str>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}
```

*Re-exported from `addr2line`*

A source location.

#### Fields

- **`file`**: `Option<&'a str>`

  The file name.

- **`line`**: `Option<u32>`

  The line number.

- **`column`**: `Option<u32>`

  The column number.
  
  A value of `Some(0)` indicates the left edge.

### `TryFromPrimitiveError<Enum: TryFromPrimitive>`

```rust
struct TryFromPrimitiveError<Enum: TryFromPrimitive> {
    pub number: <Enum as >::Primitive,
}
```

#### Implementations

- <span id="tryfromprimitiveerror-new"></span>`fn new(number: <Enum as >::Primitive) -> Self` — [`TryFromPrimitive`](#tryfromprimitive)

#### Trait Implementations

##### `impl<Enum: clone::Clone + TryFromPrimitive> Clone for TryFromPrimitiveError<Enum>`

- <span id="tryfromprimitiveerror-clone"></span>`fn clone(&self) -> TryFromPrimitiveError<Enum>` — [`TryFromPrimitiveError`](#tryfromprimitiveerror)

##### `impl<Enum: marker::Copy + TryFromPrimitive> Copy for TryFromPrimitiveError<Enum>`

##### `impl<Enum: TryFromPrimitive> Debug for TryFromPrimitiveError<Enum>`

- <span id="tryfromprimitiveerror-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Enum: TryFromPrimitive> Display for TryFromPrimitiveError<Enum>`

- <span id="tryfromprimitiveerror-display-fmt"></span>`fn fmt(&self, stream: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Enum: cmp::Eq + TryFromPrimitive> Eq for TryFromPrimitiveError<Enum>`

##### `impl<Enum: TryFromPrimitive> Error for TryFromPrimitiveError<Enum>`

##### `impl<Enum: cmp::PartialEq + TryFromPrimitive> PartialEq for TryFromPrimitiveError<Enum>`

- <span id="tryfromprimitiveerror-partialeq-eq"></span>`fn eq(&self, other: &TryFromPrimitiveError<Enum>) -> bool` — [`TryFromPrimitiveError`](#tryfromprimitiveerror)

##### `impl<Enum: TryFromPrimitive> StructuralPartialEq for TryFromPrimitiveError<Enum>`

## Traits

### `FromPrimitive`

```rust
trait FromPrimitive: Sized { ... }
```

#### Associated Types

- `type Primitive: 2`

#### Required Methods

- `fn from_primitive(number: <Self as >::Primitive) -> Self`

### `TryFromPrimitive`

```rust
trait TryFromPrimitive: Sized { ... }
```

#### Associated Types

- `type Primitive: 3`

- `type Error`

#### Associated Constants

- `const NAME: &'static str`

#### Required Methods

- `fn try_from_primitive(number: <Self as >::Primitive) -> Result<Self, <Self as >::Error>`

### `UnsafeFromPrimitive`

```rust
trait UnsafeFromPrimitive: Sized { ... }
```

#### Associated Types

- `type Primitive: 2`

#### Required Methods

- `fn unchecked_transmute_from(number: <Self as >::Primitive) -> Self`

  Transmutes into an enum from its primitive.

#### Provided Methods

- `fn from_unchecked(number: <Self as >::Primitive) -> Self`

  Transmutes into an enum from its primitive.

## Functions

### `FromPrimitive`

```rust
fn FromPrimitive(bytes: &[u8]) -> Self
```

