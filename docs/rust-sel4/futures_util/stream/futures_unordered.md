**futures_util > stream > futures_unordered**

# Module: stream::futures_unordered

## Contents

**Structs**

- [`FuturesUnordered`](#futuresunordered) - A set of futures which may complete in any order.

---

## futures_util::stream::futures_unordered::FuturesUnordered

*Struct*

A set of futures which may complete in any order.

See [`FuturesOrdered`](crate::stream::FuturesOrdered) for a version of this
type that preserves a FIFO order.

This structure is optimized to manage a large number of futures.
Futures managed by [`FuturesUnordered`] will only be polled when they
generate wake-up notifications. This reduces the required amount of work
needed to poll large numbers of futures.

[`FuturesUnordered`] can be filled by [`collect`](Iterator::collect)ing an
iterator of futures into a [`FuturesUnordered`], or by
[`push`](FuturesUnordered::push)ing futures onto an existing
[`FuturesUnordered`]. When new futures are added,
[`poll_next`](Stream::poll_next) must be called in order to begin receiving
wake-ups for new futures.

Note that you can create a ready-made [`FuturesUnordered`] via the
[`collect`](Iterator::collect) method, or you can start with an empty set
with the [`FuturesUnordered::new`] constructor.

This type is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

**Generic Parameters:**
- Fut

**Methods:**

- `fn new() -> Self` - Constructs a new, empty [`FuturesUnordered`].
- `fn len(self: &Self) -> usize` - Returns the number of futures contained in the set.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the set contains no futures.
- `fn push(self: &Self, future: Fut)` - Push a future into the set.
- `fn iter(self: &Self) -> Iter<Fut>` - Returns an iterator that allows inspecting each future in the set.
- `fn iter_pin_ref(self: Pin<&Self>) -> IterPinRef<Fut>` - Returns an iterator that allows inspecting each future in the set.
- `fn iter_mut(self: & mut Self) -> IterMut<Fut>` - Returns an iterator that allows modifying each future in the set.
- `fn iter_pin_mut(self: Pin<& mut Self>) -> IterPinMut<Fut>` - Returns an iterator that allows modifying each future in the set.
- `fn clear(self: & mut Self)` - Clears the set, removing all futures.

**Traits:** Send, Sync

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Spawn**
  - `fn spawn_obj(self: &Self, future_obj: FutureObj<'static, ()>) -> Result<(), SpawnError>`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **LocalSpawn**
  - `fn spawn_local_obj(self: &Self, future_obj: LocalFutureObj<'static, ()>) -> Result<(), SpawnError>`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self`



