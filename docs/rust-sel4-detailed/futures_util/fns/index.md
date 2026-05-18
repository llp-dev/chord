*[futures_util](../index.md) / [fns](index.md)*

---

# Module `fns`

## Contents

- [Structs](#structs)
  - [`OkFn`](#okfn)
  - [`ChainFn`](#chainfn)
  - [`MergeResultFn`](#mergeresultfn)
  - [`InspectFn`](#inspectfn)
  - [`MapOkFn`](#mapokfn)
  - [`MapErrFn`](#maperrfn)
  - [`InspectOkFn`](#inspectokfn)
  - [`InspectErrFn`](#inspecterrfn)
  - [`UnwrapOrElseFn`](#unwraporelsefn)
  - [`IntoFn`](#intofn)
- [Traits](#traits)
  - [`FnOnce1`](#fnonce1)
  - [`FnMut1`](#fnmut1)
  - [`Fn1`](#fn1)
- [Functions](#functions)
  - [`ok_fn`](#ok-fn)
  - [`chain_fn`](#chain-fn)
  - [`merge_result_fn`](#merge-result-fn)
  - [`inspect_fn`](#inspect-fn)
  - [`map_ok_fn`](#map-ok-fn)
  - [`map_err_fn`](#map-err-fn)
  - [`inspect_ok_fn`](#inspect-ok-fn)
  - [`inspect_err_fn`](#inspect-err-fn)
  - [`map_ok_or_else_fn`](#map-ok-or-else-fn)
  - [`unwrap_or_else_fn`](#unwrap-or-else-fn)
  - [`into_fn`](#into-fn)
- [Type Aliases](#type-aliases)
  - [`MapOkOrElseFn`](#mapokorelsefn)
- [Macros](#macros)
  - [`trivial_fn_impls!`](#trivial-fn-impls)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OkFn`](#okfn) | struct |  |
| [`ChainFn`](#chainfn) | struct |  |
| [`MergeResultFn`](#mergeresultfn) | struct |  |
| [`InspectFn`](#inspectfn) | struct |  |
| [`MapOkFn`](#mapokfn) | struct |  |
| [`MapErrFn`](#maperrfn) | struct |  |
| [`InspectOkFn`](#inspectokfn) | struct |  |
| [`InspectErrFn`](#inspecterrfn) | struct |  |
| [`UnwrapOrElseFn`](#unwraporelsefn) | struct |  |
| [`IntoFn`](#intofn) | struct |  |
| [`FnOnce1`](#fnonce1) | trait |  |
| [`FnMut1`](#fnmut1) | trait |  |
| [`Fn1`](#fn1) | trait |  |
| [`ok_fn`](#ok-fn) | fn |  |
| [`chain_fn`](#chain-fn) | fn |  |
| [`merge_result_fn`](#merge-result-fn) | fn |  |
| [`inspect_fn`](#inspect-fn) | fn |  |
| [`map_ok_fn`](#map-ok-fn) | fn |  |
| [`map_err_fn`](#map-err-fn) | fn |  |
| [`inspect_ok_fn`](#inspect-ok-fn) | fn |  |
| [`inspect_err_fn`](#inspect-err-fn) | fn |  |
| [`map_ok_or_else_fn`](#map-ok-or-else-fn) | fn |  |
| [`unwrap_or_else_fn`](#unwrap-or-else-fn) | fn |  |
| [`into_fn`](#into-fn) | fn |  |
| [`MapOkOrElseFn`](#mapokorelsefn) | type |  |
| [`trivial_fn_impls!`](#trivial-fn-impls) | macro |  |

## Structs

### `OkFn<E>`

```rust
struct OkFn<E>(core::marker::PhantomData<fn(E)>);
```

#### Trait Implementations

##### `impl<T> Clone for OkFn<T>`

- <span id="okfn-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Copy for OkFn<T>`

##### `impl<T> Debug for OkFn<T>`

- <span id="okfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Default for OkFn<E>`

- <span id="okfn-default"></span>`fn default() -> Self`

##### `impl<T, A> Fn1 for OkFn<T>`

- <span id="okfn-fn1-call"></span>`fn call(&self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<T, A> FnMut1 for OkFn<T>`

- <span id="okfn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<A, E> FnOnce1 for OkFn<E>`

- <span id="okfn-fnonce1-type-output"></span>`type Output = Result<A, E>`

- <span id="okfn-fnonce1-call-once"></span>`fn call_once(self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

### `ChainFn<F, G>`

```rust
struct ChainFn<F, G>(F, G);
```

#### Trait Implementations

##### `impl<F: clone::Clone, G: clone::Clone> Clone for ChainFn<F, G>`

- <span id="chainfn-clone"></span>`fn clone(&self) -> ChainFn<F, G>` — [`ChainFn`](#chainfn)

##### `impl<F: marker::Copy, G: marker::Copy> Copy for ChainFn<F, G>`

##### `impl<F: fmt::Debug, G: fmt::Debug> Debug for ChainFn<F, G>`

- <span id="chainfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: default::Default, G: default::Default> Default for ChainFn<F, G>`

- <span id="chainfn-default"></span>`fn default() -> ChainFn<F, G>` — [`ChainFn`](#chainfn)

##### `impl<F, G, A> Fn1 for ChainFn<F, G>`

- <span id="chainfn-fn1-call"></span>`fn call(&self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, G, A> FnMut1 for ChainFn<F, G>`

- <span id="chainfn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, G, A> FnOnce1 for ChainFn<F, G>`

- <span id="chainfn-fnonce1-type-output"></span>`type Output = <G as FnOnce1>::Output`

- <span id="chainfn-fnonce1-call-once"></span>`fn call_once(self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

### `MergeResultFn`

```rust
struct MergeResultFn;
```

#### Trait Implementations

##### `impl Clone for MergeResultFn`

- <span id="mergeresultfn-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for MergeResultFn`

##### `impl Debug for MergeResultFn`

- <span id="mergeresultfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MergeResultFn`

- <span id="mergeresultfn-default"></span>`fn default() -> MergeResultFn` — [`MergeResultFn`](#mergeresultfn)

##### `impl<A> Fn1 for MergeResultFn`

- <span id="mergeresultfn-fn1-call"></span>`fn call(&self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<A> FnMut1 for MergeResultFn`

- <span id="mergeresultfn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<T> FnOnce1 for MergeResultFn`

- <span id="mergeresultfn-fnonce1-type-output"></span>`type Output = T`

- <span id="mergeresultfn-fnonce1-call-once"></span>`fn call_once(self, arg: Result<T, T>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

### `InspectFn<F>`

```rust
struct InspectFn<F>(F);
```

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for InspectFn<F>`

- <span id="inspectfn-clone"></span>`fn clone(&self) -> InspectFn<F>` — [`InspectFn`](#inspectfn)

##### `impl<F: marker::Copy> Copy for InspectFn<F>`

##### `impl<F: fmt::Debug> Debug for InspectFn<F>`

- <span id="inspectfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: default::Default> Default for InspectFn<F>`

- <span id="inspectfn-default"></span>`fn default() -> InspectFn<F>` — [`InspectFn`](#inspectfn)

##### `impl<F, A> Fn1 for InspectFn<F>`

- <span id="inspectfn-fn1-call"></span>`fn call(&self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, A> FnMut1 for InspectFn<F>`

- <span id="inspectfn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, A> FnOnce1 for InspectFn<F>`

- <span id="inspectfn-fnonce1-type-output"></span>`type Output = A`

- <span id="inspectfn-fnonce1-call-once"></span>`fn call_once(self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

### `MapOkFn<F>`

```rust
struct MapOkFn<F>(F);
```

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for MapOkFn<F>`

- <span id="mapokfn-clone"></span>`fn clone(&self) -> MapOkFn<F>` — [`MapOkFn`](#mapokfn)

##### `impl<F: marker::Copy> Copy for MapOkFn<F>`

##### `impl<F: fmt::Debug> Debug for MapOkFn<F>`

- <span id="mapokfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: default::Default> Default for MapOkFn<F>`

- <span id="mapokfn-default"></span>`fn default() -> MapOkFn<F>` — [`MapOkFn`](#mapokfn)

##### `impl<F, T, E> Fn1 for MapOkFn<F>`

- <span id="mapokfn-fn1-call"></span>`fn call(&self, arg: Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnMut1 for MapOkFn<F>`

- <span id="mapokfn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnOnce1 for MapOkFn<F>`

- <span id="mapokfn-fnonce1-type-output"></span>`type Output = Result<<F as FnOnce1>::Output, E>`

- <span id="mapokfn-fnonce1-call-once"></span>`fn call_once(self, arg: Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

### `MapErrFn<F>`

```rust
struct MapErrFn<F>(F);
```

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for MapErrFn<F>`

- <span id="maperrfn-clone"></span>`fn clone(&self) -> MapErrFn<F>` — [`MapErrFn`](#maperrfn)

##### `impl<F: marker::Copy> Copy for MapErrFn<F>`

##### `impl<F: fmt::Debug> Debug for MapErrFn<F>`

- <span id="maperrfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: default::Default> Default for MapErrFn<F>`

- <span id="maperrfn-default"></span>`fn default() -> MapErrFn<F>` — [`MapErrFn`](#maperrfn)

##### `impl<F, T, E> Fn1 for MapErrFn<F>`

- <span id="maperrfn-fn1-call"></span>`fn call(&self, arg: Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnMut1 for MapErrFn<F>`

- <span id="maperrfn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnOnce1 for MapErrFn<F>`

- <span id="maperrfn-fnonce1-type-output"></span>`type Output = Result<T, <F as FnOnce1>::Output>`

- <span id="maperrfn-fnonce1-call-once"></span>`fn call_once(self, arg: Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

### `InspectOkFn<F>`

```rust
struct InspectOkFn<F>(F);
```

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for InspectOkFn<F>`

- <span id="inspectokfn-clone"></span>`fn clone(&self) -> InspectOkFn<F>` — [`InspectOkFn`](#inspectokfn)

##### `impl<F: marker::Copy> Copy for InspectOkFn<F>`

##### `impl<F: fmt::Debug> Debug for InspectOkFn<F>`

- <span id="inspectokfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F, T, E> Fn1 for InspectOkFn<F>`

- <span id="inspectokfn-fn1-call"></span>`fn call(&self, arg: &'a Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnMut1 for InspectOkFn<F>`

- <span id="inspectokfn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: &'a Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnOnce1 for InspectOkFn<F>`

- <span id="inspectokfn-fnonce1-type-output"></span>`type Output = ()`

- <span id="inspectokfn-fnonce1-call-once"></span>`fn call_once(self, arg: &'a Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

### `InspectErrFn<F>`

```rust
struct InspectErrFn<F>(F);
```

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for InspectErrFn<F>`

- <span id="inspecterrfn-clone"></span>`fn clone(&self) -> InspectErrFn<F>` — [`InspectErrFn`](#inspecterrfn)

##### `impl<F: marker::Copy> Copy for InspectErrFn<F>`

##### `impl<F: fmt::Debug> Debug for InspectErrFn<F>`

- <span id="inspecterrfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F, T, E> Fn1 for InspectErrFn<F>`

- <span id="inspecterrfn-fn1-call"></span>`fn call(&self, arg: &'a Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnMut1 for InspectErrFn<F>`

- <span id="inspecterrfn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: &'a Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnOnce1 for InspectErrFn<F>`

- <span id="inspecterrfn-fnonce1-type-output"></span>`type Output = ()`

- <span id="inspecterrfn-fnonce1-call-once"></span>`fn call_once(self, arg: &'a Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

### `UnwrapOrElseFn<F>`

```rust
struct UnwrapOrElseFn<F>(F);
```

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for UnwrapOrElseFn<F>`

- <span id="unwraporelsefn-clone"></span>`fn clone(&self) -> UnwrapOrElseFn<F>` — [`UnwrapOrElseFn`](#unwraporelsefn)

##### `impl<F: marker::Copy> Copy for UnwrapOrElseFn<F>`

##### `impl<F: fmt::Debug> Debug for UnwrapOrElseFn<F>`

- <span id="unwraporelsefn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: default::Default> Default for UnwrapOrElseFn<F>`

- <span id="unwraporelsefn-default"></span>`fn default() -> UnwrapOrElseFn<F>` — [`UnwrapOrElseFn`](#unwraporelsefn)

##### `impl<F, T, E> Fn1 for UnwrapOrElseFn<F>`

- <span id="unwraporelsefn-fn1-call"></span>`fn call(&self, arg: Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnMut1 for UnwrapOrElseFn<F>`

- <span id="unwraporelsefn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<F, T, E> FnOnce1 for UnwrapOrElseFn<F>`

- <span id="unwraporelsefn-fnonce1-type-output"></span>`type Output = T`

- <span id="unwraporelsefn-fnonce1-call-once"></span>`fn call_once(self, arg: Result<T, E>) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

### `IntoFn<T>`

```rust
struct IntoFn<T>(core::marker::PhantomData<fn() -> T>);
```

#### Trait Implementations

##### `impl<T> Clone for IntoFn<T>`

- <span id="intofn-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Copy for IntoFn<T>`

##### `impl<T> Debug for IntoFn<T>`

- <span id="intofn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IntoFn<T>`

- <span id="intofn-default"></span>`fn default() -> Self`

##### `impl<T, A> Fn1 for IntoFn<T>`

- <span id="intofn-fn1-call"></span>`fn call(&self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<T, A> FnMut1 for IntoFn<T>`

- <span id="intofn-fnmut1-call-mut"></span>`fn call_mut(&mut self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

##### `impl<A, T> FnOnce1 for IntoFn<T>`

- <span id="intofn-fnonce1-type-output"></span>`type Output = T`

- <span id="intofn-fnonce1-call-once"></span>`fn call_once(self, arg: A) -> <Self as >::Output` — [`FnOnce1`](#fnonce1)

## Traits

### `FnOnce1<A>`

```rust
trait FnOnce1<A> { ... }
```

#### Associated Types

- `type Output`

#### Required Methods

- `fn call_once(self, arg: A) -> <Self as >::Output`

#### Implementors

- [`ChainFn`](#chainfn)
- [`InspectErrFn`](#inspecterrfn)
- [`InspectFn`](#inspectfn)
- [`InspectOkFn`](#inspectokfn)
- [`IntoFn`](#intofn)
- [`MapErrFn`](#maperrfn)
- [`MapOkFn`](#mapokfn)
- [`MergeResultFn`](#mergeresultfn)
- [`NextIfEqFn`](../stream/stream/peek/index.md#nextifeqfn)
- [`OkFn`](#okfn)
- [`UnwrapOrElseFn`](#unwraporelsefn)
- `T`

### `FnMut1<A>`

```rust
trait FnMut1<A>: FnOnce1<A> { ... }
```

#### Required Methods

- `fn call_mut(&mut self, arg: A) -> <Self as >::Output`

#### Implementors

- [`ChainFn`](#chainfn)
- [`InspectErrFn`](#inspecterrfn)
- [`InspectFn`](#inspectfn)
- [`InspectOkFn`](#inspectokfn)
- [`IntoFn`](#intofn)
- [`MapErrFn`](#maperrfn)
- [`MapOkFn`](#mapokfn)
- [`MergeResultFn`](#mergeresultfn)
- [`OkFn`](#okfn)
- [`UnwrapOrElseFn`](#unwraporelsefn)
- `T`

### `Fn1<A>`

```rust
trait Fn1<A>: FnMut1<A> { ... }
```

#### Required Methods

- `fn call(&self, arg: A) -> <Self as >::Output`

#### Implementors

- [`ChainFn`](#chainfn)
- [`InspectErrFn`](#inspecterrfn)
- [`InspectFn`](#inspectfn)
- [`InspectOkFn`](#inspectokfn)
- [`IntoFn`](#intofn)
- [`MapErrFn`](#maperrfn)
- [`MapOkFn`](#mapokfn)
- [`MergeResultFn`](#mergeresultfn)
- [`OkFn`](#okfn)
- [`UnwrapOrElseFn`](#unwraporelsefn)
- `T`

## Functions

### `ok_fn`

```rust
fn ok_fn<T>() -> OkFn<T>
```

### `chain_fn`

```rust
fn chain_fn<F, G>(f: F, g: G) -> ChainFn<F, G>
```

### `merge_result_fn`

```rust
fn merge_result_fn() -> MergeResultFn
```

### `inspect_fn`

```rust
fn inspect_fn<F>(f: F) -> InspectFn<F>
```

### `map_ok_fn`

```rust
fn map_ok_fn<F>(f: F) -> MapOkFn<F>
```

### `map_err_fn`

```rust
fn map_err_fn<F>(f: F) -> MapErrFn<F>
```

### `inspect_ok_fn`

```rust
fn inspect_ok_fn<F>(f: F) -> InspectOkFn<F>
```

### `inspect_err_fn`

```rust
fn inspect_err_fn<F>(f: F) -> InspectErrFn<F>
```

### `map_ok_or_else_fn`

```rust
fn map_ok_or_else_fn<F, G>(f: F, g: G) -> ChainFn<MapOkFn<F>, ChainFn<MapErrFn<G>, MergeResultFn>>
```

### `unwrap_or_else_fn`

```rust
fn unwrap_or_else_fn<F>(f: F) -> UnwrapOrElseFn<F>
```

### `into_fn`

```rust
fn into_fn<T>() -> IntoFn<T>
```

## Type Aliases

### `MapOkOrElseFn<F, G>`

```rust
type MapOkOrElseFn<F, G> = ChainFn<MapOkFn<F>, ChainFn<MapErrFn<G>, MergeResultFn>>;
```

## Macros

### `trivial_fn_impls!`

