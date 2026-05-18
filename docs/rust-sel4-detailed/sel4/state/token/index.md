*[sel4](../../index.md) / [state](../index.md) / [token](index.md)*

---

# Module `token`

## Contents

- [Structs](#structs)
  - [`TokenCell`](#tokencell)
  - [`BorrowError`](#borrowerror)
  - [`BorrowMutError`](#borrowmuterror)
  - [`UnsyncToken`](#unsynctoken)
  - [`SyncToken`](#synctoken)
  - [`SyncTokenBorrow`](#synctokenborrow)
  - [`SyncTokenBorrowMut`](#synctokenborrowmut)
- [Traits](#traits)
  - [`Accessor`](#accessor)
  - [`Token`](#token)
- [Functions](#functions)
  - [`take_ok`](#take-ok)
- [Type Aliases](#type-aliases)
  - [`BorrowFlag`](#borrowflag)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokenCell`](#tokencell) | struct |  |
| [`BorrowError`](#borrowerror) | struct |  |
| [`BorrowMutError`](#borrowmuterror) | struct |  |
| [`UnsyncToken`](#unsynctoken) | struct |  |
| [`SyncToken`](#synctoken) | struct |  |
| [`SyncTokenBorrow`](#synctokenborrow) | struct |  |
| [`SyncTokenBorrowMut`](#synctokenborrowmut) | struct |  |
| [`Accessor`](#accessor) | trait |  |
| [`Token`](#token) | trait |  |
| [`take_ok`](#take-ok) | fn |  |
| [`BorrowFlag`](#borrowflag) | type |  |

## Structs

### `TokenCell<K, A>`

```rust
struct TokenCell<K, A> {
    token: K,
    accessor: A,
}
```

#### Implementations

- <span id="tokencell-new"></span>`const unsafe fn new(accessor: A) -> Self`

- <span id="tokencell-try-with"></span>`fn try_with<F, T, U>(&self, f: F) -> U`

- <span id="tokencell-try-with-mut"></span>`fn try_with_mut<F, T, U>(&self, f: F) -> U`

### `BorrowError`

```rust
struct BorrowError(());
```

#### Implementations

- <span id="borrowerror-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for BorrowError`

- <span id="borrowerror-clone"></span>`fn clone(&self) -> BorrowError` — [`BorrowError`](#borrowerror)

##### `impl Copy for BorrowError`

##### `impl Debug for BorrowError`

- <span id="borrowerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BorrowError`

- <span id="borrowerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BorrowError`

##### `impl Hash for BorrowError`

- <span id="borrowerror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for BorrowError`

- <span id="borrowerror-ord-cmp"></span>`fn cmp(&self, other: &BorrowError) -> cmp::Ordering` — [`BorrowError`](#borrowerror)

##### `impl PartialEq for BorrowError`

- <span id="borrowerror-partialeq-eq"></span>`fn eq(&self, other: &BorrowError) -> bool` — [`BorrowError`](#borrowerror)

##### `impl PartialOrd for BorrowError`

- <span id="borrowerror-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BorrowError) -> option::Option<cmp::Ordering>` — [`BorrowError`](#borrowerror)

##### `impl StructuralPartialEq for BorrowError`

### `BorrowMutError`

```rust
struct BorrowMutError(());
```

#### Implementations

- <span id="borrowmuterror-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for BorrowMutError`

- <span id="borrowmuterror-clone"></span>`fn clone(&self) -> BorrowMutError` — [`BorrowMutError`](#borrowmuterror)

##### `impl Copy for BorrowMutError`

##### `impl Debug for BorrowMutError`

- <span id="borrowmuterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BorrowMutError`

- <span id="borrowmuterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BorrowMutError`

##### `impl Hash for BorrowMutError`

- <span id="borrowmuterror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for BorrowMutError`

- <span id="borrowmuterror-ord-cmp"></span>`fn cmp(&self, other: &BorrowMutError) -> cmp::Ordering` — [`BorrowMutError`](#borrowmuterror)

##### `impl PartialEq for BorrowMutError`

- <span id="borrowmuterror-partialeq-eq"></span>`fn eq(&self, other: &BorrowMutError) -> bool` — [`BorrowMutError`](#borrowmuterror)

##### `impl PartialOrd for BorrowMutError`

- <span id="borrowmuterror-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BorrowMutError) -> option::Option<cmp::Ordering>` — [`BorrowMutError`](#borrowmuterror)

##### `impl StructuralPartialEq for BorrowMutError`

### `UnsyncToken`

```rust
struct UnsyncToken(core::cell::RefCell<()>);
```

#### Trait Implementations

##### `impl Token for UnsyncToken`

- <span id="unsynctoken-token-const-init"></span>`const INIT: Self`

- <span id="unsynctoken-token-type-borrow"></span>`type Borrow = Ref<'a, ()>`

- <span id="unsynctoken-token-type-borrowmut"></span>`type BorrowMut = RefMut<'a, ()>`

- <span id="unsynctoken-token-try-borrow"></span>`fn try_borrow(&self) -> Result<<Self as >::Borrow, BorrowError>` — [`Token`](#token), [`BorrowError`](#borrowerror)

- <span id="unsynctoken-token-try-borrow-mut"></span>`fn try_borrow_mut(&self) -> Result<<Self as >::BorrowMut, BorrowMutError>` — [`Token`](#token), [`BorrowMutError`](#borrowmuterror)

### `SyncToken`

```rust
struct SyncToken(core::sync::atomic::AtomicIsize);
```

#### Trait Implementations

##### `impl Token for SyncToken`

- <span id="synctoken-token-const-init"></span>`const INIT: Self`

- <span id="synctoken-token-type-borrow"></span>`type Borrow = SyncTokenBorrow<'a>`

- <span id="synctoken-token-type-borrowmut"></span>`type BorrowMut = SyncTokenBorrowMut<'a>`

- <span id="synctoken-token-try-borrow"></span>`fn try_borrow(&self) -> Result<<Self as >::Borrow, BorrowError>` — [`Token`](#token), [`BorrowError`](#borrowerror)

- <span id="synctoken-token-try-borrow-mut"></span>`fn try_borrow_mut(&self) -> Result<<Self as >::BorrowMut, BorrowMutError>` — [`Token`](#token), [`BorrowMutError`](#borrowmuterror)

### `SyncTokenBorrow<'a>`

```rust
struct SyncTokenBorrow<'a>(&'a core::sync::atomic::AtomicIsize);
```

#### Trait Implementations

##### `impl Drop for SyncTokenBorrow<'_>`

- <span id="synctokenborrow-drop"></span>`fn drop(&mut self)`

### `SyncTokenBorrowMut<'a>`

```rust
struct SyncTokenBorrowMut<'a>(&'a core::sync::atomic::AtomicIsize);
```

#### Trait Implementations

##### `impl Drop for SyncTokenBorrowMut<'_>`

- <span id="synctokenborrowmut-drop"></span>`fn drop(&mut self)`

## Traits

### `Accessor<T>`

```rust
trait Accessor<T> { ... }
```

#### Required Methods

- `fn with<F, U>(&self, f: F) -> U`

#### Implementors

- [`IpcBufferAccessor`](../index.md#ipcbufferaccessor)

### `Token`

```rust
trait Token { ... }
```

#### Associated Types

- `type Borrow`

- `type BorrowMut`

#### Associated Constants

- `const INIT: Self`

#### Required Methods

- `fn try_borrow(&self) -> Result<<Self as >::Borrow, BorrowError>`

- `fn try_borrow_mut(&self) -> Result<<Self as >::BorrowMut, BorrowMutError>`

#### Provided Methods

- `fn try_with<F, G, T, U>(&self, access_resource: G, f: F) -> T`

- `fn try_with_mut<F, G, T, U>(&self, access_resource: G, f: F) -> T`

#### Implementors

- [`SyncToken`](#synctoken)
- [`UnsyncToken`](#unsynctoken)

## Functions

### `take_ok`

```rust
fn take_ok<T, E>(r: Result<T, E>) -> (Option<T>, Result<(), E>)
```

## Type Aliases

### `BorrowFlag`

```rust
type BorrowFlag = core::sync::atomic::AtomicIsize;
```

