*[zerocopy_derive](../../index.md) / [derive](../index.md) / [try_from_bytes](index.md)*

---

# Module `try_from_bytes`

## Contents

- [Functions](#functions)
  - [`tag_ident`](#tag-ident)
  - [`generate_tag_consts`](#generate-tag-consts)
  - [`variant_struct_ident`](#variant-struct-ident)
  - [`generate_variant_structs`](#generate-variant-structs)
  - [`variants_union_field_ident`](#variants-union-field-ident)
  - [`generate_variants_union`](#generate-variants-union)
  - [`derive_is_bit_valid`](#derive-is-bit-valid)
  - [`derive_try_from_bytes`](#derive-try-from-bytes)
  - [`derive_has_field_struct_union`](#derive-has-field-struct-union)
  - [`derive_try_from_bytes_struct`](#derive-try-from-bytes-struct)
  - [`derive_try_from_bytes_union`](#derive-try-from-bytes-union)
  - [`derive_try_from_bytes_enum`](#derive-try-from-bytes-enum)
  - [`try_gen_trivial_is_bit_valid`](#try-gen-trivial-is-bit-valid)
  - [`gen_trivial_is_bit_valid_unchecked`](#gen-trivial-is-bit-valid-unchecked)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`tag_ident`](#tag-ident) | fn |  |
| [`generate_tag_consts`](#generate-tag-consts) | fn | Generates a constant for the tag associated with each variant of the enum. |
| [`variant_struct_ident`](#variant-struct-ident) | fn |  |
| [`generate_variant_structs`](#generate-variant-structs) | fn | Generates variant structs for the given enum variant. |
| [`variants_union_field_ident`](#variants-union-field-ident) | fn |  |
| [`generate_variants_union`](#generate-variants-union) | fn |  |
| [`derive_is_bit_valid`](#derive-is-bit-valid) | fn | Generates an implementation of `is_bit_valid` for an arbitrary enum. |
| [`derive_try_from_bytes`](#derive-try-from-bytes) | fn |  |
| [`derive_has_field_struct_union`](#derive-has-field-struct-union) | fn |  |
| [`derive_try_from_bytes_struct`](#derive-try-from-bytes-struct) | fn |  |
| [`derive_try_from_bytes_union`](#derive-try-from-bytes-union) | fn |  |
| [`derive_try_from_bytes_enum`](#derive-try-from-bytes-enum) | fn |  |
| [`try_gen_trivial_is_bit_valid`](#try-gen-trivial-is-bit-valid) | fn |  |
| [`gen_trivial_is_bit_valid_unchecked`](#gen-trivial-is-bit-valid-unchecked) | fn | # Safety |

## Functions

### `tag_ident`

```rust
fn tag_ident(variant_ident: &syn::Ident) -> syn::Ident
```

### `generate_tag_consts`

```rust
fn generate_tag_consts(data: &syn::DataEnum) -> proc_macro2::TokenStream
```

Generates a constant for the tag associated with each variant of the enum.
When we match on the enum's tag, each arm matches one of these constants. We
have to use constants here because:

- The type that we're matching on is not the type of the tag, it's an
  integer of the same size as the tag type and with the same bit patterns.
- We can't read the enum tag as an enum because the bytes may not represent
  a valid variant.
- Patterns do not currently support const expressions, so we have to assign
  these constants to names rather than use them inline in the `match`
  statement.

### `variant_struct_ident`

```rust
fn variant_struct_ident(variant_ident: &syn::Ident) -> syn::Ident
```

### `generate_variant_structs`

```rust
fn generate_variant_structs(ctx: &crate::util::Ctx, data: &syn::DataEnum) -> proc_macro2::TokenStream
```

Generates variant structs for the given enum variant.

These are structs associated with each variant of an enum. They are
`repr(C)` tuple structs with the same fields as the variant after a
`MaybeUninit<___ZerocopyInnerTag>`.

In order to unify the generated types for `repr(C)` and `repr(int)` enums,
we use a "fused" representation with fields for both an inner tag and an
outer tag. Depending on the repr, we will set one of these tags to the tag
type and the other to `()`. This lets us generate the same code but put the
tags in different locations.

### `variants_union_field_ident`

```rust
fn variants_union_field_ident(ident: &syn::Ident) -> syn::Ident
```

### `generate_variants_union`

```rust
fn generate_variants_union(ctx: &crate::util::Ctx, data: &syn::DataEnum) -> proc_macro2::TokenStream
```

### `derive_is_bit_valid`

```rust
fn derive_is_bit_valid(ctx: &crate::util::Ctx, data: &syn::DataEnum, repr: &Repr<PrimitiveRepr, core::convert::Infallible>) -> Result<proc_macro2::TokenStream, syn::Error>
```

Generates an implementation of `is_bit_valid` for an arbitrary enum.

The general process is:

1. Generate a tag enum. This is an enum with the same repr, variants, and
   corresponding discriminants as the original enum, but without any fields
   on the variants. This gives us access to an enum where the variants have
   the same discriminants as the one we're writing `is_bit_valid` for.
2. Make constants from the variants of the tag enum. We need these because
   we can't put const exprs in match arms.
3. Generate variant structs. These are structs which have the same fields as
   each variant of the enum, and are `#[repr(C)]` with an optional "inner
   tag".
4. Generate a variants union, with one field for each variant struct type.
5. And finally, our raw enum is a `#[repr(C)]` struct of an "outer tag" and
   the variants union.

See these reference links for fully-worked example decompositions.

- `repr(C)`: <https://doc.rust-lang.org/reference/type-layout.html#reprc-enums-with-fields>
- `repr(int)`: <https://doc.rust-lang.org/reference/type-layout.html#primitive-representation-of-enums-with-fields>
- `repr(C, int)`: <https://doc.rust-lang.org/reference/type-layout.html#combining-primitive-representations-of-enums-with-fields-and-reprc>

### `derive_try_from_bytes`

```rust
fn derive_try_from_bytes(ctx: &crate::util::Ctx, top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_has_field_struct_union`

```rust
fn derive_has_field_struct_union(ctx: &crate::util::Ctx, data: &dyn DataExt) -> proc_macro2::TokenStream
```

### `derive_try_from_bytes_struct`

```rust
fn derive_try_from_bytes_struct(ctx: &crate::util::Ctx, strct: &syn::DataStruct, top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_try_from_bytes_union`

```rust
fn derive_try_from_bytes_union(ctx: &crate::util::Ctx, unn: &syn::DataUnion, top_level: crate::util::Trait) -> proc_macro2::TokenStream
```

### `derive_try_from_bytes_enum`

```rust
fn derive_try_from_bytes_enum(ctx: &crate::util::Ctx, enm: &syn::DataEnum, top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `try_gen_trivial_is_bit_valid`

```rust
fn try_gen_trivial_is_bit_valid(ctx: &crate::util::Ctx, top_level: crate::util::Trait) -> Option<proc_macro2::TokenStream>
```

### `gen_trivial_is_bit_valid_unchecked`

```rust
unsafe fn gen_trivial_is_bit_valid_unchecked(ctx: &crate::util::Ctx) -> proc_macro2::TokenStream
```

# Safety

All initialized bit patterns must be valid for `Self`.

