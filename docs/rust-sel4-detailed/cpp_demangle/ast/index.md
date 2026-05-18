*[cpp_demangle](../index.md) / [ast](index.md)*

---

# Module `ast`

Abstract syntax tree types for mangled symbols.

## Contents

- [Structs](#structs)
  - [`AutoLogParse`](#autologparse)
  - [`AutoLogDemangle`](#autologdemangle)
  - [`ParseContextState`](#parsecontextstate)
  - [`ParseContext`](#parsecontext)
  - [`AutoParseRecursion`](#autoparserecursion)
  - [`ArgScopeStack`](#argscopestack)
  - [`DemangleState`](#demanglestate)
  - [`AutoParseDemangle`](#autoparsedemangle)
  - [`FunctionArgList`](#functionarglist)
  - [`FunctionArgListAndReturnType`](#functionarglistandreturntype)
  - [`FunctionArgSlice`](#functionargslice)
  - [`NonSubstitution`](#nonsubstitution)
  - [`CloneSuffix`](#clonesuffix)
  - [`UnscopedTemplateName`](#unscopedtemplatename)
  - [`SourceName`](#sourcename)
  - [`AbiTags`](#abitags)
  - [`AbiTag`](#abitag)
  - [`Identifier`](#identifier)
  - [`CloneTypeIdentifier`](#clonetypeidentifier)
  - [`SeqId`](#seqid)
  - [`NvOffset`](#nvoffset)
  - [`VOffset`](#voffset)
  - [`CvQualifiers`](#cvqualifiers)
  - [`QualifiedBuiltin`](#qualifiedbuiltin)
  - [`FunctionType`](#functiontype)
  - [`BareFunctionType`](#barefunctiontype)
  - [`UnnamedTypeName`](#unnamedtypename)
  - [`PointerToMemberType`](#pointertomembertype)
  - [`TemplateParam`](#templateparam)
  - [`TemplateTemplateParam`](#templatetemplateparam)
  - [`FunctionParam`](#functionparam)
  - [`TemplateArgs`](#templateargs)
  - [`MemberName`](#membername)
  - [`UnresolvedQualifierLevel`](#unresolvedqualifierlevel)
  - [`SimpleId`](#simpleid)
  - [`Initializer`](#initializer)
  - [`Discriminator`](#discriminator)
  - [`ClosureTypeName`](#closuretypename)
  - [`LambdaSig`](#lambdasig)
  - [`DataMemberPrefix`](#datamemberprefix)
  - [`ResourceName`](#resourcename)
  - [`SubobjectExpr`](#subobjectexpr)
- [Enums](#enums)
  - [`LeafName`](#leafname)
  - [`MangledName`](#mangledname)
  - [`Encoding`](#encoding)
  - [`GlobalCtorDtor`](#globalctordtor)
  - [`Name`](#name)
  - [`UnscopedName`](#unscopedname)
  - [`UnscopedTemplateNameHandle`](#unscopedtemplatenamehandle)
  - [`NestedName`](#nestedname)
  - [`Prefix`](#prefix)
  - [`PrefixHandle`](#prefixhandle)
  - [`UnqualifiedName`](#unqualifiedname)
  - [`OperatorName`](#operatorname)
  - [`SimpleOperatorName`](#simpleoperatorname)
  - [`CallOffset`](#calloffset)
  - [`CtorDtorName`](#ctordtorname)
  - [`Type`](#type)
  - [`TypeHandle`](#typehandle)
  - [`RefQualifier`](#refqualifier)
  - [`ExplicitObjectParameter`](#explicitobjectparameter)
  - [`StandardBuiltinType`](#standardbuiltintype)
  - [`ParametricBuiltinType`](#parametricbuiltintype)
  - [`BuiltinType`](#builtintype)
  - [`ExceptionSpec`](#exceptionspec)
  - [`Decltype`](#decltype)
  - [`ClassEnumType`](#classenumtype)
  - [`ArrayType`](#arraytype)
  - [`VectorType`](#vectortype)
  - [`TemplateTemplateParamHandle`](#templatetemplateparamhandle)
  - [`TemplateArg`](#templatearg)
  - [`Expression`](#expression)
  - [`UnresolvedName`](#unresolvedname)
  - [`UnresolvedType`](#unresolvedtype)
  - [`UnresolvedTypeHandle`](#unresolvedtypehandle)
  - [`BaseUnresolvedName`](#baseunresolvedname)
  - [`DestructorName`](#destructorname)
  - [`ExprPrimary`](#exprprimary)
  - [`LocalName`](#localname)
  - [`Substitution`](#substitution)
  - [`WellKnownComponent`](#wellknowncomponent)
  - [`SpecialName`](#specialname)
  - [`FoldExpr`](#foldexpr)
- [Traits](#traits)
  - [`GetTemplateArgs`](#gettemplateargs)
  - [`GetLeafName`](#getleafname)
  - [`IsCtorDtorConversion`](#isctordtorconversion)
  - [`ArgScope`](#argscope)
  - [`ArgScopeStackExt`](#argscopestackext)
  - [`DemangleAsLeaf`](#demangleasleaf)
- [Functions](#functions)
  - [`consume`](#consume)
  - [`one_or_more`](#one-or-more)
  - [`zero_or_more`](#zero-or-more)
  - [`parse_number`](#parse-number)
- [Type Aliases](#type-aliases)
  - [`Number`](#number)
- [Macros](#macros)
  - [`try_recurse!`](#try-recurse)
  - [`try_begin_parse!`](#try-begin-parse)
  - [`try_begin_demangle!`](#try-begin-demangle)
  - [`try_begin_demangle_as_inner!`](#try-begin-demangle-as-inner)
  - [`inner_barrier!`](#inner-barrier)
  - [`reference_newtype!`](#reference-newtype)
  - [`define_handle!`](#define-handle)
  - [`define_vocabulary!`](#define-vocabulary)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AutoLogParse`](#autologparse) | struct |  |
| [`AutoLogDemangle`](#autologdemangle) | struct |  |
| [`ParseContextState`](#parsecontextstate) | struct |  |
| [`ParseContext`](#parsecontext) | struct | Common context needed when parsing. |
| [`AutoParseRecursion`](#autoparserecursion) | struct | An RAII type to automatically check the recursion level against the maximum. |
| [`ArgScopeStack`](#argscopestack) | struct | An `ArgScopeStack` represents the current function and template demangling scope we are within. |
| [`DemangleState`](#demanglestate) | struct |  |
| [`AutoParseDemangle`](#autoparsedemangle) | struct | An RAII type to automatically check the recursion level against the maximum. |
| [`FunctionArgList`](#functionarglist) | struct |  |
| [`FunctionArgListAndReturnType`](#functionarglistandreturntype) | struct |  |
| [`FunctionArgSlice`](#functionargslice) | struct |  |
| [`NonSubstitution`](#nonsubstitution) | struct | A handle to a component that is usually substitutable, and lives in the substitutions table, but in this particular case does not qualify for substitutions. |
| [`CloneSuffix`](#clonesuffix) | struct | <clone-suffix> ::= [ . |
| [`UnscopedTemplateName`](#unscopedtemplatename) | struct | The `<unscoped-template-name>` production. |
| [`SourceName`](#sourcename) | struct | The `<source-name>` non-terminal. |
| [`AbiTags`](#abitags) | struct | The `<abi-tags>` non-terminal. |
| [`AbiTag`](#abitag) | struct | The `<abi-tag>` non-terminal. |
| [`Identifier`](#identifier) | struct | The `<identifier>` pseudo-terminal. |
| [`CloneTypeIdentifier`](#clonetypeidentifier) | struct | The `<clone-type-identifier>` pseudo-terminal. |
| [`SeqId`](#seqid) | struct | A <seq-id> production encoding a base-36 positive number. |
| [`NvOffset`](#nvoffset) | struct | A non-virtual offset, as described by the <nv-offset> production. |
| [`VOffset`](#voffset) | struct | A virtual offset, as described by the <v-offset> production. |
| [`CvQualifiers`](#cvqualifiers) | struct | The `<CV-qualifiers>` production. |
| [`QualifiedBuiltin`](#qualifiedbuiltin) | struct | A built-in type with CV-qualifiers. |
| [`FunctionType`](#functiontype) | struct | The `<function-type>` production. |
| [`BareFunctionType`](#barefunctiontype) | struct | The `<bare-function-type>` production. |
| [`UnnamedTypeName`](#unnamedtypename) | struct | The `<unnamed-type-name>` production. |
| [`PointerToMemberType`](#pointertomembertype) | struct | The `<pointer-to-member-type>` production. |
| [`TemplateParam`](#templateparam) | struct | The `<template-param>` production. |
| [`TemplateTemplateParam`](#templatetemplateparam) | struct | The `<template-template-param>` production. |
| [`FunctionParam`](#functionparam) | struct | The <function-param> production. |
| [`TemplateArgs`](#templateargs) | struct | The `<template-args>` production. |
| [`MemberName`](#membername) | struct | In libiberty, Member and DerefMember expressions have special handling. |
| [`UnresolvedQualifierLevel`](#unresolvedqualifierlevel) | struct | The `<unresolved-qualifier-level>` production. |
| [`SimpleId`](#simpleid) | struct | The `<simple-id>` production. |
| [`Initializer`](#initializer) | struct | The `<initializer>` production. |
| [`Discriminator`](#discriminator) | struct | The `<discriminator>` production. |
| [`ClosureTypeName`](#closuretypename) | struct | The `<closure-type-name>` production. |
| [`LambdaSig`](#lambdasig) | struct | The `<lambda-sig>` production. |
| [`DataMemberPrefix`](#datamemberprefix) | struct | The `<data-member-prefix>` production. |
| [`ResourceName`](#resourcename) | struct | The `<resource name>` pseudo-terminal. |
| [`SubobjectExpr`](#subobjectexpr) | struct | The subobject expression production. |
| [`LeafName`](#leafname) | enum | A leaf name is the part the name that describes some type or class without any leading namespace qualifiers. |
| [`MangledName`](#mangledname) | enum | The root AST node, and starting production. |
| [`Encoding`](#encoding) | enum | The `<encoding>` production. |
| [`GlobalCtorDtor`](#globalctordtor) | enum | A global constructor or destructor. |
| [`Name`](#name) | enum | The `<name>` production. |
| [`UnscopedName`](#unscopedname) | enum | The `<unscoped-name>` production. |
| [`UnscopedTemplateNameHandle`](#unscopedtemplatenamehandle) | enum | A handle to an `UnscopedTemplateName`. |
| [`NestedName`](#nestedname) | enum | The `<nested-name>` production. |
| [`Prefix`](#prefix) | enum | The `<prefix>` production. |
| [`PrefixHandle`](#prefixhandle) | enum | A reference to a parsed `<prefix>` production. |
| [`UnqualifiedName`](#unqualifiedname) | enum | The `<unqualified-name>` production. |
| [`OperatorName`](#operatorname) | enum | The `<operator-name>` production. |
| [`SimpleOperatorName`](#simpleoperatorname) | enum | The `<simple-operator-name>` production. |
| [`CallOffset`](#calloffset) | enum | The `<call-offset>` production. |
| [`CtorDtorName`](#ctordtorname) | enum | The `<ctor-dtor-name>` production. |
| [`Type`](#type) | enum | The `<type>` production. |
| [`TypeHandle`](#typehandle) | enum | A reference to a parsed `Type` production. |
| [`RefQualifier`](#refqualifier) | enum | A <ref-qualifier> production. |
| [`ExplicitObjectParameter`](#explicitobjectparameter) | enum | A named explicit object parameter. |
| [`StandardBuiltinType`](#standardbuiltintype) | enum | A one of the standard variants of the <builtin-type> production. |
| [`ParametricBuiltinType`](#parametricbuiltintype) | enum | <builtin-type> ::= DF <number> _ # ISO/IEC TS 18661 binary floating point type _FloatN (N bits), C++23 std::floatN_t ::= DF <number> x # IEEE extended precision formats, C23 _FloatNx (N bits) ::= DB <number> _        # C23 signed _BitInt(N) ::= DB <instantiation-dependent expression> _ # C23 signed _BitInt(N) ::= DU <number> _        # C23 unsigned _BitInt(N) ::= DU <instantiation-dependent expression> _ # C23 unsigned _BitInt(N) |
| [`BuiltinType`](#builtintype) | enum | The `<builtin-type>` production. |
| [`ExceptionSpec`](#exceptionspec) | enum | The `<exception-spec>` production. |
| [`Decltype`](#decltype) | enum | The `<decltype>` production. |
| [`ClassEnumType`](#classenumtype) | enum | The `<class-enum-type>` production. |
| [`ArrayType`](#arraytype) | enum | The `<array-type>` production. |
| [`VectorType`](#vectortype) | enum | The `<vector-type>` production. |
| [`TemplateTemplateParamHandle`](#templatetemplateparamhandle) | enum | A reference to a parsed `TemplateTemplateParam`. |
| [`TemplateArg`](#templatearg) | enum | A <template-arg> production. |
| [`Expression`](#expression) | enum | The `<expression>` production. |
| [`UnresolvedName`](#unresolvedname) | enum | The `<unresolved-name>` production. |
| [`UnresolvedType`](#unresolvedtype) | enum | The `<unresolved-type>` production. |
| [`UnresolvedTypeHandle`](#unresolvedtypehandle) | enum | A reference to a parsed `<unresolved-type>` production. |
| [`BaseUnresolvedName`](#baseunresolvedname) | enum | The `<base-unresolved-name>` production. |
| [`DestructorName`](#destructorname) | enum | The `<destructor-name>` production. |
| [`ExprPrimary`](#exprprimary) | enum | The `<expr-primary>` production. |
| [`LocalName`](#localname) | enum | The `<local-name>` production. |
| [`Substitution`](#substitution) | enum | The `<substitution>` form: a back-reference to some component we've already parsed. |
| [`WellKnownComponent`](#wellknowncomponent) | enum | The `<substitution>` variants that are encoded directly in the grammar, rather than as back references to other components in the substitution table. |
| [`SpecialName`](#specialname) | enum | The `<special-name>` production. |
| [`FoldExpr`](#foldexpr) | enum | The fold expressions. |
| [`GetTemplateArgs`](#gettemplateargs) | trait | Determine whether this AST node is an instantiated[*] template function, and get its concrete template arguments. |
| [`GetLeafName`](#getleafname) | trait | Determine whether this AST node is some kind (potentially namespaced) name and if so get its leaf name. |
| [`IsCtorDtorConversion`](#isctordtorconversion) | trait | Determine whether this AST node is a constructor, destructor, or conversion function. |
| [`ArgScope`](#argscope) | trait | When formatting a mangled symbol's parsed AST as a demangled symbol, we need to resolve indirect references to template and function arguments with direct `TemplateArg` and `Type` references respectively. |
| [`ArgScopeStackExt`](#argscopestackext) | trait | When we first begin demangling, we haven't entered any function or template demangling scope and we don't have any useful `ArgScopeStack`. |
| [`DemangleAsLeaf`](#demangleasleaf) | trait | Demangle this thing in the leaf name position. |
| [`consume`](#consume) | fn | Expect and consume the given byte str, and return the advanced `IndexStr` if we saw the expectation. |
| [`one_or_more`](#one-or-more) | fn |  |
| [`zero_or_more`](#zero-or-more) | fn |  |
| [`parse_number`](#parse-number) | fn | Parse a number with the given `base`. |
| [`Number`](#number) | type | The `<number>` production. |
| [`try_recurse!`](#try-recurse) | macro |  |
| [`try_begin_parse!`](#try-begin-parse) | macro | Performs the two operations that begin every parse |
| [`try_begin_demangle!`](#try-begin-demangle) | macro | Automatically log start and end demangling in an s-expression format, when the `logging` feature is enabled. |
| [`try_begin_demangle_as_inner!`](#try-begin-demangle-as-inner) | macro | Automatically log start and end demangling in an s-expression format, when the `logging` feature is enabled. |
| [`inner_barrier!`](#inner-barrier) | macro | The inner stack allows passing AST nodes down deeper into the tree so that nodes that logically precede something (e.g. PointerRef) can show up after that thing in the demangled output. |
| [`reference_newtype!`](#reference-newtype) | macro |  |
| [`define_handle!`](#define-handle) | macro | Define a handle to a AST type that lives inside the substitution table. |
| [`define_vocabulary!`](#define-vocabulary) | macro | Define a "vocabulary" nonterminal, something like `OperatorName` or `CtorDtorName` that's basically a big list of constant strings. |

## Structs

### `AutoLogParse`

```rust
struct AutoLogParse;
```

#### Implementations

- <span id="autologparse-new"></span>`fn new(_: &'static str, _: IndexStr<'_>) -> AutoLogParse` — [`IndexStr`](../index_str/index.md#indexstr), [`AutoLogParse`](#autologparse)

### `AutoLogDemangle`

```rust
struct AutoLogDemangle;
```

#### Implementations

- <span id="autologdemangle-new"></span>`fn new<P, W>(_: &P, _: &DemangleContext<'_, W>, _: Option<ArgScopeStack<'_, '_>>, _: bool) -> AutoLogDemangle` — [`ArgScopeStack`](#argscopestack), [`AutoLogDemangle`](#autologdemangle)

### `ParseContextState`

```rust
struct ParseContextState {
    recursion_level: u32,
    in_conversion: bool,
}
```

#### Trait Implementations

##### `impl Clone for ParseContextState`

- <span id="parsecontextstate-clone"></span>`fn clone(&self) -> ParseContextState` — [`ParseContextState`](#parsecontextstate)

##### `impl Copy for ParseContextState`

##### `impl Debug for ParseContextState`

- <span id="parsecontextstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ParseContextState`

- <span id="parsecontextstate-default"></span>`fn default() -> ParseContextState` — [`ParseContextState`](#parsecontextstate)

### `ParseContext`

```rust
struct ParseContext {
    max_recursion: u32,
    state: core::cell::Cell<ParseContextState>,
}
```

Common context needed when parsing.

#### Implementations

- <span id="parsecontext-new"></span>`fn new(options: ParseOptions) -> ParseContext` — [`ParseOptions`](../index.md#parseoptions), [`ParseContext`](#parsecontext)

  Construct a new `ParseContext`.

- <span id="parsecontext-recursion-level"></span>`fn recursion_level(&self) -> u32`

  Get the current recursion level for this context.

- <span id="parsecontext-enter-recursion"></span>`fn enter_recursion(&self) -> error::Result<()>` — [`Result`](../error/index.md#result)

- <span id="parsecontext-exit-recursion"></span>`fn exit_recursion(&self)`

- <span id="parsecontext-in-conversion"></span>`fn in_conversion(&self) -> bool`

- <span id="parsecontext-set-in-conversion"></span>`fn set_in_conversion(&self, in_conversion: bool) -> bool`

#### Trait Implementations

##### `impl Clone for ParseContext`

- <span id="parsecontext-clone"></span>`fn clone(&self) -> ParseContext` — [`ParseContext`](#parsecontext)

##### `impl Debug for ParseContext`

- <span id="parsecontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AutoParseRecursion<'a>`

```rust
struct AutoParseRecursion<'a>(&'a ParseContext);
```

An RAII type to automatically check the recursion level against the
maximum. If the maximum has been crossed, return an error. Otherwise,
increment the level upon construction, and decrement it upon destruction.

#### Implementations

- <span id="autoparserecursion-new"></span>`fn new(ctx: &'a ParseContext) -> error::Result<AutoParseRecursion<'a>>` — [`ParseContext`](#parsecontext), [`Result`](../error/index.md#result), [`AutoParseRecursion`](#autoparserecursion)

#### Trait Implementations

##### `impl Drop for AutoParseRecursion<'a>`

- <span id="autoparserecursion-drop"></span>`fn drop(&mut self)`

### `ArgScopeStack<'prev, 'subs>`

```rust
struct ArgScopeStack<'prev, 'subs>
where
    'subs: 'prev {
    item: &'subs dyn ArgScope<'subs, 'subs>,
    in_arg: Option<(usize, &'subs TemplateArgs)>,
    prev: Option<&'prev ArgScopeStack<'prev, 'subs>>,
}
```

An `ArgScopeStack` represents the current function and template demangling
scope we are within. As we enter new demangling scopes, we construct new
`ArgScopeStack`s whose `prev` references point back to the old ones. These
`ArgScopeStack`s are kept on the native stack, and as functions return, they
go out of scope and we use the previous `ArgScopeStack`s again.

#### Trait Implementations

##### `impl Clone for ArgScopeStack<'prev, 'subs>`

- <span id="argscopestack-clone"></span>`fn clone(&self) -> ArgScopeStack<'prev, 'subs>` — [`ArgScopeStack`](#argscopestack)

##### `impl Copy for ArgScopeStack<'prev, 'subs>`

##### `impl Debug for ArgScopeStack<'prev, 'subs>`

- <span id="argscopestack-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DemangleState`

```rust
struct DemangleState {
    pub recursion_level: u32,
}
```

#### Fields

- **`recursion_level`**: `u32`

  How deep in the demangling are we?

#### Trait Implementations

##### `impl Clone for DemangleState`

- <span id="demanglestate-clone"></span>`fn clone(&self) -> DemangleState` — [`DemangleState`](#demanglestate)

##### `impl Copy for DemangleState`

##### `impl Debug for DemangleState`

- <span id="demanglestate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AutoParseDemangle<'a, 'b, W: 'a + DemangleWrite>`

```rust
struct AutoParseDemangle<'a, 'b, W: 'a + DemangleWrite>(&'b mut DemangleContext<'a, W>);
```

An RAII type to automatically check the recursion level against the
maximum. If the maximum has been crossed, return an error. Otherwise,
increment the level upon construction, and decrement it upon destruction.

#### Implementations

- <span id="autoparsedemangle-new"></span>`fn new(ctx: &'b mut DemangleContext<'a, W>) -> core::result::Result<Self, fmt::Error>`

#### Trait Implementations

##### `impl<W: 'a + DemangleWrite> Deref for AutoParseDemangle<'a, 'b, W>`

- <span id="autoparsedemangle-deref-type-target"></span>`type Target = DemangleContext<'a, W>`

- <span id="autoparsedemangle-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<W: 'a + DemangleWrite> DerefMut for AutoParseDemangle<'a, 'b, W>`

- <span id="autoparsedemangle-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<W: 'a + DemangleWrite> Drop for AutoParseDemangle<'a, 'b, W>`

- <span id="autoparsedemangle-drop"></span>`fn drop(&mut self)`

##### `impl Receiver for AutoParseDemangle<'a, 'b, W>`

- <span id="autoparsedemangle-receiver-type-target"></span>`type Target = T`

### `FunctionArgList`

```rust
struct FunctionArgList(alloc::vec::Vec<TypeHandle>);
```

#### Implementations

- <span id="functionarglist-new"></span>`fn new(types: &Vec<TypeHandle>) -> &FunctionArgList` — [`TypeHandle`](#typehandle), [`FunctionArgList`](#functionarglist)

#### Trait Implementations

##### `impl Debug for FunctionArgList`

- <span id="functionarglist-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for FunctionArgList`

- <span id="functionarglist-deref-type-target"></span>`type Target = Vec<TypeHandle>`

- <span id="functionarglist-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Drop for FunctionArgList`

- <span id="functionarglist-drop"></span>`fn drop(&mut self)`

##### `impl Receiver for FunctionArgList`

- <span id="functionarglist-receiver-type-target"></span>`type Target = T`

### `FunctionArgListAndReturnType`

```rust
struct FunctionArgListAndReturnType(alloc::vec::Vec<TypeHandle>);
```

#### Implementations

- <span id="functionarglistandreturntype-new"></span>`fn new(types: &Vec<TypeHandle>) -> &FunctionArgListAndReturnType` — [`TypeHandle`](#typehandle), [`FunctionArgListAndReturnType`](#functionarglistandreturntype)

#### Trait Implementations

##### `impl Debug for FunctionArgListAndReturnType`

- <span id="functionarglistandreturntype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for FunctionArgListAndReturnType`

- <span id="functionarglistandreturntype-deref-type-target"></span>`type Target = Vec<TypeHandle>`

- <span id="functionarglistandreturntype-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Drop for FunctionArgListAndReturnType`

- <span id="functionarglistandreturntype-drop"></span>`fn drop(&mut self)`

##### `impl Receiver for FunctionArgListAndReturnType`

- <span id="functionarglistandreturntype-receiver-type-target"></span>`type Target = T`

### `FunctionArgSlice`

```rust
struct FunctionArgSlice([TypeHandle]);
```

#### Implementations

- <span id="functionargslice-new"></span>`fn new(types: &[TypeHandle]) -> &FunctionArgSlice` — [`TypeHandle`](#typehandle), [`FunctionArgSlice`](#functionargslice)

#### Trait Implementations

##### `impl Debug for FunctionArgSlice`

- <span id="functionargslice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for FunctionArgSlice`

- <span id="functionargslice-deref-type-target"></span>`type Target = [TypeHandle]`

- <span id="functionargslice-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Drop for FunctionArgSlice`

- <span id="functionargslice-drop"></span>`fn drop(&mut self)`

##### `impl Receiver for FunctionArgSlice`

- <span id="functionargslice-receiver-type-target"></span>`type Target = T`

### `NonSubstitution`

```rust
struct NonSubstitution(usize);
```

A handle to a component that is usually substitutable, and lives in the
substitutions table, but in this particular case does not qualify for
substitutions.

#### Trait Implementations

##### `impl Clone for NonSubstitution`

- <span id="nonsubstitution-clone"></span>`fn clone(&self) -> NonSubstitution` — [`NonSubstitution`](#nonsubstitution)

##### `impl Debug for NonSubstitution`

- <span id="nonsubstitution-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NonSubstitution`

##### `impl GetLeafName for NonSubstitution`

- <span id="nonsubstitution-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for NonSubstitution`

- <span id="nonsubstitution-partialeq-eq"></span>`fn eq(&self, other: &NonSubstitution) -> bool` — [`NonSubstitution`](#nonsubstitution)

##### `impl StructuralPartialEq for NonSubstitution`

### `CloneSuffix`

```rust
struct CloneSuffix(CloneTypeIdentifier, alloc::vec::Vec<isize>);
```

<clone-suffix> ::= [ . <clone-type-identifier> ] [ . <nonnegative number> ]*

#### Trait Implementations

##### `impl Clone for CloneSuffix`

- <span id="clonesuffix-clone"></span>`fn clone(&self) -> CloneSuffix` — [`CloneSuffix`](#clonesuffix)

##### `impl Debug for CloneSuffix`

- <span id="clonesuffix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CloneSuffix`

##### `impl PartialEq for CloneSuffix`

- <span id="clonesuffix-partialeq-eq"></span>`fn eq(&self, other: &CloneSuffix) -> bool` — [`CloneSuffix`](#clonesuffix)

##### `impl StructuralPartialEq for CloneSuffix`

### `UnscopedTemplateName`

```rust
struct UnscopedTemplateName(UnscopedName);
```

The `<unscoped-template-name>` production.

```text
<unscoped-template-name> ::= <unscoped-name>
                         ::= <substitution>
```

#### Trait Implementations

##### `impl Clone for UnscopedTemplateName`

- <span id="unscopedtemplatename-clone"></span>`fn clone(&self) -> UnscopedTemplateName` — [`UnscopedTemplateName`](#unscopedtemplatename)

##### `impl Debug for UnscopedTemplateName`

- <span id="unscopedtemplatename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnscopedTemplateName`

##### `impl GetLeafName for UnscopedTemplateName`

- <span id="unscopedtemplatename-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for UnscopedTemplateName`

- <span id="unscopedtemplatename-partialeq-eq"></span>`fn eq(&self, other: &UnscopedTemplateName) -> bool` — [`UnscopedTemplateName`](#unscopedtemplatename)

##### `impl StructuralPartialEq for UnscopedTemplateName`

### `SourceName`

```rust
struct SourceName(Identifier);
```

The `<source-name>` non-terminal.

```text
<source-name> ::= <positive length number> <identifier>
```

#### Implementations

- <span id="sourcename-starts-with"></span>`fn starts_with(byte: u8) -> bool`

#### Trait Implementations

##### `impl ArgScope for SourceName`

- <span id="sourcename-argscope-leaf-name"></span>`fn leaf_name(self: &'subs Self) -> Result<LeafName<'subs>>` — [`Result`](../error/index.md#result), [`LeafName`](#leafname)

- <span id="sourcename-argscope-get-template-arg"></span>`fn get_template_arg(self: &'subs Self, _: usize) -> Result<(&'subs TemplateArg, &'subs TemplateArgs)>` — [`Result`](../error/index.md#result), [`TemplateArg`](#templatearg), [`TemplateArgs`](#templateargs)

- <span id="sourcename-argscope-get-function-arg"></span>`fn get_function_arg(self: &'subs Self, _: usize) -> Result<&'subs Type>` — [`Result`](../error/index.md#result), [`Type`](#type)

##### `impl Clone for SourceName`

- <span id="sourcename-clone"></span>`fn clone(&self) -> SourceName` — [`SourceName`](#sourcename)

##### `impl Debug for SourceName`

- <span id="sourcename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SourceName`

##### `impl PartialEq for SourceName`

- <span id="sourcename-partialeq-eq"></span>`fn eq(&self, other: &SourceName) -> bool` — [`SourceName`](#sourcename)

##### `impl StructuralPartialEq for SourceName`

### `AbiTags`

```rust
struct AbiTags(alloc::vec::Vec<AbiTag>);
```

The `<abi-tags>` non-terminal.

```text
<abi-tags> ::= <abi-tag> [<abi-tags>]
```

To make things easier on ourselves, despite the fact that the `<abi-tags>`
production requires at least one tag, we'll allow a zero-length vector
here instead of having to use Option<AbiTags> in everything that accepts
an AbiTags.

#### Trait Implementations

##### `impl Clone for AbiTags`

- <span id="abitags-clone"></span>`fn clone(&self) -> AbiTags` — [`AbiTags`](#abitags)

##### `impl Debug for AbiTags`

- <span id="abitags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AbiTags`

- <span id="abitags-default"></span>`fn default() -> AbiTags` — [`AbiTags`](#abitags)

##### `impl Eq for AbiTags`

##### `impl PartialEq for AbiTags`

- <span id="abitags-partialeq-eq"></span>`fn eq(&self, other: &AbiTags) -> bool` — [`AbiTags`](#abitags)

##### `impl StructuralPartialEq for AbiTags`

### `AbiTag`

```rust
struct AbiTag(SourceName);
```

The `<abi-tag>` non-terminal.

```text
<abi-tag> ::= B <source-name>
```

#### Trait Implementations

##### `impl Clone for AbiTag`

- <span id="abitag-clone"></span>`fn clone(&self) -> AbiTag` — [`AbiTag`](#abitag)

##### `impl Debug for AbiTag`

- <span id="abitag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AbiTag`

##### `impl PartialEq for AbiTag`

- <span id="abitag-partialeq-eq"></span>`fn eq(&self, other: &AbiTag) -> bool` — [`AbiTag`](#abitag)

##### `impl StructuralPartialEq for AbiTag`

### `Identifier`

```rust
struct Identifier {
    start: usize,
    end: usize,
}
```

The `<identifier>` pseudo-terminal.

```text
<identifier> ::= <unqualified source code identifier>
```

> `<identifier>` is a pseudo-terminal representing the characters in the
> unqualified identifier for the entity in the source code. This ABI does not
> yet specify a mangling for identifiers containing characters outside of
> `_A-Za-z0-9.`.

Mangled symbols' identifiers also have `$` characters in the wild.

#### Trait Implementations

##### `impl Clone for Identifier`

- <span id="identifier-clone"></span>`fn clone(&self) -> Identifier` — [`Identifier`](#identifier)

##### `impl Debug for Identifier`

- <span id="identifier-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Identifier`

##### `impl PartialEq for Identifier`

- <span id="identifier-partialeq-eq"></span>`fn eq(&self, other: &Identifier) -> bool` — [`Identifier`](#identifier)

##### `impl StructuralPartialEq for Identifier`

### `CloneTypeIdentifier`

```rust
struct CloneTypeIdentifier {
    start: usize,
    end: usize,
}
```

The `<clone-type-identifier>` pseudo-terminal.

```text
<clone-type-identifier> ::= <unqualified source code identifier>
```

#### Trait Implementations

##### `impl Clone for CloneTypeIdentifier`

- <span id="clonetypeidentifier-clone"></span>`fn clone(&self) -> CloneTypeIdentifier` — [`CloneTypeIdentifier`](#clonetypeidentifier)

##### `impl Debug for CloneTypeIdentifier`

- <span id="clonetypeidentifier-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CloneTypeIdentifier`

##### `impl PartialEq for CloneTypeIdentifier`

- <span id="clonetypeidentifier-partialeq-eq"></span>`fn eq(&self, other: &CloneTypeIdentifier) -> bool` — [`CloneTypeIdentifier`](#clonetypeidentifier)

##### `impl StructuralPartialEq for CloneTypeIdentifier`

### `SeqId`

```rust
struct SeqId(usize);
```

A <seq-id> production encoding a base-36 positive number.

```text
<seq-id> ::= <0-9A-Z>+
```

#### Trait Implementations

##### `impl Clone for SeqId`

- <span id="seqid-clone"></span>`fn clone(&self) -> SeqId` — [`SeqId`](#seqid)

##### `impl Debug for SeqId`

- <span id="seqid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SeqId`

##### `impl PartialEq for SeqId`

- <span id="seqid-partialeq-eq"></span>`fn eq(&self, other: &SeqId) -> bool` — [`SeqId`](#seqid)

##### `impl StructuralPartialEq for SeqId`

### `NvOffset`

```rust
struct NvOffset(isize);
```

A non-virtual offset, as described by the <nv-offset> production.

```text
<nv-offset> ::= <offset number>
```

#### Trait Implementations

##### `impl Clone for NvOffset`

- <span id="nvoffset-clone"></span>`fn clone(&self) -> NvOffset` — [`NvOffset`](#nvoffset)

##### `impl Debug for NvOffset`

- <span id="nvoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NvOffset`

##### `impl PartialEq for NvOffset`

- <span id="nvoffset-partialeq-eq"></span>`fn eq(&self, other: &NvOffset) -> bool` — [`NvOffset`](#nvoffset)

##### `impl StructuralPartialEq for NvOffset`

### `VOffset`

```rust
struct VOffset(isize, isize);
```

A virtual offset, as described by the <v-offset> production.

```text
<v-offset> ::= <offset number> _ <virtual offset number>
```

#### Trait Implementations

##### `impl Clone for VOffset`

- <span id="voffset-clone"></span>`fn clone(&self) -> VOffset` — [`VOffset`](#voffset)

##### `impl Debug for VOffset`

- <span id="voffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for VOffset`

##### `impl PartialEq for VOffset`

- <span id="voffset-partialeq-eq"></span>`fn eq(&self, other: &VOffset) -> bool` — [`VOffset`](#voffset)

##### `impl StructuralPartialEq for VOffset`

### `CvQualifiers`

```rust
struct CvQualifiers {
    pub restrict: bool,
    pub volatile: bool,
    pub const_: bool,
}
```

The `<CV-qualifiers>` production.

```text
<CV-qualifiers> ::= [r] [V] [K]   # restrict (C99), volatile, const
```

#### Fields

- **`restrict`**: `bool`

  Is this `restrict` qualified?

- **`volatile`**: `bool`

  Is this `volatile` qualified?

- **`const_`**: `bool`

  Is this `const` qualified?

#### Implementations

- <span id="cvqualifiers-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Clone for CvQualifiers`

- <span id="cvqualifiers-clone"></span>`fn clone(&self) -> CvQualifiers` — [`CvQualifiers`](#cvqualifiers)

##### `impl Debug for CvQualifiers`

- <span id="cvqualifiers-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CvQualifiers`

- <span id="cvqualifiers-default"></span>`fn default() -> CvQualifiers` — [`CvQualifiers`](#cvqualifiers)

##### `impl Eq for CvQualifiers`

##### `impl Hash for CvQualifiers`

- <span id="cvqualifiers-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CvQualifiers`

- <span id="cvqualifiers-partialeq-eq"></span>`fn eq(&self, other: &CvQualifiers) -> bool` — [`CvQualifiers`](#cvqualifiers)

##### `impl StructuralPartialEq for CvQualifiers`

### `QualifiedBuiltin`

```rust
struct QualifiedBuiltin(CvQualifiers, BuiltinType);
```

A built-in type with CV-qualifiers.

Like unqualified built-in types, CV-qualified built-in types do not go into
the substitutions table.

#### Trait Implementations

##### `impl Clone for QualifiedBuiltin`

- <span id="qualifiedbuiltin-clone"></span>`fn clone(&self) -> QualifiedBuiltin` — [`QualifiedBuiltin`](#qualifiedbuiltin)

##### `impl Debug for QualifiedBuiltin`

- <span id="qualifiedbuiltin-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for QualifiedBuiltin`

##### `impl GetLeafName for QualifiedBuiltin`

- <span id="qualifiedbuiltin-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, _: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for QualifiedBuiltin`

- <span id="qualifiedbuiltin-partialeq-eq"></span>`fn eq(&self, other: &QualifiedBuiltin) -> bool` — [`QualifiedBuiltin`](#qualifiedbuiltin)

##### `impl StructuralPartialEq for QualifiedBuiltin`

### `FunctionType`

```rust
struct FunctionType {
    cv_qualifiers: CvQualifiers,
    exception_spec: Option<ExceptionSpec>,
    transaction_safe: bool,
    extern_c: bool,
    bare: BareFunctionType,
    ref_qualifier: Option<RefQualifier>,
}
```

The `<function-type>` production.

```text
<function-type> ::= [<CV-qualifiers>] [exception-spec] [Dx] F [Y] <bare-function-type> [<ref-qualifier>] E
```

#### Implementations

- <span id="functiontype-starts-with"></span>`fn starts_with(input: &IndexStr<'_>) -> bool` — [`IndexStr`](../index_str/index.md#indexstr)

#### Trait Implementations

##### `impl Clone for FunctionType`

- <span id="functiontype-clone"></span>`fn clone(&self) -> FunctionType` — [`FunctionType`](#functiontype)

##### `impl Debug for FunctionType`

- <span id="functiontype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FunctionType`

##### `impl PartialEq for FunctionType`

- <span id="functiontype-partialeq-eq"></span>`fn eq(&self, other: &FunctionType) -> bool` — [`FunctionType`](#functiontype)

##### `impl StructuralPartialEq for FunctionType`

### `BareFunctionType`

```rust
struct BareFunctionType(alloc::vec::Vec<TypeHandle>);
```

The `<bare-function-type>` production.

```text
<bare-function-type> ::= <signature type>+
     types are possible return type, then parameter types
```

#### Implementations

- <span id="barefunctiontype-ret"></span>`fn ret(&self) -> &TypeHandle` — [`TypeHandle`](#typehandle)

- <span id="barefunctiontype-args"></span>`fn args(&self) -> &FunctionArgListAndReturnType` — [`FunctionArgListAndReturnType`](#functionarglistandreturntype)

#### Trait Implementations

##### `impl Clone for BareFunctionType`

- <span id="barefunctiontype-clone"></span>`fn clone(&self) -> BareFunctionType` — [`BareFunctionType`](#barefunctiontype)

##### `impl Debug for BareFunctionType`

- <span id="barefunctiontype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BareFunctionType`

##### `impl PartialEq for BareFunctionType`

- <span id="barefunctiontype-partialeq-eq"></span>`fn eq(&self, other: &BareFunctionType) -> bool` — [`BareFunctionType`](#barefunctiontype)

##### `impl StructuralPartialEq for BareFunctionType`

### `UnnamedTypeName`

```rust
struct UnnamedTypeName(Option<usize>);
```

The `<unnamed-type-name>` production.

```text
<unnamed-type-name> ::= Ut [ <nonnegative number> ] _
                    ::= <closure-type-name>
    ```

We handle `<closure-type-name>` separately.

#### Implementations

- <span id="unnamedtypename-starts-with"></span>`fn starts_with(byte: u8) -> bool`

#### Trait Implementations

##### `impl ArgScope for UnnamedTypeName`

- <span id="unnamedtypename-argscope-leaf-name"></span>`fn leaf_name(self: &'subs Self) -> Result<LeafName<'subs>>` — [`Result`](../error/index.md#result), [`LeafName`](#leafname)

- <span id="unnamedtypename-argscope-get-template-arg"></span>`fn get_template_arg(self: &'subs Self, _: usize) -> Result<(&'subs TemplateArg, &'subs TemplateArgs)>` — [`Result`](../error/index.md#result), [`TemplateArg`](#templatearg), [`TemplateArgs`](#templateargs)

- <span id="unnamedtypename-argscope-get-function-arg"></span>`fn get_function_arg(self: &'subs Self, _: usize) -> Result<&'subs Type>` — [`Result`](../error/index.md#result), [`Type`](#type)

##### `impl Clone for UnnamedTypeName`

- <span id="unnamedtypename-clone"></span>`fn clone(&self) -> UnnamedTypeName` — [`UnnamedTypeName`](#unnamedtypename)

##### `impl Debug for UnnamedTypeName`

- <span id="unnamedtypename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W> DemangleAsLeaf for UnnamedTypeName`

- <span id="unnamedtypename-demangleasleaf-demangle-as-leaf"></span>`fn demangle_as_leaf<'me, 'ctx>(self: &'me Self, ctx: &'ctx mut DemangleContext<'subs, W>) -> fmt::Result`

##### `impl Eq for UnnamedTypeName`

##### `impl PartialEq for UnnamedTypeName`

- <span id="unnamedtypename-partialeq-eq"></span>`fn eq(&self, other: &UnnamedTypeName) -> bool` — [`UnnamedTypeName`](#unnamedtypename)

##### `impl StructuralPartialEq for UnnamedTypeName`

### `PointerToMemberType`

```rust
struct PointerToMemberType(TypeHandle, TypeHandle);
```

The `<pointer-to-member-type>` production.

```text
<pointer-to-member-type> ::= M <class type> <member type>
```

#### Trait Implementations

##### `impl Clone for PointerToMemberType`

- <span id="pointertomembertype-clone"></span>`fn clone(&self) -> PointerToMemberType` — [`PointerToMemberType`](#pointertomembertype)

##### `impl Debug for PointerToMemberType`

- <span id="pointertomembertype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PointerToMemberType`

##### `impl PartialEq for PointerToMemberType`

- <span id="pointertomembertype-partialeq-eq"></span>`fn eq(&self, other: &PointerToMemberType) -> bool` — [`PointerToMemberType`](#pointertomembertype)

##### `impl StructuralPartialEq for PointerToMemberType`

### `TemplateParam`

```rust
struct TemplateParam(usize);
```

The `<template-param>` production.

```text
<template-param> ::= T_ # first template parameter
                 ::= T <parameter-2 non-negative number> _
```

#### Implementations

- <span id="templateparam-resolve"></span>`fn resolve<'subs, 'prev>(self: &'subs Self, scope: Option<ArgScopeStack<'prev, 'subs>>) -> ::core::result::Result<&'subs TemplateArg, fmt::Error>` — [`ArgScopeStack`](#argscopestack), [`TemplateArg`](#templatearg)

#### Trait Implementations

##### `impl Clone for TemplateParam`

- <span id="templateparam-clone"></span>`fn clone(&self) -> TemplateParam` — [`TemplateParam`](#templateparam)

##### `impl Debug for TemplateParam`

- <span id="templateparam-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TemplateParam`

##### `impl Hash for &'a TemplateParam`

- <span id="a-templateparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for TemplateParam`

- <span id="templateparam-partialeq-eq"></span>`fn eq(&self, other: &TemplateParam) -> bool` — [`TemplateParam`](#templateparam)

##### `impl StructuralPartialEq for TemplateParam`

### `TemplateTemplateParam`

```rust
struct TemplateTemplateParam(TemplateParam);
```

The `<template-template-param>` production.

```text
<template-template-param> ::= <template-param>
                          ::= <substitution>
```

#### Trait Implementations

##### `impl Clone for TemplateTemplateParam`

- <span id="templatetemplateparam-clone"></span>`fn clone(&self) -> TemplateTemplateParam` — [`TemplateTemplateParam`](#templatetemplateparam)

##### `impl Debug for TemplateTemplateParam`

- <span id="templatetemplateparam-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TemplateTemplateParam`

##### `impl PartialEq for TemplateTemplateParam`

- <span id="templatetemplateparam-partialeq-eq"></span>`fn eq(&self, other: &TemplateTemplateParam) -> bool` — [`TemplateTemplateParam`](#templatetemplateparam)

##### `impl StructuralPartialEq for TemplateTemplateParam`

### `FunctionParam`

```rust
struct FunctionParam(usize, CvQualifiers, Option<usize>);
```

The <function-param> production.

```text
<function-param> ::= fp <top-level CV-qualifiers> _
                         L == 0, first parameter
                 ::= fp <top-level CV-qualifiers> <parameter-2 non-negative number> _
                         L == 0, second and later parameters
                 ::= fL <L-1 non-negative number> p <top-level CV-qualifiers> _
                         L > 0, first parameter
                 ::= fL <L-1 non-negative number> p <top-level CV-qualifiers> <parameter-2 non-negative number> _
                         L > 0, second and later parameters
```

#### Trait Implementations

##### `impl Clone for FunctionParam`

- <span id="functionparam-clone"></span>`fn clone(&self) -> FunctionParam` — [`FunctionParam`](#functionparam)

##### `impl Debug for FunctionParam`

- <span id="functionparam-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FunctionParam`

##### `impl PartialEq for FunctionParam`

- <span id="functionparam-partialeq-eq"></span>`fn eq(&self, other: &FunctionParam) -> bool` — [`FunctionParam`](#functionparam)

##### `impl StructuralPartialEq for FunctionParam`

### `TemplateArgs`

```rust
struct TemplateArgs(alloc::vec::Vec<TemplateArg>);
```

The `<template-args>` production.

```text
<template-args> ::= I <template-arg>+ E
```

#### Trait Implementations

##### `impl ArgScope for TemplateArgs`

- <span id="templateargs-argscope-leaf-name"></span>`fn leaf_name(self: &'subs Self) -> Result<LeafName<'subs>>` — [`Result`](../error/index.md#result), [`LeafName`](#leafname)

- <span id="templateargs-argscope-get-template-arg"></span>`fn get_template_arg(self: &'subs Self, idx: usize) -> Result<(&'subs TemplateArg, &'subs TemplateArgs)>` — [`Result`](../error/index.md#result), [`TemplateArg`](#templatearg), [`TemplateArgs`](#templateargs)

- <span id="templateargs-argscope-get-function-arg"></span>`fn get_function_arg(self: &'subs Self, _: usize) -> Result<&'subs Type>` — [`Result`](../error/index.md#result), [`Type`](#type)

##### `impl Clone for TemplateArgs`

- <span id="templateargs-clone"></span>`fn clone(&self) -> TemplateArgs` — [`TemplateArgs`](#templateargs)

##### `impl Debug for TemplateArgs`

- <span id="templateargs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TemplateArgs`

##### `impl PartialEq for TemplateArgs`

- <span id="templateargs-partialeq-eq"></span>`fn eq(&self, other: &TemplateArgs) -> bool` — [`TemplateArgs`](#templateargs)

##### `impl StructuralPartialEq for TemplateArgs`

### `MemberName`

```rust
struct MemberName(Name);
```

In libiberty, Member and DerefMember expressions have special handling.
They parse an `UnqualifiedName` (not an `UnscopedName` as the cxxabi docs
say) and optionally a `TemplateArgs` if it is present. We can't just parse
a `Name` or an `UnscopedTemplateName` here because that allows other inputs
that libiberty does not.

#### Trait Implementations

##### `impl Clone for MemberName`

- <span id="membername-clone"></span>`fn clone(&self) -> MemberName` — [`MemberName`](#membername)

##### `impl Debug for MemberName`

- <span id="membername-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MemberName`

##### `impl PartialEq for MemberName`

- <span id="membername-partialeq-eq"></span>`fn eq(&self, other: &MemberName) -> bool` — [`MemberName`](#membername)

##### `impl StructuralPartialEq for MemberName`

### `UnresolvedQualifierLevel`

```rust
struct UnresolvedQualifierLevel(SimpleId);
```

The `<unresolved-qualifier-level>` production.

```text
<unresolved-qualifier-level> ::= <simple-id>
```

#### Trait Implementations

##### `impl Clone for UnresolvedQualifierLevel`

- <span id="unresolvedqualifierlevel-clone"></span>`fn clone(&self) -> UnresolvedQualifierLevel` — [`UnresolvedQualifierLevel`](#unresolvedqualifierlevel)

##### `impl Debug for UnresolvedQualifierLevel`

- <span id="unresolvedqualifierlevel-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnresolvedQualifierLevel`

##### `impl PartialEq for UnresolvedQualifierLevel`

- <span id="unresolvedqualifierlevel-partialeq-eq"></span>`fn eq(&self, other: &UnresolvedQualifierLevel) -> bool` — [`UnresolvedQualifierLevel`](#unresolvedqualifierlevel)

##### `impl StructuralPartialEq for UnresolvedQualifierLevel`

### `SimpleId`

```rust
struct SimpleId(SourceName, Option<TemplateArgs>);
```

The `<simple-id>` production.

```text
<simple-id> ::= <source-name> [ <template-args> ]
```

#### Trait Implementations

##### `impl Clone for SimpleId`

- <span id="simpleid-clone"></span>`fn clone(&self) -> SimpleId` — [`SimpleId`](#simpleid)

##### `impl Debug for SimpleId`

- <span id="simpleid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SimpleId`

##### `impl PartialEq for SimpleId`

- <span id="simpleid-partialeq-eq"></span>`fn eq(&self, other: &SimpleId) -> bool` — [`SimpleId`](#simpleid)

##### `impl StructuralPartialEq for SimpleId`

### `Initializer`

```rust
struct Initializer(alloc::vec::Vec<Expression>);
```

The `<initializer>` production.

```text
<initializer> ::= pi <expression>* E # parenthesized initialization
```

#### Trait Implementations

##### `impl Clone for Initializer`

- <span id="initializer-clone"></span>`fn clone(&self) -> Initializer` — [`Initializer`](#initializer)

##### `impl Debug for Initializer`

- <span id="initializer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Initializer`

##### `impl PartialEq for Initializer`

- <span id="initializer-partialeq-eq"></span>`fn eq(&self, other: &Initializer) -> bool` — [`Initializer`](#initializer)

##### `impl StructuralPartialEq for Initializer`

### `Discriminator`

```rust
struct Discriminator(usize);
```

The `<discriminator>` production.

```text
<discriminator> := _ <non-negative number>      # when number < 10
                := __ <non-negative number> _   # when number >= 10
```

#### Trait Implementations

##### `impl Clone for Discriminator`

- <span id="discriminator-clone"></span>`fn clone(&self) -> Discriminator` — [`Discriminator`](#discriminator)

##### `impl Debug for Discriminator`

- <span id="discriminator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Discriminator`

##### `impl PartialEq for Discriminator`

- <span id="discriminator-partialeq-eq"></span>`fn eq(&self, other: &Discriminator) -> bool` — [`Discriminator`](#discriminator)

##### `impl StructuralPartialEq for Discriminator`

### `ClosureTypeName`

```rust
struct ClosureTypeName(LambdaSig, Option<usize>);
```

The `<closure-type-name>` production.

```text
<closure-type-name> ::= Ul <lambda-sig> E [ <nonnegative number> ] _
```

#### Implementations

- <span id="closuretypename-starts-with"></span>`fn starts_with(byte: u8, input: &IndexStr<'_>) -> bool` — [`IndexStr`](../index_str/index.md#indexstr)

#### Trait Implementations

##### `impl ArgScope for ClosureTypeName`

- <span id="closuretypename-argscope-leaf-name"></span>`fn leaf_name(self: &'subs Self) -> Result<LeafName<'subs>>` — [`Result`](../error/index.md#result), [`LeafName`](#leafname)

- <span id="closuretypename-argscope-get-template-arg"></span>`fn get_template_arg(self: &'subs Self, _: usize) -> Result<(&'subs TemplateArg, &'subs TemplateArgs)>` — [`Result`](../error/index.md#result), [`TemplateArg`](#templatearg), [`TemplateArgs`](#templateargs)

- <span id="closuretypename-argscope-get-function-arg"></span>`fn get_function_arg(self: &'subs Self, _: usize) -> Result<&'subs Type>` — [`Result`](../error/index.md#result), [`Type`](#type)

##### `impl Clone for ClosureTypeName`

- <span id="closuretypename-clone"></span>`fn clone(&self) -> ClosureTypeName` — [`ClosureTypeName`](#closuretypename)

##### `impl Debug for ClosureTypeName`

- <span id="closuretypename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClosureTypeName`

##### `impl GetLeafName for ClosureTypeName`

- <span id="closuretypename-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, _: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for ClosureTypeName`

- <span id="closuretypename-partialeq-eq"></span>`fn eq(&self, other: &ClosureTypeName) -> bool` — [`ClosureTypeName`](#closuretypename)

##### `impl StructuralPartialEq for ClosureTypeName`

### `LambdaSig`

```rust
struct LambdaSig(alloc::vec::Vec<TypeHandle>);
```

The `<lambda-sig>` production.

```text
<lambda-sig> ::= <parameter type>+  # Parameter types or "v" if the lambda has no parameters
```

#### Implementations

- <span id="lambdasig-demangle-args"></span>`fn demangle_args<'subs, 'prev, 'ctx, W>(self: &'subs Self, ctx: &'ctx mut DemangleContext<'subs, W>, scope: Option<ArgScopeStack<'prev, 'subs>>) -> fmt::Result` — [`ArgScopeStack`](#argscopestack)

#### Trait Implementations

##### `impl Clone for LambdaSig`

- <span id="lambdasig-clone"></span>`fn clone(&self) -> LambdaSig` — [`LambdaSig`](#lambdasig)

##### `impl Debug for LambdaSig`

- <span id="lambdasig-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LambdaSig`

##### `impl PartialEq for LambdaSig`

- <span id="lambdasig-partialeq-eq"></span>`fn eq(&self, other: &LambdaSig) -> bool` — [`LambdaSig`](#lambdasig)

##### `impl StructuralPartialEq for LambdaSig`

### `DataMemberPrefix`

```rust
struct DataMemberPrefix(SourceName);
```

The `<data-member-prefix>` production.

```text
<data-member-prefix> := <member source-name> M
```

#### Trait Implementations

##### `impl Clone for DataMemberPrefix`

- <span id="datamemberprefix-clone"></span>`fn clone(&self) -> DataMemberPrefix` — [`DataMemberPrefix`](#datamemberprefix)

##### `impl Debug for DataMemberPrefix`

- <span id="datamemberprefix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DataMemberPrefix`

##### `impl GetLeafName for DataMemberPrefix`

- <span id="datamemberprefix-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, _: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for DataMemberPrefix`

- <span id="datamemberprefix-partialeq-eq"></span>`fn eq(&self, other: &DataMemberPrefix) -> bool` — [`DataMemberPrefix`](#datamemberprefix)

##### `impl StructuralPartialEq for DataMemberPrefix`

### `ResourceName`

```rust
struct ResourceName {
    start: usize,
    end: usize,
}
```

The `<resource name>` pseudo-terminal.

#### Trait Implementations

##### `impl Clone for ResourceName`

- <span id="resourcename-clone"></span>`fn clone(&self) -> ResourceName` — [`ResourceName`](#resourcename)

##### `impl Debug for ResourceName`

- <span id="resourcename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ResourceName`

##### `impl PartialEq for ResourceName`

- <span id="resourcename-partialeq-eq"></span>`fn eq(&self, other: &ResourceName) -> bool` — [`ResourceName`](#resourcename)

##### `impl StructuralPartialEq for ResourceName`

### `SubobjectExpr`

```rust
struct SubobjectExpr {
    ty: TypeHandle,
    expr: alloc::boxed::Box<Expression>,
    offset: isize,
}
```

The subobject expression production.

<expression> ::= so <referent type> <expr> [<offset number>] <union-selector>* [p] E
<union-selector> ::= _ [<number>]

Not yet in the spec: https://github.com/itanium-cxx-abi/cxx-abi/issues/47
But it has been shipping in clang for some time.

#### Trait Implementations

##### `impl Clone for SubobjectExpr`

- <span id="subobjectexpr-clone"></span>`fn clone(&self) -> SubobjectExpr` — [`SubobjectExpr`](#subobjectexpr)

##### `impl Debug for SubobjectExpr`

- <span id="subobjectexpr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SubobjectExpr`

##### `impl PartialEq for SubobjectExpr`

- <span id="subobjectexpr-partialeq-eq"></span>`fn eq(&self, other: &SubobjectExpr) -> bool` — [`SubobjectExpr`](#subobjectexpr)

##### `impl StructuralPartialEq for SubobjectExpr`

## Enums

### `LeafName<'a>`

```rust
enum LeafName<'a> {
    SourceName(&'a SourceName),
    WellKnownComponent(&'a WellKnownComponent),
    Closure(&'a ClosureTypeName),
    UnnamedType(&'a UnnamedTypeName),
}
```

A leaf name is the part the name that describes some type or class without
any leading namespace qualifiers.

This is used when figuring out how to format constructors and destructors,
which are formatted as `gooble::dodo::Thing::~Thing()` but we don't have
direct access to `Thing` in the `CtorDtorName` AST.

#### Trait Implementations

##### `impl Debug for LeafName<'a>`

- <span id="leafname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W> DemangleAsLeaf for LeafName<'subs>`

- <span id="leafname-demangleasleaf-demangle-as-leaf"></span>`fn demangle_as_leaf<'me, 'ctx>(self: &'me Self, ctx: &'ctx mut DemangleContext<'subs, W>) -> fmt::Result`

### `MangledName`

```rust
enum MangledName {
    Encoding(Encoding, alloc::vec::Vec<CloneSuffix>),
    BlockInvoke(Encoding, Option<isize>),
    Type(TypeHandle),
    GlobalCtorDtor(GlobalCtorDtor),
}
```

The root AST node, and starting production.

```text
<mangled-name> ::= _Z <encoding> [<clone-suffix>]*
               ::= ___Z <encoding> <block_invoke>
               ::= <type>

<block_invoke> ::= _block_invoke
               ::= _block_invoke<decimal-digit>+
               ::= _block_invoke_<decimal-digit>+
```

#### Variants

- **`Encoding`**

  The encoding of the mangled symbol name.

- **`BlockInvoke`**

  The encoding of the mangled symbol name.

- **`Type`**

  A top-level type. Technically not allowed by the standard, however in
  practice this can happen, and is tested for by libiberty.

- **`GlobalCtorDtor`**

  A global constructor or destructor. This is another de facto standard
  extension (I think originally from `g++`?) that is not actually part of
  the standard proper.

#### Trait Implementations

##### `impl Clone for MangledName`

- <span id="mangledname-clone"></span>`fn clone(&self) -> MangledName` — [`MangledName`](#mangledname)

##### `impl Debug for MangledName`

- <span id="mangledname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MangledName`

##### `impl PartialEq for MangledName`

- <span id="mangledname-partialeq-eq"></span>`fn eq(&self, other: &MangledName) -> bool` — [`MangledName`](#mangledname)

##### `impl StructuralPartialEq for MangledName`

### `Encoding`

```rust
enum Encoding {
    Function(Name, BareFunctionType),
    Data(Name),
    Special(SpecialName),
}
```

The `<encoding>` production.

```text
<encoding> ::= <function name> <bare-function-type>
           ::= <data name>
           ::= <special-name>
```

#### Variants

- **`Function`**

  An encoded function.

- **`Data`**

  An encoded static variable.

- **`Special`**

  A special encoding.

#### Trait Implementations

##### `impl Clone for Encoding`

- <span id="encoding-clone"></span>`fn clone(&self) -> Encoding` — [`Encoding`](#encoding)

##### `impl Debug for Encoding`

- <span id="encoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Encoding`

##### `impl PartialEq for Encoding`

- <span id="encoding-partialeq-eq"></span>`fn eq(&self, other: &Encoding) -> bool` — [`Encoding`](#encoding)

##### `impl StructuralPartialEq for Encoding`

### `GlobalCtorDtor`

```rust
enum GlobalCtorDtor {
    Ctor(alloc::boxed::Box<MangledName>),
    Dtor(alloc::boxed::Box<MangledName>),
}
```

A global constructor or destructor.

#### Variants

- **`Ctor`**

  A global constructor.

- **`Dtor`**

  A global destructor.

#### Trait Implementations

##### `impl Clone for GlobalCtorDtor`

- <span id="globalctordtor-clone"></span>`fn clone(&self) -> GlobalCtorDtor` — [`GlobalCtorDtor`](#globalctordtor)

##### `impl Debug for GlobalCtorDtor`

- <span id="globalctordtor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for GlobalCtorDtor`

##### `impl PartialEq for GlobalCtorDtor`

- <span id="globalctordtor-partialeq-eq"></span>`fn eq(&self, other: &GlobalCtorDtor) -> bool` — [`GlobalCtorDtor`](#globalctordtor)

##### `impl StructuralPartialEq for GlobalCtorDtor`

### `Name`

```rust
enum Name {
    Nested(NestedName),
    Unscoped(UnscopedName),
    UnscopedTemplate(UnscopedTemplateNameHandle, TemplateArgs),
    Local(LocalName),
}
```

The `<name>` production.

```text
<name> ::= <nested-name>
       ::= <unscoped-name>
       ::= <unscoped-template-name> <template-args>
       ::= <local-name>
```

#### Variants

- **`Nested`**

  A nested name

- **`Unscoped`**

  An unscoped name.

- **`UnscopedTemplate`**

  An unscoped template.

- **`Local`**

  A local name.

#### Trait Implementations

##### `impl Clone for Name`

- <span id="name-clone"></span>`fn clone(&self) -> Name` — [`Name`](#name)

##### `impl Debug for Name`

- <span id="name-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Name`

##### `impl GetLeafName for Name`

- <span id="name-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl GetTemplateArgs for Name`

- <span id="name-gettemplateargs-get-template-args"></span>`fn get_template_args<'a>(self: &'a Self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs>` — [`TemplateArgs`](#templateargs)

##### `impl IsCtorDtorConversion for Name`

- <span id="name-isctordtorconversion-is-ctor-dtor-conversion"></span>`fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool`

##### `impl PartialEq for Name`

- <span id="name-partialeq-eq"></span>`fn eq(&self, other: &Name) -> bool` — [`Name`](#name)

##### `impl StructuralPartialEq for Name`

### `UnscopedName`

```rust
enum UnscopedName {
    Unqualified(UnqualifiedName),
    Std(UnqualifiedName),
}
```

The `<unscoped-name>` production.

```text
<unscoped-name> ::= <unqualified-name>
                ::= St <unqualified-name>   # ::std::
```

#### Variants

- **`Unqualified`**

  An unqualified name.

- **`Std`**

  A name within the `std::` namespace.

#### Trait Implementations

##### `impl Clone for UnscopedName`

- <span id="unscopedname-clone"></span>`fn clone(&self) -> UnscopedName` — [`UnscopedName`](#unscopedname)

##### `impl Debug for UnscopedName`

- <span id="unscopedname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnscopedName`

##### `impl GetLeafName for UnscopedName`

- <span id="unscopedname-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl IsCtorDtorConversion for UnscopedName`

- <span id="unscopedname-isctordtorconversion-is-ctor-dtor-conversion"></span>`fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool`

##### `impl PartialEq for UnscopedName`

- <span id="unscopedname-partialeq-eq"></span>`fn eq(&self, other: &UnscopedName) -> bool` — [`UnscopedName`](#unscopedname)

##### `impl StructuralPartialEq for UnscopedName`

### `UnscopedTemplateNameHandle`

```rust
enum UnscopedTemplateNameHandle {
    WellKnown(WellKnownComponent),
    BackReference(usize),
    NonSubstitution(NonSubstitution),
}
```

A handle to an `UnscopedTemplateName`.

#### Variants

- **`WellKnown`**

  A reference to a "well-known" component.

- **`BackReference`**

  A back-reference into the substitution table to a component we
  have already parsed.

- **`NonSubstitution`**

  A handle to some `<unscoped-name>` component that isn't by itself
  substitutable.

#### Implementations

- <span id="unscopedtemplatenamehandle-back-reference"></span>`fn back_reference(&self) -> Option<usize>`

  If this is a `BackReference`, get its index.

#### Trait Implementations

##### `impl Clone for UnscopedTemplateNameHandle`

- <span id="unscopedtemplatenamehandle-clone"></span>`fn clone(&self) -> UnscopedTemplateNameHandle` — [`UnscopedTemplateNameHandle`](#unscopedtemplatenamehandle)

##### `impl Debug for UnscopedTemplateNameHandle`

- <span id="unscopedtemplatenamehandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnscopedTemplateNameHandle`

##### `impl GetLeafName for UnscopedTemplateNameHandle`

- <span id="unscopedtemplatenamehandle-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for UnscopedTemplateNameHandle`

- <span id="unscopedtemplatenamehandle-partialeq-eq"></span>`fn eq(&self, other: &UnscopedTemplateNameHandle) -> bool` — [`UnscopedTemplateNameHandle`](#unscopedtemplatenamehandle)

##### `impl StructuralPartialEq for UnscopedTemplateNameHandle`

### `NestedName`

```rust
enum NestedName {
    Unqualified(CvQualifiers, Option<RefQualifier>, Option<PrefixHandle>, UnqualifiedName),
    Template(CvQualifiers, Option<RefQualifier>, PrefixHandle),
    UnqualifiedExplicitObject(Option<PrefixHandle>, UnqualifiedName, ExplicitObjectParameter),
    TemplateExplicitObject(PrefixHandle, ExplicitObjectParameter),
}
```

The `<nested-name>` production.

```text
<nested-name> ::= N [<CV-qualifiers>] [<ref-qualifier>] <prefix> <unqualified-name> E
              ::= N [<CV-qualifiers>] [<ref-qualifier>] <template-prefix> <template-args> E
              ::= N H <prefix> <unqualified-name> E
              ::= N H <template-prefix> <template-args> E
```

#### Variants

- **`Unqualified`**

  A nested name.

- **`Template`**

  A nested template name. The `<template-args>` are part of the `PrefixHandle`.

- **`UnqualifiedExplicitObject`**

  A nested name with an explicit object.

- **`TemplateExplicitObject`**

  A nested template name with an explicit object.

#### Implementations

- <span id="nestedname-cv-qualifiers"></span>`fn cv_qualifiers(&self) -> Option<&CvQualifiers>` — [`CvQualifiers`](#cvqualifiers)

  Get the CV-qualifiers for this name.

- <span id="nestedname-ref-qualifier"></span>`fn ref_qualifier(&self) -> Option<&RefQualifier>` — [`RefQualifier`](#refqualifier)

  Get the ref-qualifier for this name, if one exists.

- <span id="nestedname-prefix"></span>`fn prefix(&self) -> Option<&PrefixHandle>` — [`PrefixHandle`](#prefixhandle)

- <span id="nestedname-has-explicit-obj-param"></span>`fn has_explicit_obj_param(&self) -> bool`

  Check to see if the object has an explicit named parameter.

#### Trait Implementations

##### `impl Clone for NestedName`

- <span id="nestedname-clone"></span>`fn clone(&self) -> NestedName` — [`NestedName`](#nestedname)

##### `impl Debug for NestedName`

- <span id="nestedname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NestedName`

##### `impl GetLeafName for NestedName`

- <span id="nestedname-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl GetTemplateArgs for NestedName`

- <span id="nestedname-gettemplateargs-get-template-args"></span>`fn get_template_args<'a>(self: &'a Self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs>` — [`TemplateArgs`](#templateargs)

##### `impl IsCtorDtorConversion for NestedName`

- <span id="nestedname-isctordtorconversion-is-ctor-dtor-conversion"></span>`fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool`

##### `impl PartialEq for NestedName`

- <span id="nestedname-partialeq-eq"></span>`fn eq(&self, other: &NestedName) -> bool` — [`NestedName`](#nestedname)

##### `impl StructuralPartialEq for NestedName`

### `Prefix`

```rust
enum Prefix {
    Unqualified(UnqualifiedName),
    Nested(PrefixHandle, UnqualifiedName),
    Template(PrefixHandle, TemplateArgs),
    TemplateParam(TemplateParam),
    Decltype(Decltype),
    DataMember(PrefixHandle, DataMemberPrefix),
}
```

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

#### Variants

- **`Unqualified`**

  An unqualified name.

- **`Nested`**

  Some nested name.

- **`Template`**

  A prefix and template arguments.

- **`TemplateParam`**

  A template parameter.

- **`Decltype`**

  A decltype.

- **`DataMember`**

  A prefix and data member.

#### Trait Implementations

##### `impl Clone for Prefix`

- <span id="prefix-clone"></span>`fn clone(&self) -> Prefix` — [`Prefix`](#prefix)

##### `impl Debug for Prefix`

- <span id="prefix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Prefix`

##### `impl GetLeafName for Prefix`

- <span id="prefix-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl GetTemplateArgs for Prefix`

- <span id="prefix-gettemplateargs-get-template-args"></span>`fn get_template_args<'a>(self: &'a Self, _: &'a SubstitutionTable) -> Option<&'a TemplateArgs>` — [`TemplateArgs`](#templateargs)

##### `impl IsCtorDtorConversion for Prefix`

- <span id="prefix-isctordtorconversion-is-ctor-dtor-conversion"></span>`fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool`

##### `impl PartialEq for Prefix`

- <span id="prefix-partialeq-eq"></span>`fn eq(&self, other: &Prefix) -> bool` — [`Prefix`](#prefix)

##### `impl StructuralPartialEq for Prefix`

### `PrefixHandle`

```rust
enum PrefixHandle {
    WellKnown(WellKnownComponent),
    BackReference(usize),
    NonSubstitution(NonSubstitution),
}
```

A reference to a parsed `<prefix>` production.

#### Variants

- **`WellKnown`**

  A reference to a "well-known" component.

- **`BackReference`**

  A back-reference into the substitution table to a component we
  have already parsed.

- **`NonSubstitution`**

  A handle to some `<prefix>` component that isn't by itself
  substitutable; instead, it's only substitutable *with* its parent
  component.

#### Implementations

- <span id="prefixhandle-back-reference"></span>`fn back_reference(&self) -> Option<usize>`

  If this is a `BackReference`, get its index.

#### Trait Implementations

##### `impl Clone for PrefixHandle`

- <span id="prefixhandle-clone"></span>`fn clone(&self) -> PrefixHandle` — [`PrefixHandle`](#prefixhandle)

##### `impl Debug for PrefixHandle`

- <span id="prefixhandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PrefixHandle`

##### `impl GetLeafName for PrefixHandle`

- <span id="prefixhandle-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl GetTemplateArgs for PrefixHandle`

- <span id="prefixhandle-gettemplateargs-get-template-args"></span>`fn get_template_args<'a>(self: &'a Self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs>` — [`TemplateArgs`](#templateargs)

##### `impl IsCtorDtorConversion for PrefixHandle`

- <span id="prefixhandle-isctordtorconversion-is-ctor-dtor-conversion"></span>`fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool`

##### `impl PartialEq for PrefixHandle`

- <span id="prefixhandle-partialeq-eq"></span>`fn eq(&self, other: &PrefixHandle) -> bool` — [`PrefixHandle`](#prefixhandle)

##### `impl StructuralPartialEq for PrefixHandle`

### `UnqualifiedName`

```rust
enum UnqualifiedName {
    Operator(OperatorName, AbiTags),
    CtorDtor(CtorDtorName, AbiTags),
    Source(SourceName, AbiTags),
    LocalSourceName(SourceName, Option<Discriminator>, AbiTags),
    UnnamedType(UnnamedTypeName, AbiTags),
    ClosureType(ClosureTypeName, AbiTags),
}
```

The `<unqualified-name>` production.

```text
<unqualified-name> ::= [on] <operator-name> [<abi-tags>]
                   ::= <ctor-dtor-name> [<abi-tags>]
                   ::= <source-name> [<abi-tags>]
                   ::= <local-source-name> [<abi-tags>]
                   ::= <unnamed-type-name> [<abi-tags>]
                   ::= <closure-type-name> [<abi-tags>]

I think this is from an older version of the standard. It isn't in the
current version, but all the other demanglers support it, so we will too.
<local-source-name> ::= L <source-name> [<discriminator>]
```

#### Variants

- **`Operator`**

  An operator name.

- **`CtorDtor`**

  A constructor or destructor name.

- **`Source`**

  A source name.

- **`LocalSourceName`**

  A local source name.

- **`UnnamedType`**

  A generated name for an unnamed type.

- **`ClosureType`**

  A closure type name

#### Implementations

- <span id="unqualifiedname-starts-with"></span>`fn starts_with(byte: u8, input: &IndexStr<'_>) -> bool` — [`IndexStr`](../index_str/index.md#indexstr)

#### Trait Implementations

##### `impl Clone for UnqualifiedName`

- <span id="unqualifiedname-clone"></span>`fn clone(&self) -> UnqualifiedName` — [`UnqualifiedName`](#unqualifiedname)

##### `impl Debug for UnqualifiedName`

- <span id="unqualifiedname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnqualifiedName`

##### `impl GetLeafName for UnqualifiedName`

- <span id="unqualifiedname-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl IsCtorDtorConversion for UnqualifiedName`

- <span id="unqualifiedname-isctordtorconversion-is-ctor-dtor-conversion"></span>`fn is_ctor_dtor_conversion(&self, _: &SubstitutionTable) -> bool`

##### `impl PartialEq for UnqualifiedName`

- <span id="unqualifiedname-partialeq-eq"></span>`fn eq(&self, other: &UnqualifiedName) -> bool` — [`UnqualifiedName`](#unqualifiedname)

##### `impl StructuralPartialEq for UnqualifiedName`

### `OperatorName`

```rust
enum OperatorName {
    Simple(SimpleOperatorName),
    Cast(TypeHandle),
    Conversion(TypeHandle),
    Literal(SourceName),
    VendorExtension(u8, SourceName),
}
```

The `<operator-name>` production.

```text
<operator-name> ::= <simple-operator-name>
                ::= cv <type>               # (cast)
                ::= li <source-name>        # operator ""
                ::= v <digit> <source-name> # vendor extended operator
```

#### Variants

- **`Simple`**

  A simple operator name.

- **`Cast`**

  A type cast.

- **`Conversion`**

  A type conversion.

- **`Literal`**

  Operator literal, ie `operator ""`.

- **`VendorExtension`**

  A non-standard, vendor extension operator.

#### Implementations

- <span id="operatorname-starts-with"></span>`fn starts_with(byte: u8) -> bool`

- <span id="operatorname-arity"></span>`fn arity(&self) -> u8`

- <span id="operatorname-parse-from-expr"></span>`fn parse_from_expr<'a, 'b>(ctx: &'a ParseContext, subs: &'a mut SubstitutionTable, input: IndexStr<'b>) -> Result<(Expression, IndexStr<'b>)>` — [`ParseContext`](#parsecontext), [`IndexStr`](../index_str/index.md#indexstr), [`Result`](../error/index.md#result), [`Expression`](#expression)

- <span id="operatorname-parse-internal"></span>`fn parse_internal<'a, 'b>(ctx: &'a ParseContext, subs: &'a mut SubstitutionTable, input: IndexStr<'b>, from_expr: bool) -> Result<(OperatorName, IndexStr<'b>)>` — [`ParseContext`](#parsecontext), [`IndexStr`](../index_str/index.md#indexstr), [`Result`](../error/index.md#result), [`OperatorName`](#operatorname)

#### Trait Implementations

##### `impl Clone for OperatorName`

- <span id="operatorname-clone"></span>`fn clone(&self) -> OperatorName` — [`OperatorName`](#operatorname)

##### `impl Debug for OperatorName`

- <span id="operatorname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OperatorName`

##### `impl PartialEq for OperatorName`

- <span id="operatorname-partialeq-eq"></span>`fn eq(&self, other: &OperatorName) -> bool` — [`OperatorName`](#operatorname)

##### `impl StructuralPartialEq for OperatorName`

### `SimpleOperatorName`

```rust
enum SimpleOperatorName {
    New,
    NewArray,
    Delete,
    DeleteArray,
    UnaryPlus,
    Neg,
    AddressOf,
    Deref,
    BitNot,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    Shl,
    Shr,
    ShlAssign,
    ShrAssign,
    Eq,
    Ne,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Not,
    LogicalAnd,
    LogicalOr,
    PostInc,
    PostDec,
    Comma,
    DerefMemberPtr,
    DerefMember,
    Call,
    Index,
    Question,
    Spaceship,
}
```

The `<simple-operator-name>` production.

#### Variants

- **`New`**

  new

- **`NewArray`**

  new[]

- **`Delete`**

  delete

- **`DeleteArray`**

  delete[]

- **`UnaryPlus`**

  +

- **`Neg`**

  -

- **`AddressOf`**

  &

- **`Deref`**

  *

- **`BitNot`**

  ~

- **`Add`**

  +

- **`Sub`**

  -

- **`Mul`**

  *

- **`Div`**

  /

- **`Rem`**

  %

- **`BitAnd`**

  &

- **`BitOr`**

  |

- **`BitXor`**

  ^

- **`Assign`**

  =

- **`AddAssign`**

  +=

- **`SubAssign`**

  -=

- **`MulAssign`**

  *=

- **`DivAssign`**

  /=

- **`RemAssign`**

  %=

- **`BitAndAssign`**

  &=

- **`BitOrAssign`**

  |=

- **`BitXorAssign`**

  ^=

- **`Shl`**

  <<

- **`Shr`**

  >>

- **`ShlAssign`**

  <<=

- **`ShrAssign`**

  >>=

- **`Eq`**

  ==

- **`Ne`**

  !=

- **`Less`**

  <

- **`Greater`**

  >

- **`LessEq`**

  <=

- **`GreaterEq`**

  >=

- **`Not`**

  !

- **`LogicalAnd`**

  &&

- **`LogicalOr`**

  ||

- **`PostInc`**

  ++

- **`PostDec`**

  --

- **`Comma`**

  ,

- **`DerefMemberPtr`**

  ->*

- **`DerefMember`**

  ->

- **`Call`**

  ()

- **`Index`**

  []

- **`Question`**

  ?:

- **`Spaceship`**

  <=>

#### Implementations

- <span id="simpleoperatorname-starts-with"></span>`fn starts_with(byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for SimpleOperatorName`

- <span id="simpleoperatorname-clone"></span>`fn clone(&self) -> SimpleOperatorName` — [`SimpleOperatorName`](#simpleoperatorname)

##### `impl Debug for SimpleOperatorName`

- <span id="simpleoperatorname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SimpleOperatorName`

##### `impl PartialEq for SimpleOperatorName`

- <span id="simpleoperatorname-partialeq-eq"></span>`fn eq(&self, other: &SimpleOperatorName) -> bool` — [`SimpleOperatorName`](#simpleoperatorname)

##### `impl StructuralPartialEq for SimpleOperatorName`

### `CallOffset`

```rust
enum CallOffset {
    NonVirtual(NvOffset),
    Virtual(VOffset),
}
```

The `<call-offset>` production.

```text
<call-offset> ::= h <nv-offset> _
              ::= v <v-offset> _
```

#### Variants

- **`NonVirtual`**

  A non-virtual offset.

- **`Virtual`**

  A virtual offset.

#### Trait Implementations

##### `impl Clone for CallOffset`

- <span id="calloffset-clone"></span>`fn clone(&self) -> CallOffset` — [`CallOffset`](#calloffset)

##### `impl Debug for CallOffset`

- <span id="calloffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CallOffset`

##### `impl PartialEq for CallOffset`

- <span id="calloffset-partialeq-eq"></span>`fn eq(&self, other: &CallOffset) -> bool` — [`CallOffset`](#calloffset)

##### `impl StructuralPartialEq for CallOffset`

### `CtorDtorName`

```rust
enum CtorDtorName {
    CompleteConstructor(Option<TypeHandle>),
    BaseConstructor(Option<TypeHandle>),
    CompleteAllocatingConstructor(Option<TypeHandle>),
    MaybeInChargeConstructor(Option<TypeHandle>),
    DeletingDestructor,
    CompleteDestructor,
    BaseDestructor,
    MaybeInChargeDestructor,
}
```

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

```rust
/* This is the old-style "[unified]" constructor.
   In some cases, we may emit this function and call
   it from the clones in order to share code and save space.  */
```

Based on the GCC source we'll call this the "maybe in-charge constructor".
Similarly, there is a D4 destructor, the "maybe in-charge destructor".

#### Variants

- **`CompleteConstructor`**

  "C1", the "complete object constructor"

- **`BaseConstructor`**

  "C2", the "base object constructor"

- **`CompleteAllocatingConstructor`**

  "C3", the "complete object allocating constructor"

- **`MaybeInChargeConstructor`**

  "C4", the "maybe in-charge constructor"

- **`DeletingDestructor`**

  "D0", the "deleting destructor"

- **`CompleteDestructor`**

  "D1", the "complete object destructor"

- **`BaseDestructor`**

  "D2", the "base object destructor"

- **`MaybeInChargeDestructor`**

  "D4", the "maybe in-charge destructor"

#### Implementations

- <span id="ctordtorname-inheriting-mut"></span>`fn inheriting_mut(&mut self) -> &mut Option<TypeHandle>` — [`TypeHandle`](#typehandle)

#### Trait Implementations

##### `impl Clone for CtorDtorName`

- <span id="ctordtorname-clone"></span>`fn clone(&self) -> CtorDtorName` — [`CtorDtorName`](#ctordtorname)

##### `impl Debug for CtorDtorName`

- <span id="ctordtorname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CtorDtorName`

##### `impl PartialEq for CtorDtorName`

- <span id="ctordtorname-partialeq-eq"></span>`fn eq(&self, other: &CtorDtorName) -> bool` — [`CtorDtorName`](#ctordtorname)

##### `impl StructuralPartialEq for CtorDtorName`

### `Type`

```rust
enum Type {
    Function(FunctionType),
    ClassEnum(ClassEnumType),
    Array(ArrayType),
    Vector(VectorType),
    PointerToMember(PointerToMemberType),
    TemplateParam(TemplateParam),
    TemplateTemplate(TemplateTemplateParamHandle, TemplateArgs),
    Decltype(Decltype),
    Qualified(CvQualifiers, TypeHandle),
    PointerTo(TypeHandle),
    LvalueRef(TypeHandle),
    RvalueRef(TypeHandle),
    Complex(TypeHandle),
    Imaginary(TypeHandle),
    VendorExtension(SourceName, Option<TemplateArgs>, TypeHandle),
    PackExpansion(TypeHandle),
}
```

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

#### Variants

- **`Function`**

  A function type.

- **`ClassEnum`**

  A class, union, or enum type.

- **`Array`**

  An array type.

- **`Vector`**

  A vector type.

- **`PointerToMember`**

  A pointer-to-member type.

- **`TemplateParam`**

  A named template parameter type.

- **`TemplateTemplate`**

  A template template type.

- **`Decltype`**

  A decltype.

- **`Qualified`**

  A const-, restrict-, and/or volatile-qualified type.

- **`PointerTo`**

  A pointer to a type.

- **`LvalueRef`**

  An lvalue reference to a type.

- **`RvalueRef`**

  An rvalue reference to a type.

- **`Complex`**

  A complex pair of the given type.

- **`Imaginary`**

  An imaginary of the given type.

- **`VendorExtension`**

  A vendor extended type qualifier.

- **`PackExpansion`**

  A pack expansion.

#### Trait Implementations

##### `impl Clone for Type`

- <span id="type-clone"></span>`fn clone(&self) -> Type` — [`Type`](#type)

##### `impl Debug for Type`

- <span id="type-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Type`

##### `impl GetLeafName for Type`

- <span id="type-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl GetTemplateArgs for Type`

- <span id="type-gettemplateargs-get-template-args"></span>`fn get_template_args<'a>(self: &'a Self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs>` — [`TemplateArgs`](#templateargs)

##### `impl PartialEq for Type`

- <span id="type-partialeq-eq"></span>`fn eq(&self, other: &Type) -> bool` — [`Type`](#type)

##### `impl StructuralPartialEq for Type`

### `TypeHandle`

```rust
enum TypeHandle {
    WellKnown(WellKnownComponent),
    BackReference(usize),
    Builtin(BuiltinType),
    QualifiedBuiltin(QualifiedBuiltin),
}
```

A reference to a parsed `Type` production.

#### Variants

- **`WellKnown`**

  A reference to a "well-known" component.

- **`BackReference`**

  A back-reference into the substitution table to a component we
  have already parsed.

- **`Builtin`**

  A builtin type. These don't end up in the substitutions table.

- **`QualifiedBuiltin`**

  A CV-qualified builtin type. These don't end up in the table either.

#### Implementations

- <span id="typehandle-back-reference"></span>`fn back_reference(&self) -> Option<usize>`

  If this is a `BackReference`, get its index.

#### Trait Implementations

##### `impl Clone for TypeHandle`

- <span id="typehandle-clone"></span>`fn clone(&self) -> TypeHandle` — [`TypeHandle`](#typehandle)

##### `impl Debug for TypeHandle`

- <span id="typehandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TypeHandle`

##### `impl GetLeafName for TypeHandle`

- <span id="typehandle-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl GetTemplateArgs for TypeHandle`

- <span id="typehandle-gettemplateargs-get-template-args"></span>`fn get_template_args<'a>(self: &'a Self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs>` — [`TemplateArgs`](#templateargs)

##### `impl PartialEq for TypeHandle`

- <span id="typehandle-partialeq-eq"></span>`fn eq(&self, other: &TypeHandle) -> bool` — [`TypeHandle`](#typehandle)

##### `impl StructuralPartialEq for TypeHandle`

### `RefQualifier`

```rust
enum RefQualifier {
    LValueRef,
    RValueRef,
}
```

A <ref-qualifier> production.

```text
<ref-qualifier> ::= R   # & ref-qualifier
                ::= O   # && ref-qualifier
```

#### Variants

- **`LValueRef`**

  &

- **`RValueRef`**

  &&

#### Implementations

- <span id="refqualifier-starts-with"></span>`fn starts_with(byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for RefQualifier`

- <span id="refqualifier-clone"></span>`fn clone(&self) -> RefQualifier` — [`RefQualifier`](#refqualifier)

##### `impl Debug for RefQualifier`

- <span id="refqualifier-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RefQualifier`

##### `impl PartialEq for RefQualifier`

- <span id="refqualifier-partialeq-eq"></span>`fn eq(&self, other: &RefQualifier) -> bool` — [`RefQualifier`](#refqualifier)

##### `impl StructuralPartialEq for RefQualifier`

### `ExplicitObjectParameter`

```rust
enum ExplicitObjectParameter {
    ExplicitObjectParameter,
}
```

A named explicit object parameter.

#### Variants

- **`ExplicitObjectParameter`**

  this

#### Implementations

- <span id="explicitobjectparameter-starts-with"></span>`fn starts_with(byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for ExplicitObjectParameter`

- <span id="explicitobjectparameter-clone"></span>`fn clone(&self) -> ExplicitObjectParameter` — [`ExplicitObjectParameter`](#explicitobjectparameter)

##### `impl Debug for ExplicitObjectParameter`

- <span id="explicitobjectparameter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ExplicitObjectParameter`

##### `impl PartialEq for ExplicitObjectParameter`

- <span id="explicitobjectparameter-partialeq-eq"></span>`fn eq(&self, other: &ExplicitObjectParameter) -> bool` — [`ExplicitObjectParameter`](#explicitobjectparameter)

##### `impl StructuralPartialEq for ExplicitObjectParameter`

### `StandardBuiltinType`

```rust
enum StandardBuiltinType {
    Void,
    Wchar,
    Bool,
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Int128,
    Uint128,
    Float,
    Double,
    LongDouble,
    Float128,
    Ellipsis,
    DecimalFloat64,
    DecimalFloat128,
    DecimalFloat32,
    DecimalFloat16,
    BFloat16,
    Char32,
    Char16,
    Char8,
    Auto,
    Decltype,
    Nullptr,
    AccumShort,
    AccumUShort,
    Accum,
    AccumUnsigned,
    AccumLong,
    AccumULong,
    FractShort,
    FractUShort,
    Fract,
    FractUnsigned,
    FractLong,
    FractULong,
    SatAccumShort,
    SatAccumUShort,
    SatAccum,
    SatAccumUnsigned,
    SatAccumLong,
    SatAccumULong,
    SatFractShort,
    SatFractUShort,
    SatFract,
    SatFractUnsigned,
    SatFractLong,
    SatFractULong,
}
```

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

#### Variants

- **`Void`**

  void

- **`Wchar`**

  wchar_t

- **`Bool`**

  bool

- **`Char`**

  char

- **`SignedChar`**

  signed char

- **`UnsignedChar`**

  unsigned char

- **`Short`**

  short

- **`UnsignedShort`**

  unsigned short

- **`Int`**

  int

- **`UnsignedInt`**

  unsigned int

- **`Long`**

  long

- **`UnsignedLong`**

  unsigned long

- **`LongLong`**

  long long

- **`UnsignedLongLong`**

  unsigned long long

- **`Int128`**

  __int128

- **`Uint128`**

  unsigned __int128

- **`Float`**

  float

- **`Double`**

  double

- **`LongDouble`**

  long double

- **`Float128`**

  __float128

- **`Ellipsis`**

  ...

- **`DecimalFloat64`**

  decimal64

- **`DecimalFloat128`**

  decimal128

- **`DecimalFloat32`**

  decimal32

- **`DecimalFloat16`**

  half

- **`BFloat16`**

  std::bfloat16_t

- **`Char32`**

  char32_t

- **`Char16`**

  char16_t

- **`Char8`**

  char8_t

- **`Auto`**

  auto

- **`Decltype`**

  decltype(auto)

- **`Nullptr`**

  std::nullptr_t

- **`AccumShort`**

  short _Accum

- **`AccumUShort`**

  unsigned short _Accum

- **`Accum`**

  _Accum

- **`AccumUnsigned`**

  unsigned _Accum

- **`AccumLong`**

  long _Accum

- **`AccumULong`**

  unsigned long _Accum

- **`FractShort`**

  short _Fract

- **`FractUShort`**

  unsigned short _Fract

- **`Fract`**

  _Fract

- **`FractUnsigned`**

  unsigned _Fract

- **`FractLong`**

  long _Fract

- **`FractULong`**

  unsigned long _Fract

- **`SatAccumShort`**

  _Sat short _Accum

- **`SatAccumUShort`**

  _Sat unsigned short _Accum

- **`SatAccum`**

  _Sat _Accum

- **`SatAccumUnsigned`**

  _Sat unsigned _Accum

- **`SatAccumLong`**

  _Sat long _Accum

- **`SatAccumULong`**

  _Sat unsigned long _Accum

- **`SatFractShort`**

  _Sat short _Fract

- **`SatFractUShort`**

  _Sat unsigned short _Fract

- **`SatFract`**

  _Sat _Fract

- **`SatFractUnsigned`**

  _Sat unsigned _Fract

- **`SatFractLong`**

  _Sat long _Fract

- **`SatFractULong`**

  _Sat unsigned long _Fract

#### Implementations

- <span id="standardbuiltintype-starts-with"></span>`fn starts_with(byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for StandardBuiltinType`

- <span id="standardbuiltintype-clone"></span>`fn clone(&self) -> StandardBuiltinType` — [`StandardBuiltinType`](#standardbuiltintype)

##### `impl Debug for StandardBuiltinType`

- <span id="standardbuiltintype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StandardBuiltinType`

##### `impl PartialEq for StandardBuiltinType`

- <span id="standardbuiltintype-partialeq-eq"></span>`fn eq(&self, other: &StandardBuiltinType) -> bool` — [`StandardBuiltinType`](#standardbuiltintype)

##### `impl StructuralPartialEq for StandardBuiltinType`

### `ParametricBuiltinType`

```rust
enum ParametricBuiltinType {
    FloatN(isize),
    FloatNx(isize),
    SignedBitInt(isize),
    UnsignedBitInt(isize),
    SignedBitIntExpression(alloc::boxed::Box<Expression>),
    UnsignedBitIntExpression(alloc::boxed::Box<Expression>),
}
```

<builtin-type> ::= DF <number> _ # ISO/IEC TS 18661 binary floating point type _FloatN (N bits), C++23 std::floatN_t
               ::= DF <number> x # IEEE extended precision formats, C23 _FloatNx (N bits)
               ::= DB <number> _        # C23 signed _BitInt(N)
               ::= DB <instantiation-dependent expression> _ # C23 signed _BitInt(N)
               ::= DU <number> _        # C23 unsigned _BitInt(N)
               ::= DU <instantiation-dependent expression> _ # C23 unsigned _BitInt(N)

#### Variants

- **`FloatN`**

  _FloatN

- **`FloatNx`**

  _FloatNx

- **`SignedBitInt`**

  signed _BitInt(N)

- **`UnsignedBitInt`**

  unsigned _BitInt(N)

- **`SignedBitIntExpression`**

  signed _BitInt(expr)

- **`UnsignedBitIntExpression`**

  unsigned _BitInt(expr)

#### Trait Implementations

##### `impl Clone for ParametricBuiltinType`

- <span id="parametricbuiltintype-clone"></span>`fn clone(&self) -> ParametricBuiltinType` — [`ParametricBuiltinType`](#parametricbuiltintype)

##### `impl Debug for ParametricBuiltinType`

- <span id="parametricbuiltintype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParametricBuiltinType`

##### `impl PartialEq for ParametricBuiltinType`

- <span id="parametricbuiltintype-partialeq-eq"></span>`fn eq(&self, other: &ParametricBuiltinType) -> bool` — [`ParametricBuiltinType`](#parametricbuiltintype)

##### `impl StructuralPartialEq for ParametricBuiltinType`

### `BuiltinType`

```rust
enum BuiltinType {
    Standard(StandardBuiltinType),
    Parametric(ParametricBuiltinType),
    Extension(SourceName),
}
```

The `<builtin-type>` production.

#### Variants

- **`Standard`**

  A simple standards compliant builtin type.

- **`Parametric`**

  A standards compliant builtin type with a parameter, e.g. _BitInt(32).

- **`Extension`**

  A non-standard, vendor extension type.
  
  ```text
  <builtin-type> ::= u <source-name>   # vendor extended type
  ```

#### Trait Implementations

##### `impl Clone for BuiltinType`

- <span id="builtintype-clone"></span>`fn clone(&self) -> BuiltinType` — [`BuiltinType`](#builtintype)

##### `impl Debug for BuiltinType`

- <span id="builtintype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BuiltinType`

##### `impl GetLeafName for BuiltinType`

- <span id="builtintype-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, _: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for BuiltinType`

- <span id="builtintype-partialeq-eq"></span>`fn eq(&self, other: &BuiltinType) -> bool` — [`BuiltinType`](#builtintype)

##### `impl StructuralPartialEq for BuiltinType`

### `ExceptionSpec`

```rust
enum ExceptionSpec {
    NoExcept,
    Computed(Expression),
}
```

The `<exception-spec>` production.

<exception-spec> ::= Do                # non-throwing exception-specification (e.g., noexcept, throw())
                 ::= DO <expression> E # computed (instantiation-dependent) noexcept
                 ::= Dw <type>+ E      # dynamic exception specification with instantiation-dependent types

#### Variants

- **`NoExcept`**

  noexcept

- **`Computed`**

  noexcept(expression)

#### Trait Implementations

##### `impl Clone for ExceptionSpec`

- <span id="exceptionspec-clone"></span>`fn clone(&self) -> ExceptionSpec` — [`ExceptionSpec`](#exceptionspec)

##### `impl Debug for ExceptionSpec`

- <span id="exceptionspec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ExceptionSpec`

##### `impl PartialEq for ExceptionSpec`

- <span id="exceptionspec-partialeq-eq"></span>`fn eq(&self, other: &ExceptionSpec) -> bool` — [`ExceptionSpec`](#exceptionspec)

##### `impl StructuralPartialEq for ExceptionSpec`

### `Decltype`

```rust
enum Decltype {
    IdExpression(Expression),
    Expression(Expression),
}
```

The `<decltype>` production.

```text
<decltype> ::= Dt <expression> E
           ::= DT <expression> E
```

#### Variants

- **`IdExpression`**

  A `decltype` of an id-expression or class member access (C++0x).

- **`Expression`**

  A `decltype` of an expression (C++0x).

#### Trait Implementations

##### `impl Clone for Decltype`

- <span id="decltype-clone"></span>`fn clone(&self) -> Decltype` — [`Decltype`](#decltype)

##### `impl Debug for Decltype`

- <span id="decltype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Decltype`

##### `impl PartialEq for Decltype`

- <span id="decltype-partialeq-eq"></span>`fn eq(&self, other: &Decltype) -> bool` — [`Decltype`](#decltype)

##### `impl StructuralPartialEq for Decltype`

### `ClassEnumType`

```rust
enum ClassEnumType {
    Named(Name),
    ElaboratedStruct(Name),
    ElaboratedUnion(Name),
    ElaboratedEnum(Name),
}
```

The `<class-enum-type>` production.

```text
<class-enum-type> ::= <name>
                  ::= Ts <name>
                  ::= Tu <name>
                  ::= Te <name>
```

#### Variants

- **`Named`**

  A non-dependent type name, dependent type name, or dependent
  typename-specifier.

- **`ElaboratedStruct`**

  A dependent elaborated type specifier using 'struct' or 'class'.

- **`ElaboratedUnion`**

  A dependent elaborated type specifier using 'union'.

- **`ElaboratedEnum`**

  A dependent elaborated type specifier using 'enum'.

#### Trait Implementations

##### `impl Clone for ClassEnumType`

- <span id="classenumtype-clone"></span>`fn clone(&self) -> ClassEnumType` — [`ClassEnumType`](#classenumtype)

##### `impl Debug for ClassEnumType`

- <span id="classenumtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassEnumType`

##### `impl GetLeafName for ClassEnumType`

- <span id="classenumtype-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for ClassEnumType`

- <span id="classenumtype-partialeq-eq"></span>`fn eq(&self, other: &ClassEnumType) -> bool` — [`ClassEnumType`](#classenumtype)

##### `impl StructuralPartialEq for ClassEnumType`

### `ArrayType`

```rust
enum ArrayType {
    DimensionNumber(usize, TypeHandle),
    DimensionExpression(Expression, TypeHandle),
    NoDimension(TypeHandle),
}
```

The `<array-type>` production.

```text
<array-type> ::= A <positive dimension number> _ <element type>
             ::= A [<dimension expression>] _ <element type>
```

#### Variants

- **`DimensionNumber`**

  An array with a number-literal dimension.

- **`DimensionExpression`**

  An array with an expression for its dimension.

- **`NoDimension`**

  An array with no dimension.

#### Trait Implementations

##### `impl Clone for ArrayType`

- <span id="arraytype-clone"></span>`fn clone(&self) -> ArrayType` — [`ArrayType`](#arraytype)

##### `impl Debug for ArrayType`

- <span id="arraytype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArrayType`

##### `impl PartialEq for ArrayType`

- <span id="arraytype-partialeq-eq"></span>`fn eq(&self, other: &ArrayType) -> bool` — [`ArrayType`](#arraytype)

##### `impl StructuralPartialEq for ArrayType`

### `VectorType`

```rust
enum VectorType {
    DimensionNumber(usize, TypeHandle),
    DimensionExpression(Expression, TypeHandle),
}
```

The `<vector-type>` production.

```text
<vector-type> ::= Dv <number> _ <type>
              ::= Dv <expression> _ <type>
```

#### Variants

- **`DimensionNumber`**

  An vector with a number-literal dimension.

- **`DimensionExpression`**

  An vector with an expression for its dimension.

#### Trait Implementations

##### `impl Clone for VectorType`

- <span id="vectortype-clone"></span>`fn clone(&self) -> VectorType` — [`VectorType`](#vectortype)

##### `impl Debug for VectorType`

- <span id="vectortype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for VectorType`

##### `impl PartialEq for VectorType`

- <span id="vectortype-partialeq-eq"></span>`fn eq(&self, other: &VectorType) -> bool` — [`VectorType`](#vectortype)

##### `impl StructuralPartialEq for VectorType`

### `TemplateTemplateParamHandle`

```rust
enum TemplateTemplateParamHandle {
    WellKnown(WellKnownComponent),
    BackReference(usize),
}
```

A reference to a parsed `TemplateTemplateParam`.

#### Variants

- **`WellKnown`**

  A reference to a "well-known" component.

- **`BackReference`**

  A back-reference into the substitution table to a component we
  have already parsed.

#### Implementations

- <span id="templatetemplateparamhandle-back-reference"></span>`fn back_reference(&self) -> Option<usize>`

  If this is a `BackReference`, get its index.

#### Trait Implementations

##### `impl Clone for TemplateTemplateParamHandle`

- <span id="templatetemplateparamhandle-clone"></span>`fn clone(&self) -> TemplateTemplateParamHandle` — [`TemplateTemplateParamHandle`](#templatetemplateparamhandle)

##### `impl Debug for TemplateTemplateParamHandle`

- <span id="templatetemplateparamhandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TemplateTemplateParamHandle`

##### `impl GetLeafName for TemplateTemplateParamHandle`

- <span id="templatetemplateparamhandle-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for TemplateTemplateParamHandle`

- <span id="templatetemplateparamhandle-partialeq-eq"></span>`fn eq(&self, other: &TemplateTemplateParamHandle) -> bool` — [`TemplateTemplateParamHandle`](#templatetemplateparamhandle)

##### `impl StructuralPartialEq for TemplateTemplateParamHandle`

### `TemplateArg`

```rust
enum TemplateArg {
    Type(TypeHandle),
    Expression(Expression),
    SimpleExpression(ExprPrimary),
    ArgPack(alloc::vec::Vec<TemplateArg>),
}
```

A <template-arg> production.

```text
<template-arg> ::= <type>                # type or template
               ::= X <expression> E      # expression
               ::= <expr-primary>        # simple expressions
               ::= J <template-arg>* E   # argument pack
```

#### Variants

- **`Type`**

  A type or template.

- **`Expression`**

  An expression.

- **`SimpleExpression`**

  A simple expression.

- **`ArgPack`**

  An argument pack.

#### Trait Implementations

##### `impl Clone for TemplateArg`

- <span id="templatearg-clone"></span>`fn clone(&self) -> TemplateArg` — [`TemplateArg`](#templatearg)

##### `impl Debug for TemplateArg`

- <span id="templatearg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TemplateArg`

##### `impl PartialEq for TemplateArg`

- <span id="templatearg-partialeq-eq"></span>`fn eq(&self, other: &TemplateArg) -> bool` — [`TemplateArg`](#templatearg)

##### `impl StructuralPartialEq for TemplateArg`

### `Expression`

```rust
enum Expression {
    Unary(OperatorName, alloc::boxed::Box<Expression>),
    Binary(OperatorName, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>),
    Ternary(OperatorName, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>),
    PrefixInc(alloc::boxed::Box<Expression>),
    PrefixDec(alloc::boxed::Box<Expression>),
    Call(alloc::boxed::Box<Expression>, alloc::vec::Vec<Expression>),
    ConversionOne(TypeHandle, alloc::boxed::Box<Expression>),
    ConversionMany(TypeHandle, alloc::vec::Vec<Expression>),
    ConversionBraced(TypeHandle, alloc::vec::Vec<Expression>),
    BracedInitList(alloc::vec::Vec<Expression>),
    New(alloc::vec::Vec<Expression>, TypeHandle, Option<Initializer>),
    GlobalNew(alloc::vec::Vec<Expression>, TypeHandle, Option<Initializer>),
    NewArray(alloc::vec::Vec<Expression>, TypeHandle, Option<Initializer>),
    GlobalNewArray(alloc::vec::Vec<Expression>, TypeHandle, Option<Initializer>),
    Delete(alloc::boxed::Box<Expression>),
    GlobalDelete(alloc::boxed::Box<Expression>),
    DeleteArray(alloc::boxed::Box<Expression>),
    GlobalDeleteArray(alloc::boxed::Box<Expression>),
    DynamicCast(TypeHandle, alloc::boxed::Box<Expression>),
    StaticCast(TypeHandle, alloc::boxed::Box<Expression>),
    ConstCast(TypeHandle, alloc::boxed::Box<Expression>),
    ReinterpretCast(TypeHandle, alloc::boxed::Box<Expression>),
    TypeidType(TypeHandle),
    TypeidExpr(alloc::boxed::Box<Expression>),
    SizeofType(TypeHandle),
    SizeofExpr(alloc::boxed::Box<Expression>),
    AlignofType(TypeHandle),
    AlignofExpr(alloc::boxed::Box<Expression>),
    Noexcept(alloc::boxed::Box<Expression>),
    Subobject(SubobjectExpr),
    TemplateParam(TemplateParam),
    FunctionParam(FunctionParam),
    Member(alloc::boxed::Box<Expression>, MemberName),
    DerefMember(alloc::boxed::Box<Expression>, MemberName),
    PointerToMember(alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>),
    SizeofTemplatePack(TemplateParam),
    SizeofFunctionPack(FunctionParam),
    SizeofCapturedTemplatePack(alloc::vec::Vec<TemplateArg>),
    PackExpansion(alloc::boxed::Box<Expression>),
    Fold(FoldExpr),
    Throw(alloc::boxed::Box<Expression>),
    Rethrow,
    UnresolvedName(UnresolvedName),
    Primary(ExprPrimary),
}
```

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
                                                               freestanding dependent name (e.g., T::x),
                                                               objectless nonstatic member reference
              ::= <expr-primary>
```

#### Variants

- **`Unary`**

  A unary operator expression.

- **`Binary`**

  A binary operator expression.

- **`Ternary`**

  A ternary operator expression.

- **`PrefixInc`**

  A prefix `++`.

- **`PrefixDec`**

  A prefix `--`.

- **`Call`**

  A call with functor and arguments.

- **`ConversionOne`**

  A type conversion with one argument.

- **`ConversionMany`**

  A type conversion with many arguments.

- **`ConversionBraced`**

  A type conversion with many arguments.

- **`BracedInitList`**

  A braced init list expression.

- **`New`**

  The `new` operator.

- **`GlobalNew`**

  The global `::new` operator.

- **`NewArray`**

  The `new[]` operator.

- **`GlobalNewArray`**

  The global `::new[]` operator.

- **`Delete`**

  The `delete` operator.

- **`GlobalDelete`**

  The global `::delete` operator.

- **`DeleteArray`**

  The `delete[]` operator.

- **`GlobalDeleteArray`**

  The global `::delete[]` operator.

- **`DynamicCast`**

  `dynamic_cast<type> (expression)`

- **`StaticCast`**

  `static_cast<type> (expression)`

- **`ConstCast`**

  `const_cast<type> (expression)`

- **`ReinterpretCast`**

  `reinterpret_cast<type> (expression)`

- **`TypeidType`**

  `typeid (type)`

- **`TypeidExpr`**

  `typeid (expression)`

- **`SizeofType`**

  `sizeof (type)`

- **`SizeofExpr`**

  `sizeof (expression)`

- **`AlignofType`**

  `alignof (type)`

- **`AlignofExpr`**

  `alignof (expression)`

- **`Noexcept`**

  `noexcept (expression)`

- **`Subobject`**

  Subobject expression,

- **`TemplateParam`**

  A named template parameter.

- **`FunctionParam`**

  A function parameter.

- **`Member`**

  `expr.name`

- **`DerefMember`**

  `expr->name`

- **`PointerToMember`**

  `expr.*expr`

- **`SizeofTemplatePack`**

  `sizeof...(T)`, size of a template parameter pack.

- **`SizeofFunctionPack`**

  `sizeof...(parameter)`, size of a function parameter pack.

- **`SizeofCapturedTemplatePack`**

  `sizeof...(T)`, size of a captured template parameter pack from an alias
  template.

- **`PackExpansion`**

  `expression...`, pack expansion.

- **`Fold`**

  The fold expressions.

- **`Throw`**

  `throw expression`

- **`Rethrow`**

  `throw` with no operand

- **`UnresolvedName`**

  `f(p)`, `N::f(p)`, `::f(p)`, freestanding dependent name (e.g., `T::x`),
  objectless nonstatic member reference.

- **`Primary`**

  An `<expr-primary>` production.

#### Implementations

- <span id="expression-demangle-as-subexpr"></span>`fn demangle_as_subexpr<'subs, 'prev, 'ctx, W>(self: &'subs Self, ctx: &'ctx mut DemangleContext<'subs, W>, scope: Option<ArgScopeStack<'prev, 'subs>>) -> fmt::Result` — [`ArgScopeStack`](#argscopestack)

#### Trait Implementations

##### `impl Clone for Expression`

- <span id="expression-clone"></span>`fn clone(&self) -> Expression` — [`Expression`](#expression)

##### `impl Debug for Expression`

- <span id="expression-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Expression`

##### `impl PartialEq for Expression`

- <span id="expression-partialeq-eq"></span>`fn eq(&self, other: &Expression) -> bool` — [`Expression`](#expression)

##### `impl StructuralPartialEq for Expression`

### `UnresolvedName`

```rust
enum UnresolvedName {
    Name(BaseUnresolvedName),
    Global(BaseUnresolvedName),
    Nested1(UnresolvedTypeHandle, alloc::vec::Vec<UnresolvedQualifierLevel>, BaseUnresolvedName),
    Nested2(alloc::vec::Vec<UnresolvedQualifierLevel>, BaseUnresolvedName),
    GlobalNested2(alloc::vec::Vec<UnresolvedQualifierLevel>, BaseUnresolvedName),
}
```

The `<unresolved-name>` production.

```text
<unresolved-name> ::= [gs] <base-unresolved-name>

                  ::= sr <unresolved-type> <base-unresolved-name>

                  ::= srN <unresolved-type> <unresolved-qualifier-level>+ E <base-unresolved-name>

                  ::= [gs] sr <unresolved-qualifier-level>+ E <base-unresolved-name>
                         A::x, N::y, A<T>::z; "gs" means leading "::"
```

#### Variants

- **`Name`**

  `x`

- **`Global`**

  `::x`

- **`Nested1`**

  `T::x`  or `decltype(p)::x` or `T::N::x` or `decltype(p)::N::x`

- **`Nested2`**

  `A::x` or `N::y` or `A<T>::z`

- **`GlobalNested2`**

  `::A::x` or `::N::y` or `::A<T>::z`

#### Trait Implementations

##### `impl Clone for UnresolvedName`

- <span id="unresolvedname-clone"></span>`fn clone(&self) -> UnresolvedName` — [`UnresolvedName`](#unresolvedname)

##### `impl Debug for UnresolvedName`

- <span id="unresolvedname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnresolvedName`

##### `impl PartialEq for UnresolvedName`

- <span id="unresolvedname-partialeq-eq"></span>`fn eq(&self, other: &UnresolvedName) -> bool` — [`UnresolvedName`](#unresolvedname)

##### `impl StructuralPartialEq for UnresolvedName`

### `UnresolvedType`

```rust
enum UnresolvedType {
    Template(TemplateParam, Option<TemplateArgs>),
    Decltype(Decltype),
}
```

The `<unresolved-type>` production.

```text
<unresolved-type> ::= <template-param> [ <template-args> ]  # T:: or T<X,Y>::
                  ::= <decltype>                            # decltype(p)::
                  ::= <substitution>
```

#### Variants

- **`Template`**

  An unresolved template type.

- **`Decltype`**

  An unresolved `decltype`.

#### Trait Implementations

##### `impl Clone for UnresolvedType`

- <span id="unresolvedtype-clone"></span>`fn clone(&self) -> UnresolvedType` — [`UnresolvedType`](#unresolvedtype)

##### `impl Debug for UnresolvedType`

- <span id="unresolvedtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnresolvedType`

##### `impl PartialEq for UnresolvedType`

- <span id="unresolvedtype-partialeq-eq"></span>`fn eq(&self, other: &UnresolvedType) -> bool` — [`UnresolvedType`](#unresolvedtype)

##### `impl StructuralPartialEq for UnresolvedType`

### `UnresolvedTypeHandle`

```rust
enum UnresolvedTypeHandle {
    WellKnown(WellKnownComponent),
    BackReference(usize),
}
```

A reference to a parsed `<unresolved-type>` production.

#### Variants

- **`WellKnown`**

  A reference to a "well-known" component.

- **`BackReference`**

  A back-reference into the substitution table to a component we
  have already parsed.

#### Implementations

- <span id="unresolvedtypehandle-back-reference"></span>`fn back_reference(&self) -> Option<usize>`

  If this is a `BackReference`, get its index.

#### Trait Implementations

##### `impl Clone for UnresolvedTypeHandle`

- <span id="unresolvedtypehandle-clone"></span>`fn clone(&self) -> UnresolvedTypeHandle` — [`UnresolvedTypeHandle`](#unresolvedtypehandle)

##### `impl Debug for UnresolvedTypeHandle`

- <span id="unresolvedtypehandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnresolvedTypeHandle`

##### `impl GetLeafName for UnresolvedTypeHandle`

- <span id="unresolvedtypehandle-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for UnresolvedTypeHandle`

- <span id="unresolvedtypehandle-partialeq-eq"></span>`fn eq(&self, other: &UnresolvedTypeHandle) -> bool` — [`UnresolvedTypeHandle`](#unresolvedtypehandle)

##### `impl StructuralPartialEq for UnresolvedTypeHandle`

### `BaseUnresolvedName`

```rust
enum BaseUnresolvedName {
    Name(SimpleId),
    Operator(OperatorName, Option<TemplateArgs>),
    Destructor(DestructorName),
}
```

The `<base-unresolved-name>` production.

```text
<base-unresolved-name> ::= <simple-id>                        # unresolved name
                       ::= on <operator-name>                 # unresolved operator-function-id
                       ::= on <operator-name> <template-args> # unresolved operator template-id
                       ::= dn <destructor-name>               # destructor or pseudo-destructor;
                                                              e.g. ~X or ~X<N-1>
```

#### Variants

- **`Name`**

  An unresolved name.

- **`Operator`**

  An unresolved function or template function name.

- **`Destructor`**

  An unresolved destructor name.

#### Trait Implementations

##### `impl Clone for BaseUnresolvedName`

- <span id="baseunresolvedname-clone"></span>`fn clone(&self) -> BaseUnresolvedName` — [`BaseUnresolvedName`](#baseunresolvedname)

##### `impl Debug for BaseUnresolvedName`

- <span id="baseunresolvedname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BaseUnresolvedName`

##### `impl PartialEq for BaseUnresolvedName`

- <span id="baseunresolvedname-partialeq-eq"></span>`fn eq(&self, other: &BaseUnresolvedName) -> bool` — [`BaseUnresolvedName`](#baseunresolvedname)

##### `impl StructuralPartialEq for BaseUnresolvedName`

### `DestructorName`

```rust
enum DestructorName {
    Unresolved(UnresolvedTypeHandle),
    Name(SimpleId),
}
```

The `<destructor-name>` production.

```text
<destructor-name> ::= <unresolved-type> # e.g., ~T or ~decltype(f())
                  ::= <simple-id>       # e.g., ~A<2*N>
```

#### Variants

- **`Unresolved`**

  A destructor for an unresolved type.

- **`Name`**

  A destructor for a resolved type name.

#### Trait Implementations

##### `impl Clone for DestructorName`

- <span id="destructorname-clone"></span>`fn clone(&self) -> DestructorName` — [`DestructorName`](#destructorname)

##### `impl Debug for DestructorName`

- <span id="destructorname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DestructorName`

##### `impl PartialEq for DestructorName`

- <span id="destructorname-partialeq-eq"></span>`fn eq(&self, other: &DestructorName) -> bool` — [`DestructorName`](#destructorname)

##### `impl StructuralPartialEq for DestructorName`

### `ExprPrimary`

```rust
enum ExprPrimary {
    Literal(TypeHandle, usize, usize),
    External(MangledName),
}
```

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

#### Variants

- **`Literal`**

  A type literal.

- **`External`**

  An external name.

#### Trait Implementations

##### `impl Clone for ExprPrimary`

- <span id="exprprimary-clone"></span>`fn clone(&self) -> ExprPrimary` — [`ExprPrimary`](#exprprimary)

##### `impl Debug for ExprPrimary`

- <span id="exprprimary-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ExprPrimary`

##### `impl PartialEq for ExprPrimary`

- <span id="exprprimary-partialeq-eq"></span>`fn eq(&self, other: &ExprPrimary) -> bool` — [`ExprPrimary`](#exprprimary)

##### `impl StructuralPartialEq for ExprPrimary`

### `LocalName`

```rust
enum LocalName {
    Relative(alloc::boxed::Box<Encoding>, Option<alloc::boxed::Box<Name>>, Option<Discriminator>),
    Default(alloc::boxed::Box<Encoding>, Option<usize>, alloc::boxed::Box<Name>),
}
```

The `<local-name>` production.

```text
<local-name> := Z <function encoding> E <entity name> [<discriminator>]
             := Z <function encoding> E s [<discriminator>]
             := Z <function encoding> Ed [ <parameter number> ] _ <entity name>
```

#### Variants

- **`Relative`**

  The mangling of the enclosing function, the mangling of the entity
  relative to the function, and an optional discriminator.

- **`Default`**

  A default argument in a class definition.

#### Trait Implementations

##### `impl Clone for LocalName`

- <span id="localname-clone"></span>`fn clone(&self) -> LocalName` — [`LocalName`](#localname)

##### `impl Debug for LocalName`

- <span id="localname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LocalName`

##### `impl GetLeafName for LocalName`

- <span id="localname-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl GetTemplateArgs for LocalName`

- <span id="localname-gettemplateargs-get-template-args"></span>`fn get_template_args<'a>(self: &'a Self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs>` — [`TemplateArgs`](#templateargs)

##### `impl PartialEq for LocalName`

- <span id="localname-partialeq-eq"></span>`fn eq(&self, other: &LocalName) -> bool` — [`LocalName`](#localname)

##### `impl StructuralPartialEq for LocalName`

### `Substitution`

```rust
enum Substitution {
    BackReference(usize),
    WellKnown(WellKnownComponent),
}
```

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

#### Variants

- **`BackReference`**

  A reference to an entity that already occurred, ie the `S_` and `S
  <seq-id> _` forms.

- **`WellKnown`**

  A well-known substitution component. These are the components that do
  not appear in the substitution table, but have abbreviations specified
  directly in the grammar.

#### Trait Implementations

##### `impl Clone for Substitution`

- <span id="substitution-clone"></span>`fn clone(&self) -> Substitution` — [`Substitution`](#substitution)

##### `impl Debug for Substitution`

- <span id="substitution-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Substitution`

##### `impl PartialEq for Substitution`

- <span id="substitution-partialeq-eq"></span>`fn eq(&self, other: &Substitution) -> bool` — [`Substitution`](#substitution)

##### `impl StructuralPartialEq for Substitution`

### `WellKnownComponent`

```rust
enum WellKnownComponent {
    Std,
    StdAllocator,
    StdString1,
    StdString2,
    StdIstream,
    StdOstream,
    StdIostream,
}
```

The `<substitution>` variants that are encoded directly in the grammar,
rather than as back references to other components in the substitution
table.

#### Variants

- **`Std`**

  std

- **`StdAllocator`**

  std::allocator

- **`StdString1`**

  std::basic_string

- **`StdString2`**

  std::string

- **`StdIstream`**

  std::basic_istream<char, std::char_traits<char> >

- **`StdOstream`**

  std::ostream

- **`StdIostream`**

  std::basic_iostream<char, std::char_traits<char> >

#### Implementations

- <span id="wellknowncomponent-starts-with"></span>`fn starts_with(byte: u8) -> bool`

#### Trait Implementations

##### `impl ArgScope for WellKnownComponent`

- <span id="wellknowncomponent-argscope-leaf-name"></span>`fn leaf_name(self: &'a Self) -> Result<LeafName<'a>>` — [`Result`](../error/index.md#result), [`LeafName`](#leafname)

- <span id="wellknowncomponent-argscope-get-template-arg"></span>`fn get_template_arg(self: &'a Self, _: usize) -> Result<(&'a TemplateArg, &'a TemplateArgs)>` — [`Result`](../error/index.md#result), [`TemplateArg`](#templatearg), [`TemplateArgs`](#templateargs)

- <span id="wellknowncomponent-argscope-get-function-arg"></span>`fn get_function_arg(self: &'a Self, _: usize) -> Result<&'a Type>` — [`Result`](../error/index.md#result), [`Type`](#type)

##### `impl Clone for WellKnownComponent`

- <span id="wellknowncomponent-clone"></span>`fn clone(&self) -> WellKnownComponent` — [`WellKnownComponent`](#wellknowncomponent)

##### `impl Debug for WellKnownComponent`

- <span id="wellknowncomponent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W> DemangleAsLeaf for WellKnownComponent`

- <span id="wellknowncomponent-demangleasleaf-demangle-as-leaf"></span>`fn demangle_as_leaf<'me, 'ctx>(self: &'me Self, ctx: &'ctx mut DemangleContext<'subs, W>) -> fmt::Result`

##### `impl Eq for WellKnownComponent`

##### `impl GetLeafName for WellKnownComponent`

- <span id="wellknowncomponent-getleafname-get-leaf-name"></span>`fn get_leaf_name(self: &'a Self, _: &'a SubstitutionTable) -> Option<LeafName<'a>>` — [`LeafName`](#leafname)

##### `impl PartialEq for WellKnownComponent`

- <span id="wellknowncomponent-partialeq-eq"></span>`fn eq(&self, other: &WellKnownComponent) -> bool` — [`WellKnownComponent`](#wellknowncomponent)

##### `impl StructuralPartialEq for WellKnownComponent`

### `SpecialName`

```rust
enum SpecialName {
    VirtualTable(TypeHandle),
    Vtt(TypeHandle),
    Typeinfo(TypeHandle),
    TypeinfoName(TypeHandle),
    VirtualOverrideThunk(CallOffset, alloc::boxed::Box<Encoding>),
    VirtualOverrideThunkCovariant(CallOffset, CallOffset, alloc::boxed::Box<Encoding>),
    Guard(Name),
    GuardTemporary(Name, usize),
    ConstructionVtable(TypeHandle, usize, TypeHandle),
    TypeinfoFunction(TypeHandle),
    TlsInit(Name),
    TlsWrapper(Name),
    JavaResource(alloc::vec::Vec<ResourceName>),
    TransactionClone(alloc::boxed::Box<Encoding>),
    NonTransactionClone(alloc::boxed::Box<Encoding>),
}
```

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
    base is the nominal target function of thunk

<special-name> ::= Tc <call-offset> <call-offset> <base encoding>
    base is the nominal target function of thunk
    first call-offset is 'this' adjustment
    second call-offset is result adjustment
```

### 5.1.4.4 Guard Variables

```text
<special-name> ::= GV <object name> # Guard variable for one-time initialization
    No <type>
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

#### Variants

- **`VirtualTable`**

  A virtual table.

- **`Vtt`**

  A VTT structure (construction vtable index).

- **`Typeinfo`**

  A typeinfo structure.

- **`TypeinfoName`**

  A typeinfo name (null-terminated byte string).

- **`VirtualOverrideThunk`**

  A virtual override thunk.

- **`VirtualOverrideThunkCovariant`**

  A virtual override thunk with a covariant return type.

- **`Guard`**

  An initialization guard for some static storage.

- **`GuardTemporary`**

  A temporary used in the initialization of a static storage and promoted
  to a static lifetime.

- **`ConstructionVtable`**

  A construction vtable structure.

- **`TypeinfoFunction`**

  A typeinfo function.

- **`TlsInit`**

  A TLS initialization function.

- **`TlsWrapper`**

  A TLS wrapper function.

- **`JavaResource`**

  A Java Resource.

- **`TransactionClone`**

  A function declared transaction-safe

- **`NonTransactionClone`**

  A function declared non-transaction-safe

#### Trait Implementations

##### `impl Clone for SpecialName`

- <span id="specialname-clone"></span>`fn clone(&self) -> SpecialName` — [`SpecialName`](#specialname)

##### `impl Debug for SpecialName`

- <span id="specialname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SpecialName`

##### `impl PartialEq for SpecialName`

- <span id="specialname-partialeq-eq"></span>`fn eq(&self, other: &SpecialName) -> bool` — [`SpecialName`](#specialname)

##### `impl StructuralPartialEq for SpecialName`

### `FoldExpr`

```rust
enum FoldExpr {
    UnaryLeft(SimpleOperatorName, alloc::boxed::Box<Expression>),
    UnaryRight(SimpleOperatorName, alloc::boxed::Box<Expression>),
    BinaryLeft(SimpleOperatorName, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>),
    BinaryRight(SimpleOperatorName, alloc::boxed::Box<Expression>, alloc::boxed::Box<Expression>),
}
```

The fold expressions.

These are not separate productions in the grammar but our code is cleaner
if we handle them all together.

<expression>  ::= ...
              ::= fl <binary operator-name> <expression>       # (... operator expression), unary left fold
              ::= fr <binary operator-name> <expression>       # (expression operator ...), unary right fold
              ::= fL <binary operator-name> <expression> <expression> # (expression operator ... operator expression), binary left fold
              ::= fR <binary operator-name> <expression> <expression> # (expression operator ... operator expression), binary right fold
              ::= ...

#### Variants

- **`UnaryLeft`**

  (...+<expr>)

- **`UnaryRight`**

  (<expr>+...)

- **`BinaryLeft`**

  (<expr1>+...+<expr2>)

- **`BinaryRight`**

  (<expr1>+...+<expr2>)

#### Trait Implementations

##### `impl Clone for FoldExpr`

- <span id="foldexpr-clone"></span>`fn clone(&self) -> FoldExpr` — [`FoldExpr`](#foldexpr)

##### `impl Debug for FoldExpr`

- <span id="foldexpr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FoldExpr`

##### `impl PartialEq for FoldExpr`

- <span id="foldexpr-partialeq-eq"></span>`fn eq(&self, other: &FoldExpr) -> bool` — [`FoldExpr`](#foldexpr)

##### `impl StructuralPartialEq for FoldExpr`

## Traits

### `GetTemplateArgs`

```rust
trait GetTemplateArgs { ... }
```

Determine whether this AST node is an instantiated[*] template function, and
get its concrete template arguments.

[*] Note that we will never see an abstract, un-instantiated template
function, since they don't end up in object files and don't get mangled
names.

#### Required Methods

- `fn get_template_args<'a>(self: &'a Self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs>`

  Returns `Some` if this is a template function, `None` otherwise.

#### Implementors

- [`LocalName`](#localname)
- [`Name`](#name)
- [`NestedName`](#nestedname)
- [`PrefixHandle`](#prefixhandle)
- [`Prefix`](#prefix)
- [`TypeHandle`](#typehandle)
- [`Type`](#type)

### `GetLeafName<'a>`

```rust
trait GetLeafName<'a> { ... }
```

Determine whether this AST node is some kind (potentially namespaced) name
and if so get its leaf name.

#### Required Methods

- `fn get_leaf_name(self: &'a Self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>`

#### Implementors

- [`BuiltinType`](#builtintype)
- [`ClassEnumType`](#classenumtype)
- [`ClosureTypeName`](#closuretypename)
- [`DataMemberPrefix`](#datamemberprefix)
- [`LocalName`](#localname)
- [`Name`](#name)
- [`NestedName`](#nestedname)
- [`NonSubstitution`](#nonsubstitution)
- [`PrefixHandle`](#prefixhandle)
- [`Prefix`](#prefix)
- [`QualifiedBuiltin`](#qualifiedbuiltin)
- [`TemplateTemplateParamHandle`](#templatetemplateparamhandle)
- [`TypeHandle`](#typehandle)
- [`Type`](#type)
- [`UnqualifiedName`](#unqualifiedname)
- [`UnresolvedTypeHandle`](#unresolvedtypehandle)
- [`UnscopedName`](#unscopedname)
- [`UnscopedTemplateNameHandle`](#unscopedtemplatenamehandle)
- [`UnscopedTemplateName`](#unscopedtemplatename)
- [`WellKnownComponent`](#wellknowncomponent)

### `IsCtorDtorConversion`

```rust
trait IsCtorDtorConversion { ... }
```

Determine whether this AST node is a constructor, destructor, or conversion
function.

#### Required Methods

- `fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool`

#### Implementors

- [`Name`](#name)
- [`NestedName`](#nestedname)
- [`PrefixHandle`](#prefixhandle)
- [`Prefix`](#prefix)
- [`UnqualifiedName`](#unqualifiedname)
- [`UnscopedName`](#unscopedname)

### `ArgScope<'me, 'ctx>`

```rust
trait ArgScope<'me, 'ctx>: fmt::Debug { ... }
```

When formatting a mangled symbol's parsed AST as a demangled symbol, we need
to resolve indirect references to template and function arguments with
direct `TemplateArg` and `Type` references respectively.

Note that which set of arguments are implicitly referenced change as we
enter and leave different functions' scope. One might usually use de Brujin
indices to keep arguments within scopes separated from each other, but the
Itanium C++ ABI does not allow us the luxury. AFAIK, when the ABI was first
drafted, C++ did not have lambdas, and the issue did not come up at all
since a function simply couldn't refer to the types of closed over
variables.

This trait is implemented by anything that can potentially resolve arguments
for us.

#### Required Methods

- `fn leaf_name(self: &'me Self) -> Result<LeafName<'ctx>>`

  Get the current scope's leaf name.

- `fn get_template_arg(self: &'me Self, index: usize) -> Result<(&'ctx TemplateArg, &'ctx TemplateArgs)>`

  Get the current scope's `index`th template argument.

- `fn get_function_arg(self: &'me Self, index: usize) -> Result<&'ctx Type>`

  Get the current scope's `index`th function argument's type.

#### Implementors

- [`ClosureTypeName`](#closuretypename)
- [`SourceName`](#sourcename)
- [`TemplateArgs`](#templateargs)
- [`UnnamedTypeName`](#unnamedtypename)
- [`WellKnownComponent`](#wellknowncomponent)
- `Option<ArgScopeStack<'prev, 'subs>>`

### `ArgScopeStackExt<'prev, 'subs>`

```rust
trait ArgScopeStackExt<'prev, 'subs>: Copy { ... }
```

When we first begin demangling, we haven't entered any function or template
demangling scope and we don't have any useful `ArgScopeStack`. Therefore, we
are never actually dealing with `ArgScopeStack` directly in practice, but
always an `Option<ArgScopeStack>` instead. Nevertheless, we want to define
useful methods on `Option<ArgScopeStack>`.

A custom "extension" trait with exactly one implementor: Rust's principled
monkey patching!

#### Required Methods

- `fn push(self: &'prev Self, item: &'subs dyn ArgScope<'subs, 'subs>) -> Option<ArgScopeStack<'prev, 'subs>>`

  Push a new `ArgScope` onto this `ArgScopeStack` and return the new

#### Implementors

- `Option<ArgScopeStack<'prev, 'subs>>`

### `DemangleAsLeaf<'subs, W>`

```rust
trait DemangleAsLeaf<'subs, W>
where
    W: 'subs + DemangleWrite { ... }
```

Demangle this thing in the leaf name position.

For most things this should be the same as its `Demangle`
implementation. For `WellKnownComponent`s we need to strip the embedded
`std::` namespace prefix.

#### Required Methods

- `fn demangle_as_leaf<'me, 'ctx>(self: &'me Self, ctx: &'ctx mut DemangleContext<'subs, W>) -> fmt::Result`

#### Implementors

- [`LeafName`](#leafname)
- [`UnnamedTypeName`](#unnamedtypename)
- [`WellKnownComponent`](#wellknowncomponent)

## Functions

### `consume`

```rust
fn consume<'a>(expected: &[u8], input: crate::index_str::IndexStr<'a>) -> crate::error::Result<crate::index_str::IndexStr<'a>>
```

Expect and consume the given byte str, and return the advanced `IndexStr` if
we saw the expectation. Otherwise return an error of kind
`error::Error::UnexpectedText` if the input doesn't match, or
`error::Error::UnexpectedEnd` if it isn't long enough.

### `one_or_more`

```rust
fn one_or_more<'a, 'b, P>(ctx: &'a ParseContext, subs: &'a mut crate::subs::SubstitutionTable, input: crate::index_str::IndexStr<'b>) -> crate::error::Result<(alloc::vec::Vec<P>, crate::index_str::IndexStr<'b>)>
where
    P: Parse
```

### `zero_or_more`

```rust
fn zero_or_more<'a, 'b, P>(ctx: &'a ParseContext, subs: &'a mut crate::subs::SubstitutionTable, input: crate::index_str::IndexStr<'b>) -> crate::error::Result<(alloc::vec::Vec<P>, crate::index_str::IndexStr<'b>)>
where
    P: Parse
```

### `parse_number`

```rust
fn parse_number(base: u32, allow_signed: bool, input: crate::index_str::IndexStr<'_>) -> crate::error::Result<(isize, crate::index_str::IndexStr<'_>)>
```

Parse a number with the given `base`. Do not allow negative numbers
(prefixed with an 'n' instead of a '-') if `allow_signed` is false.

## Type Aliases

### `Number`

```rust
type Number = isize;
```

The `<number>` production.

```text
<number> ::= [n] <non-negative decimal integer>
```

## Macros

### `try_recurse!`

### `try_begin_parse!`

Performs the two operations that begin every parse:

1. Keeps track of recursion levels and early returns with an error if there
   is too much recursion.

2. Automatically log start and end parsing in an s-expression format, when the
   `logging` feature is enabled.

### `try_begin_demangle!`

Automatically log start and end demangling in an s-expression format, when
the `logging` feature is enabled.

### `try_begin_demangle_as_inner!`

Automatically log start and end demangling in an s-expression format, when
the `logging` feature is enabled.

### `inner_barrier!`

The inner stack allows passing AST nodes down deeper into the tree so that
nodes that logically precede something (e.g. PointerRef) can show up after
that thing in the demangled output. What's on the stack may not always be
intended for the first node that looks at the stack to grab, though.

Consider a function with template arguments and parameters, f<T>(a).
The function parameters logically precede the template arguments in the AST,
but they must be reversed in the output. The parameters end up on the inner
stack before processing the template argument nodes. If we're not careful,
a node inside the template arguments might pick the function parameters
off of the inner stack!

To solve this, certain nodes act as "inner barriers". By using this macro,
they set the existing inner stack aside and replace it with an empty stack
while visiting their children. This allows these barrier nodes to have
completely self-contained children.

### `reference_newtype!`

### `define_handle!`

Define a handle to a AST type that lives inside the substitution table. A
handle is always either an index into the substitution table, or it is a
reference to a "well-known" component.

This declares:

- The enum of either a back reference into the substitution table or a
  reference to a "well-known" component
- a `Demangle` impl that proxies to the appropriate `Substitutable` in the
  `SubstitutionTable`

### `define_vocabulary!`

Define a "vocabulary" nonterminal, something like `OperatorName` or
`CtorDtorName` that's basically a big list of constant strings.

This declares:

- the enum itself
- a `Parse` impl
- a `Demangle` impl

See the definition of `CTorDtorName` for an example of its use.

Optionally, a piece of user data can be attached to the definitions
and be returned by a generated accessor. See `SimpleOperatorName` for
an example.

