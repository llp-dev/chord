*[syn](../index.md) / [buffer](index.md)*

---

# Module `buffer`

A stably addressed token buffer supporting efficient traversal based on a
cheaply copyable cursor.

## Contents

- [Structs](#structs)
  - [`TokenBuffer`](#tokenbuffer)
  - [`Cursor`](#cursor)
- [Enums](#enums)
  - [`Entry`](#entry)
- [Functions](#functions)
  - [`same_scope`](#same-scope)
  - [`same_buffer`](#same-buffer)
  - [`start_of_buffer`](#start-of-buffer)
  - [`cmp_assuming_same_buffer`](#cmp-assuming-same-buffer)
  - [`open_span_of_group`](#open-span-of-group)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokenBuffer`](#tokenbuffer) | struct | A buffer that can be efficiently traversed multiple times, unlike `TokenStream` which requires a deep copy in order to traverse more than once. |
| [`Cursor`](#cursor) | struct | A cheaply copyable cursor into a `TokenBuffer`. |
| [`Entry`](#entry) | enum | Internal type which is used instead of `TokenTree` to represent a token tree within a `TokenBuffer`. |
| [`same_scope`](#same-scope) | fn |  |
| [`same_buffer`](#same-buffer) | fn |  |
| [`start_of_buffer`](#start-of-buffer) | fn |  |
| [`cmp_assuming_same_buffer`](#cmp-assuming-same-buffer) | fn |  |
| [`open_span_of_group`](#open-span-of-group) | fn |  |

## Structs

### `TokenBuffer`

```rust
struct TokenBuffer {
    entries: alloc::boxed::Box<[Entry]>,
}
```

A buffer that can be efficiently traversed multiple times, unlike
`TokenStream` which requires a deep copy in order to traverse more than
once.

#### Implementations

- <span id="tokenbuffer-recursive-new"></span>`fn recursive_new(entries: &mut Vec<Entry>, stream: TokenStream)` — [`Entry`](#entry)

- <span id="tokenbuffer-new"></span>`fn new(stream: proc_macro::TokenStream) -> Self`

  Creates a `TokenBuffer` containing all the tokens from the input

  `proc_macro::TokenStream`.

- <span id="tokenbuffer-new2"></span>`fn new2(stream: TokenStream) -> Self`

  Creates a `TokenBuffer` containing all the tokens from the input

  `proc_macro2::TokenStream`.

- <span id="tokenbuffer-begin"></span>`fn begin(&self) -> Cursor<'_>` — [`Cursor`](#cursor)

  Creates a cursor referencing the first token in the buffer and able to

  traverse until the end of the buffer.

### `Cursor<'a>`

```rust
struct Cursor<'a> {
    ptr: *const Entry,
    scope: *const Entry,
    marker: core::marker::PhantomData<&'a Entry>,
}
```

A cheaply copyable cursor into a `TokenBuffer`.

This cursor holds a shared reference into the immutable data which is used
internally to represent a `TokenStream`, and can be efficiently manipulated
and copied around.

An empty `Cursor` can be created directly, or one may create a `TokenBuffer`
object and get a cursor to its first token with `begin()`.

#### Implementations

- <span id="cursor-empty"></span>`fn empty() -> Self`

  Creates a cursor referencing a static empty TokenStream.

- <span id="cursor-create"></span>`unsafe fn create(ptr: *const Entry, scope: *const Entry) -> Self` — [`Entry`](#entry)

  This create method intelligently exits non-explicitly-entered

  `None`-delimited scopes when the cursor reaches the end of them,

  allowing for them to be treated transparently.

- <span id="cursor-entry"></span>`fn entry(self) -> &'a Entry` — [`Entry`](#entry)

  Get the current entry.

- <span id="cursor-bump-ignore-group"></span>`unsafe fn bump_ignore_group(self) -> Cursor<'a>` — [`Cursor`](#cursor)

  Bump the cursor to point at the next token after the current one. This

  is undefined behavior if the cursor is currently looking at an

  `Entry::End`.

  

  If the cursor is looking at an `Entry::Group`, the bumped cursor will

  point at the first token in the group (with the same scope end).

- <span id="cursor-ignore-none"></span>`fn ignore_none(&mut self)`

  While the cursor is looking at a `None`-delimited group, move it to look

  at the first token inside instead. If the group is empty, this will move

  the cursor past the `None`-delimited group.

  

  WARNING: This mutates its argument.

- <span id="cursor-eof"></span>`fn eof(self) -> bool`

  Checks whether the cursor is currently pointing at the end of its valid

  scope.

- <span id="cursor-ident"></span>`fn ident(self) -> Option<(Ident, Cursor<'a>)>` — [`Ident`](../ident/index.md#ident), [`Cursor`](#cursor)

  If the cursor is pointing at a `Ident`, returns it along with a cursor

  pointing at the next `TokenTree`.

- <span id="cursor-punct"></span>`fn punct(self) -> Option<(Punct, Cursor<'a>)>` — [`Cursor`](#cursor)

  If the cursor is pointing at a `Punct`, returns it along with a cursor

  pointing at the next `TokenTree`.

- <span id="cursor-literal"></span>`fn literal(self) -> Option<(Literal, Cursor<'a>)>` — [`Cursor`](#cursor)

  If the cursor is pointing at a `Literal`, return it along with a cursor

  pointing at the next `TokenTree`.

- <span id="cursor-lifetime"></span>`fn lifetime(self) -> Option<(Lifetime, Cursor<'a>)>` — [`Lifetime`](../lifetime/index.md#lifetime), [`Cursor`](#cursor)

  If the cursor is pointing at a `Lifetime`, returns it along with a

  cursor pointing at the next `TokenTree`.

- <span id="cursor-group"></span>`fn group(self, delim: Delimiter) -> Option<(Cursor<'a>, DelimSpan, Cursor<'a>)>` — [`Cursor`](#cursor)

  If the cursor is pointing at a `Group` with the given delimiter, returns

  a cursor into that group and one pointing to the next `TokenTree`.

- <span id="cursor-any-group"></span>`fn any_group(self) -> Option<(Cursor<'a>, Delimiter, DelimSpan, Cursor<'a>)>` — [`Cursor`](#cursor)

  If the cursor is pointing at a `Group`, returns a cursor into the group

  and one pointing to the next `TokenTree`.

- <span id="cursor-any-group-token"></span>`fn any_group_token(self) -> Option<(Group, Cursor<'a>)>` — [`Cursor`](#cursor)

- <span id="cursor-token-stream"></span>`fn token_stream(self) -> TokenStream`

  Copies all remaining tokens visible from this cursor into a

  `TokenStream`.

- <span id="cursor-token-tree"></span>`fn token_tree(self) -> Option<(TokenTree, Cursor<'a>)>` — [`Cursor`](#cursor)

  If the cursor is pointing at a `TokenTree`, returns it along with a

  cursor pointing at the next `TokenTree`.

  

  Returns `None` if the cursor has reached the end of its stream.

  

  This method does not treat `None`-delimited groups as transparent, and

  will return a `Group(None, ..)` if the cursor is looking at one.

- <span id="cursor-span"></span>`fn span(self) -> Span`

  Returns the `Span` of the current token, or `Span::call_site()` if this

  cursor points to eof.

- <span id="cursor-prev-span"></span>`fn prev_span(self) -> Span`

  Returns the `Span` of the token immediately prior to the position of

  this cursor, or of the current token if there is no previous one.

- <span id="cursor-skip"></span>`fn skip(self) -> Option<Cursor<'a>>` — [`Cursor`](#cursor)

  Skip over the next token that is not a None-delimited group, without

  cloning it. Returns `None` if this cursor points to eof.

  

  This method treats `'lifetimes` as a single token.

- <span id="cursor-scope-delimiter"></span>`fn scope_delimiter(self) -> Delimiter`

#### Trait Implementations

##### `impl Clone for Cursor<'a>`

- <span id="cursor-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Cursor<'a>`

##### `impl Eq for Cursor<'a>`

##### `impl PartialEq for Cursor<'a>`

- <span id="cursor-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for Cursor<'a>`

- <span id="cursor-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

## Enums

### `Entry`

```rust
enum Entry {
    Group(proc_macro2::Group, usize),
    Ident(proc_macro2::Ident),
    Punct(proc_macro2::Punct),
    Literal(proc_macro2::Literal),
    End(isize, isize),
}
```

Internal type which is used instead of `TokenTree` to represent a token tree
within a `TokenBuffer`.

## Functions

### `same_scope`

```rust
fn same_scope(a: Cursor<'_>, b: Cursor<'_>) -> bool
```

### `same_buffer`

```rust
fn same_buffer(a: Cursor<'_>, b: Cursor<'_>) -> bool
```

### `start_of_buffer`

```rust
fn start_of_buffer(cursor: Cursor<'_>) -> *const Entry
```

### `cmp_assuming_same_buffer`

```rust
fn cmp_assuming_same_buffer(a: Cursor<'_>, b: Cursor<'_>) -> core::cmp::Ordering
```

### `open_span_of_group`

```rust
fn open_span_of_group(cursor: Cursor<'_>) -> proc_macro2::Span
```

