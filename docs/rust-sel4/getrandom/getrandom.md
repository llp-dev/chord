**getrandom**

# Module: getrandom

## Contents

**Macros**

- [`register_custom_getrandom`](#register_custom_getrandom) - Register a function to be invoked by `getrandom` on unsupported targets.

**Functions**

- [`getrandom`](#getrandom) - Fill `dest` with random bytes from the system's preferred random number
- [`getrandom_uninit`](#getrandom_uninit) - Version of the `getrandom` function which fills `dest` with random bytes

---

## getrandom::getrandom

*Function*

Fill `dest` with random bytes from the system's preferred random number
source.

This function returns an error on any failure, including partial reads. We
make no guarantees regarding the contents of `dest` on error. If `dest` is
empty, `getrandom` immediately returns success, making no calls to the
underlying operating system.

Blocking is possible, at least during early boot; see module documentation.

In general, `getrandom` will be fast enough for interactive usage, though
significantly slower than a user-space CSPRNG; for the latter consider
[`rand::thread_rng`](https://docs.rs/rand/*/rand/fn.thread_rng.html).

```rust
fn getrandom(dest: & mut [u8]) -> Result<(), Error>
```



## getrandom::getrandom_uninit

*Function*

Version of the `getrandom` function which fills `dest` with random bytes
returns a mutable reference to those bytes.

On successful completion this function is guaranteed to return a slice
which points to the same memory as `dest` and has the same length.
In other words, it's safe to assume that `dest` is initialized after
this function has returned `Ok`.

No part of `dest` will ever be de-initialized at any point, regardless
of what is returned.

# Examples

```ignore
# // We ignore this test since `uninit_array` is unstable.
#![feature(maybe_uninit_uninit_array)]
# fn main() -> Result<(), getrandom::Error> {
let mut buf = core::mem::MaybeUninit::uninit_array::<1024>();
let buf: &mut [u8] = getrandom::getrandom_uninit(&mut buf)?;
# Ok(()) }
```

```rust
fn getrandom_uninit(dest: & mut [core::mem::MaybeUninit<u8>]) -> Result<& mut [u8], Error>
```



## getrandom::register_custom_getrandom

*Declarative Macro*

Register a function to be invoked by `getrandom` on unsupported targets.

## Writing a custom `getrandom` implementation

The function to register must have the same signature as
[`getrandom::getrandom`](crate::getrandom). The function can be defined
wherever you want, either in root crate or a dependent crate.

For example, if we wanted a `failure-getrandom` crate containing an
implementation that always fails, we would first depend on `getrandom`
(for the [`Error`] type) in `failure-getrandom/Cargo.toml`:
```toml
[dependencies]
getrandom = "0.2"
```
Note that the crate containing this function does **not** need to enable the
`"custom"` Cargo feature.

Next, in `failure-getrandom/src/lib.rs`, we define our function:
```rust
use core::num::NonZeroU32;
use getrandom::Error;

// Some application-specific error code
const MY_CUSTOM_ERROR_CODE: u32 = Error::CUSTOM_START + 42;
pub fn always_fail(buf: &mut [u8]) -> Result<(), Error> {
    let code = NonZeroU32::new(MY_CUSTOM_ERROR_CODE).unwrap();
    Err(Error::from(code))
}
```

## Registering a custom `getrandom` implementation

Functions can only be registered in the root binary crate. Attempting to
register a function in a non-root crate will result in a linker error.
This is similar to
[`#[panic_handler]`](https://doc.rust-lang.org/nomicon/panic-handler.html) or
[`#[global_allocator]`](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/global-allocators.html),
where helper crates define handlers/allocators but only the binary crate
actually _uses_ the functionality.

To register the function, we first depend on `failure-getrandom` _and_
`getrandom` in `Cargo.toml`:
```toml
[dependencies]
failure-getrandom = "0.1"
getrandom = { version = "0.2", features = ["custom"] }
```

Then, we register the function in `src/main.rs`:
```rust
# mod failure_getrandom { pub fn always_fail(_: &mut [u8]) -> Result<(), getrandom::Error> { unimplemented!() } }
use failure_getrandom::always_fail;
use getrandom::register_custom_getrandom;

register_custom_getrandom!(always_fail);
```

Now any user of `getrandom` (direct or indirect) on this target will use the
registered function. As noted in the
[top-level documentation](index.html#custom-implementations) this
registration only has an effect on unsupported targets.

```rust
macro_rules! register_custom_getrandom {
    ($path:path) => { ... };
}
```



