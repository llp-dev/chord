**scopeguard**

# Module: scopeguard

## Contents

**Macros**

- [`defer`](#defer) - Macro to create a `ScopeGuard` (always run).

**Structs**

- [`ScopeGuard`](#scopeguard) - `ScopeGuard` is a scope guard that may own a protected value.

**Enums**

- [`Always`](#always) - Always run on scope exit.

**Functions**

- [`guard`](#guard) - Create a new `ScopeGuard` owning `v` and with deferred closure `dropfn`.

**Traits**

- [`Strategy`](#strategy) - Controls in which cases the associated code should be run

---

## scopeguard::Always

*Enum*

Always run on scope exit.

“Always” run: on regular exit from a scope or on unwinding from a panic.
Can not run on abort, process exit, and other catastrophic events where
destructors don’t run.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Strategy**
  - `fn should_run() -> bool`



## scopeguard::ScopeGuard

*Struct*

`ScopeGuard` is a scope guard that may own a protected value.

If you place a guard in a local variable, the closure can
run regardless how you leave the scope — through regular return or panic
(except if panic or other code aborts; so as long as destructors run).
It is run only once.

The `S` parameter for [`Strategy`](trait.Strategy.html) determines if
the closure actually runs.

The guard's closure will be called with the held value in the destructor.

The `ScopeGuard` implements `Deref` so that you can access the inner value.

**Generic Parameters:**
- T
- F
- S

**Methods:**

- `fn with_strategy(v: T, dropfn: F) -> ScopeGuard<T, F, S>` - Create a `ScopeGuard` that owns `v` (accessible through deref) and calls
- `fn into_inner(guard: Self) -> T` - “Defuse” the guard and extract the value without calling the closure.

**Traits:** Sync

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Drop**
  - `fn drop(self: & mut Self)`



## scopeguard::Strategy

*Trait*

Controls in which cases the associated code should be run

**Methods:**

- `should_run`: Return `true` if the guard’s associated code should run



## scopeguard::defer

*Declarative Macro*

Macro to create a `ScopeGuard` (always run).

The macro takes statements, which are the body of a closure
that will run when the scope is exited.

```rust
macro_rules! defer {
    ($($t:tt)*) => { ... };
}
```



## scopeguard::guard

*Function*

Create a new `ScopeGuard` owning `v` and with deferred closure `dropfn`.

```rust
fn guard<T, F>(v: T, dropfn: F) -> ScopeGuard<T, F, Always>
```



