*[num_enum_derive](../index.md) / [enum_attributes](index.md)*

---

# Module `enum_attributes`

## Contents

- [Modules](#modules)
  - [`kw`](#kw)
- [Structs](#structs)
  - [`Attributes`](#attributes)
  - [`ErrorTypeAttribute`](#errortypeattribute)
  - [`ErrorTypeNameAttribute`](#errortypenameattribute)
  - [`ErrorTypeConstructorAttribute`](#errortypeconstructorattribute)
  - [`CrateAttribute`](#crateattribute)
- [Enums](#enums)
  - [`AttributeItem`](#attributeitem)
  - [`ErrorTypeAttributeNamedArgument`](#errortypeattributenamedargument)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`kw`](#kw) | mod |  |
| [`Attributes`](#attributes) | struct |  |
| [`ErrorTypeAttribute`](#errortypeattribute) | struct |  |
| [`ErrorTypeNameAttribute`](#errortypenameattribute) | struct |  |
| [`ErrorTypeConstructorAttribute`](#errortypeconstructorattribute) | struct |  |
| [`CrateAttribute`](#crateattribute) | struct |  |
| [`AttributeItem`](#attributeitem) | enum |  |
| [`ErrorTypeAttributeNamedArgument`](#errortypeattributenamedargument) | enum |  |

## Modules

- [`kw`](kw/index.md)

## Structs

### `Attributes`

```rust
struct Attributes {
    error_type: Option<ErrorTypeAttribute>,
    crate_path: Option<CrateAttribute>,
}
```

#### Implementations

- <span id="attributes-exclusive-union"></span>`fn exclusive_union(&mut self, other: Self) -> Result<()>`

#### Trait Implementations

##### `impl Default for Attributes`

- <span id="attributes-default"></span>`fn default() -> Attributes` — [`Attributes`](#attributes)

##### `impl Parse for Attributes`

- <span id="attributes-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

### `ErrorTypeAttribute`

```rust
struct ErrorTypeAttribute {
    name: ErrorTypeNameAttribute,
    constructor: ErrorTypeConstructorAttribute,
    span: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl Clone for ErrorTypeAttribute`

- <span id="errortypeattribute-clone"></span>`fn clone(&self) -> ErrorTypeAttribute` — [`ErrorTypeAttribute`](#errortypeattribute)

##### `impl Parse for ErrorTypeAttribute`

- <span id="errortypeattribute-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

### `ErrorTypeNameAttribute`

```rust
struct ErrorTypeNameAttribute {
    path: syn::Path,
}
```

#### Trait Implementations

##### `impl Clone for ErrorTypeNameAttribute`

- <span id="errortypenameattribute-clone"></span>`fn clone(&self) -> ErrorTypeNameAttribute` — [`ErrorTypeNameAttribute`](#errortypenameattribute)

##### `impl Parse for ErrorTypeNameAttribute`

- <span id="errortypenameattribute-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

### `ErrorTypeConstructorAttribute`

```rust
struct ErrorTypeConstructorAttribute {
    path: syn::Path,
}
```

#### Trait Implementations

##### `impl Clone for ErrorTypeConstructorAttribute`

- <span id="errortypeconstructorattribute-clone"></span>`fn clone(&self) -> ErrorTypeConstructorAttribute` — [`ErrorTypeConstructorAttribute`](#errortypeconstructorattribute)

##### `impl Parse for ErrorTypeConstructorAttribute`

- <span id="errortypeconstructorattribute-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

### `CrateAttribute`

```rust
struct CrateAttribute {
    path: syn::Path,
    span: proc_macro2::Span,
}
```

#### Trait Implementations

##### `impl Clone for CrateAttribute`

- <span id="crateattribute-clone"></span>`fn clone(&self) -> CrateAttribute` — [`CrateAttribute`](#crateattribute)

##### `impl Parse for CrateAttribute`

- <span id="crateattribute-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

## Enums

### `AttributeItem`

```rust
enum AttributeItem {
    ErrorType(ErrorTypeAttribute),
    CratePath(CrateAttribute),
}
```

#### Trait Implementations

##### `impl Parse for AttributeItem`

- <span id="attributeitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

### `ErrorTypeAttributeNamedArgument`

```rust
enum ErrorTypeAttributeNamedArgument {
    Name(ErrorTypeNameAttribute),
    Constructor(ErrorTypeConstructorAttribute),
}
```

#### Trait Implementations

##### `impl Parse for ErrorTypeAttributeNamedArgument`

- <span id="errortypeattributenamedargument-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

