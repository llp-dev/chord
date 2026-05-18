**syn > buffer**

# Module: buffer

## Contents

**Structs**

- [`Cursor`](#cursor) - A cheaply copyable cursor into a `TokenBuffer`.
- [`TokenBuffer`](#tokenbuffer) - A buffer that can be efficiently traversed multiple times, unlike

**Enums**

- [`Entry`](#entry) - Internal type which is used instead of `TokenTree` to represent a token tree

**Functions**

- [`cmp_assuming_same_buffer`](#cmp_assuming_same_buffer)
- [`open_span_of_group`](#open_span_of_group)
- [`same_buffer`](#same_buffer)
- [`same_scope`](#same_scope)
- [`start_of_buffer`](#start_of_buffer)

---

## syn::buffer::Cursor

*Struct*

A cheaply copyable cursor into a `TokenBuffer`.

This cursor holds a shared reference into the immutable data which is used
internally to represent a `TokenStream`, and can be efficiently manipulated
and copied around.

An empty `Cursor` can be created directly, or one may create a `TokenBuffer`
object and get a cursor to its first token with `begin()`.

**Generic Parameters:**
- 'a

**Fields:**
- `ptr: *const Entry`
- `scope: *const Entry`
- `marker: core::marker::PhantomData<&'a Entry>`

**Methods:**

- `fn empty() -> Self` - Creates a cursor referencing a static empty TokenStream.
- `fn create(ptr: *const Entry, scope: *const Entry) -> Self` - This create method intelligently exits non-explicitly-entered
- `fn entry(self: Self) -> &'a Entry` - Get the current entry.
- `fn bump_ignore_group(self: Self) -> Cursor<'a>` - Bump the cursor to point at the next token after the current one. This
- `fn ignore_none(self: & mut Self)` - While the cursor is looking at a `None`-delimited group, move it to look
- `fn eof(self: Self) -> bool` - Checks whether the cursor is currently pointing at the end of its valid
- `fn ident(self: Self) -> Option<(Ident, Cursor<'a>)>` - If the cursor is pointing at a `Ident`, returns it along with a cursor
- `fn punct(self: Self) -> Option<(Punct, Cursor<'a>)>` - If the cursor is pointing at a `Punct`, returns it along with a cursor
- `fn literal(self: Self) -> Option<(Literal, Cursor<'a>)>` - If the cursor is pointing at a `Literal`, return it along with a cursor
- `fn lifetime(self: Self) -> Option<(Lifetime, Cursor<'a>)>` - If the cursor is pointing at a `Lifetime`, returns it along with a
- `fn group(self: Self, delim: Delimiter) -> Option<(Cursor<'a>, DelimSpan, Cursor<'a>)>` - If the cursor is pointing at a `Group` with the given delimiter, returns
- `fn any_group(self: Self) -> Option<(Cursor<'a>, Delimiter, DelimSpan, Cursor<'a>)>` - If the cursor is pointing at a `Group`, returns a cursor into the group
- `fn any_group_token(self: Self) -> Option<(Group, Cursor<'a>)>`
- `fn token_stream(self: Self) -> TokenStream` - Copies all remaining tokens visible from this cursor into a
- `fn token_tree(self: Self) -> Option<(TokenTree, Cursor<'a>)>` - If the cursor is pointing at a `TokenTree`, returns it along with a
- `fn span(self: Self) -> Span` - Returns the `Span` of the current token, or `Span::call_site()` if this
- `fn prev_span(self: Self) -> Span` - Returns the `Span` of the token immediately prior to the position of
- `fn skip(self: Self) -> Option<Cursor<'a>>` - Skip over the next token that is not a None-delimited group, without
- `fn scope_delimiter(self: Self) -> Delimiter`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`



## syn::buffer::Entry

*Enum*

Internal type which is used instead of `TokenTree` to represent a token tree
within a `TokenBuffer`.

**Variants:**
- `Group(proc_macro2::Group, usize)`
- `Ident(proc_macro2::Ident)`
- `Punct(proc_macro2::Punct)`
- `Literal(proc_macro2::Literal)`
- `End(isize, isize)`



## syn::buffer::TokenBuffer

*Struct*

A buffer that can be efficiently traversed multiple times, unlike
`TokenStream` which requires a deep copy in order to traverse more than
once.

**Fields:**
- `entries: alloc::boxed::Box<[Entry]>`

**Methods:**

- `fn recursive_new(entries: & mut Vec<Entry>, stream: TokenStream)`
- `fn new(stream: proc_macro::TokenStream) -> Self` - Creates a `TokenBuffer` containing all the tokens from the input
- `fn new2(stream: TokenStream) -> Self` - Creates a `TokenBuffer` containing all the tokens from the input
- `fn begin(self: &Self) -> Cursor` - Creates a cursor referencing the first token in the buffer and able to



## syn::buffer::cmp_assuming_same_buffer

*Function*

```rust
fn cmp_assuming_same_buffer(a: Cursor, b: Cursor) -> core::cmp::Ordering
```



## syn::buffer::open_span_of_group

*Function*

```rust
fn open_span_of_group(cursor: Cursor) -> proc_macro2::Span
```



## syn::buffer::same_buffer

*Function*

```rust
fn same_buffer(a: Cursor, b: Cursor) -> bool
```



## syn::buffer::same_scope

*Function*

```rust
fn same_scope(a: Cursor, b: Cursor) -> bool
```



## syn::buffer::start_of_buffer

*Function*

```rust
fn start_of_buffer(cursor: Cursor) -> *const Entry
```



