# Crate `scopeguard`

A scope guard will run a given closure when it goes out of scope,
even if the code between panics.
(as long as panic doesn't abort)

# Examples

## Hello World

This example creates a scope guard with an example function:

```rust
extern crate scopeguard;

fn f() {
    let _guard = scopeguard::guard((), |_| {
        println!("Hello Scope Exit!");
    });

    // rest of the code here.

    // Here, at the end of `_guard`'s scope, the guard's closure is called.
    // It is also called if we exit this scope through unwinding instead.
}
fn main() {
   f();
}
```

## `defer!`

Use the `defer` macro to run an operation at scope exit,
either regular scope exit or during unwinding from a panic.

```rust
#[macro_use(defer)] extern crate scopeguard;

use std::cell::Cell;

fn main() {
    // use a cell to observe drops during and after the scope guard is active
    let drop_counter = Cell::new(0);
    {
        // Create a scope guard using `defer!` for the current scope
        defer! {
            drop_counter.set(1 + drop_counter.get());
        }

        // Do regular operations here in the meantime.

        // Just before scope exit: it hasn't run yet.
        assert_eq!(drop_counter.get(), 0);

        // The following scope end is where the defer closure is called
    }
    assert_eq!(drop_counter.get(), 1);
}
```

## Scope Guard with Value

If the scope guard closure needs to access an outer value that is also
mutated outside of the scope guard, then you may want to use the scope guard
with a value. The guard works like a smart pointer, so the inner value can
be accessed by reference or by mutable reference.

### 1. The guard owns a file

In this example, the scope guard owns a file and ensures pending writes are
synced at scope exit.

```rust
extern crate scopeguard;

use std::fs::*;
use std::io::{self, Write};
// Mock file so that we don't actually write a file
struct MockFile;
impl MockFile {
    fn create(_s: &str) -> io::Result<Self> { Ok(MockFile) }
    fn write_all(&self, _b: &[u8]) -> io::Result<()> { Ok(()) }
    fn sync_all(&self) -> io::Result<()> { Ok(()) }
}
use self::MockFile as File;

fn try_main() -> io::Result<()> {
    let f = File::create("newfile.txt")?;
    let mut file = scopeguard::guard(f, |f| {
        // ensure we flush file at return or panic
        let _ = f.sync_all();
    });
    // Access the file through the scope guard itself
    file.write_all(b"test me\n").map(|_| ())
}

fn main() {
    try_main().unwrap();
}

```

### 2. The guard restores an invariant on scope exit

```rust
extern crate scopeguard;

use std::mem::ManuallyDrop;
use std::ptr;

// This function, just for this example, takes the first element
// and inserts it into the assumed sorted tail of the vector.
//
// For optimization purposes we temporarily violate an invariant of the
// Vec, that it owns all of its elements.
//
// The safe approach is to use swap, which means two writes to memory,
// the optimization is to use a “hole” which uses only one write of memory
// for each position it moves.
//
// We *must* use a scope guard to run this code safely. We
// are running arbitrary user code (comparison operators) that may panic.
// The scope guard ensures we restore the invariant after successful
// exit or during unwinding from panic.
fn insertion_sort_first<T>(v: &mut Vec<T>)
    where T: PartialOrd
{
    struct Hole<'a, T: 'a> {
        v: &'a mut Vec<T>,
        index: usize,
        value: ManuallyDrop<T>,
    }

    unsafe {
        // Create a moved-from location in the vector, a “hole”.
        let value = ptr::read(&v[0]);
        let mut hole = Hole { v: v, index: 0, value: ManuallyDrop::new(value) };

        // Use a scope guard with a value.
        // At scope exit, plug the hole so that the vector is fully
        // initialized again.
        // The scope guard owns the hole, but we can access it through the guard.
        let mut hole_guard = scopeguard::guard(hole, |hole| {
            // plug the hole in the vector with the value that was // taken out
            let index = hole.index;
            ptr::copy_nonoverlapping(&*hole.value, &mut hole.v[index], 1);
        });

        // run algorithm that moves the hole in the vector here
        // move the hole until it's in a sorted position
        for i in 1..hole_guard.v.len() {
            if *hole_guard.value >= hole_guard.v[i] {
                // move the element back and the hole forward
                let index = hole_guard.index;
                hole_guard.v.swap(index, index + 1);
                hole_guard.index += 1;
            } else {
                break;
            }
        }

        // When the scope exits here, the Vec becomes whole again!
    }
}

fn main() {
    let string = String::from;
    let mut data = vec![string("c"), string("a"), string("b"), string("d")];
    insertion_sort_first(&mut data);
    assert_eq!(data, vec!["a", "b", "c", "d"]);
}

```


# Crate Features

- `use_std`
  + Enabled by default. Enables the `OnUnwind` and `OnSuccess` strategies.
  + Disable to use `no_std`.

# Rust Version

This version of the crate requires Rust 1.20 or later.

The scopeguard 1.x release series will use a carefully considered version
upgrade policy, where in a later 1.x version, we will raise the minimum
required Rust version.

## Contents

- [Structs](#structs)
  - [`ScopeGuard`](#scopeguard)
- [Enums](#enums)
  - [`Always`](#always)
- [Traits](#traits)
  - [`Strategy`](#strategy)
- [Functions](#functions)
  - [`guard`](#guard)
- [Macros](#macros)
  - [`defer!`](#defer)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ScopeGuard`](#scopeguard) | struct | `ScopeGuard` is a scope guard that may own a protected value. |
| [`Always`](#always) | enum | Always run on scope exit. |
| [`Strategy`](#strategy) | trait | Controls in which cases the associated code should be run |
| [`guard`](#guard) | fn | Create a new `ScopeGuard` owning `v` and with deferred closure `dropfn`. |
| [`defer!`](#defer) | macro | Macro to create a `ScopeGuard` (always run). |

## Structs

### `ScopeGuard<T, F, S>`

```rust
struct ScopeGuard<T, F, S>
where
    F: FnOnce(T),
    S: Strategy {
    value: std::mem::ManuallyDrop<T>,
    dropfn: std::mem::ManuallyDrop<F>,
    strategy: std::marker::PhantomData<fn(S) -> S>,
}
```

`ScopeGuard` is a scope guard that may own a protected value.

If you place a guard in a local variable, the closure can
run regardless how you leave the scope — through regular return or panic
(except if panic or other code aborts; so as long as destructors run).
It is run only once.

The `S` parameter for [`Strategy`](#strategy) determines if
the closure actually runs.

The guard's closure will be called with the held value in the destructor.

The `ScopeGuard` implements `Deref` so that you can access the inner value.

#### Implementations

- <span id="scopeguard-with-strategy"></span>`fn with_strategy(v: T, dropfn: F) -> ScopeGuard<T, F, S>` — [`ScopeGuard`](#scopeguard)

  Create a `ScopeGuard` that owns `v` (accessible through deref) and calls

  `dropfn` when its destructor runs.

  

  The `Strategy` decides whether the scope guard's closure should run.

- <span id="scopeguard-into-inner"></span>`fn into_inner(guard: Self) -> T`

  “Defuse” the guard and extract the value without calling the closure.

  

  ```rust

  extern crate scopeguard;

  

  use scopeguard::{guard, ScopeGuard};

  

  fn conditional() -> bool { true }

  

  fn main() {

      let mut guard = guard(Vec::new(), |mut v| v.clear());

      guard.push(1);

  

      if conditional() {

          // a condition maybe makes us decide to

          // “defuse” the guard and get back its inner parts

          let value = ScopeGuard::into_inner(guard);

      } else {

          // guard still exists in this branch

      }

  }

  ```

#### Trait Implementations

##### `impl<T, F, S> Debug for ScopeGuard<T, F, S>`

- <span id="scopeguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F, S> Deref for ScopeGuard<T, F, S>`

- <span id="scopeguard-deref-type-target"></span>`type Target = T`

- <span id="scopeguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<T, F, S> DerefMut for ScopeGuard<T, F, S>`

- <span id="scopeguard-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T, F, S> Drop for ScopeGuard<T, F, S>`

- <span id="scopeguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for ScopeGuard<T, F, S>`

- <span id="scopeguard-receiver-type-target"></span>`type Target = T`

##### `impl<T, F, S> Sync for ScopeGuard<T, F, S>`

## Enums

### `Always`

```rust
enum Always {
}
```

Always run on scope exit.

“Always” run: on regular exit from a scope or on unwinding from a panic.
Can not run on abort, process exit, and other catastrophic events where
destructors don’t run.

#### Trait Implementations

##### `impl Debug for Always`

- <span id="always-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Strategy for Always`

- <span id="always-strategy-should-run"></span>`fn should_run() -> bool`

## Traits

### `Strategy`

```rust
trait Strategy { ... }
```

Controls in which cases the associated code should be run

#### Required Methods

- `fn should_run() -> bool`

  Return `true` if the guard’s associated code should run

#### Implementors

- [`Always`](#always)

## Functions

### `guard`

```rust
fn guard<T, F>(v: T, dropfn: F) -> ScopeGuard<T, F, Always>
where
    F: FnOnce(T)
```

Create a new `ScopeGuard` owning `v` and with deferred closure `dropfn`.

## Macros

### `defer!`

Macro to create a `ScopeGuard` (always run).

The macro takes statements, which are the body of a closure
that will run when the scope is exited.

