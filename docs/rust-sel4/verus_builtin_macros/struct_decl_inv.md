**verus_builtin_macros > struct_decl_inv**

# Module: struct_decl_inv

## Contents

**Structs**

- [`FillInferVisitor`](#fillinfervisitor)
- [`FindPartialTypeVisitor`](#findpartialtypevisitor)
- [`InferVisitor`](#infervisitor)
- [`PartialField`](#partialfield)
- [`PartialType`](#partialtype)
- [`SDI`](#sdi)
- [`UsedParamsVisitor`](#usedparamsvisitor)

**Enums**

- [`InvariantDecl`](#invariantdecl)

**Functions**

- [`check_dep_in_fields`](#check_dep_in_fields)
- [`check_deps_acyclic`](#check_deps_acyclic)
- [`check_dupe_field_names`](#check_dupe_field_names)
- [`check_dupe_param_names`](#check_dupe_param_names)
- [`check_invdecl_params_match`](#check_invdecl_params_match)
- [`check_invs_match_partial_types`](#check_invs_match_partial_types)
- [`check_wf_sig`](#check_wf_sig)
- [`combine_errors_or_ok`](#combine_errors_or_ok)
- [`count_infers`](#count_infers)
- [`field_used_type_params`](#field_used_type_params)
- [`fields_contains`](#fields_contains)
- [`fill_in_infers`](#fill_in_infers)
- [`fill_in_item_struct`](#fill_in_item_struct)
- [`fill_in_type`](#fill_in_type)
- [`generic_param_to_string`](#generic_param_to_string)
- [`get_builtin_concrete_arg`](#get_builtin_concrete_arg)
- [`get_concrete_args`](#get_concrete_args)
- [`get_constant_type`](#get_constant_type)
- [`get_field_by_name`](#get_field_by_name)
- [`get_fields`](#get_fields)
- [`get_invariant_decls_by_name`](#get_invariant_decls_by_name)
- [`get_params_used_in_type`](#get_params_used_in_type)
- [`get_partial_field_by_name`](#get_partial_field_by_name)
- [`get_partial_type_fields`](#get_partial_type_fields)
- [`get_partial_types`](#get_partial_types)
- [`get_pred_typename`](#get_pred_typename)
- [`get_self_type`](#get_self_type)
- [`get_type_alias`](#get_type_alias)
- [`is_first_param_self`](#is_first_param_self)
- [`is_spec`](#is_spec)
- [`keyword`](#keyword)
- [`maybe_tuple`](#maybe_tuple)
- [`output_field_type_alias`](#output_field_type_alias)
- [`output_invariant`](#output_invariant)
- [`output_wf`](#output_wf)
- [`parse_quants`](#parse_quants)
- [`peek_keyword`](#peek_keyword)
- [`remove_bounds`](#remove_bounds)
- [`remove_bounds_vec`](#remove_bounds_vec)
- [`struct_decl_inv`](#struct_decl_inv)
- [`struct_decl_inv_main`](#struct_decl_inv_main)

---

## verus_builtin_macros::struct_decl_inv::FillInferVisitor

*Struct*

**Fields:**
- `v: std::vec::IntoIter<verus_syn::Type>`

**Trait Implementations:**

- **VisitMut**
  - `fn visit_type_mut(self: & mut Self, ty: & mut Type)`



## verus_builtin_macros::struct_decl_inv::FindPartialTypeVisitor

*Struct*

**Fields:**
- `ptypes: Vec<PartialType>`
- `errors: Vec<verus_syn::Error>`

**Trait Implementations:**

- **Visit**
  - `fn visit_type_path(self: & mut Self, type_path: &'ast TypePath)`
  - `fn visit_type_infer(self: & mut Self, infer: &TypeInfer)`



## verus_builtin_macros::struct_decl_inv::InferVisitor

*Struct*

**Fields:**
- `count: usize`

**Trait Implementations:**

- **Visit**
  - `fn visit_type_infer(self: & mut Self, _i: &'ast TypeInfer)`



## verus_builtin_macros::struct_decl_inv::InvariantDecl

*Enum*

**Variants:**
- `NormalExpr(verus_syn::Block)`
- `Invariant{ field_name: verus_syn::Ident, depends_on: Vec<verus_syn::Ident>, quants: Vec<verus_syn::PatType>, condition: Option<Box<verus_syn::Expr>>, specifically: Option<Box<verus_syn::Expr>>, params: Vec<verus_syn::FnArg>, params_span: proc_macro2::Span, predicate: verus_syn::Block }`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> parse::Result<InvariantDecl>`



## verus_builtin_macros::struct_decl_inv::PartialField

*Struct*

**Fields:**
- `name: verus_syn::Ident`
- `partial_types: Vec<PartialType>`



## verus_builtin_macros::struct_decl_inv::PartialType

*Struct*

**Fields:**
- `is_atomic_ghost: bool`
- `concrete_args: Vec<verus_syn::Type>`



## verus_builtin_macros::struct_decl_inv::SDI

*Struct*

**Fields:**
- `item_struct: verus_syn::ItemStruct`
- `wf_attrs: Vec<verus_syn::Attribute>`
- `wf_vis: verus_syn::Visibility`
- `wf_sig: verus_syn::Signature`
- `invariant_decls: Vec<InvariantDecl>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> parse::Result<SDI>`



## verus_builtin_macros::struct_decl_inv::UsedParamsVisitor

*Struct*

**Fields:**
- `params: std::collections::HashSet<String>`
- `result: std::collections::HashSet<String>`

**Trait Implementations:**

- **Visit**
  - `fn visit_type_path(self: & mut Self, type_path: &TypePath)`
  - `fn visit_lifetime(self: & mut Self, lt: &Lifetime)`



## verus_builtin_macros::struct_decl_inv::check_dep_in_fields

*Function*

```rust
fn check_dep_in_fields(fields: &Vec<verus_syn::Field>, dep: &verus_syn::Ident) -> parse::Result<()>
```



## verus_builtin_macros::struct_decl_inv::check_deps_acyclic

*Function*

```rust
fn check_deps_acyclic(fields: &Vec<verus_syn::Field>, invs: &Vec<InvariantDecl>) -> parse::Result<Vec<String>>
```



## verus_builtin_macros::struct_decl_inv::check_dupe_field_names

*Function*

```rust
fn check_dupe_field_names(fields: &Vec<verus_syn::Field>) -> parse::Result<()>
```



## verus_builtin_macros::struct_decl_inv::check_dupe_param_names

*Function*

```rust
fn check_dupe_param_names(sdi: &SDI) -> parse::Result<()>
```



## verus_builtin_macros::struct_decl_inv::check_invdecl_params_match

*Function*

```rust
fn check_invdecl_params_match(params_span: &proc_macro2::Span, params: &Vec<verus_syn::FnArg>, partial_type: &PartialType) -> parse::Result<()>
```



## verus_builtin_macros::struct_decl_inv::check_invs_match_partial_types

*Function*

```rust
fn check_invs_match_partial_types(all_fields: &Vec<verus_syn::Field>, partial_fields: &Vec<PartialField>, invariant_decls: &Vec<InvariantDecl>) -> parse::Result<()>
```



## verus_builtin_macros::struct_decl_inv::check_wf_sig

*Function*

```rust
fn check_wf_sig(sig: &verus_syn::Signature) -> parse::Result<()>
```



## verus_builtin_macros::struct_decl_inv::combine_errors_or_ok

*Function*

```rust
fn combine_errors_or_ok(errors: Vec<verus_syn::Error>) -> parse::Result<()>
```



## verus_builtin_macros::struct_decl_inv::count_infers

*Function*

```rust
fn count_infers(ty: &verus_syn::Type) -> usize
```



## verus_builtin_macros::struct_decl_inv::field_used_type_params

*Function*

```rust
fn field_used_type_params(sdi: &SDI, fields: &Vec<verus_syn::Field>, ordering: &Vec<String>) -> std::collections::HashMap<String, Vec<verus_syn::GenericParam>>
```



## verus_builtin_macros::struct_decl_inv::fields_contains

*Function*

```rust
fn fields_contains(fields: &Vec<verus_syn::Field>, name: &str) -> bool
```



## verus_builtin_macros::struct_decl_inv::fill_in_infers

*Function*

```rust
fn fill_in_infers(ty: &verus_syn::Type, v: Vec<verus_syn::Type>) -> verus_syn::Type
```



## verus_builtin_macros::struct_decl_inv::fill_in_item_struct

*Function*

```rust
fn fill_in_item_struct(main_name: &str, item_struct: & mut verus_syn::ItemStruct, invariant_decls: &Vec<InvariantDecl>, used_type_params: &std::collections::HashMap<String, Vec<verus_syn::GenericParam>>)
```



## verus_builtin_macros::struct_decl_inv::fill_in_type

*Function*

```rust
fn fill_in_type(ty: &verus_syn::Type, main_name: &str, inv_decls: Vec<&InvariantDecl>, used_type_params: &std::collections::HashMap<String, Vec<verus_syn::GenericParam>>) -> verus_syn::Type
```



## verus_builtin_macros::struct_decl_inv::generic_param_to_string

*Function*

```rust
fn generic_param_to_string(gp: &verus_syn::GenericParam) -> String
```



## verus_builtin_macros::struct_decl_inv::get_builtin_concrete_arg

*Function*

```rust
fn get_builtin_concrete_arg(name: &str) -> Option<verus_syn::Type>
```



## verus_builtin_macros::struct_decl_inv::get_concrete_args

*Function*

```rust
fn get_concrete_args(type_path: &verus_syn::TypePath) -> parse::Result<Option<PartialType>>
```



## verus_builtin_macros::struct_decl_inv::get_constant_type

*Function*

```rust
fn get_constant_type(main_name: &str, depends_on: &Vec<verus_syn::Ident>, quants: &Vec<verus_syn::PatType>, used_type_params: &std::collections::HashMap<String, Vec<verus_syn::GenericParam>>) -> verus_syn::Type
```



## verus_builtin_macros::struct_decl_inv::get_field_by_name

*Function*

```rust
fn get_field_by_name<'a>(fields: &'a Vec<verus_syn::Field>, name: &str) -> Option<&'a verus_syn::Field>
```



## verus_builtin_macros::struct_decl_inv::get_fields

*Function*

```rust
fn get_fields(f: &verus_syn::Fields) -> parse::Result<Vec<verus_syn::Field>>
```



## verus_builtin_macros::struct_decl_inv::get_invariant_decls_by_name

*Function*

```rust
fn get_invariant_decls_by_name<'a>(invariant_decls: &'a Vec<InvariantDecl>, name: &str) -> Vec<&'a InvariantDecl>
```



## verus_builtin_macros::struct_decl_inv::get_params_used_in_type

*Function*

```rust
fn get_params_used_in_type(params: &verus_syn::punctuated::Punctuated<verus_syn::GenericParam, verus_syn::token::Comma>, ty: &verus_syn::Type) -> std::collections::HashSet<String>
```



## verus_builtin_macros::struct_decl_inv::get_partial_field_by_name

*Function*

```rust
fn get_partial_field_by_name<'a>(partial_fields: &'a Vec<PartialField>, name: &str) -> Option<&'a PartialField>
```



## verus_builtin_macros::struct_decl_inv::get_partial_type_fields

*Function*

```rust
fn get_partial_type_fields(fields: &Vec<verus_syn::Field>) -> parse::Result<Vec<PartialField>>
```



## verus_builtin_macros::struct_decl_inv::get_partial_types

*Function*

```rust
fn get_partial_types(ty: &verus_syn::Type) -> parse::Result<Vec<PartialType>>
```



## verus_builtin_macros::struct_decl_inv::get_pred_typename

*Function*

```rust
fn get_pred_typename(main_name: &str, field_name: &verus_syn::Ident) -> verus_syn::Ident
```



## verus_builtin_macros::struct_decl_inv::get_self_type

*Function*

```rust
fn get_self_type(item_struct: &verus_syn::ItemStruct) -> verus_syn::Type
```



## verus_builtin_macros::struct_decl_inv::get_type_alias

*Function*

```rust
fn get_type_alias(main_name: &str, field_ident: &verus_syn::Ident, used_type_params: &std::collections::HashMap<String, Vec<verus_syn::GenericParam>>) -> proc_macro2::TokenStream
```



## verus_builtin_macros::struct_decl_inv::is_first_param_self

*Function*

```rust
fn is_first_param_self(sig: &verus_syn::Signature) -> bool
```



## verus_builtin_macros::struct_decl_inv::is_spec

*Function*

```rust
fn is_spec(sig: &verus_syn::Signature) -> bool
```



## verus_builtin_macros::struct_decl_inv::keyword

*Function*

```rust
fn keyword(input: verus_syn::parse::ParseStream, token: &str) -> parse::Result<proc_macro2::Span>
```



## verus_builtin_macros::struct_decl_inv::maybe_tuple

*Function*

```rust
fn maybe_tuple<T>(v: &Vec<T>) -> proc_macro2::TokenStream
```



## verus_builtin_macros::struct_decl_inv::output_field_type_alias

*Function*

```rust
fn output_field_type_alias(main_name: &str, vis: &verus_syn::Visibility, stream: & mut proc_macro2::TokenStream, field: &verus_syn::Field, used_type_params: &std::collections::HashMap<String, Vec<verus_syn::GenericParam>>)
```



## verus_builtin_macros::struct_decl_inv::output_invariant

*Function*

```rust
fn output_invariant(main_name: &str, sdi: &SDI, stream: & mut proc_macro2::TokenStream, wf_stream: & mut proc_macro2::TokenStream, partial_fields: &Vec<PartialField>, invariant_decl: &InvariantDecl, used_type_params: &std::collections::HashMap<String, Vec<verus_syn::GenericParam>>)
```



## verus_builtin_macros::struct_decl_inv::output_wf

*Function*

```rust
fn output_wf(sdi: &SDI, stream: & mut proc_macro2::TokenStream, wf_body_stream: proc_macro2::TokenStream)
```



## verus_builtin_macros::struct_decl_inv::parse_quants

*Function*

```rust
fn parse_quants(input: verus_syn::parse::ParseStream) -> parse::Result<Vec<verus_syn::PatType>>
```



## verus_builtin_macros::struct_decl_inv::peek_keyword

*Function*

```rust
fn peek_keyword(cursor: verus_syn::buffer::Cursor, token: &str) -> bool
```



## verus_builtin_macros::struct_decl_inv::remove_bounds

*Function*

```rust
fn remove_bounds(p: &verus_syn::punctuated::Punctuated<verus_syn::GenericParam, verus_syn::token::Comma>) -> proc_macro2::TokenStream
```



## verus_builtin_macros::struct_decl_inv::remove_bounds_vec

*Function*

```rust
fn remove_bounds_vec(p: &Vec<verus_syn::GenericParam>) -> proc_macro2::TokenStream
```



## verus_builtin_macros::struct_decl_inv::struct_decl_inv

*Function*

```rust
fn struct_decl_inv(input: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::struct_decl_inv::struct_decl_inv_main

*Function*

```rust
fn struct_decl_inv_main(sdi: SDI) -> parse::Result<proc_macro2::TokenStream>
```



