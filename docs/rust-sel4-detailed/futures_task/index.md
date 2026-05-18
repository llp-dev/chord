# Crate `futures_task`

Tools for working with tasks.

## Contents

- [Modules](#modules)
  - [`spawn`](#spawn)
  - [`arc_wake`](#arc-wake)
  - [`waker`](#waker)
  - [`waker_ref`](#waker-ref)
  - [`future_obj`](#future-obj)
  - [`noop_waker`](#noop-waker)
- [Structs](#structs)
  - [`SpawnError`](#spawnerror)
  - [`WakerRef`](#wakerref)
  - [`FutureObj`](#futureobj)
  - [`LocalFutureObj`](#localfutureobj)
- [Traits](#traits)
  - [`LocalSpawn`](#localspawn)
  - [`Spawn`](#spawn)
  - [`ArcWake`](#arcwake)
  - [`UnsafeFutureObj`](#unsafefutureobj)
- [Functions](#functions)
  - [`waker`](#waker)
  - [`waker_ref`](#waker-ref)
  - [`noop_waker`](#noop-waker)
  - [`noop_waker_ref`](#noop-waker-ref)
  - [`RawWakerVTable`](#rawwakervtable)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`spawn`](#spawn) | mod |  |
| [`arc_wake`](#arc-wake) | mod |  |
| [`waker`](#waker) | mod |  |
| [`waker_ref`](#waker-ref) | mod |  |
| [`future_obj`](#future-obj) | mod |  |
| [`noop_waker`](#noop-waker) | mod | Utilities for creating zero-cost wakers that don't do anything. |
| [`SpawnError`](#spawnerror) | struct |  |
| [`WakerRef`](#wakerref) | struct |  |
| [`FutureObj`](#futureobj) | struct |  |
| [`LocalFutureObj`](#localfutureobj) | struct |  |
| [`LocalSpawn`](#localspawn) | trait |  |
| [`Spawn`](#spawn) | trait |  |
| [`ArcWake`](#arcwake) | trait |  |
| [`UnsafeFutureObj`](#unsafefutureobj) | trait |  |
| [`waker`](#waker) | fn |  |
| [`waker_ref`](#waker-ref) | fn |  |
| [`noop_waker`](#noop-waker) | fn |  |
| [`noop_waker_ref`](#noop-waker-ref) | fn |  |
| [`RawWakerVTable`](#rawwakervtable) | fn |  |

## Modules

- [`spawn`](spawn/index.md)
- [`arc_wake`](arc_wake/index.md)
- [`waker`](waker/index.md)
- [`waker_ref`](waker_ref/index.md)
- [`future_obj`](future_obj/index.md)
- [`noop_waker`](noop_waker/index.md) — Utilities for creating zero-cost wakers that don't do anything.

## Structs

### `SpawnError`

```rust
struct SpawnError {
    _priv: (),
}
```

An error that occurred during spawning.

#### Implementations

- <span id="spawnerror-shutdown"></span>`fn shutdown() -> Self`

  Spawning failed because the executor has been shut down.

- <span id="spawnerror-is-shutdown"></span>`fn is_shutdown(&self) -> bool`

  Check whether spawning failed to the executor being shut down.

#### Trait Implementations

##### `impl Debug for SpawnError`

- <span id="spawnerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SpawnError`

- <span id="spawnerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for SpawnError`

- <span id="spawnerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `WakerRef<'a>`

```rust
struct WakerRef<'a> {
    waker: core::mem::ManuallyDrop<core::task::Waker>,
    _marker: core::marker::PhantomData<&'a ()>,
}
```

A [`Waker`](#waker) that is only valid for a given lifetime.

Note: this type implements [`Deref<Target = Waker>`](std::ops::Deref),
so it can be used to get a `&Waker`.

#### Implementations

- <span id="wakerref-new"></span>`fn new(waker: &'a Waker) -> Self` — [`Waker`](#waker)

  Create a new [`WakerRef`](waker_ref/index.md) from a [`Waker`](#waker) reference.

- <span id="wakerref-new-unowned"></span>`fn new_unowned(waker: ManuallyDrop<Waker>) -> Self` — [`Waker`](#waker)

  Create a new [`WakerRef`](waker_ref/index.md) from a [`Waker`](#waker) that must not be dropped.

  

  Note: this if for rare cases where the caller created a [`Waker`](#waker) in

  an unsafe way (that will be valid only for a lifetime to be determined

  by the caller), and the [`Waker`](#waker) doesn't need to or must not be

  destroyed.

#### Trait Implementations

##### `impl Debug for WakerRef<'a>`

- <span id="wakerref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for WakerRef<'_>`

- <span id="wakerref-deref-type-target"></span>`type Target = Waker`

- <span id="wakerref-deref"></span>`fn deref(&self) -> &Waker` — [`Waker`](#waker)

##### `impl Receiver for WakerRef<'a>`

- <span id="wakerref-receiver-type-target"></span>`type Target = T`

### `FutureObj<'a, T>`

```rust
struct FutureObj<'a, T>(LocalFutureObj<'a, T>);
```

A custom trait object for polling futures, roughly akin to
`Box<dyn Future<Output = T> + Send + 'a>`.

This custom trait object was introduced as currently it is not possible to
take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std
contexts.

You should generally not need to use this type outside of `no_std` or when
implementing `Spawn`, consider using `BoxFuture` instead.

#### Implementations

- <span id="futureobj-new"></span>`fn new<F: UnsafeFutureObj<'a, T> + Send>(f: F) -> Self`

  Create a `FutureObj` from a custom trait object representation.

#### Trait Implementations

##### `impl<T> Debug for FutureObj<'_, T>`

- <span id="futureobj-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Future for FutureObj<'_, T>`

- <span id="futureobj-future-type-output"></span>`type Output = T`

- <span id="futureobj-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](#context), [`Poll`](#poll)

##### `impl IntoFuture for FutureObj<'a, T>`

- <span id="futureobj-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="futureobj-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="futureobj-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> Send for FutureObj<'_, T>`

##### `impl<T> Unpin for FutureObj<'_, T>`

### `LocalFutureObj<'a, T>`

```rust
struct LocalFutureObj<'a, T> {
    future: *mut dyn Future<Output = T>,
    drop_fn: fn(*mut dyn Future<Output = T>),
    _marker: core::marker::PhantomData<&'a ()>,
}
```

A custom trait object for polling futures, roughly akin to
`Box<dyn Future<Output = T> + 'a>`.

This custom trait object was introduced as currently it is not possible to
take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std
contexts.

#### Implementations

- <span id="localfutureobj-new"></span>`fn new<F: UnsafeFutureObj<'a, T> + 'a>(f: F) -> Self`

  Create a `LocalFutureObj` from a custom trait object representation.

- <span id="localfutureobj-into-future-obj"></span>`unsafe fn into_future_obj(self) -> FutureObj<'a, T>` — [`FutureObj`](future_obj/index.md#futureobj)

  Converts the `LocalFutureObj` into a `FutureObj`.

  

  # Safety

  

  To make this operation safe one has to ensure that the `UnsafeFutureObj`

  instance from which this `LocalFutureObj` was created actually

  implements `Send`.

#### Trait Implementations

##### `impl<T> Debug for LocalFutureObj<'_, T>`

- <span id="localfutureobj-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for LocalFutureObj<'_, T>`

- <span id="localfutureobj-drop"></span>`fn drop(&mut self)`

##### `impl<T> Future for LocalFutureObj<'_, T>`

- <span id="localfutureobj-future-type-output"></span>`type Output = T`

- <span id="localfutureobj-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](#context), [`Poll`](#poll)

##### `impl IntoFuture for LocalFutureObj<'a, T>`

- <span id="localfutureobj-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="localfutureobj-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="localfutureobj-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> Unpin for LocalFutureObj<'_, T>`

## Traits

### `LocalSpawn`

```rust
trait LocalSpawn { ... }
```

The `LocalSpawn` is similar to [`Spawn`](spawn/index.md), but allows spawning futures
that don't implement `Send`.

#### Required Methods

- `fn spawn_local_obj(&self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError>`

  Spawns a future that will be run to completion.

#### Provided Methods

- `fn status_local(&self) -> Result<(), SpawnError>`

  Determines whether the executor is able to spawn new tasks.

#### Implementors

- `&Sp`
- `&mut Sp`
- `alloc::boxed::Box<Sp>`
- `alloc::rc::Rc<Sp>`
- `alloc::sync::Arc<Sp>`

### `Spawn`

```rust
trait Spawn { ... }
```

The `Spawn` trait allows for pushing futures onto an executor that will
run them to completion.

#### Required Methods

- `fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError>`

  Spawns a future that will be run to completion.

#### Provided Methods

- `fn status(&self) -> Result<(), SpawnError>`

  Determines whether the executor is able to spawn new tasks.

#### Implementors

- `&Sp`
- `&mut Sp`
- `alloc::boxed::Box<Sp>`
- `alloc::rc::Rc<Sp>`
- `alloc::sync::Arc<Sp>`

### `ArcWake`

```rust
trait ArcWake: Send + Sync { ... }
```

A way of waking up a specific task.

By implementing this trait, types that are expected to be wrapped in an `Arc`
can be converted into [`Waker`](../futures_core/task/index.md) objects.
Those Wakers can be used to signal executors that a task it owns
is ready to be `poll`ed again.

Currently, there are two ways to convert `ArcWake` into [`Waker`](../futures_core/task/index.md):

* [`waker`](super::waker()) converts `Arc<impl ArcWake>` into [`Waker`](../futures_core/task/index.md).
* [`waker_ref`](super::waker_ref()) converts `&Arc<impl ArcWake>` into [`WakerRef`](waker_ref/index.md) that
  provides access to a [`&Waker`][`Waker`](../futures_core/task/index.md).



#### Required Methods

- `fn wake_by_ref(arc_self: &Arc<Self>)`

  Indicates that the associated task is ready to make progress and should

#### Provided Methods

- `fn wake(self: Arc<Self>)`

  Indicates that the associated task is ready to make progress and should

### `UnsafeFutureObj<'a, T>`

```rust
trait UnsafeFutureObj<'a, T>: 'a { ... }
```

A custom implementation of a future trait object for `FutureObj`, providing
a vtable with drop support.

This custom representation is typically used only in `no_std` contexts,
where the default `Box`-based implementation is not available.

# Safety

See the safety notes on individual methods for what guarantees an
implementor must provide.

#### Required Methods

- `fn into_raw(self) -> *mut dyn Future<Output = T>`

  Convert an owned instance into a (conceptually owned) fat pointer.

- `fn drop(ptr: *mut dyn Future<Output = T>)`

  Drops the future represented by the given fat pointer.

#### Implementors

- `&'a mut F`
- `&'a mut dyn Future<Output = T> + Unpin`
- `alloc::boxed::Box<F>`
- `alloc::boxed::Box<dyn Future<Output = T> + Send>`
- `alloc::boxed::Box<dyn Future<Output = T>>`
- `core::pin::Pin<&'a mut F>`
- `core::pin::Pin<&'a mut dyn Future<Output = T>>`
- `core::pin::Pin<alloc::boxed::Box<F>>`
- `core::pin::Pin<alloc::boxed::Box<dyn Future<Output = T> + Send>>`
- `core::pin::Pin<alloc::boxed::Box<dyn Future<Output = T>>>`

## Functions

### `waker`

```rust
fn waker<W>(wake: alloc::sync::Arc<W>) -> core::task::Waker
where
    W: ArcWake + 'static
```

Creates a [`Waker`](#waker) from an `Arc<impl ArcWake>`.

The returned [`Waker`](#waker) will call
[`ArcWake.wake()`](ArcWake::wake) if awoken.

### `waker_ref`

```rust
fn waker_ref<W>(wake: &alloc::sync::Arc<W>) -> WakerRef<'_>
where
    W: ArcWake + 'static
```

Creates a reference to a [`Waker`](#waker) from a reference to `Arc<impl ArcWake>`.

The resulting [`Waker`](#waker) will call
[`ArcWake.wake()`](ArcWake::wake) if awoken.

### `noop_waker`

```rust
fn noop_waker() -> core::task::Waker
```

Create a new [`Waker`](#waker) which does
nothing when `wake()` is called on it.

# Examples

```rust
use futures::task::noop_waker;
let waker = noop_waker();
waker.wake();
```

### `noop_waker_ref`

```rust
fn noop_waker_ref() -> &'static core::task::Waker
```

Get a static reference to a [`Waker`](#waker) which
does nothing when `wake()` is called on it.

# Examples

```rust
use futures::task::noop_waker_ref;
let waker = noop_waker_ref();
waker.wake_by_ref();
```

### `RawWakerVTable`

```rust
fn RawWakerVTable(&mut self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>
```

