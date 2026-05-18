**anyhow**

# Module: anyhow

## Contents

**Macros**

- [`anyhow`](#anyhow) - Construct an ad-hoc error from a string or existing non-`anyhow` error
- [`bail`](#bail) - Return early with an error.
- [`ensure`](#ensure) - Return early with an error if a condition is not satisfied.

**Structs**

- [`Chain`](#chain) - Iterator of a chain of source errors.
- [`Error`](#error) - The `Error` type, a wrapper around a dynamic error type.

**Functions**

- [`Ok`](#ok) - Equivalent to `Ok::<_, anyhow::Error>(value)`.

**Traits**

- [`Context`](#context) - Provides the `context` method for `Result`.

**Type Aliases**

- [`Result`](#result) - `Result<T, Error>`

---

## anyhow::Chain

*Struct*

Iterator of a chain of source errors.

This type is the iterator returned by [`Error::chain`].

# Example

```
use anyhow::Error;
use std::io;

pub fn underlying_io_error_kind(error: &Error) -> Option<io::ErrorKind> {
    for cause in error.chain() {
        if let Some(io_error) = cause.downcast_ref::<io::Error>() {
            return Some(io_error.kind());
        }
    }
    None
}
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(head: &'a dyn StdError) -> Self`

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Chain<'a>`
- **Default**
  - `fn default() -> Self`



## anyhow::Context

*Trait*

Provides the `context` method for `Result`.

This trait is sealed and cannot be implemented for types outside of
`anyhow`.

<br>

# Example

```
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    # const IGNORE: &'static str = stringify! {
    pub fn detach(&mut self) -> Result<()> {...}
    # };
    # fn detach(&mut self) -> Result<()> {
    #     unimplemented!()
    # }
}

pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
    it.detach().context("Failed to detach the important thing")?;

    let path = &it.path;
    let content = fs::read(path)
        .with_context(|| format!("Failed to read instrs from {}", path.display()))?;

    Ok(content)
}
```

When printed, the outermost context would be printed first and the lower
level underlying causes would be enumerated below.

```console
Error: Failed to read instrs from ./path/to/instrs.json

Caused by:
    No such file or directory (os error 2)
```

Refer to the [Display representations] documentation for other forms in
which this context chain can be rendered.

[Display representations]: Error#display-representations

<br>

# Effect on downcasting

After attaching context of type `C` onto an error of type `E`, the resulting
`anyhow::Error` may be downcast to `C` **or** to `E`.

That is, in codebases that rely on downcasting, Anyhow's context supports
both of the following use cases:

  - **Attaching context whose type is insignificant onto errors whose type
    is used in downcasts.**

    In other error libraries whose context is not designed this way, it can
    be risky to introduce context to existing code because new context might
    break existing working downcasts. In Anyhow, any downcast that worked
    before adding context will continue to work after you add a context, so
    you should freely add human-readable context to errors wherever it would
    be helpful.

    ```
    # use anyhow::bail;
    # use thiserror::Error;
    #
    # #[derive(Error, Debug)]
    # #[error("???")]
    # struct SuspiciousError;
    #
    # fn helper() -> Result<()> {
    #     bail!(SuspiciousError);
    # }
    #
    use anyhow::{Context, Result};

    fn do_it() -> Result<()> {
        helper().context("Failed to complete the work")?;
        # const IGNORE: &str = stringify! {
        ...
        # };
        # unreachable!()
    }

    fn main() {
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<SuspiciousError>() {
            // If helper() returned SuspiciousError, this downcast will
            // correctly succeed even with the context in between.
            # return;
        }
        # panic!("expected downcast to succeed");
    }
    ```

  - **Attaching context whose type is used in downcasts onto errors whose
    type is insignificant.**

    Some codebases prefer to use machine-readable context to categorize
    lower level errors in a way that will be actionable to higher levels of
    the application.

    ```
    # use anyhow::bail;
    # use thiserror::Error;
    #
    # #[derive(Error, Debug)]
    # #[error("???")]
    # struct HelperFailed;
    #
    # fn helper() -> Result<()> {
    #     bail!("no such file or directory");
    # }
    #
    use anyhow::{Context, Result};

    fn do_it() -> Result<()> {
        helper().context(HelperFailed)?;
        # const IGNORE: &str = stringify! {
        ...
        # };
        # unreachable!()
    }

    fn main() {
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<HelperFailed>() {
            // If helper failed, this downcast will succeed because
            // HelperFailed is the context that has been attached to
            // that error.
            # return;
        }
        # panic!("expected downcast to succeed");
    }
    ```

**Methods:**

- `context`: Wrap the error value with additional context.
- `with_context`: Wrap the error value with additional context that is evaluated lazily



## anyhow::Error

*Struct*

The `Error` type, a wrapper around a dynamic error type.

`Error` works a lot like `Box<dyn std::error::Error>`, but with these
differences:

- `Error` requires that the error is `Send`, `Sync`, and `'static`.
- `Error` guarantees that a backtrace is available, even if the underlying
  error type does not provide one.
- `Error` is represented as a narrow pointer &mdash; exactly one word in
  size instead of two.

<br>

# Display representations

When you print an error object using "{}" or to_string(), only the outermost
underlying error or context is printed, not any of the lower level causes.
This is exactly as if you had called the Display impl of the error from
which you constructed your anyhow::Error.

```console
Failed to read instrs from ./path/to/instrs.json
```

To print causes as well using anyhow's default formatting of causes, use the
alternate selector "{:#}".

```console
Failed to read instrs from ./path/to/instrs.json: No such file or directory (os error 2)
```

The Debug format "{:?}" includes your backtrace if one was captured. Note
that this is the representation you get by default if you return an error
from `fn main` instead of printing it explicitly yourself.

```console
Error: Failed to read instrs from ./path/to/instrs.json

Caused by:
    No such file or directory (os error 2)
```

and if there is a backtrace available:

```console
Error: Failed to read instrs from ./path/to/instrs.json

Caused by:
    No such file or directory (os error 2)

Stack backtrace:
   0: <E as anyhow::context::ext::StdError>::ext_context
             at /git/anyhow/src/backtrace.rs:26
   1: core::result::Result<T,E>::map_err
             at /git/rustc/src/libcore/result.rs:596
   2: anyhow::context::<impl anyhow::Context<T,E> for core::result::Result<T,E>>::with_context
             at /git/anyhow/src/context.rs:58
   3: testing::main
             at src/main.rs:5
   4: std::rt::lang_start
             at /git/rustc/src/libstd/rt.rs:61
   5: main
   6: __libc_start_main
   7: _start
```

To see a conventional struct-style Debug representation, use "{:#?}".

```console
Error {
    context: "Failed to read instrs from ./path/to/instrs.json",
    source: Os {
        code: 2,
        kind: NotFound,
        message: "No such file or directory",
    },
}
```

If none of the built-in representations are appropriate and you would prefer
to render the error and its cause chain yourself, it can be done something
like this:

```
use anyhow::{Context, Result};

fn main() {
    if let Err(err) = try_main() {
        eprintln!("ERROR: {}", err);
        err.chain().skip(1).for_each(|cause| eprintln!("because: {}", cause));
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    # const IGNORE: &str = stringify! {
    ...
    # };
    # Ok(())
}
```

**Methods:**

- `fn new<E>(error: E) -> Self` - Create a new error object from any error type.
- `fn msg<M>(message: M) -> Self` - Create a new error object from a printable error message.
- `fn from_boxed(boxed_error: Box<dyn StdError>) -> Self` - Construct an error object from a type-erased standard library error.
- `fn context<C>(self: Self, context: C) -> Self` - Wrap the error value with additional context.
- `fn backtrace(self: &Self) -> &Backtrace` - Get the backtrace for this Error.
- `fn chain(self: &Self) -> Chain` - An iterator of the chain of source errors contained by this Error.
- `fn root_cause(self: &Self) -> &dyn StdError` - The lowest level cause of this error &mdash; this error's cause's
- `fn is<E>(self: &Self) -> bool` - Returns true if `E` is the type held by this error object.
- `fn downcast<E>(self: Self) -> Result<E, Self>` - Attempt to downcast the error object to a concrete type.
- `fn downcast_ref<E>(self: &Self) -> Option<&E>` - Downcast this error object by reference.
- `fn downcast_mut<E>(self: & mut Self) -> Option<& mut E>` - Downcast this error object by mutable reference.
- `fn into_boxed_dyn_error(self: Self) -> Box<dyn StdError>` - Convert to a standard library error trait object.
- `fn reallocate_into_boxed_dyn_error_without_backtrace(self: Self) -> Box<dyn StdError>` - Convert to a standard library error trait object.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Drop**
  - `fn drop(self: & mut Self)`
- **AsRef**
  - `fn as_ref(self: &Self) -> &dyn StdError`
- **From**
  - `fn from(error: E) -> Self`
- **Debug**
  - `fn fmt(self: &Self, formatter: & mut fmt::Formatter) -> fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &dyn StdError`



## anyhow::Ok

*Function*

Equivalent to `Ok::<_, anyhow::Error>(value)`.

This simplifies creation of an `anyhow::Result` in places where type
inference cannot deduce the `E` type of the result &mdash; without needing
to write `Ok::<_, anyhow::Error>(value)`.

One might think that `anyhow::Result::Ok(value)` would work in such cases
but it does not.

```console
error[E0282]: type annotations needed for `std::result::Result<i32, E>`
  --> src/main.rs:11:13
   |
11 |     let _ = anyhow::Result::Ok(1);
   |         -   ^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `E` declared on the enum `Result`
   |         |
   |         consider giving this pattern the explicit type `std::result::Result<i32, E>`, where the type parameter `E` is specified
```

```rust
fn Ok<T>(value: T) -> Result<T>
```



## anyhow::Result

*Type Alias*: `core::result::Result<T, E>`

`Result<T, Error>`

This is a reasonable return type to use throughout your application but also
for `fn main`; if you do, failures will be printed along with any
[context][Context] and a backtrace if one was captured.

`anyhow::Result` may be used with one *or* two type parameters.

```rust
use anyhow::Result;

# const IGNORE: &str = stringify! {
fn demo1() -> Result<T> {...}
           // ^ equivalent to std::result::Result<T, anyhow::Error>

fn demo2() -> Result<T, OtherError> {...}
           // ^ equivalent to std::result::Result<T, OtherError>
# };
```

# Example

```
# pub trait Deserialize {}
#
# mod serde_json {
#     use super::Deserialize;
#     use std::io;
#
#     pub fn from_str<T: Deserialize>(json: &str) -> io::Result<T> {
#         unimplemented!()
#     }
# }
#
# #[derive(Debug)]
# struct ClusterMap;
#
# impl Deserialize for ClusterMap {}
#
use anyhow::Result;

fn main() -> Result<()> {
    # return Ok(());
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    println!("cluster info: {:#?}", map);
    Ok(())
}
```



## anyhow::anyhow

*Declarative Macro*

Construct an ad-hoc error from a string or existing non-`anyhow` error
value.

This evaluates to an [`Error`][crate::Error]. It can take either just a
string, or a format string with arguments. It also can take any custom type
which implements `Debug` and `Display`.

If called with a single argument whose type implements `std::error::Error`
(in addition to `Debug` and `Display`, which are always required), then that
Error impl's `source` is preserved as the `source` of the resulting
`anyhow::Error`.

# Example

```
# type V = ();
#
use anyhow::{anyhow, Result};

fn lookup(key: &str) -> Result<V> {
    if key.len() != 16 {
        return Err(anyhow!("key length must be 16 characters, got {:?}", key));
    }

    // ...
    # Ok(())
}
```

```rust
macro_rules! anyhow {
    ($msg:literal $(,)?) => { ... };
    ($err:expr $(,)?) => { ... };
    ($fmt:expr, $($arg:tt)*) => { ... };
}
```



## anyhow::bail

*Declarative Macro*

Return early with an error.

This macro is equivalent to
<code>return Err([anyhow!($args\...)][anyhow!])</code>.

The surrounding function's or closure's return value is required to be
<code>Result&lt;_, [anyhow::Error][crate::Error]&gt;</code>.

[anyhow!]: crate::anyhow

# Example

```
# use anyhow::{bail, Result};
#
# fn has_permission(user: usize, resource: usize) -> bool {
#     true
# }
#
# fn main() -> Result<()> {
#     let user = 0;
#     let resource = 0;
#
if !has_permission(user, resource) {
    bail!("permission denied for accessing {}", resource);
}
#     Ok(())
# }
```

```
# use anyhow::{bail, Result};
# use thiserror::Error;
#
# const MAX_DEPTH: usize = 1;
#
#[derive(Error, Debug)]
enum ScienceError {
    #[error("recursion limit exceeded")]
    RecursionLimitExceeded,
    # #[error("...")]
    # More = (stringify! {
    ...
    # }, 1).1,
}

# fn main() -> Result<()> {
#     let depth = 0;
#
if depth > MAX_DEPTH {
    bail!(ScienceError::RecursionLimitExceeded);
}
#     Ok(())
# }
```

```rust
macro_rules! bail {
    ($msg:literal $(,)?) => { ... };
    ($err:expr $(,)?) => { ... };
    ($fmt:expr, $($arg:tt)*) => { ... };
}
```



## anyhow::ensure

*Declarative Macro*

Return early with an error if a condition is not satisfied.

This macro is equivalent to
<code>if !$cond { return Err([anyhow!($args\...)][anyhow!]); }</code>.

The surrounding function's or closure's return value is required to be
<code>Result&lt;_, [anyhow::Error][crate::Error]&gt;</code>.

Analogously to `assert!`, `ensure!` takes a condition and exits the function
if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`
rather than panicking.

[anyhow!]: crate::anyhow

# Example

```
# use anyhow::{ensure, Result};
#
# fn main() -> Result<()> {
#     let user = 0;
#
ensure!(user == 0, "only user 0 is allowed");
#     Ok(())
# }
```

```
# use anyhow::{ensure, Result};
# use thiserror::Error;
#
# const MAX_DEPTH: usize = 1;
#
#[derive(Error, Debug)]
enum ScienceError {
    #[error("recursion limit exceeded")]
    RecursionLimitExceeded,
    # #[error("...")]
    # More = (stringify! {
    ...
    # }, 1).1,
}

# fn main() -> Result<()> {
#     let depth = 0;
#
ensure!(depth <= MAX_DEPTH, ScienceError::RecursionLimitExceeded);
#     Ok(())
# }
```

```rust
macro_rules! ensure {
    ($cond:expr $(,)?) => { ... };
    ($cond:expr, $msg:literal $(,)?) => { ... };
    ($cond:expr, $err:expr $(,)?) => { ... };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => { ... };
}
```



