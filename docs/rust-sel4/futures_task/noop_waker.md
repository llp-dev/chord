**futures_task > noop_waker**

# Module: noop_waker

## Contents

**Functions**

- [`noop_waker`](#noop_waker) - Create a new [`Waker`] which does
- [`noop_waker_ref`](#noop_waker_ref) - Get a static reference to a [`Waker`] which

---

## futures_task::noop_waker::noop_waker

*Function*

Create a new [`Waker`] which does
nothing when `wake()` is called on it.

# Examples

```
use futures::task::noop_waker;
let waker = noop_waker();
waker.wake();
```

```rust
fn noop_waker() -> core::task::Waker
```



## futures_task::noop_waker::noop_waker_ref

*Function*

Get a static reference to a [`Waker`] which
does nothing when `wake()` is called on it.

# Examples

```
use futures::task::noop_waker_ref;
let waker = noop_waker_ref();
waker.wake_by_ref();
```

```rust
fn noop_waker_ref() -> &'static core::task::Waker
```



