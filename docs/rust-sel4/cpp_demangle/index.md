# cpp_demangle

This crate can parse a C++ “mangled” linker symbol name into a Rust value
describing what the name refers to: a variable, a function, a virtual table,
etc. The description type implements functions such as `demangle()`,
producing human-readable text describing the mangled name. Debuggers and
profilers can use this crate to provide more meaningful output.

C++ requires the compiler to choose names for linker symbols consistently
across compilation units, so that two compilation units that have seen the
same declarations can pair up definitions in one unit with references in
another.  Almost all platforms other than Microsoft Windows follow the
[Itanium C++ ABI][itanium]'s rules for this.

[itanium]: https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangling

For example, suppose a C++ compilation unit has the definition:

```c++
namespace space {
  int foo(int x, int y) { return x+y; }
}
```

The Itanium C++ ABI specifies that the linker symbol for that function must
be named `_ZN5space3fooEii`. This crate can parse that name into a Rust
value representing its structure. That Rust value can be `demangle()`d to the
string `space::foo(int, int)`, which is more meaningful to the C++
developer.

## Modules

### [`cpp_demangle`](cpp_demangle.md)

*1 enum, 1 trait, 2 modules, 2 type aliases, 3 structs*

### [`ast`](ast.md)

*33 structs, 40 enums*

### [`error`](error.md)

*1 enum, 1 type alias*

### [`index_str`](index_str.md)

*1 struct*

