**verus_state_machines_macros > parse_token_stream**

# Module: parse_token_stream

## Contents

**Structs**

- [`ParseResult`](#parseresult)
- [`SMBundle`](#smbundle)

**Enums**

- [`FnAttrInfo`](#fnattrinfo)
- [`ShardingType`](#shardingtype)

**Functions**

- [`attr_is_any_mode`](#attr_is_any_mode)
- [`attr_is_polarity`](#attr_is_polarity)
- [`check_polarity_attribute`](#check_polarity_attribute)
- [`check_untemplated_type`](#check_untemplated_type) - Checks the given type to be of the form `type_name`.
- [`ensure_no_mode`](#ensure_no_mode)
- [`err_on_dupe`](#err_on_dupe)
- [`extract_template_params`](#extract_template_params) - Checks the given type to be of the form `type_name<...>` and if so, extracts
- [`get_sharding_type`](#get_sharding_type) - Get the sharding type from the attributes of the field.
- [`impl_item_method_from_item_fn`](#impl_item_method_from_item_fn)
- [`is_okay_label_generic_ident`](#is_okay_label_generic_ident)
- [`item_type_check_name`](#item_type_check_name)
- [`item_type_check_no_bounds`](#item_type_check_no_bounds)
- [`item_type_check_vis`](#item_type_check_vis)
- [`keyword`](#keyword)
- [`parse_fn_attr_info`](#parse_fn_attr_info)
- [`parse_result_to_smir`](#parse_result_to_smir)
- [`peek_keyword`](#peek_keyword)
- [`to_fields`](#to_fields)
- [`to_invariant`](#to_invariant)
- [`to_lemma`](#to_lemma)

---

## verus_state_machines_macros::parse_token_stream::FnAttrInfo

*Enum*

**Variants:**
- `NoneFound`
- `Invariant`
- `Lemma(crate::ast::LemmaPurpose)`



## verus_state_machines_macros::parse_token_stream::ParseResult

*Struct*

**Fields:**
- `attrs: Vec<verus_syn::Attribute>`
- `name: verus_syn::Ident`
- `items: Vec<verus_syn::Item>`
- `fields: Option<verus_syn::FieldsNamed>`
- `generics: Option<verus_syn::Generics>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> parse::Result<Self>`



## verus_state_machines_macros::parse_token_stream::SMBundle

*Struct*

**Fields:**
- `name: verus_syn::Ident`
- `sm: crate::ast::SM`
- `extras: crate::ast::Extras`
- `normal_fns: Vec<verus_syn::ImplItemFn>`



## verus_state_machines_macros::parse_token_stream::ShardingType

*Enum*

**Variants:**
- `Variable`
- `Constant`
- `NotTokenized`
- `Option`
- `Map`
- `Set`
- `Multiset`
- `Count`
- `Bool`
- `PersistentOption`
- `PersistentMap`
- `PersistentSet`
- `PersistentCount`
- `PersistentBool`
- `StorageOption`
- `StorageMap`



## verus_state_machines_macros::parse_token_stream::attr_is_any_mode

*Function*

```rust
fn attr_is_any_mode(attr: &verus_syn::Attribute) -> bool
```



## verus_state_machines_macros::parse_token_stream::attr_is_polarity

*Function*

```rust
fn attr_is_polarity(attr: &verus_syn::Attribute) -> bool
```



## verus_state_machines_macros::parse_token_stream::check_polarity_attribute

*Function*

```rust
fn check_polarity_attribute(name: &str) -> bool
```



## verus_state_machines_macros::parse_token_stream::check_untemplated_type

*Function*

Checks the given type to be of the form `type_name`.
Returns an Error (using the given strategy name in the error message) if the given
type is not of the right form.

```rust
fn check_untemplated_type(ty: &verus_syn::Type, strategy: &str, type_name: &str) -> parse::Result<()>
```



## verus_state_machines_macros::parse_token_stream::ensure_no_mode

*Function*

```rust
fn ensure_no_mode(impl_item_method: &verus_syn::ImplItemFn, msg: &str) -> parse::Result<()>
```



## verus_state_machines_macros::parse_token_stream::err_on_dupe

*Function*

```rust
fn err_on_dupe(info: &FnAttrInfo, span: proc_macro2::Span) -> parse::Result<()>
```



## verus_state_machines_macros::parse_token_stream::extract_template_params

*Function*

Checks the given type to be of the form `type_name<...>` and if so, extracts
the type parameter and returns it.
Returns an Error (using the given strategy name in the error message) if the given
type is not of the right form.

```rust
fn extract_template_params(ty: &verus_syn::Type, strategy: &str, type_name: &str, num_expected_args: usize) -> parse::Result<Vec<verus_syn::Type>>
```



## verus_state_machines_macros::parse_token_stream::get_sharding_type

*Function*

Get the sharding type from the attributes of the field.
In a tokenized state machine, we require this for each field.
In a 'normal' state machine, we error if we find such an attribute
(but internally we represent the field as having the 'variable' strategy).

```rust
fn get_sharding_type(field_span: proc_macro2::Span, attrs: &[verus_syn::Attribute], concurrent: bool) -> parse::Result<ShardingType>
```



## verus_state_machines_macros::parse_token_stream::impl_item_method_from_item_fn

*Function*

```rust
fn impl_item_method_from_item_fn(item_fn: verus_syn::ItemFn) -> verus_syn::ImplItemFn
```



## verus_state_machines_macros::parse_token_stream::is_okay_label_generic_ident

*Function*

```rust
fn is_okay_label_generic_ident(ident: &verus_syn::Ident, main_generics: &Option<verus_syn::Generics>) -> bool
```



## verus_state_machines_macros::parse_token_stream::item_type_check_name

*Function*

```rust
fn item_type_check_name(ident: &verus_syn::Ident) -> parse::Result<bool>
```



## verus_state_machines_macros::parse_token_stream::item_type_check_no_bounds

*Function*

```rust
fn item_type_check_no_bounds(span: proc_macro2::Span, generics: &verus_syn::Generics, main_generics: &Option<verus_syn::Generics>) -> parse::Result<()>
```



## verus_state_machines_macros::parse_token_stream::item_type_check_vis

*Function*

```rust
fn item_type_check_vis(span: proc_macro2::Span, vis: &verus_syn::Visibility) -> parse::Result<()>
```



## verus_state_machines_macros::parse_token_stream::keyword

*Function*

```rust
fn keyword(input: verus_syn::parse::ParseStream, token: &str) -> parse::Result<proc_macro2::Span>
```



## verus_state_machines_macros::parse_token_stream::parse_fn_attr_info

*Function*

```rust
fn parse_fn_attr_info(attrs: &Vec<verus_syn::Attribute>) -> parse::Result<FnAttrInfo>
```



## verus_state_machines_macros::parse_token_stream::parse_result_to_smir

*Function*

```rust
fn parse_result_to_smir(pr: ParseResult, concurrent: bool) -> parse::Result<SMBundle>
```



## verus_state_machines_macros::parse_token_stream::peek_keyword

*Function*

```rust
fn peek_keyword(cursor: verus_syn::buffer::Cursor, token: &str) -> bool
```



## verus_state_machines_macros::parse_token_stream::to_fields

*Function*

```rust
fn to_fields(fields_named: & mut verus_syn::FieldsNamed, concurrent: bool) -> parse::Result<Vec<crate::ast::Field>>
```



## verus_state_machines_macros::parse_token_stream::to_invariant

*Function*

```rust
fn to_invariant(impl_item_method: verus_syn::ImplItemFn) -> parse::Result<crate::ast::Invariant>
```



## verus_state_machines_macros::parse_token_stream::to_lemma

*Function*

```rust
fn to_lemma(impl_item_method: verus_syn::ImplItemFn, purpose: crate::ast::LemmaPurpose) -> parse::Result<crate::ast::Lemma>
```



