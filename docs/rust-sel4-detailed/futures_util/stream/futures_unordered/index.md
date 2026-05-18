*[futures_util](../../index.md) / [stream](../index.md) / [futures_unordered](index.md)*

---

# Module `futures_unordered`

An unbounded set of futures.

This module is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

## Contents

- [Modules](#modules)
  - [`abort`](#abort)
  - [`iter`](#iter)
  - [`task`](#task)
  - [`ready_to_run_queue`](#ready-to-run-queue)
- [Structs](#structs)
  - [`IntoIter`](#intoiter)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
  - [`IterPinMut`](#iterpinmut)
  - [`IterPinRef`](#iterpinref)
  - [`FuturesUnordered`](#futuresunordered)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`abort`](#abort) | mod |  |
| [`iter`](#iter) | mod |  |
| [`task`](#task) | mod |  |
| [`ready_to_run_queue`](#ready-to-run-queue) | mod |  |
| [`IntoIter`](#intoiter) | struct |  |
| [`Iter`](#iter) | struct |  |
| [`IterMut`](#itermut) | struct |  |
| [`IterPinMut`](#iterpinmut) | struct |  |
| [`IterPinRef`](#iterpinref) | struct |  |
| [`FuturesUnordered`](#futuresunordered) | struct | A set of futures which may complete in any order. |

## Modules

- [`abort`](abort/index.md)
- [`iter`](iter/index.md)
- [`task`](task/index.md)
- [`ready_to_run_queue`](ready_to_run_queue/index.md)

## Structs

### `IntoIter<Fut: Unpin>`

```rust
struct IntoIter<Fut: Unpin> {
    len: usize,
    inner: super::FuturesUnordered<Fut>,
}
```

Owned iterator over all futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + Unpin> Debug for IntoIter<Fut>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Unpin> ExactSizeIterator for IntoIter<Fut>`

##### `impl IntoIterator for IntoIter<Fut>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut: Unpin> Iterator for IntoIter<Fut>`

- <span id="intoiter-iterator-type-item"></span>`type Item = Fut`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<Fut: Send + Unpin> Send for IntoIter<Fut>`

##### `impl<Fut: Sync + Unpin> Sync for IntoIter<Fut>`

### `Iter<'a, Fut: Unpin>`

```rust
struct Iter<'a, Fut: Unpin>(IterPinRef<'a, Fut>);
```

Immutable iterator over all the futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + Unpin> Debug for Iter<'a, Fut>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Unpin> ExactSizeIterator for Iter<'_, Fut>`

##### `impl IntoIterator for Iter<'a, Fut>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut: Unpin> Iterator for Iter<'a, Fut>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a Fut`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterMut<'a, Fut: Unpin>`

```rust
struct IterMut<'a, Fut: Unpin>(IterPinMut<'a, Fut>);
```

Mutable iterator over all futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + Unpin> Debug for IterMut<'a, Fut>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Unpin> ExactSizeIterator for IterMut<'_, Fut>`

##### `impl IntoIterator for IterMut<'a, Fut>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut: Unpin> Iterator for IterMut<'a, Fut>`

- <span id="itermut-iterator-type-item"></span>`type Item = &'a mut Fut`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterPinMut<'a, Fut>`

```rust
struct IterPinMut<'a, Fut> {
    task: *const super::task::Task<Fut>,
    len: usize,
    _marker: core::marker::PhantomData<&'a mut super::FuturesUnordered<Fut>>,
}
```

Mutable iterator over all futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for IterPinMut<'a, Fut>`

- <span id="iterpinmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> ExactSizeIterator for IterPinMut<'_, Fut>`

##### `impl IntoIterator for IterPinMut<'a, Fut>`

- <span id="iterpinmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterpinmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterpinmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut> Iterator for IterPinMut<'a, Fut>`

- <span id="iterpinmut-iterator-type-item"></span>`type Item = Pin<&'a mut Fut>`

- <span id="iterpinmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterpinmut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<Fut: Send> Send for IterPinMut<'_, Fut>`

##### `impl<Fut: Sync> Sync for IterPinMut<'_, Fut>`

### `IterPinRef<'a, Fut>`

```rust
struct IterPinRef<'a, Fut> {
    task: *const super::task::Task<Fut>,
    len: usize,
    pending_next_all: *mut super::task::Task<Fut>,
    _marker: core::marker::PhantomData<&'a super::FuturesUnordered<Fut>>,
}
```

Immutable iterator over all futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for IterPinRef<'a, Fut>`

- <span id="iterpinref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> ExactSizeIterator for IterPinRef<'_, Fut>`

##### `impl IntoIterator for IterPinRef<'a, Fut>`

- <span id="iterpinref-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterpinref-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterpinref-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut> Iterator for IterPinRef<'a, Fut>`

- <span id="iterpinref-iterator-type-item"></span>`type Item = Pin<&'a Fut>`

- <span id="iterpinref-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterpinref-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<Fut: Send> Send for IterPinRef<'_, Fut>`

##### `impl<Fut: Sync> Sync for IterPinRef<'_, Fut>`

### `FuturesUnordered<Fut>`

```rust
struct FuturesUnordered<Fut> {
    ready_to_run_queue: alloc::sync::Arc<self::ready_to_run_queue::ReadyToRunQueue<Fut>>,
    head_all: core::sync::atomic::AtomicPtr<self::task::Task<Fut>>,
    is_terminated: core::sync::atomic::AtomicBool,
}
```

A set of futures which may complete in any order.

See [`FuturesOrdered`](crate::stream::FuturesOrdered) for a version of this
type that preserves a FIFO order.

This structure is optimized to manage a large number of futures.
Futures managed by [`FuturesUnordered`](#futuresunordered) will only be polled when they
generate wake-up notifications. This reduces the required amount of work
needed to poll large numbers of futures.

[`FuturesUnordered`](#futuresunordered) can be filled by [`collect`](Iterator::collect)ing an
iterator of futures into a [`FuturesUnordered`](#futuresunordered), or by
[`push`](FuturesUnordered::push)ing futures onto an existing
[`FuturesUnordered`](#futuresunordered). When new futures are added,
[`poll_next`](Stream::poll_next) must be called in order to begin receiving
wake-ups for new futures.

Note that you can create a ready-made [`FuturesUnordered`](#futuresunordered) via the
[`collect`](Iterator::collect) method, or you can start with an empty set
with the `FuturesUnordered::new` constructor.

This type is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

#### Implementations

- <span id="futuresunordered-new"></span>`fn new() -> Self`

  Constructs a new, empty [`FuturesUnordered`](#futuresunordered).

  

  The returned [`FuturesUnordered`](#futuresunordered) does not contain any futures.

  In this state, [`FuturesUnordered::poll_next`](Stream::poll_next) will

  return [`Poll::Ready(None)`](Poll::Ready).

- <span id="futuresunordered-len"></span>`fn len(&self) -> usize`

  Returns the number of futures contained in the set.

  

  This represents the total number of in-flight futures.

- <span id="futuresunordered-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the set contains no futures.

- <span id="futuresunordered-push"></span>`fn push(&self, future: Fut)`

  Push a future into the set.

  

  This method adds the given future to the set. This method will not

  call [`poll`](core::future::Future::poll) on the submitted future. The caller must

  ensure that [`FuturesUnordered::poll_next`](Stream::poll_next) is called

  in order to receive wake-up notifications for the given future.

- <span id="futuresunordered-iter"></span>`fn iter(&self) -> Iter<'_, Fut>` — [`Iter`](iter/index.md#iter)

  Returns an iterator that allows inspecting each future in the set.

- <span id="futuresunordered-iter-pin-ref"></span>`fn iter_pin_ref(self: Pin<&Self>) -> IterPinRef<'_, Fut>` — [`IterPinRef`](iter/index.md#iterpinref)

  Returns an iterator that allows inspecting each future in the set.

- <span id="futuresunordered-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, Fut>` — [`IterMut`](iter/index.md#itermut)

  Returns an iterator that allows modifying each future in the set.

- <span id="futuresunordered-iter-pin-mut"></span>`fn iter_pin_mut(self: Pin<&mut Self>) -> IterPinMut<'_, Fut>` — [`IterPinMut`](iter/index.md#iterpinmut)

  Returns an iterator that allows modifying each future in the set.

- <span id="futuresunordered-atomic-load-head-and-len-all"></span>`fn atomic_load_head_and_len_all(&self) -> (*const Task<Fut>, usize)` — [`Task`](task/index.md#task)

  Returns the current head node and number of futures in the list of all

  futures within a context where access is shared with other threads

  (mostly for use with the `len` and `iter_pin_ref` methods).

- <span id="futuresunordered-release-task"></span>`fn release_task(&mut self, task: Arc<Task<Fut>>)` — [`Task`](task/index.md#task)

  Releases the task. It destroys the future inside and either drops

  the `Arc<Task>` or transfers ownership to the ready to run queue.

  The task this method is called on must have been unlinked before.

- <span id="futuresunordered-link"></span>`fn link(&self, task: Arc<Task<Fut>>) -> *const Task<Fut>` — [`Task`](task/index.md#task)

  Insert a new task into the internal linked list.

- <span id="futuresunordered-unlink"></span>`unsafe fn unlink(&mut self, task: *const Task<Fut>) -> Arc<Task<Fut>>` — [`Task`](task/index.md#task)

  Remove the task from the linked list tracking all tasks currently

  managed by `FuturesUnordered`.

  This method is unsafe because it has be guaranteed that `task` is a

  valid pointer.

- <span id="futuresunordered-pending-next-all"></span>`fn pending_next_all(&self) -> *mut Task<Fut>` — [`Task`](task/index.md#task)

  Returns the reserved value for `Task::next_all` to indicate a pending

  assignment from the thread that inserted the task.

  

  `FuturesUnordered::link` needs to update `Task` pointers in an order

  that ensures any iterators created on other threads can correctly

  traverse the entire `Task` list using the chain of `next_all` pointers.

  This could be solved with a compare-exchange loop that stores the

  current `head_all` in `next_all` and swaps out `head_all` with the new

  `Task` pointer if the head hasn't already changed. Under heavy thread

  contention, this compare-exchange loop could become costly.

  

  An alternative is to initialize `next_all` to a reserved pending state

  first, perform an atomic swap on `head_all`, and finally update

  `next_all` with the old head node. Iterators will then either see the

  pending state value or the correct next node pointer, and can reload

  `next_all` as needed until the correct value is loaded. The number of

  retries needed (if any) would be small and will always be finite, so

  this should generally perform better than the compare-exchange loop.

  

  A valid `Task` pointer in the `head_all` list is guaranteed to never be

  this value, so it is safe to use as a reserved value until the correct

  value can be written.

#### Trait Implementations

##### `impl<Fut> Debug for FuturesUnordered<Fut>`

- <span id="futuresunordered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> Default for FuturesUnordered<Fut>`

- <span id="futuresunordered-default"></span>`fn default() -> Self`

##### `impl<Fut> Drop for FuturesUnordered<Fut>`

- <span id="futuresunordered-drop"></span>`fn drop(&mut self)`

##### `impl<Fut> Extend for FuturesUnordered<Fut>`

- <span id="futuresunordered-extend"></span>`fn extend<I>(&mut self, iter: I)`

##### `impl<Fut> FromIterator for FuturesUnordered<Fut>`

- <span id="futuresunordered-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Self`

##### `impl<Fut: Future> FusedStream for FuturesUnordered<Fut>`

- <span id="futuresunordered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Unpin> IntoIterator for &'a FuturesUnordered<Fut>`

- <span id="a-futuresunordered-intoiterator-type-item"></span>`type Item = &'a Fut`

- <span id="a-futuresunordered-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, Fut>`

- <span id="a-futuresunordered-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LocalSpawn for FuturesUnordered<futures_task::LocalFutureObj<'_, ()>>`

- <span id="futuresunordered-localspawn-spawn-local-obj"></span>`fn spawn_local_obj(&self, future_obj: LocalFutureObj<'static, ()>) -> Result<(), SpawnError>` — [`LocalFutureObj`](../../future/index.md#localfutureobj), [`SpawnError`](../../task/index.md#spawnerror)

##### `impl LocalSpawnExt for FuturesUnordered<Fut>`

##### `impl<Fut: Send> Send for FuturesUnordered<Fut>`

##### `impl Spawn for FuturesUnordered<futures_task::FutureObj<'_, ()>>`

- <span id="futuresunordered-spawn-spawn-obj"></span>`fn spawn_obj(&self, future_obj: FutureObj<'static, ()>) -> Result<(), SpawnError>` — [`FutureObj`](../../future/index.md#futureobj), [`SpawnError`](../../task/index.md#spawnerror)

##### `impl SpawnExt for FuturesUnordered<Fut>`

##### `impl<Fut: Future> Stream for FuturesUnordered<Fut>`

- <span id="futuresunordered-stream-type-item"></span>`type Item = <Fut as Future>::Output`

- <span id="futuresunordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="futuresunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FuturesUnordered<Fut>`

##### `impl<Fut: Send + Sync> Sync for FuturesUnordered<Fut>`

##### `impl TryStream for FuturesUnordered<Fut>`

- <span id="futuresunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="futuresunordered-trystream-type-error"></span>`type Error = E`

- <span id="futuresunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for FuturesUnordered<Fut>`

##### `impl<Fut> Unpin for FuturesUnordered<Fut>`

