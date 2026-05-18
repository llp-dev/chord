**syn > ty**

# Module: ty

## Contents

**Modules**

- [`parsing`](#parsing)
- [`printing`](#printing)

**Structs**

- [`Abi`](#abi) - The binary interface of a function: `extern "C"`.
- [`BareFnArg`](#barefnarg) - An argument in a function type: the `usize` in `fn(usize) -> bool`.
- [`BareVariadic`](#barevariadic) - The variadic argument of a function pointer like `fn(usize, ...)`.
- [`TypeArray`](#typearray) - A fixed size array type: `[T; n]`.
- [`TypeBareFn`](#typebarefn) - A bare function type: `fn(usize) -> bool`.
- [`TypeGroup`](#typegroup) - A type contained within invisible delimiters.
- [`TypeImplTrait`](#typeimpltrait) - An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
- [`TypeInfer`](#typeinfer) - Indication that a type should be inferred by the compiler: `_`.
- [`TypeMacro`](#typemacro) - A macro in the type position.
- [`TypeNever`](#typenever) - The never type: `!`.
- [`TypeParen`](#typeparen) - A parenthesized type equivalent to the inner type.
- [`TypePath`](#typepath) - A path like `core::slice::Iter`, optionally qualified with a
- [`TypePtr`](#typeptr) - A raw pointer type: `*const T` or `*mut T`.
- [`TypeReference`](#typereference) - A reference type: `&'a T` or `&'a mut T`.
- [`TypeSlice`](#typeslice) - A dynamically sized slice type: `[T]`.
- [`TypeTraitObject`](#typetraitobject) - A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
- [`TypeTuple`](#typetuple) - A tuple type: `(A, B, C, String)`.

**Enums**

- [`ReturnType`](#returntype) - Return type of a function signature.
- [`Type`](#type) - The possible types that a Rust value could have.

---

## syn::ty::Abi

*Struct*

The binary interface of a function: `extern "C"`.

**Fields:**
- `extern_token: $crate::token::Extern`
- `name: Option<crate::lit::LitStr>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::BareFnArg

*Struct*

An argument in a function type: the `usize` in `fn(usize) -> bool`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `name: Option<(crate::ident::Ident, $crate::token::Colon)>`
- `ty: Type`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::ty::BareVariadic

*Struct*

The variadic argument of a function pointer like `fn(usize, ...)`.

**Fields:**
- `attrs: alloc::vec::Vec<crate::attr::Attribute>`
- `name: Option<(crate::ident::Ident, $crate::token::Colon)>`
- `dots: $crate::token::DotDotDot`
- `comma: Option<$crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::ReturnType

*Enum*

Return type of a function signature.

**Variants:**
- `Default` - Return type is not specified.
- `Type($crate::token::RArrow, alloc::boxed::Box<Type>)` - A particular type is returned.

**Methods:**

- `fn without_plus(input: ParseStream) -> Result<Self>`
- `fn parse(input: ParseStream, allow_plus: bool) -> Result<Self>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::ty::Type

*Enum*

The possible types that a Rust value could have.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

**Variants:**
- `Array(TypeArray)` - A fixed size array type: `[T; n]`.
- `BareFn(TypeBareFn)` - A bare function type: `fn(usize) -> bool`.
- `Group(TypeGroup)` - A type contained within invisible delimiters.
- `ImplTrait(TypeImplTrait)` - An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
- `Infer(TypeInfer)` - Indication that a type should be inferred by the compiler: `_`.
- `Macro(TypeMacro)` - A macro in the type position.
- `Never(TypeNever)` - The never type: `!`.
- `Paren(TypeParen)` - A parenthesized type equivalent to the inner type.
- `Path(TypePath)` - A path like `core::slice::Iter`, optionally qualified with a
- `Ptr(TypePtr)` - A raw pointer type: `*const T` or `*mut T`.
- `Reference(TypeReference)` - A reference type: `&'a T` or `&'a mut T`.
- `Slice(TypeSlice)` - A dynamically sized slice type: `[T]`.
- `TraitObject(TypeTraitObject)` - A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
- `Tuple(TypeTuple)` - A tuple type: `(A, B, C, String)`.
- `Verbatim(proc_macro2::TokenStream)` - Tokens in type position not interpreted by Syn.

**Methods:**

- `fn without_plus(input: ParseStream) -> Result<Self>` - In some positions, types may not contain the `+` character, to

**Trait Implementations:**

- **From**
  - `fn from(e: TypeNever) -> Type`
- **From**
  - `fn from(e: TypePtr) -> Type`
- **From**
  - `fn from(e: TypeTraitObject) -> Type`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(e: TypeGroup) -> Type`
- **From**
  - `fn from(e: TypeMacro) -> Type`
- **From**
  - `fn from(e: TypePath) -> Type`
- **From**
  - `fn from(e: TypeSlice) -> Type`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut ::proc_macro2::TokenStream)`
- **From**
  - `fn from(e: TypeBareFn) -> Type`
- **From**
  - `fn from(e: TypeInfer) -> Type`
- **From**
  - `fn from(e: TypeParen) -> Type`
- **From**
  - `fn from(e: TypeReference) -> Type`
- **From**
  - `fn from(e: TypeTuple) -> Type`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **From**
  - `fn from(e: TypeArray) -> Type`
- **From**
  - `fn from(e: TypeImplTrait) -> Type`



## syn::ty::TypeArray

*Struct*

A fixed size array type: `[T; n]`.

**Fields:**
- `bracket_token: token::Bracket`
- `elem: alloc::boxed::Box<Type>`
- `semi_token: $crate::token::Semi`
- `len: crate::expr::Expr`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`



## syn::ty::TypeBareFn

*Struct*

A bare function type: `fn(usize) -> bool`.

**Fields:**
- `lifetimes: Option<crate::generics::BoundLifetimes>`
- `unsafety: Option<$crate::token::Unsafe>`
- `abi: Option<Abi>`
- `fn_token: $crate::token::Fn`
- `paren_token: token::Paren`
- `inputs: crate::punctuated::Punctuated<BareFnArg, $crate::token::Comma>`
- `variadic: Option<BareVariadic>`
- `output: ReturnType`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypeGroup

*Struct*

A type contained within invisible delimiters.

**Fields:**
- `group_token: token::Group`
- `elem: alloc::boxed::Box<Type>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypeImplTrait

*Struct*

An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
a lifetime.

**Fields:**
- `impl_token: $crate::token::Impl`
- `bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>`

**Methods:**

- `fn without_plus(input: ParseStream) -> Result<Self>`
- `fn parse(input: ParseStream, allow_plus: bool) -> Result<Self>`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::ty::TypeInfer

*Struct*

Indication that a type should be inferred by the compiler: `_`.

**Fields:**
- `underscore_token: $crate::token::Underscore`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::ty::TypeMacro

*Struct*

A macro in the type position.

**Fields:**
- `mac: crate::mac::Macro`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypeNever

*Struct*

The never type: `!`.

**Fields:**
- `bang_token: $crate::token::Not`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypeParen

*Struct*

A parenthesized type equivalent to the inner type.

**Fields:**
- `paren_token: token::Paren`
- `elem: alloc::boxed::Box<Type>`

**Methods:**

- `fn parse(input: ParseStream, allow_plus: bool) -> Result<Self>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypePath

*Struct*

A path like `core::slice::Iter`, optionally qualified with a
self-type as in `<Vec<T> as SomeTrait>::Associated`.

**Fields:**
- `qself: Option<crate::path::QSelf>`
- `path: crate::path::Path`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypePtr

*Struct*

A raw pointer type: `*const T` or `*mut T`.

**Fields:**
- `star_token: $crate::token::Star`
- `const_token: Option<$crate::token::Const>`
- `mutability: Option<$crate::token::Mut>`
- `elem: alloc::boxed::Box<Type>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypeReference

*Struct*

A reference type: `&'a T` or `&'a mut T`.

**Fields:**
- `and_token: $crate::token::And`
- `lifetime: Option<crate::lifetime::Lifetime>`
- `mutability: Option<$crate::token::Mut>`
- `elem: alloc::boxed::Box<Type>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypeSlice

*Struct*

A dynamically sized slice type: `[T]`.

**Fields:**
- `bracket_token: token::Bracket`
- `elem: alloc::boxed::Box<Type>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypeTraitObject

*Struct*

A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
trait or a lifetime.

**Fields:**
- `dyn_token: Option<$crate::token::Dyn>`
- `bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>`

**Methods:**

- `fn without_plus(input: ParseStream) -> Result<Self>`
- `fn parse(input: ParseStream, allow_plus: bool) -> Result<Self>`
- `fn parse_bounds(dyn_span: Span, input: ParseStream, allow_plus: bool) -> Result<Punctuated<TypeParamBound, $crate::token::Plus>>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## syn::ty::TypeTuple

*Struct*

A tuple type: `(A, B, C, String)`.

**Fields:**
- `paren_token: token::Paren`
- `elems: crate::punctuated::Punctuated<Type, $crate::token::Comma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Parse**
  - `fn parse(input: ParseStream) -> Result<Self>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## Module: parsing



## Module: printing



