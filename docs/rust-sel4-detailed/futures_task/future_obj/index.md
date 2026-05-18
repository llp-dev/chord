*[futures_task](../index.md) / [future_obj](index.md)*

---

# Module `future_obj`

## Contents

- [Modules](#modules)
  - [`if_alloc`](#if-alloc)
- [Structs](#structs)
  - [`LocalFutureObj`](#localfutureobj)
  - [`FutureObj`](#futureobj)
- [Traits](#traits)
  - [`UnsafeFutureObj`](#unsafefutureobj)
- [Functions](#functions)
  - [`remove_future_lifetime`](#remove-future-lifetime)
  - [`remove_drop_lifetime`](#remove-drop-lifetime)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`if_alloc`](#if-alloc) | mod |  |
| [`LocalFutureObj`](#localfutureobj) | struct | A custom trait object for polling futures, roughly akin to `Box<dyn Future<Output = T> + 'a>`. |
| [`FutureObj`](#futureobj) | struct | A custom trait object for polling futures, roughly akin to `Box<dyn Future<Output = T> + Send + 'a>`. |
| [`UnsafeFutureObj`](#unsafefutureobj) | trait | A custom implementation of a future trait object for `FutureObj`, providing a vtable with drop support. |
| [`remove_future_lifetime`](#remove-future-lifetime) | fn |  |
| [`remove_drop_lifetime`](#remove-drop-lifetime) | fn |  |

## Modules

- [`if_alloc`](if_alloc/index.md)

## Structs

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

- <span id="localfutureobj-into-future-obj"></span>`unsafe fn into_future_obj(self) -> FutureObj<'a, T>` — [`FutureObj`](#futureobj)

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

- <span id="localfutureobj-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../index.md#context), [`Poll`](../index.md#poll)

##### `impl IntoFuture for LocalFutureObj<'a, T>`

- <span id="localfutureobj-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="localfutureobj-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="localfutureobj-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> Unpin for LocalFutureObj<'_, T>`

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

- <span id="futureobj-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../index.md#context), [`Poll`](../index.md#poll)

##### `impl IntoFuture for FutureObj<'a, T>`

- <span id="futureobj-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="futureobj-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="futureobj-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> Send for FutureObj<'_, T>`

##### `impl<T> Unpin for FutureObj<'_, T>`

## Traits

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

### `remove_future_lifetime`

```rust
unsafe fn remove_future_lifetime<'a, T>(ptr: *mut dyn Future<Output = T>) -> *mut dyn Future<Output = T>
```

### `remove_drop_lifetime`

```rust
unsafe fn remove_drop_lifetime<'a, T>(ptr: fn(*mut dyn Future<Output = T>)) -> fn(*mut dyn Future<Output = T>)
```

