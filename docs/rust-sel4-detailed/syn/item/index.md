*[syn](../index.md) / [item](index.md)*

---

# Module `item`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`ItemConst`](#itemconst)
  - [`ItemEnum`](#itemenum)
  - [`ItemExternCrate`](#itemexterncrate)
  - [`ItemFn`](#itemfn)
  - [`ItemForeignMod`](#itemforeignmod)
  - [`ItemImpl`](#itemimpl)
  - [`ItemMacro`](#itemmacro)
  - [`ItemMod`](#itemmod)
  - [`ItemStatic`](#itemstatic)
  - [`ItemStruct`](#itemstruct)
  - [`ItemTrait`](#itemtrait)
  - [`ItemTraitAlias`](#itemtraitalias)
  - [`ItemType`](#itemtype)
  - [`ItemUnion`](#itemunion)
  - [`ItemUse`](#itemuse)
  - [`UsePath`](#usepath)
  - [`UseName`](#usename)
  - [`UseRename`](#userename)
  - [`UseGlob`](#useglob)
  - [`UseGroup`](#usegroup)
  - [`ForeignItemFn`](#foreignitemfn)
  - [`ForeignItemStatic`](#foreignitemstatic)
  - [`ForeignItemType`](#foreignitemtype)
  - [`ForeignItemMacro`](#foreignitemmacro)
  - [`TraitItemConst`](#traititemconst)
  - [`TraitItemFn`](#traititemfn)
  - [`TraitItemType`](#traititemtype)
  - [`TraitItemMacro`](#traititemmacro)
  - [`ImplItemConst`](#implitemconst)
  - [`ImplItemFn`](#implitemfn)
  - [`ImplItemType`](#implitemtype)
  - [`ImplItemMacro`](#implitemmacro)
  - [`Signature`](#signature)
  - [`Receiver`](#receiver)
  - [`Variadic`](#variadic)
- [Enums](#enums)
  - [`Item`](#item)
  - [`UseTree`](#usetree)
  - [`ForeignItem`](#foreignitem)
  - [`TraitItem`](#traititem)
  - [`ImplItem`](#implitem)
  - [`FnArg`](#fnarg)
  - [`StaticMutability`](#staticmutability)
  - [`ImplRestriction`](#implrestriction)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`ItemConst`](#itemconst) | struct | A constant item: `const MAX: u16 = 65535`. |
| [`ItemEnum`](#itemenum) | struct | An enum definition: `enum Foo<A, B> { A(A), B(B) }`. |
| [`ItemExternCrate`](#itemexterncrate) | struct | An `extern crate` item: `extern crate serde`. |
| [`ItemFn`](#itemfn) | struct | A free-standing function: `fn process(n: usize) -> Result<()> { ... |
| [`ItemForeignMod`](#itemforeignmod) | struct | A block of foreign items: `extern "C" { ... |
| [`ItemImpl`](#itemimpl) | struct | An impl block providing trait or associated items: `impl<A> Trait for Data<A> { ... |
| [`ItemMacro`](#itemmacro) | struct | A macro invocation, which includes `macro_rules!` definitions. |
| [`ItemMod`](#itemmod) | struct | A module or module declaration: `mod m` or `mod m { ... |
| [`ItemStatic`](#itemstatic) | struct | A static item: `static BIKE: Shed = Shed(42)`. |
| [`ItemStruct`](#itemstruct) | struct | A struct definition: `struct Foo<A> { x: A }`. |
| [`ItemTrait`](#itemtrait) | struct | A trait definition: `pub trait Iterator { ... |
| [`ItemTraitAlias`](#itemtraitalias) | struct | A trait alias: `pub trait SharableIterator = Iterator + Sync`. |
| [`ItemType`](#itemtype) | struct | A type alias: `type Result<T> = core::result::Result<T, MyError>`. |
| [`ItemUnion`](#itemunion) | struct | A union definition: `union Foo<A, B> { x: A, y: B }`. |
| [`ItemUse`](#itemuse) | struct | A use declaration: `use alloc::collections::HashMap`. |
| [`UsePath`](#usepath) | struct | A path prefix of imports in a `use` item: `core::...`. |
| [`UseName`](#usename) | struct | An identifier imported by a `use` item: `HashMap`. |
| [`UseRename`](#userename) | struct | An renamed identifier imported by a `use` item: `HashMap as Map`. |
| [`UseGlob`](#useglob) | struct | A glob import in a `use` item: `*`. |
| [`UseGroup`](#usegroup) | struct | A braced group of imports in a `use` item: `{A, B, C}`. |
| [`ForeignItemFn`](#foreignitemfn) | struct | A foreign function in an `extern` block. |
| [`ForeignItemStatic`](#foreignitemstatic) | struct | A foreign static item in an `extern` block: `static ext: u8`. |
| [`ForeignItemType`](#foreignitemtype) | struct | A foreign type in an `extern` block: `type void`. |
| [`ForeignItemMacro`](#foreignitemmacro) | struct | A macro invocation within an extern block. |
| [`TraitItemConst`](#traititemconst) | struct | An associated constant within the definition of a trait. |
| [`TraitItemFn`](#traititemfn) | struct | An associated function within the definition of a trait. |
| [`TraitItemType`](#traititemtype) | struct | An associated type within the definition of a trait. |
| [`TraitItemMacro`](#traititemmacro) | struct | A macro invocation within the definition of a trait. |
| [`ImplItemConst`](#implitemconst) | struct | An associated constant within an impl block. |
| [`ImplItemFn`](#implitemfn) | struct | An associated function within an impl block. |
| [`ImplItemType`](#implitemtype) | struct | An associated type within an impl block. |
| [`ImplItemMacro`](#implitemmacro) | struct | A macro invocation within an impl block. |
| [`Signature`](#signature) | struct | A function signature in a trait or implementation: `unsafe fn initialize(&self)`. |
| [`Receiver`](#receiver) | struct | The `self` argument of an associated method. |
| [`Variadic`](#variadic) | struct | The variadic argument of a foreign function. |
| [`Item`](#item) | enum | Things that can appear directly inside of a module or scope. |
| [`UseTree`](#usetree) | enum | A suffix of an import tree in a `use` item: `Type as Renamed` or `*`. |
| [`ForeignItem`](#foreignitem) | enum | An item within an `extern` block. |
| [`TraitItem`](#traititem) | enum | An item declaration within the definition of a trait. |
| [`ImplItem`](#implitem) | enum | An item within an impl block. |
| [`FnArg`](#fnarg) | enum | An argument in a function signature: the `n: usize` in `fn f(n: usize)`. |
| [`StaticMutability`](#staticmutability) | enum | The mutability of an `Item::Static` or `ForeignItem::Static`. |
| [`ImplRestriction`](#implrestriction) | enum | Unused, but reserved for RFC 3323 restrictions. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `ItemConst`

```rust
struct ItemConst {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: token::Colon,
    pub ty: alloc::boxed::Box<crate::ty::Type>,
    pub eq_token: token::Eq,
    pub expr: alloc::boxed::Box<crate::expr::Expr>,
    pub semi_token: token::Semi,
}
```

A constant item: `const MAX: u16 = 65535`.

#### Implementations

- <span id="crateitemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemConst`

- <span id="crateitemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemConst`

- <span id="crateitemconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemConst`

##### `impl Hash for crate::ItemConst`

- <span id="crateitemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemConst`

- <span id="crateitemitemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemConst`

- <span id="crateitemconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemConst`

##### `impl Spanned for ItemConst`

- <span id="itemconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemConst`

- <span id="crateitemitemconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemEnum`

```rust
struct ItemEnum {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub enum_token: token::Enum,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, token::Comma>,
}
```

An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

#### Implementations

- <span id="crateitemenum-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemEnum`

- <span id="crateitemenum-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemEnum`

- <span id="crateitemenum-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemEnum`

##### `impl Hash for crate::ItemEnum`

- <span id="crateitemenum-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemEnum`

- <span id="crateitemitemenum-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemEnum`

- <span id="crateitemenum-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemEnum`

##### `impl Spanned for ItemEnum`

- <span id="itemenum-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemEnum`

- <span id="crateitemitemenum-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemExternCrate`

```rust
struct ItemExternCrate {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub extern_token: token::Extern,
    pub crate_token: token::Crate,
    pub ident: crate::ident::Ident,
    pub rename: Option<(token::As, crate::ident::Ident)>,
    pub semi_token: token::Semi,
}
```

An `extern crate` item: `extern crate serde`.

#### Implementations

- <span id="crateitemexterncrate-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemExternCrate`

- <span id="crateitemexterncrate-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemExternCrate`

- <span id="crateitemexterncrate-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemExternCrate`

##### `impl Hash for crate::ItemExternCrate`

- <span id="crateitemexterncrate-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemExternCrate`

- <span id="crateitemitemexterncrate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemExternCrate`

- <span id="crateitemexterncrate-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemExternCrate`

##### `impl Spanned for ItemExternCrate`

- <span id="itemexterncrate-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemExternCrate`

- <span id="crateitemitemexterncrate-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemFn`

```rust
struct ItemFn {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub block: alloc::boxed::Box<crate::stmt::Block>,
}
```

A free-standing function: `fn process(n: usize) -> Result<()> { ... }`.

#### Implementations

- <span id="crateitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemFn`

- <span id="crateitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemFn`

- <span id="crateitemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemFn`

##### `impl Hash for crate::ItemFn`

- <span id="crateitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemFn`

- <span id="crateitemitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemFn`

- <span id="crateitemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemFn`

##### `impl Spanned for ItemFn`

- <span id="itemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemFn`

- <span id="crateitemitemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemForeignMod`

```rust
struct ItemForeignMod {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub unsafety: Option<token::Unsafe>,
    pub abi: crate::ty::Abi,
    pub brace_token: token::Brace,
    pub items: alloc::vec::Vec<ForeignItem>,
}
```

A block of foreign items: `extern "C" { ... }`.

#### Implementations

- <span id="crateitemforeignmod-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemForeignMod`

- <span id="crateitemforeignmod-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemForeignMod`

- <span id="crateitemforeignmod-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemForeignMod`

##### `impl Hash for crate::ItemForeignMod`

- <span id="crateitemforeignmod-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemForeignMod`

- <span id="crateitemitemforeignmod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemForeignMod`

- <span id="crateitemforeignmod-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemForeignMod`

##### `impl Spanned for ItemForeignMod`

- <span id="itemforeignmod-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemForeignMod`

- <span id="crateitemitemforeignmod-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemImpl`

```rust
struct ItemImpl {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub defaultness: Option<token::Default>,
    pub unsafety: Option<token::Unsafe>,
    pub impl_token: token::Impl,
    pub generics: crate::generics::Generics,
    pub trait_: Option<(Option<token::Not>, crate::path::Path, token::For)>,
    pub self_ty: alloc::boxed::Box<crate::ty::Type>,
    pub brace_token: token::Brace,
    pub items: alloc::vec::Vec<ImplItem>,
}
```

An impl block providing trait or associated items: `impl<A> Trait
for Data<A> { ... }`.

#### Fields

- **`trait_`**: `Option<(Option<token::Not>, crate::path::Path, token::For)>`

  Trait this impl implements.

- **`self_ty`**: `alloc::boxed::Box<crate::ty::Type>`

  The Self type of the impl.

#### Implementations

- <span id="crateitemimpl-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemImpl`

- <span id="crateitemimpl-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemImpl`

- <span id="crateitemimpl-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemImpl`

##### `impl Hash for crate::ItemImpl`

- <span id="crateitemimpl-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemImpl`

- <span id="crateitemitemimpl-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemImpl`

- <span id="crateitemimpl-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemImpl`

##### `impl Spanned for ItemImpl`

- <span id="itemimpl-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemImpl`

- <span id="crateitemitemimpl-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemMacro`

```rust
struct ItemMacro {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub ident: Option<crate::ident::Ident>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation, which includes `macro_rules!` definitions.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  The `example` in `macro_rules! example { ... }`.

#### Implementations

- <span id="crateitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemMacro`

- <span id="crateitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemMacro`

- <span id="crateitemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMacro`

##### `impl Hash for crate::ItemMacro`

- <span id="crateitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemMacro`

- <span id="crateitemitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemMacro`

- <span id="crateitemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemMacro`

##### `impl Spanned for ItemMacro`

- <span id="itemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemMacro`

- <span id="crateitemitemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemMod`

```rust
struct ItemMod {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<token::Unsafe>,
    pub mod_token: token::Mod,
    pub ident: crate::ident::Ident,
    pub content: Option<(token::Brace, alloc::vec::Vec<Item>)>,
    pub semi: Option<token::Semi>,
}
```

A module or module declaration: `mod m` or `mod m { ... }`.

#### Implementations

- <span id="crateitemmod-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemMod`

- <span id="crateitemmod-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemMod`

- <span id="crateitemmod-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMod`

##### `impl Hash for crate::ItemMod`

- <span id="crateitemmod-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemMod`

- <span id="crateitemitemmod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemMod`

- <span id="crateitemmod-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemMod`

##### `impl Spanned for ItemMod`

- <span id="itemmod-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemMod`

- <span id="crateitemitemmod-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemStatic`

```rust
struct ItemStatic {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: token::Colon,
    pub ty: alloc::boxed::Box<crate::ty::Type>,
    pub eq_token: token::Eq,
    pub expr: alloc::boxed::Box<crate::expr::Expr>,
    pub semi_token: token::Semi,
}
```

A static item: `static BIKE: Shed = Shed(42)`.

#### Implementations

- <span id="crateitemstatic-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemStatic`

- <span id="crateitemstatic-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemStatic`

- <span id="crateitemstatic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStatic`

##### `impl Hash for crate::ItemStatic`

- <span id="crateitemstatic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemStatic`

- <span id="crateitemitemstatic-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemStatic`

- <span id="crateitemstatic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemStatic`

##### `impl Spanned for ItemStatic`

- <span id="itemstatic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemStatic`

- <span id="crateitemitemstatic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemStruct`

```rust
struct ItemStruct {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub struct_token: token::Struct,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::Fields,
    pub semi_token: Option<token::Semi>,
}
```

A struct definition: `struct Foo<A> { x: A }`.

#### Implementations

- <span id="crateitemstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemStruct`

- <span id="crateitemstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemStruct`

- <span id="crateitemstruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStruct`

##### `impl Hash for crate::ItemStruct`

- <span id="crateitemstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemStruct`

- <span id="crateitemitemstruct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemStruct`

- <span id="crateitemstruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemStruct`

##### `impl Spanned for ItemStruct`

- <span id="itemstruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemStruct`

- <span id="crateitemitemstruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemTrait`

```rust
struct ItemTrait {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<token::Unsafe>,
    pub auto_token: Option<token::Auto>,
    pub restriction: Option<ImplRestriction>,
    pub trait_token: token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<token::Colon>,
    pub supertraits: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    pub brace_token: token::Brace,
    pub items: alloc::vec::Vec<TraitItem>,
}
```

A trait definition: `pub trait Iterator { ... }`.

#### Implementations

- <span id="crateitemtrait-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemTrait`

- <span id="crateitemtrait-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemTrait`

- <span id="crateitemtrait-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTrait`

##### `impl Hash for crate::ItemTrait`

- <span id="crateitemtrait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemTrait`

- <span id="crateitemitemtrait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemTrait`

- <span id="crateitemtrait-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemTrait`

##### `impl Spanned for ItemTrait`

- <span id="itemtrait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemTrait`

- <span id="crateitemitemtrait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemTraitAlias`

```rust
struct ItemTraitAlias {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub trait_token: token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: token::Eq,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    pub semi_token: token::Semi,
}
```

A trait alias: `pub trait SharableIterator = Iterator + Sync`.

#### Implementations

- <span id="crateitemtraitalias-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTraitAlias`

##### `impl Hash for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemTraitAlias`

- <span id="crateitemitemtraitalias-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemTraitAlias`

##### `impl Spanned for ItemTraitAlias`

- <span id="itemtraitalias-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemTraitAlias`

- <span id="crateitemitemtraitalias-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemType`

```rust
struct ItemType {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: token::Eq,
    pub ty: alloc::boxed::Box<crate::ty::Type>,
    pub semi_token: token::Semi,
}
```

A type alias: `type Result<T> = core::result::Result<T, MyError>`.

#### Implementations

- <span id="crateitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemType`

- <span id="crateitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemType`

- <span id="crateitemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemType`

##### `impl Hash for crate::ItemType`

- <span id="crateitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemType`

- <span id="crateitemitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemType`

- <span id="crateitemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemType`

##### `impl Spanned for ItemType`

- <span id="itemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemType`

- <span id="crateitemitemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemUnion`

```rust
struct ItemUnion {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub union_token: token::Union,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::FieldsNamed,
}
```

A union definition: `union Foo<A, B> { x: A, y: B }`.

#### Implementations

- <span id="crateitemunion-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemUnion`

- <span id="crateitemunion-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemUnion`

- <span id="crateitemunion-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUnion`

##### `impl Hash for crate::ItemUnion`

- <span id="crateitemunion-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemUnion`

- <span id="crateitemitemunion-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemUnion`

- <span id="crateitemunion-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemUnion`

##### `impl Spanned for ItemUnion`

- <span id="itemunion-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemUnion`

- <span id="crateitemitemunion-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ItemUse`

```rust
struct ItemUse {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub use_token: token::Use,
    pub leading_colon: Option<token::PathSep>,
    pub tree: UseTree,
    pub semi_token: token::Semi,
}
```

A use declaration: `use alloc::collections::HashMap`.

#### Implementations

- <span id="crateitemuse-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemUse`

- <span id="crateitemuse-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ItemUse`

- <span id="crateitemuse-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUse`

##### `impl Hash for crate::ItemUse`

- <span id="crateitemuse-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ItemUse`

- <span id="crateitemitemuse-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemUse`

- <span id="crateitemuse-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemUse`

##### `impl Spanned for ItemUse`

- <span id="itemuse-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ItemUse`

- <span id="crateitemitemuse-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UsePath`

```rust
struct UsePath {
    pub ident: crate::ident::Ident,
    pub colon2_token: token::PathSep,
    pub tree: alloc::boxed::Box<UseTree>,
}
```

A path prefix of imports in a `use` item: `core::...`.

#### Trait Implementations

##### `impl Clone for crate::UsePath`

- <span id="crateusepath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UsePath`

- <span id="crateusepath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UsePath`

##### `impl Hash for crate::UsePath`

- <span id="crateusepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::UsePath`

- <span id="crateusepath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UsePath`

##### `impl Spanned for UsePath`

- <span id="usepath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UsePath`

- <span id="crateitemusepath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UseName`

```rust
struct UseName {
    pub ident: crate::ident::Ident,
}
```

An identifier imported by a `use` item: `HashMap`.

#### Trait Implementations

##### `impl Clone for crate::UseName`

- <span id="crateusename-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseName`

- <span id="crateusename-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseName`

##### `impl Hash for crate::UseName`

- <span id="crateusename-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::UseName`

- <span id="crateusename-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseName`

##### `impl Spanned for UseName`

- <span id="usename-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UseName`

- <span id="crateitemusename-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UseRename`

```rust
struct UseRename {
    pub ident: crate::ident::Ident,
    pub as_token: token::As,
    pub rename: crate::ident::Ident,
}
```

An renamed identifier imported by a `use` item: `HashMap as Map`.

#### Trait Implementations

##### `impl Clone for crate::UseRename`

- <span id="crateuserename-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseRename`

- <span id="crateuserename-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseRename`

##### `impl Hash for crate::UseRename`

- <span id="crateuserename-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::UseRename`

- <span id="crateuserename-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseRename`

##### `impl Spanned for UseRename`

- <span id="userename-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UseRename`

- <span id="crateitemuserename-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UseGlob`

```rust
struct UseGlob {
    pub star_token: token::Star,
}
```

A glob import in a `use` item: `*`.

#### Trait Implementations

##### `impl Clone for crate::UseGlob`

- <span id="crateuseglob-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseGlob`

- <span id="crateuseglob-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGlob`

##### `impl Hash for crate::UseGlob`

- <span id="crateuseglob-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl PartialEq for crate::UseGlob`

- <span id="crateuseglob-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl Sealed for UseGlob`

##### `impl Spanned for UseGlob`

- <span id="useglob-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UseGlob`

- <span id="crateitemuseglob-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UseGroup`

```rust
struct UseGroup {
    pub brace_token: token::Brace,
    pub items: crate::punctuated::Punctuated<UseTree, token::Comma>,
}
```

A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl Clone for crate::UseGroup`

- <span id="crateusegroup-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseGroup`

- <span id="crateusegroup-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGroup`

##### `impl Hash for crate::UseGroup`

- <span id="crateusegroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::UseGroup`

- <span id="crateusegroup-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseGroup`

##### `impl Spanned for UseGroup`

- <span id="usegroup-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::UseGroup`

- <span id="crateitemusegroup-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ForeignItemFn`

```rust
struct ForeignItemFn {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub semi_token: token::Semi,
}
```

A foreign function in an `extern` block.

#### Implementations

- <span id="crateforeignitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemFn`

- <span id="crateforeignitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItemFn`

- <span id="crateforeignitemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemFn`

##### `impl Hash for crate::ForeignItemFn`

- <span id="crateforeignitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemFn`

- <span id="crateitemforeignitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItemFn`

- <span id="crateforeignitemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemFn`

##### `impl Spanned for ForeignItemFn`

- <span id="foreignitemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemFn`

- <span id="crateitemforeignitemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ForeignItemStatic`

```rust
struct ForeignItemStatic {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: token::Colon,
    pub ty: alloc::boxed::Box<crate::ty::Type>,
    pub semi_token: token::Semi,
}
```

A foreign static item in an `extern` block: `static ext: u8`.

#### Implementations

- <span id="crateforeignitemstatic-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemStatic`

##### `impl Hash for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemStatic`

- <span id="crateitemforeignitemstatic-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemStatic`

##### `impl Spanned for ForeignItemStatic`

- <span id="foreignitemstatic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemStatic`

- <span id="crateitemforeignitemstatic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ForeignItemType`

```rust
struct ForeignItemType {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub semi_token: token::Semi,
}
```

A foreign type in an `extern` block: `type void`.

#### Implementations

- <span id="crateforeignitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemType`

- <span id="crateforeignitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItemType`

- <span id="crateforeignitemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemType`

##### `impl Hash for crate::ForeignItemType`

- <span id="crateforeignitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemType`

- <span id="crateitemforeignitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItemType`

- <span id="crateforeignitemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemType`

##### `impl Spanned for ForeignItemType`

- <span id="foreignitemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemType`

- <span id="crateitemforeignitemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ForeignItemMacro`

```rust
struct ForeignItemMacro {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation within an extern block.

#### Implementations

- <span id="crateforeignitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemMacro`

##### `impl Hash for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemMacro`

- <span id="crateitemforeignitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemMacro`

##### `impl Spanned for ForeignItemMacro`

- <span id="foreignitemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemMacro`

- <span id="crateitemforeignitemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitItemConst`

```rust
struct TraitItemConst {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: token::Colon,
    pub ty: crate::ty::Type,
    pub default: Option<(token::Eq, crate::expr::Expr)>,
    pub semi_token: token::Semi,
}
```

An associated constant within the definition of a trait.

#### Implementations

- <span id="cratetraititemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemConst`

- <span id="cratetraititemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItemConst`

- <span id="cratetraititemconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemConst`

##### `impl Hash for crate::TraitItemConst`

- <span id="cratetraititemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemConst`

- <span id="crateitemtraititemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItemConst`

- <span id="cratetraititemconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemConst`

##### `impl Spanned for TraitItemConst`

- <span id="traititemconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::TraitItemConst`

- <span id="crateitemtraititemconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitItemFn`

```rust
struct TraitItemFn {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub sig: Signature,
    pub default: Option<crate::stmt::Block>,
    pub semi_token: Option<token::Semi>,
}
```

An associated function within the definition of a trait.

#### Implementations

- <span id="cratetraititemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemFn`

- <span id="cratetraititemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItemFn`

- <span id="cratetraititemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemFn`

##### `impl Hash for crate::TraitItemFn`

- <span id="cratetraititemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemFn`

- <span id="crateitemtraititemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItemFn`

- <span id="cratetraititemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemFn`

##### `impl Spanned for TraitItemFn`

- <span id="traititemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::TraitItemFn`

- <span id="crateitemtraititemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitItemType`

```rust
struct TraitItemType {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    pub default: Option<(token::Eq, crate::ty::Type)>,
    pub semi_token: token::Semi,
}
```

An associated type within the definition of a trait.

#### Implementations

- <span id="cratetraititemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemType`

- <span id="cratetraititemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItemType`

- <span id="cratetraititemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemType`

##### `impl Hash for crate::TraitItemType`

- <span id="cratetraititemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemType`

- <span id="crateitemtraititemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItemType`

- <span id="cratetraititemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemType`

##### `impl Spanned for TraitItemType`

- <span id="traititemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::TraitItemType`

- <span id="crateitemtraititemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitItemMacro`

```rust
struct TraitItemMacro {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation within the definition of a trait.

#### Implementations

- <span id="cratetraititemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemMacro`

- <span id="cratetraititemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItemMacro`

- <span id="cratetraititemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemMacro`

##### `impl Hash for crate::TraitItemMacro`

- <span id="cratetraititemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemMacro`

- <span id="crateitemtraititemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItemMacro`

- <span id="cratetraititemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemMacro`

##### `impl Spanned for TraitItemMacro`

- <span id="traititemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::TraitItemMacro`

- <span id="crateitemtraititemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplItemConst`

```rust
struct ImplItemConst {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<token::Default>,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: token::Colon,
    pub ty: crate::ty::Type,
    pub eq_token: token::Eq,
    pub expr: crate::expr::Expr,
    pub semi_token: token::Semi,
}
```

An associated constant within an impl block.

#### Implementations

- <span id="crateimplitemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemConst`

- <span id="crateimplitemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItemConst`

- <span id="crateimplitemconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemConst`

##### `impl Hash for crate::ImplItemConst`

- <span id="crateimplitemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemConst`

- <span id="crateitemimplitemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItemConst`

- <span id="crateimplitemconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemConst`

##### `impl Spanned for ImplItemConst`

- <span id="implitemconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ImplItemConst`

- <span id="crateitemimplitemconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplItemFn`

```rust
struct ImplItemFn {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<token::Default>,
    pub sig: Signature,
    pub block: crate::stmt::Block,
}
```

An associated function within an impl block.

#### Implementations

- <span id="crateimplitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemFn`

- <span id="crateimplitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItemFn`

- <span id="crateimplitemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemFn`

##### `impl Hash for crate::ImplItemFn`

- <span id="crateimplitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemFn`

- <span id="crateitemimplitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItemFn`

- <span id="crateimplitemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemFn`

##### `impl Spanned for ImplItemFn`

- <span id="implitemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ImplItemFn`

- <span id="crateitemimplitemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplItemType`

```rust
struct ImplItemType {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<token::Default>,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: token::Eq,
    pub ty: crate::ty::Type,
    pub semi_token: token::Semi,
}
```

An associated type within an impl block.

#### Implementations

- <span id="crateimplitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemType`

- <span id="crateimplitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItemType`

- <span id="crateimplitemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemType`

##### `impl Hash for crate::ImplItemType`

- <span id="crateimplitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemType`

- <span id="crateitemimplitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItemType`

- <span id="crateimplitemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemType`

##### `impl Spanned for ImplItemType`

- <span id="implitemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ImplItemType`

- <span id="crateitemimplitemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplItemMacro`

```rust
struct ImplItemMacro {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation within an impl block.

#### Implementations

- <span id="crateimplitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemMacro`

- <span id="crateimplitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItemMacro`

- <span id="crateimplitemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemMacro`

##### `impl Hash for crate::ImplItemMacro`

- <span id="crateimplitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemMacro`

- <span id="crateitemimplitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItemMacro`

- <span id="crateimplitemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemMacro`

##### `impl Spanned for ImplItemMacro`

- <span id="implitemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::ImplItemMacro`

- <span id="crateitemimplitemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Signature`

```rust
struct Signature {
    pub constness: Option<token::Const>,
    pub asyncness: Option<token::Async>,
    pub unsafety: Option<token::Unsafe>,
    pub abi: Option<crate::ty::Abi>,
    pub fn_token: token::Fn,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<FnArg, token::Comma>,
    pub variadic: Option<Variadic>,
    pub output: crate::ty::ReturnType,
}
```

A function signature in a trait or implementation: `unsafe fn
initialize(&self)`.

#### Implementations

- <span id="signature-receiver"></span>`fn receiver(&self) -> Option<&Receiver>` — [`Receiver`](#receiver)

  A method's `self` receiver, such as `&self` or `self: Box<Self>`.

#### Trait Implementations

##### `impl Clone for crate::Signature`

- <span id="cratesignature-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Signature`

- <span id="cratesignature-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Signature`

##### `impl Hash for crate::Signature`

- <span id="cratesignature-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::Signature`

- <span id="crateitemsignature-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Signature`

- <span id="cratesignature-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Signature`

##### `impl Spanned for Signature`

- <span id="signature-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::Signature`

- <span id="crateitemsignature-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Receiver`

```rust
struct Receiver {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub reference: Option<(token::And, Option<crate::lifetime::Lifetime>)>,
    pub mutability: Option<token::Mut>,
    pub self_token: token::SelfValue,
    pub colon_token: Option<token::Colon>,
    pub ty: alloc::boxed::Box<crate::ty::Type>,
}
```

The `self` argument of an associated method.

If `colon_token` is present, the receiver is written with an explicit
type such as `self: Box<Self>`. If `colon_token` is absent, the receiver
is written in shorthand such as `self` or `&self` or `&mut self`. In the
shorthand case, the type in `ty` is reconstructed as one of `Self`,
`&Self`, or `&mut Self`.

#### Implementations

- <span id="receiver-lifetime"></span>`fn lifetime(&self) -> Option<&Lifetime>` — [`Lifetime`](../lifetime/index.md#lifetime)

#### Trait Implementations

##### `impl Clone for crate::Receiver`

- <span id="cratereceiver-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Receiver`

- <span id="cratereceiver-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Receiver`

##### `impl Hash for crate::Receiver`

- <span id="cratereceiver-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::Receiver`

- <span id="crateitemreceiver-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Receiver`

- <span id="cratereceiver-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Receiver`

##### `impl Spanned for Receiver`

- <span id="receiver-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::Receiver`

- <span id="crateitemreceiver-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Variadic`

```rust
struct Variadic {
    pub attrs: alloc::vec::Vec<crate::attr::Attribute>,
    pub pat: Option<(alloc::boxed::Box<crate::pat::Pat>, token::Colon)>,
    pub dots: token::DotDotDot,
    pub comma: Option<token::Comma>,
}
```

The variadic argument of a foreign function.

```rust
struct c_char;
struct c_int;

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    //                               ^^^
}
```

#### Trait Implementations

##### `impl Clone for crate::Variadic`

- <span id="cratevariadic-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Variadic`

- <span id="cratevariadic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variadic`

##### `impl Hash for crate::Variadic`

- <span id="cratevariadic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Variadic`

- <span id="cratevariadic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Variadic`

##### `impl Spanned for Variadic`

- <span id="variadic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::Variadic`

- <span id="crateitemvariadic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `Item`

```rust
enum Item {
    Const(ItemConst),
    Enum(ItemEnum),
    ExternCrate(ItemExternCrate),
    Fn(ItemFn),
    ForeignMod(ItemForeignMod),
    Impl(ItemImpl),
    Macro(ItemMacro),
    Mod(ItemMod),
    Static(ItemStatic),
    Struct(ItemStruct),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Use(ItemUse),
    Verbatim(proc_macro2::TokenStream),
}
```

Things that can appear directly inside of a module or scope.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  A constant item: `const MAX: u16 = 65535`.

- **`Enum`**

  An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

- **`ExternCrate`**

  An `extern crate` item: `extern crate serde`.

- **`Fn`**

  A free-standing function: `fn process(n: usize) -> Result<()> { ...
  }`.

- **`ForeignMod`**

  A block of foreign items: `extern "C" { ... }`.

- **`Impl`**

  An impl block providing trait or associated items: `impl<A> Trait
  for Data<A> { ... }`.

- **`Macro`**

  A macro invocation, which includes `macro_rules!` definitions.

- **`Mod`**

  A module or module declaration: `mod m` or `mod m { ... }`.

- **`Static`**

  A static item: `static BIKE: Shed = Shed(42)`.

- **`Struct`**

  A struct definition: `struct Foo<A> { x: A }`.

- **`Trait`**

  A trait definition: `pub trait Iterator { ... }`.

- **`TraitAlias`**

  A trait alias: `pub trait SharableIterator = Iterator + Sync`.

- **`Type`**

  A type alias: `type Result<T> = core::result::Result<T, MyError>`.

- **`Union`**

  A union definition: `union Foo<A, B> { x: A, y: B }`.

- **`Use`**

  A use declaration: `use alloc::collections::HashMap`.

- **`Verbatim`**

  Tokens forming an item not interpreted by Syn.

#### Implementations

- <span id="item-replace-attrs"></span>`fn replace_attrs(&mut self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](../attr/index.md#attribute)

#### Trait Implementations

##### `impl Clone for crate::Item`

- <span id="crateitem-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Item`

- <span id="crateitem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Item`

##### `impl Hash for crate::Item`

- <span id="crateitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::Item`

- <span id="crateitemitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Item`

- <span id="crateitem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Item`

##### `impl Spanned for Item`

- <span id="item-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Item`

- <span id="item-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `UseTree`

```rust
enum UseTree {
    Path(UsePath),
    Name(UseName),
    Rename(UseRename),
    Glob(UseGlob),
    Group(UseGroup),
}
```

A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Path`**

  A path prefix of imports in a `use` item: `core::...`.

- **`Name`**

  An identifier imported by a `use` item: `HashMap`.

- **`Rename`**

  An renamed identifier imported by a `use` item: `HashMap as Map`.

- **`Glob`**

  A glob import in a `use` item: `*`.

- **`Group`**

  A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl Clone for crate::UseTree`

- <span id="crateusetree-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::UseTree`

- <span id="crateusetree-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseTree`

##### `impl Hash for crate::UseTree`

- <span id="crateusetree-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::UseTree`

- <span id="crateitemusetree-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<UseTree>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`UseTree`](#usetree)

##### `impl PartialEq for crate::UseTree`

- <span id="crateusetree-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseTree`

##### `impl Spanned for UseTree`

- <span id="usetree-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for UseTree`

- <span id="usetree-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `ForeignItem`

```rust
enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item within an `extern` block.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Fn`**

  A foreign function in an `extern` block.

- **`Static`**

  A foreign static item in an `extern` block: `static ext: u8`.

- **`Type`**

  A foreign type in an `extern` block: `type void`.

- **`Macro`**

  A macro invocation within an extern block.

- **`Verbatim`**

  Tokens in an `extern` block not interpreted by Syn.

#### Trait Implementations

##### `impl Clone for crate::ForeignItem`

- <span id="crateforeignitem-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ForeignItem`

- <span id="crateforeignitem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItem`

##### `impl Hash for crate::ForeignItem`

- <span id="crateforeignitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItem`

- <span id="crateitemforeignitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItem`

- <span id="crateforeignitem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItem`

##### `impl Spanned for ForeignItem`

- <span id="foreignitem-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for ForeignItem`

- <span id="foreignitem-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `TraitItem`

```rust
enum TraitItem {
    Const(TraitItemConst),
    Fn(TraitItemFn),
    Type(TraitItemType),
    Macro(TraitItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item declaration within the definition of a trait.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  An associated constant within the definition of a trait.

- **`Fn`**

  An associated function within the definition of a trait.

- **`Type`**

  An associated type within the definition of a trait.

- **`Macro`**

  A macro invocation within the definition of a trait.

- **`Verbatim`**

  Tokens within the definition of a trait not interpreted by Syn.

#### Trait Implementations

##### `impl Clone for crate::TraitItem`

- <span id="cratetraititem-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitItem`

- <span id="cratetraititem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItem`

##### `impl Hash for crate::TraitItem`

- <span id="cratetraititem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::TraitItem`

- <span id="crateitemtraititem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItem`

- <span id="cratetraititem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItem`

##### `impl Spanned for TraitItem`

- <span id="traititem-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for TraitItem`

- <span id="traititem-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `ImplItem`

```rust
enum ImplItem {
    Const(ImplItemConst),
    Fn(ImplItemFn),
    Type(ImplItemType),
    Macro(ImplItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item within an impl block.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  An associated constant within an impl block.

- **`Fn`**

  An associated function within an impl block.

- **`Type`**

  An associated type within an impl block.

- **`Macro`**

  A macro invocation within an impl block.

- **`Verbatim`**

  Tokens within an impl block not interpreted by Syn.

#### Trait Implementations

##### `impl Clone for crate::ImplItem`

- <span id="crateimplitem-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplItem`

- <span id="crateimplitem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItem`

##### `impl Hash for crate::ImplItem`

- <span id="crateimplitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::ImplItem`

- <span id="crateitemimplitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItem`

- <span id="crateimplitem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItem`

##### `impl Spanned for ImplItem`

- <span id="implitem-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for ImplItem`

- <span id="implitem-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `FnArg`

```rust
enum FnArg {
    Receiver(Receiver),
    Typed(crate::pat::PatType),
}
```

An argument in a function signature: the `n: usize` in `fn f(n: usize)`.

#### Variants

- **`Receiver`**

  The `self` argument of an associated method.

- **`Typed`**

  A function argument accepted by pattern and type.

#### Trait Implementations

##### `impl Clone for crate::FnArg`

- <span id="cratefnarg-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FnArg`

- <span id="cratefnarg-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FnArg`

##### `impl Hash for crate::FnArg`

- <span id="cratefnarg-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::FnArg`

- <span id="crateitemfnarg-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::FnArg`

- <span id="cratefnarg-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FnArg`

##### `impl Spanned for FnArg`

- <span id="fnarg-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for FnArg`

- <span id="fnarg-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `StaticMutability`

```rust
enum StaticMutability {
    Mut(token::Mut),
    None,
}
```

The mutability of an `Item::Static` or `ForeignItem::Static`.

#### Trait Implementations

##### `impl Clone for crate::StaticMutability`

- <span id="cratestaticmutability-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::StaticMutability`

- <span id="cratestaticmutability-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StaticMutability`

##### `impl Hash for crate::StaticMutability`

- <span id="cratestaticmutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::item::StaticMutability`

- <span id="crateitemstaticmutability-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::StaticMutability`

- <span id="cratestaticmutability-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for StaticMutability`

##### `impl Spanned for StaticMutability`

- <span id="staticmutability-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::item::StaticMutability`

- <span id="crateitemstaticmutability-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ImplRestriction`

```rust
enum ImplRestriction {
}
```

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Clone for crate::ImplRestriction`

- <span id="crateimplrestriction-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ImplRestriction`

- <span id="crateimplrestriction-debug-fmt"></span>`fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplRestriction`

##### `impl Hash for crate::ImplRestriction`

- <span id="crateimplrestriction-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl PartialEq for crate::ImplRestriction`

- <span id="crateimplrestriction-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

