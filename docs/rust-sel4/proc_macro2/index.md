# proc_macro2

[![github]](https://github.com/dtolnay/proc-macro2)&ensp;[![crates-io]](https://crates.io/crates/proc-macro2)&ensp;[![docs-rs]](crate)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A wrapper around the procedural macro API of the compiler's [`proc_macro`]
crate. This library serves two purposes:

- **Bring proc-macro-like functionality to other contexts like build.rs and
  main.rs.** Types from `proc_macro` are entirely specific to procedural
  macros and cannot ever exist in code outside of a procedural macro.
  Meanwhile `proc_macro2` types may exist anywhere including non-macro code.
  By developing foundational libraries like [syn] and [quote] against
  `proc_macro2` rather than `proc_macro`, the procedural macro ecosystem
  becomes easily applicable to many other use cases and we avoid
  reimplementing non-macro equivalents of those libraries.

- **Make procedural macros unit testable.** As a consequence of being
  specific to procedural macros, nothing that uses `proc_macro` can be
  executed from a unit test. In order for helper libraries or components of
  a macro to be testable in isolation, they must be implemented using
  `proc_macro2`.

[syn]: https://github.com/dtolnay/syn
[quote]: https://github.com/dtolnay/quote

# Usage

The skeleton of a typical procedural macro typically looks like this:

```
extern crate proc_macro;

# const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
# };
# #[cfg(wrap_proc_macro)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = {
        /* transform input */
        # input
    };

    proc_macro::TokenStream::from(output)
}
```

If parsing with [Syn], you'll use [`parse_macro_input!`] instead to
propagate parse errors correctly back to the compiler when parsing fails.

[`parse_macro_input!`]: https://docs.rs/syn/2.0/syn/macro.parse_macro_input.html

# Unstable features

The default feature set of proc-macro2 tracks the most recent stable
compiler API. Functionality in `proc_macro` that is not yet stable is not
exposed by proc-macro2 by default.

To opt into the additional APIs available in the most recent nightly
compiler, the `procmacro2_semver_exempt` config flag must be passed to
rustc. We will polyfill those nightly-only APIs back to Rust 1.68.0. As
these are unstable APIs that track the nightly compiler, minor versions of
proc-macro2 may make breaking changes to them at any time.

```sh
RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build
```

Note that this must not only be done for your crate, but for any crate that
depends on your crate. This infectious nature is intentional, as it serves
as a reminder that you are outside of the normal semver guarantees.

Semver exempt methods are marked as such in the proc-macro2 documentation.

# Thread-Safety

Most types in this crate are `!Sync` because the underlying compiler
types make use of thread-local memory, meaning they cannot be accessed from
a different thread.

## Modules

### [`proc_macro2`](proc_macro2.md)

*2 modules, 3 enums, 7 structs*

### [`extra`](extra.md)

*1 struct*

### [`token_stream`](token_stream.md)

*1 struct*

