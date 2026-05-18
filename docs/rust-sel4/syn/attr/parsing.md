**syn > attr > parsing**

# Module: attr::parsing

## Contents

**Structs**

- [`DisplayAttrStyle`](#displayattrstyle)
- [`DisplayPath`](#displaypath)

**Functions**

- [`parse_inner`](#parse_inner)
- [`parse_meta_after_path`](#parse_meta_after_path)
- [`parse_meta_list_after_path`](#parse_meta_list_after_path)
- [`parse_meta_name_value_after_path`](#parse_meta_name_value_after_path)
- [`parse_outermost_meta_path`](#parse_outermost_meta_path)
- [`single_parse_inner`](#single_parse_inner)
- [`single_parse_outer`](#single_parse_outer)

---

## syn::attr::parsing::DisplayAttrStyle

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `(&'a crate::attr::AttrStyle)`

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`



## syn::attr::parsing::DisplayPath

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `(&'a crate::path::Path)`

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`



## syn::attr::parsing::parse_inner

*Function*

```rust
fn parse_inner(input: crate::parse::ParseStream, attrs: & mut alloc::vec::Vec<crate::attr::Attribute>) -> crate::error::Result<()>
```



## syn::attr::parsing::parse_meta_after_path

*Function*

```rust
fn parse_meta_after_path(path: crate::path::Path, input: crate::parse::ParseStream) -> crate::error::Result<crate::attr::Meta>
```



## syn::attr::parsing::parse_meta_list_after_path

*Function*

```rust
fn parse_meta_list_after_path(path: crate::path::Path, input: crate::parse::ParseStream) -> crate::error::Result<crate::attr::MetaList>
```



## syn::attr::parsing::parse_meta_name_value_after_path

*Function*

```rust
fn parse_meta_name_value_after_path(path: crate::path::Path, input: crate::parse::ParseStream) -> crate::error::Result<crate::attr::MetaNameValue>
```



## syn::attr::parsing::parse_outermost_meta_path

*Function*

```rust
fn parse_outermost_meta_path(input: crate::parse::ParseStream) -> crate::error::Result<crate::path::Path>
```



## syn::attr::parsing::single_parse_inner

*Function*

```rust
fn single_parse_inner(input: crate::parse::ParseStream) -> crate::error::Result<crate::attr::Attribute>
```



## syn::attr::parsing::single_parse_outer

*Function*

```rust
fn single_parse_outer(input: crate::parse::ParseStream) -> crate::error::Result<crate::attr::Attribute>
```



