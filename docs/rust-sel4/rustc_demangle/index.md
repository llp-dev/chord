# rustc_demangle

Demangle Rust compiler symbol names.

This crate provides a `demangle` function which will return a `Demangle`
sentinel value that can be used to learn about the demangled version of a
symbol name. The demangled representation will be the same as the original
if it doesn't look like a mangled symbol name.

`Demangle` can be formatted with the `Display` trait. The alternate
modifier (`#`) can be used to format the symbol name without the
trailing hash value.

# Examples

```
use rustc_demangle::demangle;

assert_eq!(demangle("_ZN4testE").to_string(), "test");
assert_eq!(demangle("_ZN3foo3barE").to_string(), "foo::bar");
assert_eq!(demangle("foo").to_string(), "foo");
// With hash
assert_eq!(format!("{}", demangle("_ZN3foo17h05af221e174051e9E")), "foo::h05af221e174051e9");
// Without hash
assert_eq!(format!("{:#}", demangle("_ZN3foo17h05af221e174051e9E")), "foo");
```

## Modules

### [`rustc_demangle`](rustc_demangle.md)

*2 functions, 2 structs*

