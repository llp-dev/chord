*[sel4_abstract_ptr](../index.md) / [access](index.md)*

---

# Module `access`

Marker types for limiting access.

## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`ReadWrite`](#readwrite)
  - [`ReadOnly`](#readonly)
  - [`WriteOnly`](#writeonly)
  - [`NoAccess`](#noaccess)
- [Traits](#traits)
  - [`RestrictAccess`](#restrictaccess)
  - [`Access`](#access)
  - [`Readable`](#readable)
  - [`Writable`](#writable)
  - [`Copyable`](#copyable)
- [Macros](#macros)
  - [`restrict_impl!`](#restrict-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`ReadWrite`](#readwrite) | struct | Zero-sized marker type for allowing both read and write access. |
| [`ReadOnly`](#readonly) | struct | Zero-sized marker type for allowing only read access. |
| [`WriteOnly`](#writeonly) | struct | Zero-sized marker type for allowing only write access. |
| [`NoAccess`](#noaccess) | struct | Zero-sized marker type that grants no access. |
| [`RestrictAccess`](#restrictaccess) | trait | A trait for restricting one [`Access`] type to another [`Access`] type. |
| [`Access`](#access) | trait | Sealed trait that is implemented for the types in this module. |
| [`Readable`](#readable) | trait | Helper trait that is implemented by [`ReadWrite`] and [`ReadOnly`]. |
| [`Writable`](#writable) | trait | Helper trait that is implemented by [`ReadWrite`] and [`WriteOnly`]. |
| [`Copyable`](#copyable) | trait | Implemented for access types that permit copying of `AbstractRef`. |
| [`restrict_impl!`](#restrict-impl) | macro |  |

## Modules

- [`private`](private/index.md)

## Structs

### `ReadWrite`

```rust
struct ReadWrite;
```

Zero-sized marker type for allowing both read and write access.

#### Trait Implementations

##### `impl Access for ReadWrite`

##### `impl Clone for ReadWrite`

- <span id="readwrite-clone"></span>`fn clone(&self) -> ReadWrite` — [`ReadWrite`](#readwrite)

##### `impl Copy for ReadWrite`

##### `impl Debug for ReadWrite`

- <span id="readwrite-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ReadWrite`

- <span id="readwrite-default"></span>`fn default() -> ReadWrite` — [`ReadWrite`](#readwrite)

##### `impl Pointee for ReadWrite`

- <span id="readwrite-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Readable for ReadWrite`

##### `impl<To: Access> RestrictAccess for ReadWrite`

- <span id="readwrite-restrictaccess-type-restricted"></span>`type Restricted = To`

##### `impl Sealed for super::ReadWrite`

##### `impl Writable for ReadWrite`

### `ReadOnly`

```rust
struct ReadOnly;
```

Zero-sized marker type for allowing only read access.

#### Trait Implementations

##### `impl Access for ReadOnly`

##### `impl Clone for ReadOnly`

- <span id="readonly-clone"></span>`fn clone(&self) -> ReadOnly` — [`ReadOnly`](#readonly)

##### `impl Copy for ReadOnly`

##### `impl Copyable for ReadOnly`

##### `impl Debug for ReadOnly`

- <span id="readonly-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ReadOnly`

- <span id="readonly-default"></span>`fn default() -> ReadOnly` — [`ReadOnly`](#readonly)

##### `impl Pointee for ReadOnly`

- <span id="readonly-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Readable for ReadOnly`

##### `impl RestrictAccess for ReadOnly`

- <span id="readonly-restrictaccess-type-restricted"></span>`type Restricted = ReadOnly`

##### `impl Sealed for super::ReadOnly`

### `WriteOnly`

```rust
struct WriteOnly;
```

Zero-sized marker type for allowing only write access.

#### Trait Implementations

##### `impl Access for WriteOnly`

##### `impl Clone for WriteOnly`

- <span id="writeonly-clone"></span>`fn clone(&self) -> WriteOnly` — [`WriteOnly`](#writeonly)

##### `impl Copy for WriteOnly`

##### `impl Debug for WriteOnly`

- <span id="writeonly-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WriteOnly`

- <span id="writeonly-default"></span>`fn default() -> WriteOnly` — [`WriteOnly`](#writeonly)

##### `impl Pointee for WriteOnly`

- <span id="writeonly-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl RestrictAccess for ReadOnly`

- <span id="readonly-restrictaccess-type-restricted"></span>`type Restricted = NoAccess`

##### `impl Sealed for super::WriteOnly`

##### `impl Writable for WriteOnly`

### `NoAccess`

```rust
struct NoAccess;
```

Zero-sized marker type that grants no access.

#### Trait Implementations

##### `impl Access for NoAccess`

##### `impl Clone for NoAccess`

- <span id="noaccess-clone"></span>`fn clone(&self) -> NoAccess` — [`NoAccess`](#noaccess)

##### `impl Copy for NoAccess`

##### `impl Copyable for NoAccess`

##### `impl Debug for NoAccess`

- <span id="noaccess-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NoAccess`

- <span id="noaccess-default"></span>`fn default() -> NoAccess` — [`NoAccess`](#noaccess)

##### `impl Pointee for NoAccess`

- <span id="noaccess-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl<To> RestrictAccess for NoAccess`

- <span id="noaccess-restrictaccess-type-restricted"></span>`type Restricted = NoAccess`

##### `impl Sealed for super::NoAccess`

## Traits

### `RestrictAccess<To>`

```rust
trait RestrictAccess<To>: Access { ... }
```

A trait for restricting one [`Access`](#access) type to another [`Access`](#access) type.

Restricting `Self` to `To` results in `Self::Restricted`.

Restriction is a symmetric operation which is denoted by ∩, as it is the intersection of permissions.
The following table holds:

| `Self`        | `To`          | `Self` ∩ `To` |
| ------------- | ------------- | ------------- |
| `T`           | `T`           | `T`           |
| [`ReadWrite`](#readwrite) | `T`           | `T`           |
| [`NoAccess`](#noaccess)  | `T`           | [`NoAccess`](#noaccess)  |
| [`ReadOnly`](#readonly)  | [`WriteOnly`](#writeonly) | [`NoAccess`](#noaccess)  |

#### Associated Types

- `type Restricted: 1`

#### Implementors

- [`NoAccess`](#noaccess)
- [`ReadOnly`](#readonly)
- [`ReadWrite`](#readwrite)
- [`WriteOnly`](#writeonly)

### `Access`

```rust
trait Access: Copy + Default + private::Sealed { ... }
```

Sealed trait that is implemented for the types in this module.

#### Implementors

- [`NoAccess`](#noaccess)
- [`ReadOnly`](#readonly)
- [`ReadWrite`](#readwrite)
- [`WriteOnly`](#writeonly)

### `Readable`

```rust
trait Readable: Access { ... }
```

Helper trait that is implemented by [`ReadWrite`](#readwrite) and [`ReadOnly`](#readonly).

#### Implementors

- `A`

### `Writable`

```rust
trait Writable: Access { ... }
```

Helper trait that is implemented by [`ReadWrite`](#readwrite) and [`WriteOnly`](#writeonly).

#### Implementors

- `A`

### `Copyable`

```rust
trait Copyable: Access { ... }
```

Implemented for access types that permit copying of `AbstractRef`.

#### Implementors

- `A`

## Macros

### `restrict_impl!`

