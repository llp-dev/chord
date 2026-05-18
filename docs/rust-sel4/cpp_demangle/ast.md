**cpp_demangle > ast**

# Module: ast

## Contents

**Structs**

- [`AbiTag`](#abitag) - The `<abi-tag>` non-terminal.
- [`AbiTags`](#abitags) - The `<abi-tags>` non-terminal.
- [`ArgScopeStack`](#argscopestack) - An `ArgScopeStack` represents the current function and template demangling
- [`BareFunctionType`](#barefunctiontype) - The `<bare-function-type>` production.
- [`CloneSuffix`](#clonesuffix) - <clone-suffix> ::= [ . <clone-type-identifier> ] [ . <nonnegative number> ]*
- [`CloneTypeIdentifier`](#clonetypeidentifier) - The `<clone-type-identifier>` pseudo-terminal.
- [`ClosureTypeName`](#closuretypename) - The `<closure-type-name>` production.
- [`CvQualifiers`](#cvqualifiers) - The `<CV-qualifiers>` production.
- [`DataMemberPrefix`](#datamemberprefix) - The `<data-member-prefix>` production.
- [`Discriminator`](#discriminator) - The `<discriminator>` production.
- [`FunctionParam`](#functionparam) - The <function-param> production.
- [`FunctionType`](#functiontype) - The `<function-type>` production.
- [`Identifier`](#identifier) - The `<identifier>` pseudo-terminal.
- [`Initializer`](#initializer) - The `<initializer>` production.
- [`LambdaSig`](#lambdasig) - The `<lambda-sig>` production.
- [`MemberName`](#membername) - In libiberty, Member and DerefMember expressions have special handling.
- [`NonSubstitution`](#nonsubstitution) - A handle to a component that is usually substitutable, and lives in the
- [`NvOffset`](#nvoffset) - A non-virtual offset, as described by the <nv-offset> production.
- [`ParseContext`](#parsecontext) - Common context needed when parsing.
- [`PointerToMemberType`](#pointertomembertype) - The `<pointer-to-member-type>` production.
- [`QualifiedBuiltin`](#qualifiedbuiltin) - A built-in type with CV-qualifiers.
- [`ResourceName`](#resourcename) - The `<resource name>` pseudo-terminal.
- [`SeqId`](#seqid) - A <seq-id> production encoding a base-36 positive number.
- [`SimpleId`](#simpleid) - The `<simple-id>` production.
- [`SourceName`](#sourcename) - The `<source-name>` non-terminal.
- [`SubobjectExpr`](#subobjectexpr) - The subobject expression production.
- [`TemplateArgs`](#templateargs) - The `<template-args>` production.
- [`TemplateParam`](#templateparam) - The `<template-param>` production.
- [`TemplateTemplateParam`](#templatetemplateparam) - The `<template-template-param>` production.
- [`UnnamedTypeName`](#unnamedtypename) - The `<unnamed-type-name>` production.
- [`UnresolvedQualifierLevel`](#unresolvedqualifierlevel) - The `<unresolved-qualifier-level>` production.
- [`UnscopedTemplateName`](#unscopedtemplatename) - The `<unscoped-template-name>` production.
- [`VOffset`](#voffset) - A virtual offset, as described by the <v-offset> production.

**Enums**

- [`ArrayType`](#arraytype) - The `<array-type>` production.
- [`BaseUnresolvedName`](#baseunresolvedname) - The `<base-unresolved-name>` production.
- [`BuiltinType`](#builtintype) - The `<builtin-type>` production.
- [`CallOffset`](#calloffset) - The `<call-offset>` production.
- [`ClassEnumType`](#classenumtype) - The `<class-enum-type>` production.
- [`CtorDtorName`](#ctordtorname) - The `<ctor-dtor-name>` production.
- [`Decltype`](#decltype) - The `<decltype>` production.
- [`DestructorName`](#destructorname) - The `<destructor-name>` production.
- [`Encoding`](#encoding) - The `<encoding>` production.
- [`ExceptionSpec`](#exceptionspec) - The `<exception-spec>` production.
- [`ExplicitObjectParameter`](#explicitobjectparameter) - A named explicit object parameter.
- [`ExprPrimary`](#exprprimary) - The `<expr-primary>` production.
- [`Expression`](#expression) - The `<expression>` production.
- [`FoldExpr`](#foldexpr) - The fold expressions.
- [`GlobalCtorDtor`](#globalctordtor) - A global constructor or destructor.
- [`LocalName`](#localname) - The `<local-name>` production.
- [`MangledName`](#mangledname) - The root AST node, and starting production.
- [`Name`](#name) - The `<name>` production.
- [`NestedName`](#nestedname) - The `<nested-name>` production.
- [`OperatorName`](#operatorname) - The `<operator-name>` production.
- [`ParametricBuiltinType`](#parametricbuiltintype) - <builtin-type> ::= DF <number> _ # ISO/IEC TS 18661 binary floating point type _FloatN (N bits), C++23 std::floatN_t
- [`Prefix`](#prefix) - The `<prefix>` production.
- [`PrefixHandle`](#prefixhandle) - A reference to a parsed `<prefix>` production.
- [`RefQualifier`](#refqualifier) - A <ref-qualifier> production.
- [`SimpleOperatorName`](#simpleoperatorname) - The `<simple-operator-name>` production.
- [`SpecialName`](#specialname) - The `<special-name>` production.
- [`StandardBuiltinType`](#standardbuiltintype) - A one of the standard variants of the <builtin-type> production.
- [`Substitution`](#substitution) - The `<substitution>` form: a back-reference to some component we've already
- [`TemplateArg`](#templatearg) - A <template-arg> production.
- [`TemplateTemplateParamHandle`](#templatetemplateparamhandle) - A reference to a parsed `TemplateTemplateParam`.
- [`Type`](#type) - The `<type>` production.
- [`TypeHandle`](#typehandle) - A reference to a parsed `Type` production.
- [`UnqualifiedName`](#unqualifiedname) - The `<unqualified-name>` production.
- [`UnresolvedName`](#unresolvedname) - The `<unresolved-name>` production.
- [`UnresolvedType`](#unresolvedtype) - The `<unresolved-type>` production.
- [`UnresolvedTypeHandle`](#unresolvedtypehandle) - A reference to a parsed `<unresolved-type>` production.
- [`UnscopedName`](#unscopedname) - The `<unscoped-name>` production.
- [`UnscopedTemplateNameHandle`](#unscopedtemplatenamehandle) - A handle to an `UnscopedTemplateName`.
- [`VectorType`](#vectortype) - The `<vector-type>` production.
- [`WellKnownComponent`](#wellknowncomponent) - The `<substitution>` variants that are encoded directly in the grammar,

---

## cpp_demangle::ast::AbiTag

*Struct*

The `<abi-tag>` non-terminal.

```text
<abi-tag> ::= B <source-name>
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &AbiTag) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AbiTag`



## cpp_demangle::ast::AbiTags

*Struct*

The `<abi-tags>` non-terminal.

```text
<abi-tags> ::= <abi-tag> [<abi-tags>]
```

To make things easier on ourselves, despite the fact that the `<abi-tags>`
production requires at least one tag, we'll allow a zero-length vector
here instead of having to use Option<AbiTags> in everything that accepts
an AbiTags.

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &AbiTags) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AbiTags`
- **Default**
  - `fn default() -> AbiTags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::ArgScopeStack

*Struct*

An `ArgScopeStack` represents the current function and template demangling
scope we are within. As we enter new demangling scopes, we construct new
`ArgScopeStack`s whose `prev` references point back to the old ones. These
`ArgScopeStack`s are kept on the native stack, and as functions return, they
go out of scope and we use the previous `ArgScopeStack`s again.

**Generic Parameters:**
- 'prev
- 'subs

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ArgScopeStack<'prev, 'subs>`



## cpp_demangle::ast::ArrayType

*Enum*

The `<array-type>` production.

```text
<array-type> ::= A <positive dimension number> _ <element type>
             ::= A [<dimension expression>] _ <element type>
```

**Variants:**
- `DimensionNumber(usize, TypeHandle)` - An array with a number-literal dimension.
- `DimensionExpression(Expression, TypeHandle)` - An array with an expression for its dimension.
- `NoDimension(TypeHandle)` - An array with no dimension.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArrayType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ArrayType`



## cpp_demangle::ast::BareFunctionType

*Struct*

The `<bare-function-type>` production.

```text
<bare-function-type> ::= <signature type>+
     # types are possible return type, then parameter types
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BareFunctionType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BareFunctionType`



## cpp_demangle::ast::BaseUnresolvedName

*Enum*

The `<base-unresolved-name>` production.

```text
<base-unresolved-name> ::= <simple-id>                        # unresolved name
                       ::= on <operator-name>                 # unresolved operator-function-id
                       ::= on <operator-name> <template-args> # unresolved operator template-id
                       ::= dn <destructor-name>               # destructor or pseudo-destructor;
                                                              # e.g. ~X or ~X<N-1>
```

**Variants:**
- `Name(SimpleId)` - An unresolved name.
- `Operator(OperatorName, Option<TemplateArgs>)` - An unresolved function or template function name.
- `Destructor(DestructorName)` - An unresolved destructor name.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BaseUnresolvedName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BaseUnresolvedName`



## cpp_demangle::ast::BuiltinType

*Enum*

The `<builtin-type>` production.

**Variants:**
- `Standard(StandardBuiltinType)` - A simple standards compliant builtin type.
- `Parametric(ParametricBuiltinType)` - A standards compliant builtin type with a parameter, e.g. _BitInt(32).
- `Extension(SourceName)` - A non-standard, vendor extension type.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BuiltinType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BuiltinType`



## cpp_demangle::ast::CallOffset

*Enum*

The `<call-offset>` production.

```text
<call-offset> ::= h <nv-offset> _
              ::= v <v-offset> _
```

**Variants:**
- `NonVirtual(NvOffset)` - A non-virtual offset.
- `Virtual(VOffset)` - A virtual offset.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CallOffset) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CallOffset`



## cpp_demangle::ast::ClassEnumType

*Enum*

The `<class-enum-type>` production.

```text
<class-enum-type> ::= <name>
                  ::= Ts <name>
                  ::= Tu <name>
                  ::= Te <name>
```

**Variants:**
- `Named(Name)` - A non-dependent type name, dependent type name, or dependent
- `ElaboratedStruct(Name)` - A dependent elaborated type specifier using 'struct' or 'class'.
- `ElaboratedUnion(Name)` - A dependent elaborated type specifier using 'union'.
- `ElaboratedEnum(Name)` - A dependent elaborated type specifier using 'enum'.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ClassEnumType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ClassEnumType`



## cpp_demangle::ast::CloneSuffix

*Struct*

<clone-suffix> ::= [ . <clone-type-identifier> ] [ . <nonnegative number> ]*

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &CloneSuffix) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CloneSuffix`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::CloneTypeIdentifier

*Struct*

The `<clone-type-identifier>` pseudo-terminal.

```text
<clone-type-identifier> ::= <unqualified source code identifier>
```

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CloneTypeIdentifier) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CloneTypeIdentifier`



## cpp_demangle::ast::ClosureTypeName

*Struct*

The `<closure-type-name>` production.

```text
<closure-type-name> ::= Ul <lambda-sig> E [ <nonnegative number> ] _
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ClosureTypeName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ClosureTypeName`



## cpp_demangle::ast::CtorDtorName

*Enum*

The `<ctor-dtor-name>` production.

```text
<ctor-dtor-name> ::= C1  # complete object constructor
                 ::= C2  # base object constructor
                 ::= C3  # complete object allocating constructor
                 ::= D0  # deleting destructor
                 ::= D1  # complete object destructor
                 ::= D2  # base object destructor
```

GCC also emits a C4 constructor under some conditions when building
an optimized binary. GCC's source says:

```
/* This is the old-style "[unified]" constructor.
   In some cases, we may emit this function and call
   it from the clones in order to share code and save space.  */
```

Based on the GCC source we'll call this the "maybe in-charge constructor".
Similarly, there is a D4 destructor, the "maybe in-charge destructor".

**Variants:**
- `CompleteConstructor(Option<TypeHandle>)` - "C1", the "complete object constructor"
- `BaseConstructor(Option<TypeHandle>)` - "C2", the "base object constructor"
- `CompleteAllocatingConstructor(Option<TypeHandle>)` - "C3", the "complete object allocating constructor"
- `MaybeInChargeConstructor(Option<TypeHandle>)` - "C4", the "maybe in-charge constructor"
- `DeletingDestructor` - "D0", the "deleting destructor"
- `CompleteDestructor` - "D1", the "complete object destructor"
- `BaseDestructor` - "D2", the "base object destructor"
- `MaybeInChargeDestructor` - "D4", the "maybe in-charge destructor"

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CtorDtorName`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CtorDtorName) -> bool`



## cpp_demangle::ast::CvQualifiers

*Struct*

The `<CV-qualifiers>` production.

```text
<CV-qualifiers> ::= [r] [V] [K]   # restrict (C99), volatile, const
```

**Fields:**
- `restrict: bool` - Is this `restrict` qualified?
- `volatile: bool` - Is this `volatile` qualified?
- `const_: bool` - Is this `const` qualified?

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CvQualifiers`
- **PartialEq**
  - `fn eq(self: &Self, other: &CvQualifiers) -> bool`
- **Default**
  - `fn default() -> CvQualifiers`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::DataMemberPrefix

*Struct*

The `<data-member-prefix>` production.

```text
<data-member-prefix> := <member source-name> M
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DataMemberPrefix) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DataMemberPrefix`



## cpp_demangle::ast::Decltype

*Enum*

The `<decltype>` production.

```text
<decltype> ::= Dt <expression> E
           ::= DT <expression> E
```

**Variants:**
- `IdExpression(Expression)` - A `decltype` of an id-expression or class member access (C++0x).
- `Expression(Expression)` - A `decltype` of an expression (C++0x).

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Decltype`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Decltype) -> bool`



## cpp_demangle::ast::DestructorName

*Enum*

The `<destructor-name>` production.

```text
<destructor-name> ::= <unresolved-type> # e.g., ~T or ~decltype(f())
                  ::= <simple-id>       # e.g., ~A<2*N>
```

**Variants:**
- `Unresolved(UnresolvedTypeHandle)` - A destructor for an unresolved type.
- `Name(SimpleId)` - A destructor for a resolved type name.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DestructorName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DestructorName`



## cpp_demangle::ast::Discriminator

*Struct*

The `<discriminator>` production.

```text
<discriminator> := _ <non-negative number>      # when number < 10
                := __ <non-negative number> _   # when number >= 10
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Discriminator) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Discriminator`



## cpp_demangle::ast::Encoding

*Enum*

The `<encoding>` production.

```text
<encoding> ::= <function name> <bare-function-type>
           ::= <data name>
           ::= <special-name>
```

**Variants:**
- `Function(Name, BareFunctionType)` - An encoded function.
- `Data(Name)` - An encoded static variable.
- `Special(SpecialName)` - A special encoding.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Encoding) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Encoding`



## cpp_demangle::ast::ExceptionSpec

*Enum*

The `<exception-spec>` production.

<exception-spec> ::= Do                # non-throwing exception-specification (e.g., noexcept, throw())
                 ::= DO <expression> E # computed (instantiation-dependent) noexcept
                 ::= Dw <type>+ E      # dynamic exception specification with instantiation-dependent types

**Variants:**
- `NoExcept` - noexcept
- `Computed(Expression)` - noexcept(expression)

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ExceptionSpec) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ExceptionSpec`



## cpp_demangle::ast::ExplicitObjectParameter

*Enum*

A named explicit object parameter.

**Variants:**
- `ExplicitObjectParameter` - this

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ExplicitObjectParameter) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ExplicitObjectParameter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::ExprPrimary

*Enum*

The `<expr-primary>` production.

```text
<expr-primary> ::= L <type> <value number> E                        # integer literal
               ::= L <type> <value float> E                         # floating literal
               ::= L <string type> E                                # string literal
               ::= L <nullptr type> E                               # nullptr literal (i.e., "LDnE")
               ::= L <pointer type> 0 E                             # null pointer template argument
               ::= L <type> <real-part float> _ <imag-part float> E # complex floating point literal (C 2000)
               ::= L <mangled-name> E                               # external name
```

**Variants:**
- `Literal(TypeHandle, usize, usize)` - A type literal.
- `External(MangledName)` - An external name.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ExprPrimary) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ExprPrimary`



## cpp_demangle::ast::Expression

*Enum*

The `<expression>` production.

```text
 <expression> ::= <unary operator-name> <expression>
              ::= <binary operator-name> <expression> <expression>
              ::= <ternary operator-name> <expression> <expression> <expression>
              ::= pp_ <expression>                             # prefix ++
              ::= mm_ <expression>                             # prefix --
              ::= cl <expression>+ E                           # expression (expr-list), call
              ::= cv <type> <expression>                       # type (expression), conversion with one argument
              ::= cv <type> _ <expression>* E                  # type (expr-list), conversion with other than one argument
              ::= tl <type> <expression>* E                    # type {expr-list}, conversion with braced-init-list argument
              ::= il <expression> E                            # {expr-list}, braced-init-list in any other context
              ::= [gs] nw <expression>* _ <type> E             # new (expr-list) type
              ::= [gs] nw <expression>* _ <type> <initializer> # new (expr-list) type (init)
              ::= [gs] na <expression>* _ <type> E             # new[] (expr-list) type
              ::= [gs] na <expression>* _ <type> <initializer> # new[] (expr-list) type (init)
              ::= [gs] dl <expression>                         # delete expression
              ::= [gs] da <expression>                         # delete[] expression
              ::= dc <type> <expression>                       # dynamic_cast<type> (expression)
              ::= sc <type> <expression>                       # static_cast<type> (expression)
              ::= cc <type> <expression>                       # const_cast<type> (expression)
              ::= rc <type> <expression>                       # reinterpret_cast<type> (expression)
              ::= ti <type>                                    # typeid (type)
              ::= te <expression>                              # typeid (expression)
              ::= st <type>                                    # sizeof (type)
              ::= sz <expression>                              # sizeof (expression)
              ::= at <type>                                    # alignof (type)
              ::= az <expression>                              # alignof (expression)
              ::= nx <expression>                              # noexcept (expression)
              ::= so <subobject-expr>
              ::= <template-param>
              ::= <function-param>
              ::= dt <expression> <unresolved-name>            # expr.name
              ::= pt <expression> <unresolved-name>            # expr->name
              ::= ds <expression> <expression>                 # expr.*expr
              ::= sZ <template-param>                          # sizeof...(T), size of a template parameter pack
              ::= sZ <function-param>                          # sizeof...(parameter), size of a function parameter pack
              ::= sP <template-arg>* E                         # sizeof...(T), size of a captured template parameter pack from an alias template
              ::= sp <expression>                              # expression..., pack expansion
              ::= fl <binary operator-name> <expression>       # (... operator expression), unary left fold
              ::= fr <binary operator-name> <expression>       # (expression operator ...), unary right fold
              ::= fL <binary operator-name> <expression> <expression> # (expression operator ... operator expression), binary left fold
              ::= fR <binary operator-name> <expression> <expression> # (expression operator ... operator expression), binary right fold
              ::= tw <expression>                              # throw expression
              ::= tr                                           # throw with no operand (rethrow)
              ::= <unresolved-name>                            # f(p), N::f(p), ::f(p),
                                                               # freestanding dependent name (e.g., T::x),
                                                               # objectless nonstatic member reference
              ::= <expr-primary>
```

**Variants:**
- `Unary(OperatorName, alloc::boxed::Box<Expression>)` - A unary operator expression.
- `Binary(OperatorName, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>)` - A binary operator expression.
- `Ternary(OperatorName, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>)` - A ternary operator expression.
- `PrefixInc(alloc::boxed::Box<Expression>)` - A prefix `++`.
- `PrefixDec(alloc::boxed::Box<Expression>)` - A prefix `--`.
- `Call(alloc::boxed::Box<Expression>, alloc::vec::Vec<Expression>)` - A call with functor and arguments.
- `ConversionOne(TypeHandle, alloc::boxed::Box<Expression>)` - A type conversion with one argument.
- `ConversionMany(TypeHandle, alloc::vec::Vec<Expression>)` - A type conversion with many arguments.
- `ConversionBraced(TypeHandle, alloc::vec::Vec<Expression>)` - A type conversion with many arguments.
- `BracedInitList(alloc::vec::Vec<Expression>)` - A braced init list expression.
- `New(alloc::vec::Vec<Expression>, TypeHandle, Option<Initializer>)` - The `new` operator.
- `GlobalNew(alloc::vec::Vec<Expression>, TypeHandle, Option<Initializer>)` - The global `::new` operator.
- `NewArray(alloc::vec::Vec<Expression>, TypeHandle, Option<Initializer>)` - The `new[]` operator.
- `GlobalNewArray(alloc::vec::Vec<Expression>, TypeHandle, Option<Initializer>)` - The global `::new[]` operator.
- `Delete(alloc::boxed::Box<Expression>)` - The `delete` operator.
- `GlobalDelete(alloc::boxed::Box<Expression>)` - The global `::delete` operator.
- `DeleteArray(alloc::boxed::Box<Expression>)` - The `delete[]` operator.
- `GlobalDeleteArray(alloc::boxed::Box<Expression>)` - The global `::delete[]` operator.
- `DynamicCast(TypeHandle, alloc::boxed::Box<Expression>)` - `dynamic_cast<type> (expression)`
- `StaticCast(TypeHandle, alloc::boxed::Box<Expression>)` - `static_cast<type> (expression)`
- `ConstCast(TypeHandle, alloc::boxed::Box<Expression>)` - `const_cast<type> (expression)`
- `ReinterpretCast(TypeHandle, alloc::boxed::Box<Expression>)` - `reinterpret_cast<type> (expression)`
- `TypeidType(TypeHandle)` - `typeid (type)`
- `TypeidExpr(alloc::boxed::Box<Expression>)` - `typeid (expression)`
- `SizeofType(TypeHandle)` - `sizeof (type)`
- `SizeofExpr(alloc::boxed::Box<Expression>)` - `sizeof (expression)`
- `AlignofType(TypeHandle)` - `alignof (type)`
- `AlignofExpr(alloc::boxed::Box<Expression>)` - `alignof (expression)`
- `Noexcept(alloc::boxed::Box<Expression>)` - `noexcept (expression)`
- `Subobject(SubobjectExpr)` - Subobject expression,
- `TemplateParam(TemplateParam)` - A named template parameter.
- `FunctionParam(FunctionParam)` - A function parameter.
- `Member(alloc::boxed::Box<Expression>, MemberName)` - `expr.name`
- `DerefMember(alloc::boxed::Box<Expression>, MemberName)` - `expr->name`
- `PointerToMember(alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>)` - `expr.*expr`
- `SizeofTemplatePack(TemplateParam)` - `sizeof...(T)`, size of a template parameter pack.
- `SizeofFunctionPack(FunctionParam)` - `sizeof...(parameter)`, size of a function parameter pack.
- `SizeofCapturedTemplatePack(alloc::vec::Vec<TemplateArg>)` - `sizeof...(T)`, size of a captured template parameter pack from an alias
- `PackExpansion(alloc::boxed::Box<Expression>)` - `expression...`, pack expansion.
- `Fold(FoldExpr)` - The fold expressions.
- `Throw(alloc::boxed::Box<Expression>)` - `throw expression`
- `Rethrow` - `throw` with no operand
- `UnresolvedName(UnresolvedName)` - `f(p)`, `N::f(p)`, `::f(p)`, freestanding dependent name (e.g., `T::x`),
- `Primary(ExprPrimary)` - An `<expr-primary>` production.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Expression`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Expression) -> bool`



## cpp_demangle::ast::FoldExpr

*Enum*

The fold expressions.

These are not separate productions in the grammar but our code is cleaner
if we handle them all together.

<expression>  ::= ...
              ::= fl <binary operator-name> <expression>       # (... operator expression), unary left fold
              ::= fr <binary operator-name> <expression>       # (expression operator ...), unary right fold
              ::= fL <binary operator-name> <expression> <expression> # (expression operator ... operator expression), binary left fold
              ::= fR <binary operator-name> <expression> <expression> # (expression operator ... operator expression), binary right fold
              ::= ...

**Variants:**
- `UnaryLeft(SimpleOperatorName, alloc::boxed::Box<Expression>)` - (...+<expr>)
- `UnaryRight(SimpleOperatorName, alloc::boxed::Box<Expression>)` - (<expr>+...)
- `BinaryLeft(SimpleOperatorName, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>)` - (<expr1>+...+<expr2>)
- `BinaryRight(SimpleOperatorName, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>)` - (<expr1>+...+<expr2>)

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &FoldExpr) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> FoldExpr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::FunctionParam

*Struct*

The <function-param> production.

```text
<function-param> ::= fp <top-level CV-qualifiers> _
                         # L == 0, first parameter
                 ::= fp <top-level CV-qualifiers> <parameter-2 non-negative number> _
                         # L == 0, second and later parameters
                 ::= fL <L-1 non-negative number> p <top-level CV-qualifiers> _
                         # L > 0, first parameter
                 ::= fL <L-1 non-negative number> p <top-level CV-qualifiers> <parameter-2 non-negative number> _
                         # L > 0, second and later parameters
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FunctionParam) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> FunctionParam`



## cpp_demangle::ast::FunctionType

*Struct*

The `<function-type>` production.

```text
<function-type> ::= [<CV-qualifiers>] [exception-spec] [Dx] F [Y] <bare-function-type> [<ref-qualifier>] E
```

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FunctionType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> FunctionType`



## cpp_demangle::ast::GlobalCtorDtor

*Enum*

A global constructor or destructor.

**Variants:**
- `Ctor(alloc::boxed::Box<MangledName>)` - A global constructor.
- `Dtor(alloc::boxed::Box<MangledName>)` - A global destructor.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &GlobalCtorDtor) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> GlobalCtorDtor`



## cpp_demangle::ast::Identifier

*Struct*

The `<identifier>` pseudo-terminal.

```text
<identifier> ::= <unqualified source code identifier>
```

> `<identifier>` is a pseudo-terminal representing the characters in the
> unqualified identifier for the entity in the source code. This ABI does not
> yet specify a mangling for identifiers containing characters outside of
> `_A-Za-z0-9.`.

Mangled symbols' identifiers also have `$` characters in the wild.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Identifier) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Identifier`



## cpp_demangle::ast::Initializer

*Struct*

The `<initializer>` production.

```text
<initializer> ::= pi <expression>* E # parenthesized initialization
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Initializer) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Initializer`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::LambdaSig

*Struct*

The `<lambda-sig>` production.

```text
<lambda-sig> ::= <parameter type>+  # Parameter types or "v" if the lambda has no parameters
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &LambdaSig) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> LambdaSig`



## cpp_demangle::ast::LocalName

*Enum*

The `<local-name>` production.

```text
<local-name> := Z <function encoding> E <entity name> [<discriminator>]
             := Z <function encoding> E s [<discriminator>]
             := Z <function encoding> Ed [ <parameter number> ] _ <entity name>
```

**Variants:**
- `Relative(alloc::boxed::Box<Encoding>, Option<alloc::boxed::Box<Name>>, Option<Discriminator>)` - The mangling of the enclosing function, the mangling of the entity
- `Default(alloc::boxed::Box<Encoding>, Option<usize>, alloc::boxed::Box<Name>)` - A default argument in a class definition.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LocalName`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &LocalName) -> bool`



## cpp_demangle::ast::MangledName

*Enum*

The root AST node, and starting production.

```text
<mangled-name> ::= _Z <encoding> [<clone-suffix>]*
               ::= ___Z <encoding> <block_invoke>
               ::= <type>

<block_invoke> ::= _block_invoke
               ::= _block_invoke<decimal-digit>+
               ::= _block_invoke_<decimal-digit>+
```

**Variants:**
- `Encoding(Encoding, alloc::vec::Vec<CloneSuffix>)` - The encoding of the mangled symbol name.
- `BlockInvoke(Encoding, Option<isize>)` - The encoding of the mangled symbol name.
- `Type(TypeHandle)` - A top-level type. Technically not allowed by the standard, however in
- `GlobalCtorDtor(GlobalCtorDtor)` - A global constructor or destructor. This is another de facto standard

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &MangledName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> MangledName`



## cpp_demangle::ast::MemberName

*Struct*

In libiberty, Member and DerefMember expressions have special handling.
They parse an `UnqualifiedName` (not an `UnscopedName` as the cxxabi docs
say) and optionally a `TemplateArgs` if it is present. We can't just parse
a `Name` or an `UnscopedTemplateName` here because that allows other inputs
that libiberty does not.

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &MemberName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> MemberName`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::Name

*Enum*

The `<name>` production.

```text
<name> ::= <nested-name>
       ::= <unscoped-name>
       ::= <unscoped-template-name> <template-args>
       ::= <local-name>
```

**Variants:**
- `Nested(NestedName)` - A nested name
- `Unscoped(UnscopedName)` - An unscoped name.
- `UnscopedTemplate(UnscopedTemplateNameHandle, TemplateArgs)` - An unscoped template.
- `Local(LocalName)` - A local name.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Name) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Name`



## cpp_demangle::ast::NestedName

*Enum*

The `<nested-name>` production.

```text
<nested-name> ::= N [<CV-qualifiers>] [<ref-qualifier>] <prefix> <unqualified-name> E
              ::= N [<CV-qualifiers>] [<ref-qualifier>] <template-prefix> <template-args> E
              ::= N H <prefix> <unqualified-name> E
              ::= N H <template-prefix> <template-args> E
```

**Variants:**
- `Unqualified(CvQualifiers, Option<RefQualifier>, Option<PrefixHandle>, UnqualifiedName)` - A nested name.
- `Template(CvQualifiers, Option<RefQualifier>, PrefixHandle)` - A nested template name. The `<template-args>` are part of the `PrefixHandle`.
- `UnqualifiedExplicitObject(Option<PrefixHandle>, UnqualifiedName, ExplicitObjectParameter)` - A nested name with an explicit object.
- `TemplateExplicitObject(PrefixHandle, ExplicitObjectParameter)` - A nested template name with an explicit object.

**Methods:**

- `fn cv_qualifiers(self: &Self) -> Option<&CvQualifiers>` - Get the CV-qualifiers for this name.
- `fn ref_qualifier(self: &Self) -> Option<&RefQualifier>` - Get the ref-qualifier for this name, if one exists.
- `fn has_explicit_obj_param(self: &Self) -> bool` - Check to see if the object has an explicit named parameter.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NestedName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NestedName`



## cpp_demangle::ast::NonSubstitution

*Struct*

A handle to a component that is usually substitutable, and lives in the
substitutions table, but in this particular case does not qualify for
substitutions.

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> NonSubstitution`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonSubstitution) -> bool`



## cpp_demangle::ast::NvOffset

*Struct*

A non-virtual offset, as described by the <nv-offset> production.

```text
<nv-offset> ::= <offset number>
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NvOffset`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NvOffset) -> bool`



## cpp_demangle::ast::OperatorName

*Enum*

The `<operator-name>` production.

```text
<operator-name> ::= <simple-operator-name>
                ::= cv <type>               # (cast)
                ::= li <source-name>        # operator ""
                ::= v <digit> <source-name> # vendor extended operator
```

**Variants:**
- `Simple(SimpleOperatorName)` - A simple operator name.
- `Cast(TypeHandle)` - A type cast.
- `Conversion(TypeHandle)` - A type conversion.
- `Literal(SourceName)` - Operator literal, ie `operator ""`.
- `VendorExtension(u8, SourceName)` - A non-standard, vendor extension operator.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &OperatorName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> OperatorName`



## cpp_demangle::ast::ParametricBuiltinType

*Enum*

<builtin-type> ::= DF <number> _ # ISO/IEC TS 18661 binary floating point type _FloatN (N bits), C++23 std::floatN_t
               ::= DF <number> x # IEEE extended precision formats, C23 _FloatNx (N bits)
               ::= DB <number> _        # C23 signed _BitInt(N)
               ::= DB <instantiation-dependent expression> _ # C23 signed _BitInt(N)
               ::= DU <number> _        # C23 unsigned _BitInt(N)
               ::= DU <instantiation-dependent expression> _ # C23 unsigned _BitInt(N)

**Variants:**
- `FloatN(isize)` - _FloatN
- `FloatNx(isize)` - _FloatNx
- `SignedBitInt(isize)` - signed _BitInt(N)
- `UnsignedBitInt(isize)` - unsigned _BitInt(N)
- `SignedBitIntExpression(alloc::boxed::Box<Expression>)` - signed _BitInt(expr)
- `UnsignedBitIntExpression(alloc::boxed::Box<Expression>)` - unsigned _BitInt(expr)

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ParametricBuiltinType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ParametricBuiltinType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::ParseContext

*Struct*

Common context needed when parsing.

**Methods:**

- `fn new(options: ParseOptions) -> ParseContext` - Construct a new `ParseContext`.
- `fn recursion_level(self: &Self) -> u32` - Get the current recursion level for this context.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ParseContext`



## cpp_demangle::ast::PointerToMemberType

*Struct*

The `<pointer-to-member-type>` production.

```text
<pointer-to-member-type> ::= M <class type> <member type>
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PointerToMemberType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> PointerToMemberType`



## cpp_demangle::ast::Prefix

*Enum*

The `<prefix>` production.

```text
<prefix> ::= <unqualified-name>
         ::= <prefix> <unqualified-name>
         ::= <template-prefix> <template-args>
         ::= <template-param>
         ::= <decltype>
         ::= <prefix> <data-member-prefix>
         ::= <substitution>

<template-prefix> ::= <template unqualified-name>
                  ::= <prefix> <template unqualified-name>
                  ::= <template-param>
                  ::= <substitution>
```

**Variants:**
- `Unqualified(UnqualifiedName)` - An unqualified name.
- `Nested(PrefixHandle, UnqualifiedName)` - Some nested name.
- `Template(PrefixHandle, TemplateArgs)` - A prefix and template arguments.
- `TemplateParam(TemplateParam)` - A template parameter.
- `Decltype(Decltype)` - A decltype.
- `DataMember(PrefixHandle, DataMemberPrefix)` - A prefix and data member.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Prefix) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Prefix`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::PrefixHandle

*Enum*

A reference to a parsed `<prefix>` production.

**Variants:**
- `WellKnown(WellKnownComponent)` - A reference to a "well-known" component.
- `BackReference(usize)` - A back-reference into the substitution table to a component we
- `NonSubstitution(NonSubstitution)` - A handle to some `<prefix>` component that isn't by itself

**Methods:**

- `fn back_reference(self: &Self) -> Option<usize>` - If this is a `BackReference`, get its index.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PrefixHandle`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PrefixHandle) -> bool`



## cpp_demangle::ast::QualifiedBuiltin

*Struct*

A built-in type with CV-qualifiers.

Like unqualified built-in types, CV-qualified built-in types do not go into
the substitutions table.

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &QualifiedBuiltin) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> QualifiedBuiltin`



## cpp_demangle::ast::RefQualifier

*Enum*

A <ref-qualifier> production.

```text
<ref-qualifier> ::= R   # & ref-qualifier
                ::= O   # && ref-qualifier
```

**Variants:**
- `LValueRef` - &
- `RValueRef` - &&

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RefQualifier) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RefQualifier`



## cpp_demangle::ast::ResourceName

*Struct*

The `<resource name>` pseudo-terminal.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ResourceName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ResourceName`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::SeqId

*Struct*

A <seq-id> production encoding a base-36 positive number.

```text
<seq-id> ::= <0-9A-Z>+
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SeqId) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SeqId`



## cpp_demangle::ast::SimpleId

*Struct*

The `<simple-id>` production.

```text
<simple-id> ::= <source-name> [ <template-args> ]
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SimpleId) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SimpleId`



## cpp_demangle::ast::SimpleOperatorName

*Enum*

The `<simple-operator-name>` production.

**Variants:**
- `New` - new
- `NewArray` - new[]
- `Delete` - delete
- `DeleteArray` - delete[]
- `UnaryPlus` - +
- `Neg` - -
- `AddressOf` - &
- `Deref` - *
- `BitNot` - ~
- `Add` - +
- `Sub` - -
- `Mul` - *
- `Div` - /
- `Rem` - %
- `BitAnd` - &
- `BitOr` - |
- `BitXor` - ^
- `Assign` - =
- `AddAssign` - +=
- `SubAssign` - -=
- `MulAssign` - *=
- `DivAssign` - /=
- `RemAssign` - %=
- `BitAndAssign` - &=
- `BitOrAssign` - |=
- `BitXorAssign` - ^=
- `Shl` - <<
- `Shr` - >>
- `ShlAssign` - <<=
- `ShrAssign` - >>=
- `Eq` - ==
- `Ne` - !=
- `Less` - <
- `Greater` - >
- `LessEq` - <=
- `GreaterEq` - >=
- `Not` - !
- `LogicalAnd` - &&
- `LogicalOr` - ||
- `PostInc` - ++
- `PostDec` - --
- `Comma` - ,
- `DerefMemberPtr` - ->*
- `DerefMember` - ->
- `Call` - ()
- `Index` - []
- `Question` - ?:
- `Spaceship` - <=>

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SimpleOperatorName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SimpleOperatorName`



## cpp_demangle::ast::SourceName

*Struct*

The `<source-name>` non-terminal.

```text
<source-name> ::= <positive length number> <identifier>
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SourceName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SourceName`



## cpp_demangle::ast::SpecialName

*Enum*

The `<special-name>` production.

The `<special-name>` production is spread in pieces through out the ABI
spec, and then there are a bunch of `g++` extensions that have become de
facto.

### 5.1.4.1 Virtual Tables and RTTI

```text
<special-name> ::= TV <type>    # virtual table
               ::= TT <type>    # VTT structure (construction vtable index)
               ::= TI <type>    # typeinfo structure
               ::= TS <type>    # typeinfo name (null-terminated byte string)
```

### 5.1.4.2 Virtual Override Thunks

```text
<special-name> ::= T <call-offset> <base encoding>
    # base is the nominal target function of thunk

<special-name> ::= Tc <call-offset> <call-offset> <base encoding>
    # base is the nominal target function of thunk
    # first call-offset is 'this' adjustment
    # second call-offset is result adjustment
```

### 5.1.4.4 Guard Variables

```text
<special-name> ::= GV <object name> # Guard variable for one-time initialization
    # No <type>
```

### 5.1.4.5 Lifetime-Extended Temporaries

```text
<special-name> ::= GR <object name> _             # First temporary
<special-name> ::= GR <object name> <seq-id> _    # Subsequent temporaries
```

### De Facto Standard Extensions

```text
<special-name> ::= TC <type> <number> _ <type>    # construction vtable
               ::= TF <type>                      # typinfo function
               ::= TH <name>                      # TLS initialization function
               ::= TW <name>                      # TLS wrapper function
               ::= Gr <resource name>             # Java Resource
               ::= GTt <encoding>                 # Transaction-Safe function
               ::= GTn <encoding>                 # Non-Transaction-Safe function
```

**Variants:**
- `VirtualTable(TypeHandle)` - A virtual table.
- `Vtt(TypeHandle)` - A VTT structure (construction vtable index).
- `Typeinfo(TypeHandle)` - A typeinfo structure.
- `TypeinfoName(TypeHandle)` - A typeinfo name (null-terminated byte string).
- `VirtualOverrideThunk(CallOffset, alloc::boxed::Box<Encoding>)` - A virtual override thunk.
- `VirtualOverrideThunkCovariant(CallOffset, CallOffset, alloc::boxed::Box<Encoding>)` - A virtual override thunk with a covariant return type.
- `Guard(Name)` - An initialization guard for some static storage.
- `GuardTemporary(Name, usize)` - A temporary used in the initialization of a static storage and promoted
- `ConstructionVtable(TypeHandle, usize, TypeHandle)` - A construction vtable structure.
- `TypeinfoFunction(TypeHandle)` - A typeinfo function.
- `TlsInit(Name)` - A TLS initialization function.
- `TlsWrapper(Name)` - A TLS wrapper function.
- `JavaResource(alloc::vec::Vec<ResourceName>)` - A Java Resource.
- `TransactionClone(alloc::boxed::Box<Encoding>)` - A function declared transaction-safe
- `NonTransactionClone(alloc::boxed::Box<Encoding>)` - A function declared non-transaction-safe

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SpecialName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SpecialName`



## cpp_demangle::ast::StandardBuiltinType

*Enum*

A one of the standard variants of the <builtin-type> production.

```text
<builtin-type> ::= v  # void
               ::= w  # wchar_t
               ::= b  # bool
               ::= c  # char
               ::= a  # signed char
               ::= h  # unsigned char
               ::= s  # short
               ::= t  # unsigned short
               ::= i  # int
               ::= j  # unsigned int
               ::= l  # long
               ::= m  # unsigned long
               ::= x  # long long, __int64
               ::= y  # unsigned long long, __int64
               ::= n  # __int128
               ::= o  # unsigned __int128
               ::= f  # float
               ::= d  # double
               ::= e  # long double, __float80
               ::= g  # __float128
               ::= z  # ellipsis
               ::= Dd # IEEE 754r decimal floating point (64 bits)
               ::= De # IEEE 754r decimal floating point (128 bits)
               ::= Df # IEEE 754r decimal floating point (32 bits)
               ::= Dh # IEEE 754r half-precision floating point (16 bits)
               ::= DF16b # C++23 std::bfloat16_t
               ::= Di # char32_t
               ::= Ds # char16_t
               ::= Du # char8_t
               ::= Da # auto
               ::= Dc # decltype(auto)
               ::= Dn # std::nullptr_t (i.e., decltype(nullptr))
               ::= [DS] DA  # N1169 fixed-point [_Sat] T _Accum
               ::= [DS] DR  # N1169 fixed-point [_Sat] T _Fract

 <fixed-point-size> ::= s # short
                    ::= t # unsigned short
                    ::= i # plain
                    ::= j # unsigned
                    ::= l # long
                    ::= m # unsigned long
```

**Variants:**
- `Void` - void
- `Wchar` - wchar_t
- `Bool` - bool
- `Char` - char
- `SignedChar` - signed char
- `UnsignedChar` - unsigned char
- `Short` - short
- `UnsignedShort` - unsigned short
- `Int` - int
- `UnsignedInt` - unsigned int
- `Long` - long
- `UnsignedLong` - unsigned long
- `LongLong` - long long
- `UnsignedLongLong` - unsigned long long
- `Int128` - __int128
- `Uint128` - unsigned __int128
- `Float` - float
- `Double` - double
- `LongDouble` - long double
- `Float128` - __float128
- `Ellipsis` - ...
- `DecimalFloat64` - decimal64
- `DecimalFloat128` - decimal128
- `DecimalFloat32` - decimal32
- `DecimalFloat16` - half
- `BFloat16` - std::bfloat16_t
- `Char32` - char32_t
- `Char16` - char16_t
- `Char8` - char8_t
- `Auto` - auto
- `Decltype` - decltype(auto)
- `Nullptr` - std::nullptr_t
- `AccumShort` - short _Accum
- `AccumUShort` - unsigned short _Accum
- `Accum` - _Accum
- `AccumUnsigned` - unsigned _Accum
- `AccumLong` - long _Accum
- `AccumULong` - unsigned long _Accum
- `FractShort` - short _Fract
- `FractUShort` - unsigned short _Fract
- `Fract` - _Fract
- `FractUnsigned` - unsigned _Fract
- `FractLong` - long _Fract
- `FractULong` - unsigned long _Fract
- `SatAccumShort` - _Sat short _Accum
- `SatAccumUShort` - _Sat unsigned short _Accum
- `SatAccum` - _Sat _Accum
- `SatAccumUnsigned` - _Sat unsigned _Accum
- `SatAccumLong` - _Sat long _Accum
- `SatAccumULong` - _Sat unsigned long _Accum
- `SatFractShort` - _Sat short _Fract
- `SatFractUShort` - _Sat unsigned short _Fract
- `SatFract` - _Sat _Fract
- `SatFractUnsigned` - _Sat unsigned _Fract
- `SatFractLong` - _Sat long _Fract
- `SatFractULong` - _Sat unsigned long _Fract

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &StandardBuiltinType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> StandardBuiltinType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::SubobjectExpr

*Struct*

The subobject expression production.

<expression> ::= so <referent type> <expr> [<offset number>] <union-selector>* [p] E
<union-selector> ::= _ [<number>]

Not yet in the spec: https://github.com/itanium-cxx-abi/cxx-abi/issues/47
But it has been shipping in clang for some time.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SubobjectExpr) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SubobjectExpr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::Substitution

*Enum*

The `<substitution>` form: a back-reference to some component we've already
parsed.

```text
<substitution> ::= S <seq-id> _
               ::= S_
               ::= St # ::std::
               ::= Sa # ::std::allocator
               ::= Sb # ::std::basic_string
               ::= Ss # ::std::basic_string < char,
                                              ::std::char_traits<char>,
                                              ::std::allocator<char> >
               ::= Si # ::std::basic_istream<char,  std::char_traits<char> >
               ::= So # ::std::basic_ostream<char,  std::char_traits<char> >
               ::= Sd # ::std::basic_iostream<char, std::char_traits<char> >
```

**Variants:**
- `BackReference(usize)` - A reference to an entity that already occurred, ie the `S_` and `S
- `WellKnown(WellKnownComponent)` - A well-known substitution component. These are the components that do

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Substitution) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Substitution`



## cpp_demangle::ast::TemplateArg

*Enum*

A <template-arg> production.

```text
<template-arg> ::= <type>                # type or template
               ::= X <expression> E      # expression
               ::= <expr-primary>        # simple expressions
               ::= J <template-arg>* E   # argument pack
```

**Variants:**
- `Type(TypeHandle)` - A type or template.
- `Expression(Expression)` - An expression.
- `SimpleExpression(ExprPrimary)` - A simple expression.
- `ArgPack(alloc::vec::Vec<TemplateArg>)` - An argument pack.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TemplateArg) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TemplateArg`



## cpp_demangle::ast::TemplateArgs

*Struct*

The `<template-args>` production.

```text
<template-args> ::= I <template-arg>+ E
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TemplateArgs`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TemplateArgs) -> bool`



## cpp_demangle::ast::TemplateParam

*Struct*

The `<template-param>` production.

```text
<template-param> ::= T_ # first template parameter
                 ::= T <parameter-2 non-negative number> _
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TemplateParam) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TemplateParam`



## cpp_demangle::ast::TemplateTemplateParam

*Struct*

The `<template-template-param>` production.

```text
<template-template-param> ::= <template-param>
                          ::= <substitution>
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TemplateTemplateParam) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TemplateTemplateParam`



## cpp_demangle::ast::TemplateTemplateParamHandle

*Enum*

A reference to a parsed `TemplateTemplateParam`.

**Variants:**
- `WellKnown(WellKnownComponent)` - A reference to a "well-known" component.
- `BackReference(usize)` - A back-reference into the substitution table to a component we

**Methods:**

- `fn back_reference(self: &Self) -> Option<usize>` - If this is a `BackReference`, get its index.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TemplateTemplateParamHandle) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TemplateTemplateParamHandle`



## cpp_demangle::ast::Type

*Enum*

The `<type>` production.

```text
<type> ::= <builtin-type>
       ::= <function-type>
       ::= <class-enum-type>
       ::= <array-type>
       ::= <vector-type>
       ::= <pointer-to-member-type>
       ::= <template-param>
       ::= <template-template-param> <template-args>
       ::= <decltype>
       ::= <CV-qualifiers> <type>
       ::= P <type>                                 # pointer-to
       ::= R <type>                                 # reference-to
       ::= O <type>                                 # rvalue reference-to (C++0x)
       ::= C <type>                                 # complex pair (C 2000)
       ::= G <type>                                 # imaginary (C 2000)
       ::= U <source-name> [<template-args>] <type> # vendor extended type qualifier
       ::= Dp <type>                                # pack expansion (C++0x)
       ::= <substitution>
```

**Variants:**
- `Function(FunctionType)` - A function type.
- `ClassEnum(ClassEnumType)` - A class, union, or enum type.
- `Array(ArrayType)` - An array type.
- `Vector(VectorType)` - A vector type.
- `PointerToMember(PointerToMemberType)` - A pointer-to-member type.
- `TemplateParam(TemplateParam)` - A named template parameter type.
- `TemplateTemplate(TemplateTemplateParamHandle, TemplateArgs)` - A template template type.
- `Decltype(Decltype)` - A decltype.
- `Qualified(CvQualifiers, TypeHandle)` - A const-, restrict-, and/or volatile-qualified type.
- `PointerTo(TypeHandle)` - A pointer to a type.
- `LvalueRef(TypeHandle)` - An lvalue reference to a type.
- `RvalueRef(TypeHandle)` - An rvalue reference to a type.
- `Complex(TypeHandle)` - A complex pair of the given type.
- `Imaginary(TypeHandle)` - An imaginary of the given type.
- `VendorExtension(SourceName, Option<TemplateArgs>, TypeHandle)` - A vendor extended type qualifier.
- `PackExpansion(TypeHandle)` - A pack expansion.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Type) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Type`



## cpp_demangle::ast::TypeHandle

*Enum*

A reference to a parsed `Type` production.

**Variants:**
- `WellKnown(WellKnownComponent)` - A reference to a "well-known" component.
- `BackReference(usize)` - A back-reference into the substitution table to a component we
- `Builtin(BuiltinType)` - A builtin type. These don't end up in the substitutions table.
- `QualifiedBuiltin(QualifiedBuiltin)` - A CV-qualified builtin type. These don't end up in the table either.

**Methods:**

- `fn back_reference(self: &Self) -> Option<usize>` - If this is a `BackReference`, get its index.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TypeHandle`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TypeHandle) -> bool`



## cpp_demangle::ast::UnnamedTypeName

*Struct*

The `<unnamed-type-name>` production.

```text
<unnamed-type-name> ::= Ut [ <nonnegative number> ] _
                    ::= <closure-type-name>
    ```

We handle `<closure-type-name>` separately.

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UnnamedTypeName`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnnamedTypeName) -> bool`



## cpp_demangle::ast::UnqualifiedName

*Enum*

The `<unqualified-name>` production.

```text
<unqualified-name> ::= [on] <operator-name> [<abi-tags>]
                   ::= <ctor-dtor-name> [<abi-tags>]
                   ::= <source-name> [<abi-tags>]
                   ::= <local-source-name> [<abi-tags>]
                   ::= <unnamed-type-name> [<abi-tags>]
                   ::= <closure-type-name> [<abi-tags>]

# I think this is from an older version of the standard. It isn't in the
# current version, but all the other demanglers support it, so we will too.
<local-source-name> ::= L <source-name> [<discriminator>]
```

**Variants:**
- `Operator(OperatorName, AbiTags)` - An operator name.
- `CtorDtor(CtorDtorName, AbiTags)` - A constructor or destructor name.
- `Source(SourceName, AbiTags)` - A source name.
- `LocalSourceName(SourceName, Option<Discriminator>, AbiTags)` - A local source name.
- `UnnamedType(UnnamedTypeName, AbiTags)` - A generated name for an unnamed type.
- `ClosureType(ClosureTypeName, AbiTags)` - A closure type name

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnqualifiedName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnqualifiedName`



## cpp_demangle::ast::UnresolvedName

*Enum*

The `<unresolved-name>` production.

```text
<unresolved-name> ::= [gs] <base-unresolved-name>
                         #
                  ::= sr <unresolved-type> <base-unresolved-name>
                         #
                  ::= srN <unresolved-type> <unresolved-qualifier-level>+ E <base-unresolved-name>
                         #
                  ::= [gs] sr <unresolved-qualifier-level>+ E <base-unresolved-name>
                         # A::x, N::y, A<T>::z; "gs" means leading "::"
```

**Variants:**
- `Name(BaseUnresolvedName)` - `x`
- `Global(BaseUnresolvedName)` - `::x`
- `Nested1(UnresolvedTypeHandle, alloc::vec::Vec<UnresolvedQualifierLevel>, BaseUnresolvedName)` - `T::x`  or `decltype(p)::x` or `T::N::x` or `decltype(p)::N::x`
- `Nested2(alloc::vec::Vec<UnresolvedQualifierLevel>, BaseUnresolvedName)` - `A::x` or `N::y` or `A<T>::z`
- `GlobalNested2(alloc::vec::Vec<UnresolvedQualifierLevel>, BaseUnresolvedName)` - `::A::x` or `::N::y` or `::A<T>::z`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &UnresolvedName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnresolvedName`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpp_demangle::ast::UnresolvedQualifierLevel

*Struct*

The `<unresolved-qualifier-level>` production.

```text
<unresolved-qualifier-level> ::= <simple-id>
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnresolvedQualifierLevel) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnresolvedQualifierLevel`



## cpp_demangle::ast::UnresolvedType

*Enum*

The `<unresolved-type>` production.

```text
<unresolved-type> ::= <template-param> [ <template-args> ]  # T:: or T<X,Y>::
                  ::= <decltype>                            # decltype(p)::
                  ::= <substitution>
```

**Variants:**
- `Template(TemplateParam, Option<TemplateArgs>)` - An unresolved template type.
- `Decltype(Decltype)` - An unresolved `decltype`.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnresolvedType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnresolvedType`



## cpp_demangle::ast::UnresolvedTypeHandle

*Enum*

A reference to a parsed `<unresolved-type>` production.

**Variants:**
- `WellKnown(WellKnownComponent)` - A reference to a "well-known" component.
- `BackReference(usize)` - A back-reference into the substitution table to a component we

**Methods:**

- `fn back_reference(self: &Self) -> Option<usize>` - If this is a `BackReference`, get its index.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnresolvedTypeHandle) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnresolvedTypeHandle`



## cpp_demangle::ast::UnscopedName

*Enum*

The `<unscoped-name>` production.

```text
<unscoped-name> ::= <unqualified-name>
                ::= St <unqualified-name>   # ::std::
```

**Variants:**
- `Unqualified(UnqualifiedName)` - An unqualified name.
- `Std(UnqualifiedName)` - A name within the `std::` namespace.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnscopedName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnscopedName`



## cpp_demangle::ast::UnscopedTemplateName

*Struct*

The `<unscoped-template-name>` production.

```text
<unscoped-template-name> ::= <unscoped-name>
                         ::= <substitution>
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnscopedTemplateName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnscopedTemplateName`



## cpp_demangle::ast::UnscopedTemplateNameHandle

*Enum*

A handle to an `UnscopedTemplateName`.

**Variants:**
- `WellKnown(WellKnownComponent)` - A reference to a "well-known" component.
- `BackReference(usize)` - A back-reference into the substitution table to a component we
- `NonSubstitution(NonSubstitution)` - A handle to some `<unscoped-name>` component that isn't by itself

**Methods:**

- `fn back_reference(self: &Self) -> Option<usize>` - If this is a `BackReference`, get its index.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnscopedTemplateNameHandle) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnscopedTemplateNameHandle`



## cpp_demangle::ast::VOffset

*Struct*

A virtual offset, as described by the <v-offset> production.

```text
<v-offset> ::= <offset number> _ <virtual offset number>
```

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VOffset) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> VOffset`



## cpp_demangle::ast::VectorType

*Enum*

The `<vector-type>` production.

```text
<vector-type> ::= Dv <number> _ <type>
              ::= Dv <expression> _ <type>
```

**Variants:**
- `DimensionNumber(usize, TypeHandle)` - An vector with a number-literal dimension.
- `DimensionExpression(Expression, TypeHandle)` - An vector with an expression for its dimension.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VectorType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VectorType) -> bool`



## cpp_demangle::ast::WellKnownComponent

*Enum*

The `<substitution>` variants that are encoded directly in the grammar,
rather than as back references to other components in the substitution
table.

**Variants:**
- `Std` - std
- `StdAllocator` - std::allocator
- `StdString1` - std::basic_string
- `StdString2` - std::string
- `StdIstream` - std::basic_istream<char, std::char_traits<char> >
- `StdOstream` - std::ostream
- `StdIostream` - std::basic_iostream<char, std::char_traits<char> >

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &WellKnownComponent) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> WellKnownComponent`



