*[heapless](../index.md) / [history_buf](index.md)*

---

# Module `history_buf`

A "history buffer", similar to a write-only ring buffer of fixed length.

This buffer keeps a fixed number of elements.  On write, the oldest element
is overwritten. Thus, the buffer is useful to keep a history of values with
some desired depth, and for example calculate a rolling average.

# Examples
```rust
use heapless::HistoryBuf;

// Initialize a new buffer with 8 elements.
let mut buf = HistoryBuf::<_, 8>::new();

// Starts with no data
assert_eq!(buf.recent(), None);

buf.write(3);
buf.write(5);
buf.extend(&[4, 4]);

// The most recent written element is a four.
assert_eq!(buf.recent(), Some(&4));

// To access all elements in an unspecified order, use `as_slice()`.
for el in buf.as_slice() {
    println!("{:?}", el);
}

// Now we can prepare an average of all values, which comes out to 4.
let avg = buf.as_slice().iter().sum::<usize>() / buf.len();
assert_eq!(avg, 4);
```

## Contents

- [Modules](#modules)
  - [`storage`](#storage)
- [Structs](#structs)
  - [`HistoryBufInner`](#historybufinner)
  - [`OldestOrdered`](#oldestordered)
- [Traits](#traits)
  - [`HistoryBufStorage`](#historybufstorage)
- [Type Aliases](#type-aliases)
  - [`OwnedHistoryBufStorage`](#ownedhistorybufstorage)
  - [`ViewHistoryBufStorage`](#viewhistorybufstorage)
  - [`HistoryBuf`](#historybuf)
  - [`HistoryBufView`](#historybufview)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`storage`](#storage) | mod |  |
| [`HistoryBufInner`](#historybufinner) | struct | Base struct for [`HistoryBuf`] and [`HistoryBufView`], generic over the [`HistoryBufStorage`]. |
| [`OldestOrdered`](#oldestordered) | struct | Double ended iterator on the underlying buffer ordered from the oldest data to the newest. |
| [`HistoryBufStorage`](#historybufstorage) | trait |  |
| [`OwnedHistoryBufStorage`](#ownedhistorybufstorage) | type |  |
| [`ViewHistoryBufStorage`](#viewhistorybufstorage) | type |  |
| [`HistoryBuf`](#historybuf) | type | A "history buffer", similar to a write-only ring buffer of fixed length. |
| [`HistoryBufView`](#historybufview) | type | A "view" into a [`HistoryBuf`] |

## Modules

- [`storage`](storage/index.md)

## Structs

### `HistoryBufInner<T, S: HistoryBufStorage<T> + ?Sized>`

```rust
struct HistoryBufInner<T, S: HistoryBufStorage<T> + ?Sized> {
    phantom: core::marker::PhantomData<T>,
    write_at: usize,
    filled: bool,
    data: S,
}
```

Base struct for [`HistoryBuf`](#historybuf) and [`HistoryBufView`](#historybufview), generic over the [`HistoryBufStorage`](storage/index.md).

In most cases you should use [`HistoryBuf`](#historybuf) or [`HistoryBufView`](#historybufview) directly. Only use this
struct if you want to write code that's generic over both.

#### Implementations

- <span id="historybufinner-const-init"></span>`const INIT: MaybeUninit<T>`

- <span id="historybufinner-new"></span>`const fn new() -> Self`

  Constructs a new history buffer.

  

  The construction of a `HistoryBuf` works in `const` contexts.

  

  # Examples

  

  ```rust

  use heapless::HistoryBuf;

  

  // Allocate a 16-element buffer on the stack

  let x: HistoryBuf<u8, 16> = HistoryBuf::new();

  assert_eq!(x.len(), 0);

  ```

#### Trait Implementations

##### `impl<T, S: HistoryBufStorage<T> + ?Sized> AsRef for HistoryBufInner<T, S>`

- <span id="historybufinner-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T, S: HistoryBufStorage<T> + ?Sized> Debug for HistoryBufInner<T, S>`

- <span id="historybufinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S: HistoryBufStorage<T> + ?Sized> Deref for HistoryBufInner<T, S>`

- <span id="historybufinner-deref-type-target"></span>`type Target = [T]`

- <span id="historybufinner-deref"></span>`fn deref(&self) -> &[T]`

##### `impl<T> DeserializeOwned for HistoryBufInner<T, S>`

##### `impl<T, S: HistoryBufStorage<T> + ?Sized> Drop for HistoryBufInner<T, S>`

- <span id="historybufinner-drop"></span>`fn drop(&mut self)`

##### `impl<T, S: HistoryBufStorage<T> + ?Sized> Extend for HistoryBufInner<T, S>`

- <span id="historybufinner-extend"></span>`fn extend<I>(&mut self, iter: I)`

##### `impl<T, S: HistoryBufStorage<T> + ?Sized> PartialEq for HistoryBufInner<T, S>`

- <span id="historybufinner-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Receiver for HistoryBufInner<T, S>`

- <span id="historybufinner-receiver-type-target"></span>`type Target = T`

##### `impl<T, S: HistoryBufStorage<T> + ?Sized> Serialize for crate::history_buf::HistoryBufInner<T, S>`

- <span id="cratehistory-bufhistorybufinner-serialize"></span>`fn serialize<SER>(&self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`

### `OldestOrdered<'a, T>`

```rust
struct OldestOrdered<'a, T> {
    inner: core::iter::Chain<core::slice::Iter<'a, T>, core::slice::Iter<'a, T>>,
}
```

Double ended iterator on the underlying buffer ordered from the oldest data
to the newest.

#### Trait Implementations

##### `impl<T> Clone for OldestOrdered<'_, T>`

- <span id="oldestordered-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> DoubleEndedIterator for OldestOrdered<'_, T>`

- <span id="oldestordered-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl IntoIterator for OldestOrdered<'a, T>`

- <span id="oldestordered-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="oldestordered-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="oldestordered-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for OldestOrdered<'a, T>`

- <span id="oldestordered-iterator-type-item"></span>`type Item = &'a T`

- <span id="oldestordered-iterator-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="oldestordered-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Traits

### `HistoryBufStorage<T>`

```rust
trait HistoryBufStorage<T>: HistoryBufSealedStorage<T> { ... }
```

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedHistoryBufStorage`](storage/index.md): stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewHistoryBufStorage`](storage/index.md): stores the data in an unsized `[T]`.

This allows [`HistoryBuf`](#historybuf) to be generic over either sized or unsized storage. The `histbuf`
module contains a [`HistoryBufInner`](#historybufinner) struct that's generic on [`HistoryBufStorage`](storage/index.md),
and two type aliases for convenience:

- [`HistoryBuf<T, N>`](super::HistoryBuf) = `HistoryBufInner<T, OwnedHistoryBufStorage<T, N>>`
- [`HistoryBufView<T>`](super::HistoryBufView) = `HistoryBufInner<T, ViewHistoryBufStorage<T>>`

`HistoryBuf` can be unsized into `HistoryBufView`, either by unsizing coercions such as `&mut HistoryBuf -> &mut HistoryBufView` or
`Box<HistoryBuf> -> Box<HistoryBufView>`, or explicitly with [`.as_view()`](super::HistoryBuf::as_view) or [`.as_mut_view()`](super::HistoryBuf::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.





#### Implementors

- [`OwnedHistoryBufStorage`](storage/index.md#ownedhistorybufstorage)
- [`ViewHistoryBufStorage`](storage/index.md#viewhistorybufstorage)

## Type Aliases

### `OwnedHistoryBufStorage<T, const N: usize>`

```rust
type OwnedHistoryBufStorage<T, const N: usize> = HistoryBufStorageInner<[core::mem::MaybeUninit<T>; N]>;
```

Implementation of [`HistoryBufStorage`](storage/index.md) that stores the data in an array `[T; N]` whose size is known at compile time.

### `ViewHistoryBufStorage<T>`

```rust
type ViewHistoryBufStorage<T> = HistoryBufStorageInner<[core::mem::MaybeUninit<T>]>;
```

Implementation of [`HistoryBufStorage`](storage/index.md) that stores the data in an unsized `[T]`.

### `HistoryBuf<T, const N: usize>`

```rust
type HistoryBuf<T, const N: usize> = HistoryBufInner<T, OwnedHistoryBufStorage<T, N>>;
```

A "history buffer", similar to a write-only ring buffer of fixed length.

This buffer keeps a fixed number of elements.  On write, the oldest element
is overwritten. Thus, the buffer is useful to keep a history of values with
some desired depth, and for example calculate a rolling average.

# Examples
```rust
use heapless::HistoryBuf;

// Initialize a new buffer with 8 elements.
let mut buf = HistoryBuf::<_, 8>::new();

// Starts with no data
assert_eq!(buf.recent(), None);

buf.write(3);
buf.write(5);
buf.extend(&[4, 4]);

// The most recent written element is a four.
assert_eq!(buf.recent(), Some(&4));

// To access all elements in an unspecified order, use `as_slice()`.
for el in buf.as_slice() {
    println!("{:?}", el);
}

// Now we can prepare an average of all values, which comes out to 4.
let avg = buf.as_slice().iter().sum::<usize>() / buf.len();
assert_eq!(avg, 4);
```

### `HistoryBufView<T>`

```rust
type HistoryBufView<T> = HistoryBufInner<T, ViewHistoryBufStorage<T>>;
```

A "view" into a [`HistoryBuf`](#historybuf)

Unlike [`HistoryBuf`](#historybuf), it doesn't have the `const N: usize` in its type signature.

# Examples
```rust
use heapless::history_buf::{HistoryBuf, HistoryBufView};

// Initialize a new buffer with 8 elements.
let mut owned_buf = HistoryBuf::<_, 8>::new();
let buf: &mut HistoryBufView<_> = &mut owned_buf;

// Starts with no data
assert_eq!(buf.recent(), None);

buf.write(3);
buf.write(5);
buf.extend(&[4, 4]);

// The most recent written element is a four.
assert_eq!(buf.recent(), Some(&4));

// To access all elements in an unspecified order, use `as_slice()`.
for el in buf.as_slice() {
    println!("{:?}", el);
}

// Now we can prepare an average of all values, which comes out to 4.
let avg = buf.as_slice().iter().sum::<usize>() / buf.len();
assert_eq!(avg, 4);
```

