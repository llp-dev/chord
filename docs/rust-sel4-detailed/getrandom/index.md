# Crate `getrandom`

Interface to the operating system's random number generator.

# Supported targets

| Target            | Target Triple      | Implementation
| ----------------- | ------------------ | --------------
| Linux, Android    | `*‑linux‑*`        | [`getrandom`][1] system call if available, otherwise [`/dev/urandom`][2] after successfully polling `/dev/random`
| Windows           | `*‑windows‑*`      | `BCryptGenRandom`
| macOS             | `*‑apple‑darwin`   | [`getentropy`][3]
| iOS, tvOS, watchOS | `*‑apple‑ios`, `*-apple-tvos`, `*-apple-watchos` | `CCRandomGenerateBytes`
| FreeBSD           | `*‑freebsd`        | [`getrandom`][5]
| OpenBSD           | `*‑openbsd`        | [`getentropy`][7]
| NetBSD            | `*‑netbsd`         | [`getrandom`][16] if available, otherwise [`kern.arandom`][8]
| Dragonfly BSD     | `*‑dragonfly`      | [`getrandom`][9]
| Solaris           | `*‑solaris`        | [`getrandom`][11] (with `GRND_RANDOM`)
| illumos           | `*‑illumos`        | [`getrandom`][12]
| Fuchsia OS        | `*‑fuchsia`        | `cprng_draw`
| Redox             | `*‑redox`          | `/dev/urandom`
| Haiku             | `*‑haiku`          | `/dev/urandom` (identical to `/dev/random`)
| Hermit            | `*-hermit`         | `sys_read_entropy`
| Hurd              | `*-hurd-*`         | [`getrandom`][17]
| SGX               | `x86_64‑*‑sgx`     | `RDRAND`
| VxWorks           | `*‑wrs‑vxworks‑*`  | `randABytes` after checking entropy pool initialization with `randSecure`
| ESP-IDF           | `*‑espidf`         | `esp_fill_random`
| Emscripten        | `*‑emscripten`     | [`getentropy`][13]
| WASI              | `wasm32‑wasi`      | `random_get`
| Web Browser and Node.js | `wasm*‑*‑unknown` | `Crypto.getRandomValues` if available, then `crypto.randomFillSync` if on Node.js, see [WebAssembly support]
| SOLID             | `*-kmc-solid_*`    | `SOLID_RNG_SampleRandomBytes`
| Nintendo 3DS      | `*-nintendo-3ds`   | [`getrandom`][18]
| PS Vita           | `*-vita-*`         | [`getentropy`][13]
| QNX Neutrino      | `*‑nto-qnx*`       | [`/dev/urandom`][14] (identical to `/dev/random`)
| AIX               | `*-ibm-aix`        | [`/dev/urandom`][15]
| Cygwin            | `*-cygwin`         | [`getrandom`][19] (based on `RtlGenRandom`)

Pull Requests that add support for new targets to `getrandom` are always welcome.

## Unsupported targets

By default, `getrandom` will not compile on unsupported targets, but certain
features allow a user to select a "fallback" implementation if no supported
implementation exists.

All of the below mechanisms only affect unsupported
targets. Supported targets will _always_ use their supported implementations.
This prevents a crate from overriding a secure source of randomness
(either accidentally or intentionally).

## `/dev/urandom` fallback on Linux and Android

On Linux targets the fallback is present only if either `target_env` is `musl`,
or `target_arch` is one of the following: `aarch64`, `arm`, `powerpc`, `powerpc64`,
`s390x`, `x86`, `x86_64`. Other supported targets [require][platform-support]
kernel versions which support `getrandom` system call, so fallback is not needed.

On Android targets the fallback is present only for the following `target_arch`es:
`aarch64`, `arm`, `x86`, `x86_64`. Other `target_arch`es (e.g. RISC-V) require
sufficiently high API levels.

The fallback can be disabled by enabling the `linux_disable_fallback` crate feature.
Note that doing so will bump minimum supported Linux kernel version to 3.17 and
Android API level to 23 (Marshmallow).

### RDRAND on x86

*If the `rdrand` Cargo feature is enabled*, `getrandom` will fallback to using
the `RDRAND` instruction to get randomness on `no_std` `x86`/`x86_64`
targets. This feature has no effect on other CPU architectures.

### WebAssembly support

