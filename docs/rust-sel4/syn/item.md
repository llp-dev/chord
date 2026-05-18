**syn > item**

# Module: item

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`ForeignItemFn`](#foreignitemfn) - A foreign function in an `extern` block.
- [`ForeignItemMacro`](#foreignitemmacro) - A macro invocation within an extern block.
- [`ForeignItemStatic`](#foreignitemstatic) - A foreign static item in an `extern` block: `static ext: u8`.
- [`ForeignItemType`](#foreignitemtype) - A foreign type in an `extern` block: `type void`.
- [`ImplItemConst`](#implitemconst) - An associated constant within an impl block.
- [`ImplItemFn`](#implitemfn) - An associated function within an impl block.
- [`ImplItemMacro`](#implitemmacro) - A macro invocation within an impl block.
- [`ImplItemType`](#implitemtype) - An associated type within an impl block.
- [`ItemConst`](#itemconst) - A constant item: `const MAX: u16 = 65535`.
- [`ItemEnum`](#itemenum) - An enum definition: `enum Foo<A, B> { A(A), B(B) }`.
- [`ItemExternCrate`](#itemexterncrate) - An `extern crate` item: `extern crate serde`.
- [`ItemFn`](#itemfn) - A free-standing function: `fn process(n: usize) -> Result<()> { ... }`.
- [`ItemForeignMod`](#itemforeignmod) - A block of foreign items: `extern "C" { ... }`.
- [`ItemImpl`](#itemimpl) - An impl block providing trait or associated items: `impl<A> Trait
- [`ItemMacro`](#itemmacro) - A macro invocation, which includes `macro_rules!` definitions.
- [`ItemMod`](#itemmod) - A module or module declaration: `mod m` or `mod m { ... }`.
- [`ItemStatic`](#itemstatic) - A static item: `static BIKE: Shed = Shed(42)`.
- [`ItemStruct`](#itemstruct) - A struct definition: `struct Foo<A> { x: A }`.
- [`ItemTrait`](#itemtrait) - A trait definition: `pub trait Iterator { ... }`.
- [`ItemTraitAlias`](#itemtraitalias) - A trait alias: `pub trait SharableIterator = Iterator + Sync`.
- [`ItemType`](#itemtype) - A type alias: `type Result<T> = core::result::Result<T, MyError>`.
- [`ItemUnion`](#itemunion) - A union definition: `union Foo<A, B> { x: A, y: B }`.
- [`ItemUse`](#itemuse) - A use declaration: `use alloc::collections::HashMap`.
- [`Receiver`](#receiver) - The `self` argument of an associated method.
- [`Signature`](#signature) - A function signature in a trait or implementation: `unsafe fn
- [`TraitItemConst`](#traititemconst) - An associated constant within the definition of a trait.
- [`TraitItemFn`](#traititemfn) - An associated function within the definition of a trait.
- [`TraitItemMacro`](#traititemmacro) - A macro invocation within the definition of a trait.
- [`TraitItemType`](#traititemtype) - An associated type within the definition of a trait.
- [`UseGlob`](#useglob) - A glob import in a `use` item: `*`.
- [`UseGroup`](#usegroup) - A braced group of imports in a `use` item: `{A, B, C}`.
- [`UseName`](#usename) - An identifier imported by a `use` item: `HashMap`.
- [`UsePath`](#usepath) - A path prefix of imports in a `use` item: `core::...`.
- [`UseRename`](#userename) - An renamed identifier imported by a `use` item: `HashMap as Map`.
- [`Variadic`](#variadic) - The variadic argument of a foreign function.

**Enums**

- [`FnArg`](#fnarg) - An argument in a function signature: the `n: usize` in `fn f(n: usize)`.
- [`ForeignItem`](#foreignitem) - An item within an `extern` block.
- [`ImplItem`](#implitem) - An item within an impl block.
- [`ImplRestriction`](#implrestriction) - Unused, but reserved for RFC 3323 restrictions.
- [`Item`](#item) - Things that can appear directly inside of a module or scope.
- [`StaticMutability`](#staticmutability) - The mutability of an `Item::Static` or `ForeignItem::Static`.
- [`TraitItem`](#traititem) - An item declaration within the definition of a trait.
- [`UseTree`](#usetree) - A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.

---

## syn::item::FnArg

*Enum*

An argument in a function signature: the `n: usize` in `fn f(n: usize)`.

**Variants:**
- `Receiver(Receiver)` - The `self` argument of an associated method.
- `Typed(crate::pat::PatType)` - A function argument accepted by pattern and type.

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **From**
  - `fn from(e: PatType) -> FnArg`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: Receiver) -> FnArg`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ForeignItem

*Enum*

An item within an `extern` block.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Fn(ForeignItemFn)` - A foreign function in an `extern` block.
- `Static(ForeignItemStatic)` - A foreign static item in an `extern` block: `static ext: u8`.
- `Type(ForeignItemType)` - A foreign type in an `extern` block: `type void`.
- `Macro(ForeignItemMacro)` - A macro invocation within an extern block.
- `Verbatim(proc_macro2::TokenStream)` - Tokens in an `extern` block not interpreted by Syn.

**Trait Implementations:**

- **From**
  - `fn from(e: ForeignItemFn) -> ForeignItem`
- **From**
  - `fn from(e: ForeignItemMacro) -> ForeignItem`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: ForeignItemType) -> ForeignItem`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(e: ForeignItemStatic) -> ForeignItem`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`



## syn::item::ForeignItemFn

*Struct*

A foreign function in an `extern` block.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `sig: Signature`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::ForeignItemMacro

*Struct*

A macro invocation within an extern block.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `mac: crate::mac::Macro`
- `semi_token: Option<$crate::token::Semi>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ForeignItemStatic

*Struct*

A foreign static item in an `extern` block: `static ext: u8`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `static_token: $crate::token::Static`
- `mutability: StaticMutability`
- `ident: crate::ident::Ident`
- `colon_token: $crate::token::Colon`
- `ty: alloc::boxed::Box<crate::ty::Type>`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::ForeignItemType

*Struct*

A foreign type in an `extern` block: `type void`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `type_token: $crate::token::Type`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::ImplItem

*Enum*

An item within an impl block.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Const(ImplItemConst)` - An associated constant within an impl block.
- `Fn(ImplItemFn)` - An associated function within an impl block.
- `Type(ImplItemType)` - An associated type within an impl block.
- `Macro(ImplItemMacro)` - A macro invocation within an impl block.
- `Verbatim(proc_macro2::TokenStream)` - Tokens within an impl block not interpreted by Syn.

**Trait Implementations:**

- **From**
  - `fn from(e: ImplItemConst) -> ImplItem`
- **From**
  - `fn from(e: ImplItemMacro) -> ImplItem`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: ImplItemType) -> ImplItem`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(e: ImplItemFn) -> ImplItem`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`



## syn::item::ImplItemConst

*Struct*

An associated constant within an impl block.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `defaultness: Option<$crate::token::Default>`
- `const_token: $crate::token::Const`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `colon_token: $crate::token::Colon`
- `ty: crate::ty::Type`
- `eq_token: $crate::token::Eq`
- `expr: crate::expr::Expr`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ImplItemFn

*Struct*

An associated function within an impl block.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `defaultness: Option<$crate::token::Default>`
- `sig: Signature`
- `block: crate::stmt::Block`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ImplItemMacro

*Struct*

A macro invocation within an impl block.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `mac: crate::mac::Macro`
- `semi_token: Option<$crate::token::Semi>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::ImplItemType

*Struct*

An associated type within an impl block.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `defaultness: Option<$crate::token::Default>`
- `type_token: $crate::token::Type`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `eq_token: $crate::token::Eq`
- `ty: crate::ty::Type`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::ImplRestriction

*Enum*

Unused, but reserved for RFC 3323 restrictions.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::Item

*Enum*

Things that can appear directly inside of a module or scope.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Const(ItemConst)` - A constant item: `const MAX: u16 = 65535`.
- `Enum(ItemEnum)` - An enum definition: `enum Foo<A, B> { A(A), B(B) }`.
- `ExternCrate(ItemExternCrate)` - An `extern crate` item: `extern crate serde`.
- `Fn(ItemFn)` - A free-standing function: `fn process(n: usize) -> Result<()> { ...
- `ForeignMod(ItemForeignMod)` - A block of foreign items: `extern "C" { ... }`.
- `Impl(ItemImpl)` - An impl block providing trait or associated items: `impl<A> Trait
- `Macro(ItemMacro)` - A macro invocation, which includes `macro_rules!` definitions.
- `Mod(ItemMod)` - A module or module declaration: `mod m` or `mod m { ... }`.
- `Static(ItemStatic)` - A static item: `static BIKE: Shed = Shed(42)`.
- `Struct(ItemStruct)` - A struct definition: `struct Foo<A> { x: A }`.
- `Trait(ItemTrait)` - A trait definition: `pub trait Iterator { ... }`.
- `TraitAlias(ItemTraitAlias)` - A trait alias: `pub trait SharableIterator = Iterator + Sync`.
- `Type(ItemType)` - A type alias: `type Result<T> = core::result::Result<T, MyError>`.
- `Union(ItemUnion)` - A union definition: `union Foo<A, B> { x: A, y: B }`.
- `Use(ItemUse)` - A use declaration: `use alloc::collections::HashMap`.
- `Verbatim(proc_macro2::TokenStream)` - Tokens forming an item not interpreted by Syn.

**Methods:**

- `fn replace_attrs(self: & mut Self, new: Vec<Attribute>) -> Vec<Attribute>`

**Trait Implementations:**

- **From**
  - `fn from(e: ItemStruct) -> Item`
- **From**
  - `fn from(e: ItemType) -> Item`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **From**
  - `fn from(e: ItemExternCrate) -> Item`
- **From**
  - `fn from(e: ItemImpl) -> Item`
- **From**
  - `fn from(e: ItemStatic) -> Item`
- **From**
  - `fn from(e: ItemTraitAlias) -> Item`
- **From**
  - `fn from(e: ItemUse) -> Item`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: ItemEnum) -> Item`
- **From**
  - `fn from(e: ItemForeignMod) -> Item`
- **From**
  - `fn from(e: ItemMod) -> Item`
- **From**
  - `fn from(input: DeriveInput) -> Item`
- **From**
  - `fn from(e: ItemTrait) -> Item`
- **From**
  - `fn from(e: ItemUnion) -> Item`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(e: ItemConst) -> Item`
- **From**
  - `fn from(e: ItemFn) -> Item`
- **From**
  - `fn from(e: ItemMacro) -> Item`



## syn::item::ItemConst

*Struct*

A constant item: `const MAX: u16 = 65535`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `const_token: $crate::token::Const`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `colon_token: $crate::token::Colon`
- `ty: alloc::boxed::Box<crate::ty::Type>`
- `eq_token: $crate::token::Eq`
- `expr: alloc::boxed::Box<crate::expr::Expr>`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ItemEnum

*Struct*

An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `enum_token: $crate::token::Enum`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `brace_token: token::Brace`
- `variants: crate::punctuated::Punctuated<crate::data::Variant, $crate::token::Comma>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::ItemExternCrate

*Struct*

An `extern crate` item: `extern crate serde`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `extern_token: $crate::token::Extern`
- `crate_token: $crate::token::Crate`
- `ident: crate::ident::Ident`
- `rename: Option<($crate::token::As, crate::ident::Ident)>`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::ItemFn

*Struct*

A free-standing function: `fn process(n: usize) -> Result<()> { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `sig: Signature`
- `block: alloc::boxed::Box<crate::stmt::Block>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::ItemForeignMod

*Struct*

A block of foreign items: `extern "C" { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `unsafety: Option<$crate::token::Unsafe>`
- `abi: crate::ty::Abi`
- `brace_token: token::Brace`
- `items: alloc::vec::Vec<ForeignItem>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ItemImpl

*Struct*

An impl block providing trait or associated items: `impl<A> Trait
for Data<A> { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `defaultness: Option<$crate::token::Default>`
- `unsafety: Option<$crate::token::Unsafe>`
- `impl_token: $crate::token::Impl`
- `generics: crate::generics::Generics`
- `trait_: Option<(Option<$crate::token::Not>, crate::path::Path, $crate::token::For)>` - Trait this impl implements.
- `self_ty: alloc::boxed::Box<crate::ty::Type>` - The Self type of the impl.
- `brace_token: token::Brace`
- `items: alloc::vec::Vec<ImplItem>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::ItemMacro

*Struct*

A macro invocation, which includes `macro_rules!` definitions.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `ident: Option<crate::ident::Ident>` - The `example` in `macro_rules! example { ... }`.
- `mac: crate::mac::Macro`
- `semi_token: Option<$crate::token::Semi>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ItemMod

*Struct*

A module or module declaration: `mod m` or `mod m { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `unsafety: Option<$crate::token::Unsafe>`
- `mod_token: $crate::token::Mod`
- `ident: crate::ident::Ident`
- `content: Option<(token::Brace, alloc::vec::Vec<Item>)>`
- `semi: Option<$crate::token::Semi>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::ItemStatic

*Struct*

A static item: `static BIKE: Shed = Shed(42)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `static_token: $crate::token::Static`
- `mutability: StaticMutability`
- `ident: crate::ident::Ident`
- `colon_token: $crate::token::Colon`
- `ty: alloc::boxed::Box<crate::ty::Type>`
- `eq_token: $crate::token::Eq`
- `expr: alloc::boxed::Box<crate::expr::Expr>`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::ItemStruct

*Struct*

A struct definition: `struct Foo<A> { x: A }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `struct_token: $crate::token::Struct`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `fields: crate::data::Fields`
- `semi_token: Option<$crate::token::Semi>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::ItemTrait

*Struct*

A trait definition: `pub trait Iterator { ... }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `unsafety: Option<$crate::token::Unsafe>`
- `auto_token: Option<$crate::token::Auto>`
- `restriction: Option<ImplRestriction>`
- `trait_token: $crate::token::Trait`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `colon_token: Option<$crate::token::Colon>`
- `supertraits: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>`
- `brace_token: token::Brace`
- `items: alloc::vec::Vec<TraitItem>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ItemTraitAlias

*Struct*

A trait alias: `pub trait SharableIterator = Iterator + Sync`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `trait_token: $crate::token::Trait`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `eq_token: $crate::token::Eq`
- `bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ItemType

*Struct*

A type alias: `type Result<T> = core::result::Result<T, MyError>`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `type_token: $crate::token::Type`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `eq_token: $crate::token::Eq`
- `ty: alloc::boxed::Box<crate::ty::Type>`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::ItemUnion

*Struct*

A union definition: `union Foo<A, B> { x: A, y: B }`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `union_token: $crate::token::Union`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `fields: crate::data::FieldsNamed`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::ItemUse

*Struct*

A use declaration: `use alloc::collections::HashMap`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `vis: crate::restriction::Visibility`
- `use_token: $crate::token::Use`
- `leading_colon: Option<$crate::token::PathSep>`
- `tree: UseTree`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::Receiver

*Struct*

The `self` argument of an associated method.

If `colon_token` is present, the receiver is written with an explicit
type such as `self: Box<Self>`. If `colon_token` is absent, the receiver
is written in shorthand such as `self` or `&self` or `&mut self`. In the
shorthand case, the type in `ty` is reconstructed as one of `Self`,
`&Self`, or `&mut Self`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `reference: Option<($crate::token::And, Option<crate::lifetime::Lifetime>)>`
- `mutability: Option<$crate::token::Mut>`
- `self_token: $crate::token::SelfValue`
- `colon_token: Option<$crate::token::Colon>`
- `ty: alloc::boxed::Box<crate::ty::Type>`

**Methods:**

- `fn lifetime(self: &Self) -> Option<&Lifetime>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::Signature

*Struct*

A function signature in a trait or implementation: `unsafe fn
initialize(&self)`.

**Fields:**
- `constness: Option<$crate::token::Const>`
- `asyncness: Option<$crate::token::Async>`
- `unsafety: Option<$crate::token::Unsafe>`
- `abi: Option<crate::ty::Abi>`
- `fn_token: $crate::token::Fn`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `paren_token: token::Paren`
- `inputs: crate::punctuated::Punctuated<FnArg, $crate::token::Comma>`
- `variadic: Option<Variadic>`
- `output: crate::ty::ReturnType`

**Methods:**

- `fn receiver(self: &Self) -> Option<&Receiver>` - A method's `self` receiver, such as `&self` or `self: Box<Self>`.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::item::StaticMutability

*Enum*

The mutability of an `Item::Static` or `ForeignItem::Static`.

**Variants:**
- `Mut($crate::token::Mut)`
- `None`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::TraitItem

*Enum*

An item declaration within the definition of a trait.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Const(TraitItemConst)` - An associated constant within the definition of a trait.
- `Fn(TraitItemFn)` - An associated function within the definition of a trait.
- `Type(TraitItemType)` - An associated type within the definition of a trait.
- `Macro(TraitItemMacro)` - A macro invocation within the definition of a trait.
- `Verbatim(proc_macro2::TokenStream)` - Tokens within the definition of a trait not interpreted by Syn.

**Trait Implementations:**

- **From**
  - `fn from(e: TraitItemType) -> TraitItem`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(e: TraitItemFn) -> TraitItem`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **From**
  - `fn from(e: TraitItemConst) -> TraitItem`
- **From**
  - `fn from(e: TraitItemMacro) -> TraitItem`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::TraitItemConst

*Struct*

An associated constant within the definition of a trait.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `const_token: $crate::token::Const`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `colon_token: $crate::token::Colon`
- `ty: crate::ty::Type`
- `default: Option<($crate::token::Eq, crate::expr::Expr)>`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::TraitItemFn

*Struct*

An associated function within the definition of a trait.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `sig: Signature`
- `default: Option<crate::stmt::Block>`
- `semi_token: Option<$crate::token::Semi>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::TraitItemMacro

*Struct*

A macro invocation within the definition of a trait.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `mac: crate::mac::Macro`
- `semi_token: Option<$crate::token::Semi>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::TraitItemType

*Struct*

An associated type within the definition of a trait.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `type_token: $crate::token::Type`
- `ident: crate::ident::Ident`
- `generics: crate::generics::Generics`
- `colon_token: Option<$crate::token::Colon>`
- `bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>`
- `default: Option<($crate::token::Eq, crate::ty::Type)>`
- `semi_token: $crate::token::Semi`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::UseGlob

*Struct*

A glob import in a `use` item: `*`.

**Fields:**
- `star_token: $crate::token::Star`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::UseGroup

*Struct*

A braced group of imports in a `use` item: `{A, B, C}`.

**Fields:**
- `brace_token: token::Brace`
- `items: crate::punctuated::Punctuated<UseTree, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::UseName

*Struct*

An identifier imported by a `use` item: `HashMap`.

**Fields:**
- `ident: crate::ident::Ident`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::UsePath

*Struct*

A path prefix of imports in a `use` item: `core::...`.

**Fields:**
- `ident: crate::ident::Ident`
- `colon2_token: $crate::token::PathSep`
- `tree: alloc::boxed::Box<UseTree>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::item::UseRename

*Struct*

An renamed identifier imported by a `use` item: `HashMap as Map`.

**Fields:**
- `ident: crate::ident::Ident`
- `as_token: $crate::token::As`
- `rename: crate::ident::Ident`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::item::UseTree

*Enum*

A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Path(UsePath)` - A path prefix of imports in a `use` item: `core::...`.
- `Name(UseName)` - An identifier imported by a `use` item: `HashMap`.
- `Rename(UseRename)` - An renamed identifier imported by a `use` item: `HashMap as Map`.
- `Glob(UseGlob)` - A glob import in a `use` item: `*`.
- `Group(UseGroup)` - A braced group of imports in a `use` item: `{A, B, C}`.

**Trait Implementations:**

- **From**
  - `fn from(e: UseName) -> UseTree`
- **From**
  - `fn from(e: UseGroup) -> UseTree`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: UsePath) -> UseTree`
- **From**
  - `fn from(e: UseGlob) -> UseTree`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<UseTree>`
- **From**
  - `fn from(e: UseRename) -> UseTree`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`



## syn::item::Variadic

*Struct*

The variadic argument of a foreign function.

```rust
# struct c_char;
# struct c_int;
#
extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    //                               ^^^
}
```

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `pat: Option<(alloc::boxed::Box<crate::pat::Pat>, $crate::token::Colon)>`
- `dots: $crate::token::DotDotDot`
- `comma: Option<$crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## Module: parsing



## Module: printing



