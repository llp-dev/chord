**heapless > history_buf**

# Module: history_buf

## Contents

**Structs**

- [`HistoryBufInner`](#historybufinner) - Base struct for [`HistoryBuf`] and [`HistoryBufView`], generic over the [`HistoryBufStorage`].
- [`OldestOrdered`](#oldestordered) - Double ended iterator on the underlying buffer ordered from the oldest data

**Type Aliases**

- [`HistoryBuf`](#historybuf) - A "history buffer", similar to a write-only ring buffer of fixed length.
- [`HistoryBufView`](#historybufview) - A "view" into a [`HistoryBuf`]

---

## heapless::history_buf::HistoryBuf

*Type Alias*: `HistoryBufInner<T, OwnedHistoryBufStorage<T, N>>`

A "history buffer", similar to a write-only ring buffer of fixed length.

This buffer keeps a fixed number of elements.  On write, the oldest element
is overwritten. Thus, the buffer is useful to keep a history of values with
some desired depth, and for example calculate a rolling average.

# Examples
```
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



## heapless::history_buf::HistoryBufInner

*Struct*

Base struct for [`HistoryBuf`] and [`HistoryBufView`], generic over the [`HistoryBufStorage`].

In most cases you should use [`HistoryBuf`] or [`HistoryBufView`] directly. Only use this
struct if you want to write code that's generic over both.

**Generic Parameters:**
- T
- S

**Methods:**

- `fn clear(self: & mut Self)` - Clears the buffer
- `fn new_with(t: T) -> Self` - Constructs a new history buffer, where every element is the given value.
- `fn len(self: &Self) -> usize` - Returns the current fill level of the buffer.
- `fn is_empty(self: &Self) -> bool` - Returns true if the buffer is empty.
- `fn capacity(self: &Self) -> usize` - Returns the capacity of the buffer, which is the length of the
- `fn is_full(self: &Self) -> bool` - Returns whether the buffer is full
- `fn write(self: & mut Self, t: T)` - Writes an element to the buffer, overwriting the oldest value.
- `fn extend_from_slice(self: & mut Self, other: &[T])` - Clones and writes all elements in a slice to the buffer.
- `fn recent(self: &Self) -> Option<&T>` - Returns a reference to the most recently written value.
- `fn recent_index(self: &Self) -> Option<usize>` - Returns index of the most recently written value in the underlying slice.
- `fn oldest(self: &Self) -> Option<&T>` - Returns a reference to the oldest value in the buffer.
- `fn oldest_index(self: &Self) -> Option<usize>` - Returns index of the oldest value in the underlying slice.
- `fn as_slice(self: &Self) -> &[T]` - Returns the array slice backing the buffer, without keeping track
- `fn as_slices(self: &Self) -> (&[T], &[T])` - Returns a pair of slices which contain, in order, the contents of the buffer.
- `fn oldest_ordered(self: &Self) -> OldestOrdered<T>` - Returns double ended iterator for iterating over the buffer from
- `fn as_view(self: &Self) -> &HistoryBufView<T>` - Get a reference to the `HistoryBuf`, erasing the `N` const-generic.
- `fn as_mut_view(self: & mut Self) -> & mut HistoryBufView<T>` - Get a mutable reference to the `HistoryBuf`, erasing the `N` const-generic.
- `fn clear_with(self: & mut Self, t: T)` - Clears the buffer, replacing every element with the given value.
- `fn new() -> Self` - Constructs a new history buffer.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **Serialize**
  - `fn serialize<SER>(self: &Self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &[T]`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[T]`



## heapless::history_buf::HistoryBufView

*Type Alias*: `HistoryBufInner<T, ViewHistoryBufStorage<T>>`

A "view" into a [`HistoryBuf`]

Unlike [`HistoryBuf`], it doesn't have the `const N: usize` in its type signature.

# Examples
```
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



## heapless::history_buf::OldestOrdered

*Struct*

Double ended iterator on the underlying buffer ordered from the oldest data
to the newest.

**Generic Parameters:**
- 'a
- T

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<&'a T>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`