This crate fully supports the
[`wasm32-wasi`](https://github.com/CraneStation/wasi) and
[`wasm32-unknown-emscripten`](https://www.hellorust.com/setup/emscripten/)
targets. However, the `wasm32-unknown-unknown` target (i.e. the target used
by `wasm-pack`) is not automatically
supported since, from the target name alone, we cannot deduce which
JavaScript interface is in use (or if JavaScript is available at all).

Instead, *if the `js` Cargo feature is enabled*, this crate will assume
that you are building for an environment containing JavaScript, and will
call the appropriate methods. Both web browser (main window and Web Workers)
and Node.js environments are supported, invoking the methods
[described above](#supported-targets) using the `wasm-bindgen` toolchain.

To enable the `js` Cargo feature, add the following to the `dependencies`
section in your `Cargo.toml` file:
```toml
[dependencies]
getrandom = { version = "0.2", features = ["js"] }
```

This can be done even if `getrandom` is not a direct dependency. Cargo
allows crates to enable features for indirect dependencies.

This feature should only be enabled for binary, test, or benchmark crates.
Library crates should generally not enable this feature, leaving such a
decision to *users* of their library. Also, libraries should not introduce
their own `js` features *just* to enable `getrandom`'s `js` feature.

This feature has no effect on targets other than `wasm32-unknown-unknown`.

#### Node.js ES module support

Node.js supports both [CommonJS modules] and [ES modules]. Due to
limitations in wasm-bindgen's `module` support, we cannot directly
support ES Modules running on Node.js. However, on Node v15 and later, the
module author can add a simple shim to support the Web Cryptography API:
```js
import { webcrypto } from 'node:crypto'
globalThis.crypto = webcrypto
```
This crate will then use the provided `webcrypto` implementation.

### Platform Support
This crate generally supports the same operating system and platform versions
that the Rust standard library does. Additional targets may be supported using
pluggable custom implementations.

This means that as Rust drops support for old versions of operating systems
(such as old Linux kernel versions, Android API levels, etc) in stable releases,
`getrandom` may create new patch releases (`0.N.x`) that remove support for
outdated platform versions.

### Custom implementations

The [`register_custom_getrandom!`](#register-custom-getrandom) macro allows a user to mark their own
function as the backing implementation for [`getrandom`](#getrandom). See the macro's
documentation for more information about writing and registering your own
custom implementations.

Note that registering a custom implementation only has an effect on targets
that would otherwise not compile. Any supported targets (including those
using `rdrand` and `js` Cargo features) continue using their normal
implementations even if a function is registered.

## Early boot

Sometimes, early in the boot process, the OS has not collected enough
entropy to securely seed its RNG. This is especially common on virtual
machines, where standard "random" events are hard to come by.

Some operating system interfaces always block until the RNG is securely
seeded. This can take anywhere from a few seconds to more than a minute.
A few (Linux, NetBSD and Solaris) offer a choice between blocking and
getting an error; in these cases, we always choose to block.

On Linux (when the `getrandom` system call is not available), reading from
`/dev/urandom` never blocks, even when the OS hasn't collected enough
entropy yet. To avoid returning low-entropy bytes, we first poll
`/dev/random` and only switch to `/dev/urandom` once this has succeeded.

On OpenBSD, this kind of entropy accounting isn't available, and on
NetBSD, blocking on it is discouraged. On these platforms, nonblocking
interfaces are used, even when reliable entropy may not be available.
On the platforms where it is used, the reliability of entropy accounting
itself isn't free from controversy. This library provides randomness
sourced according to the platform's best practices, but each platform has
its own limits on the grade of randomness it can promise in environments
with few sources of entropy.

## Error handling

We always choose failure over returning known insecure "random" bytes. In
general, on supported platforms, failure is highly unlikely, though not
impossible. If an error does occur, then it is likely that it will occur
on every call to `getrandom`, hence after the first successful call one
can be reasonably confident that no errors will occur.

































## Contents

- [Modules](#modules)
  - [`error`](#error)
  - [`util`](#util)
  - [`custom`](#custom)
  - [`util_libc`](#util-libc)
  - [`use_file`](#use-file)
  - [`lazy`](#lazy)
  - [`imp`](#imp)
- [Structs](#structs)
  - [`Error`](#error)
- [Functions](#functions)
  - [`getrandom`](#getrandom)
  - [`getrandom_uninit`](#getrandom-uninit)
- [Macros](#macros)
  - [`register_custom_getrandom!`](#register-custom-getrandom)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`error`](#error) | mod |  |
| [`util`](#util) | mod |  |
| [`custom`](#custom) | mod | An implementation which calls out to an externally defined function. |
| [`util_libc`](#util-libc) | mod |  |
| [`use_file`](#use-file) | mod | Implementations that just need to read from a file |
| [`lazy`](#lazy) | mod |  |
| [`imp`](#imp) | mod | Implementation for Linux / Android with `/dev/urandom` fallback |
| [`Error`](#error) | struct |  |
| [`getrandom`](#getrandom) | fn | Fill `dest` with random bytes from the system's preferred random number source. |
| [`getrandom_uninit`](#getrandom-uninit) | fn | Version of the `getrandom` function which fills `dest` with random bytes returns a mutable reference to those bytes. |
| [`register_custom_getrandom!`](#register-custom-getrandom) | macro | Register a function to be invoked by `getrandom` on unsupported targets. |

## Modules

- [`error`](error/index.md)
- [`util`](util/index.md)
- [`custom`](custom/index.md) — An implementation which calls out to an externally defined function.
- [`util_libc`](util_libc/index.md)
- [`use_file`](use_file/index.md) — Implementations that just need to read from a file
- [`lazy`](lazy/index.md)
- [`imp`](imp/index.md) — Implementation for Linux / Android with `/dev/urandom` fallback

## Structs

### `Error`

```rust
struct Error(core::num::NonZeroU32);
```

A small and `no_std` compatible error type

The `Error::raw_os_error()` will indicate if the error is from the OS, and
if so, which error code the OS gave the application. If such an error is
encountered, please consult with your system documentation.

Internally this type is a NonZeroU32, with certain values reserved for
certain purposes, see `Error::INTERNAL_START` and `Error::CUSTOM_START`.

*If this crate's `"std"` Cargo feature is enabled*, then:
- [`getrandom::Error`][Error] implements
  [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
- [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) implements
  [`From<getrandom::Error>`](https://doc.rust-lang.org/std/convert/trait.From.html).

#### Implementations

- <span id="error-const-unsupported"></span>`const UNSUPPORTED: Error`

- <span id="error-const-errno-not-positive"></span>`const ERRNO_NOT_POSITIVE: Error`

- <span id="error-const-unexpected"></span>`const UNEXPECTED: Error`

- <span id="error-const-ios-sec-random"></span>`const IOS_SEC_RANDOM: Error`

- <span id="error-const-windows-rtl-gen-random"></span>`const WINDOWS_RTL_GEN_RANDOM: Error`

- <span id="error-const-failed-rdrand"></span>`const FAILED_RDRAND: Error`

- <span id="error-const-no-rdrand"></span>`const NO_RDRAND: Error`

- <span id="error-const-web-crypto"></span>`const WEB_CRYPTO: Error`

- <span id="error-const-web-get-random-values"></span>`const WEB_GET_RANDOM_VALUES: Error`

- <span id="error-const-vxworks-rand-secure"></span>`const VXWORKS_RAND_SECURE: Error`

- <span id="error-const-node-crypto"></span>`const NODE_CRYPTO: Error`

- <span id="error-const-node-random-fill-sync"></span>`const NODE_RANDOM_FILL_SYNC: Error`

- <span id="error-const-node-es-module"></span>`const NODE_ES_MODULE: Error`

- <span id="error-const-internal-start"></span>`const INTERNAL_START: u32`

- <span id="error-const-custom-start"></span>`const CUSTOM_START: u32`

- <span id="error-raw-os-error"></span>`fn raw_os_error(self) -> Option<i32>`

  Extract the raw OS error code (if this error came from the OS)

  

  This method is identical to [`std::io::Error::raw_os_error()`][1], except

  that it works in `no_std` contexts. If this method returns `None`, the

  error value can still be formatted via the `Display` implementation.

- <span id="error-code"></span>`const fn code(self) -> NonZeroU32`

  Extract the bare error code.

  

  This code can either come from the underlying OS, or be a custom error.

  Use `Error::raw_os_error()` to disambiguate.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](error/index.md#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](error/index.md#error)

##### `impl StructuralPartialEq for Error`

## Functions

### `getrandom`

```rust
fn getrandom(dest: &mut [u8]) -> Result<(), Error>
```

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

### `getrandom_uninit`

```rust
fn getrandom_uninit(dest: &mut [core::mem::MaybeUninit<u8>]) -> Result<&mut [u8], Error>
```

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
// We ignore this test since `uninit_array` is unstable.
#![feature(maybe_uninit_uninit_array)]
fn main() -> Result<(), getrandom::Error> {
let mut buf = core::mem::MaybeUninit::uninit_array::<1024>();
let buf: &mut [u8] = getrandom::getrandom_uninit(&mut buf)?;
Ok(()) }
```

## Macros

### `register_custom_getrandom!`

Register a function to be invoked by `getrandom` on unsupported targets.

## Writing a custom `getrandom` implementation

The function to register must have the same signature as
[`getrandom::getrandom`](crate::getrandom). The function can be defined
wherever you want, either in root crate or a dependent crate.

For example, if we wanted a `failure-getrandom` crate containing an
implementation that always fails, we would first depend on `getrandom`
(for the [`Error`](error/index.md) type) in `failure-getrandom/Cargo.toml`:
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
mod failure_getrandom { pub fn always_fail(_: &mut [u8]) -> Result<(), getrandom::Error> { unimplemented!() } }
use failure_getrandom::always_fail;
use getrandom::register_custom_getrandom;

register_custom_getrandom!(always_fail);
```

Now any user of `getrandom` (direct or indirect) on this target will use the
registered function. As noted in the
[top-level documentation](index.html#custom-implementations) this
registration only has an effect on unsupported targets.

