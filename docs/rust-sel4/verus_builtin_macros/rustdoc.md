**verus_builtin_macros > rustdoc**

# Module: rustdoc

## Contents

**Functions**

- [`assume_specification_link_line`](#assume_specification_link_line) - Get the assume_specification line
- [`attr_for_broadcast_group`](#attr_for_broadcast_group)
- [`attr_for_sig`](#attr_for_sig) - Process a signature to get all the information, apply the codeblock
- [`doc_attr_from_string`](#doc_attr_from_string) - Create an attr that looks like #[doc = "doc_str"]
- [`encoded_body`](#encoded_body)
- [`encoded_expr`](#encoded_expr) - Pretty print the expression, then wrap in a code block.
- [`encoded_sig_info`](#encoded_sig_info)
- [`encoded_str`](#encoded_str) - Wrap the given string into a code block,
- [`env_rustdoc`](#env_rustdoc) - Check if VERUSDOC=1.
- [`fn_mode_to_string`](#fn_mode_to_string)
- [`is_spec`](#is_spec)
- [`module_path_to_string`](#module_path_to_string)
- [`process_impl_item_method`](#process_impl_item_method)
- [`process_item_fn`](#process_item_fn)
- [`process_item_fn_assume_specification`](#process_item_fn_assume_specification)
- [`process_item_fn_broadcast_group`](#process_item_fn_broadcast_group)
- [`process_trait_item_method`](#process_trait_item_method)
- [`show_body`](#show_body) - Do we want to show the body for the given spec function?

---

## verus_builtin_macros::rustdoc::assume_specification_link_line

*Function*

Get the assume_specification line

```rust
fn assume_specification_link_line(e: &verus_syn::Expr) -> String
```



## verus_builtin_macros::rustdoc::attr_for_broadcast_group

*Function*

```rust
fn attr_for_broadcast_group(sig: &verus_syn::Signature) -> Option<verus_syn::Attribute>
```



## verus_builtin_macros::rustdoc::attr_for_sig

*Function*

Process a signature to get all the information, apply the codeblock
formatting tricks, and then package it all up into a #[doc = "..."] attribute
(as a verus_syn::Attribute object) that we can apply to the item.

```rust
fn attr_for_sig(sig: &verus_syn::Signature, block: Option<&verus_syn::Block>, as_spec: Option<&verus_syn::AssumeSpecification>) -> Option<verus_syn::Attribute>
```



## verus_builtin_macros::rustdoc::doc_attr_from_string

*Function*

Create an attr that looks like #[doc = "doc_str"]

```rust
fn doc_attr_from_string(doc_str: &str, span: proc_macro2::Span) -> verus_syn::Attribute
```



## verus_builtin_macros::rustdoc::encoded_body

*Function*

```rust
fn encoded_body(kind: &str, code: &verus_syn::Expr) -> String
```



## verus_builtin_macros::rustdoc::encoded_expr

*Function*

Pretty print the expression, then wrap in a code block.

```rust
fn encoded_expr(kind: &str, code: &verus_syn::Expr) -> String
```



## verus_builtin_macros::rustdoc::encoded_sig_info

*Function*

```rust
fn encoded_sig_info(sig: &verus_syn::Signature) -> String
```



## verus_builtin_macros::rustdoc::encoded_str

*Function*

Wrap the given string into a code block,
into the format that the postprocessor will recognize.

```rust
fn encoded_str(kind: &str, data: &str) -> String
```



## verus_builtin_macros::rustdoc::env_rustdoc

*Function*

Check if VERUSDOC=1.

```rust
fn env_rustdoc() -> bool
```



## verus_builtin_macros::rustdoc::fn_mode_to_string

*Function*

```rust
fn fn_mode_to_string(mode: &verus_syn::FnMode, publish: &verus_syn::Publish) -> String
```



## verus_builtin_macros::rustdoc::is_spec

*Function*

```rust
fn is_spec(sig: &verus_syn::Signature) -> bool
```



## verus_builtin_macros::rustdoc::module_path_to_string

*Function*

```rust
fn module_path_to_string(p: &verus_syn::Path) -> String
```



## verus_builtin_macros::rustdoc::process_impl_item_method

*Function*

```rust
fn process_impl_item_method(item: & mut verus_syn::ImplItemFn)
```



## verus_builtin_macros::rustdoc::process_item_fn

*Function*

```rust
fn process_item_fn(item: & mut verus_syn::ItemFn)
```



## verus_builtin_macros::rustdoc::process_item_fn_assume_specification

*Function*

```rust
fn process_item_fn_assume_specification(item: & mut verus_syn::ItemFn, as_spec: &verus_syn::AssumeSpecification)
```



## verus_builtin_macros::rustdoc::process_item_fn_broadcast_group

*Function*

```rust
fn process_item_fn_broadcast_group(item: & mut verus_syn::ItemFn)
```



## verus_builtin_macros::rustdoc::process_trait_item_method

*Function*

```rust
fn process_trait_item_method(item: & mut verus_syn::TraitItemFn)
```



## verus_builtin_macros::rustdoc::show_body

*Function*

Do we want to show the body for the given spec function?
If it's 'open', then yes

```rust
fn show_body(sig: &verus_syn::Signature) -> bool
```



