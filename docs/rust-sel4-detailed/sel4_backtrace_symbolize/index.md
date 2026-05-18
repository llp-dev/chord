# Crate `sel4_backtrace_symbolize`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Options`](#options) | struct |  |
| [`print_loc`](#print-loc) | fn |  |
| [`print_function`](#print-function) | fn |  |
| [`symbolize`](#symbolize) | fn |  |

## Structs

### `Options`

```rust
struct Options {
    pub do_functions: bool,
    pub do_inlines: bool,
    pub demangle: bool,
}
```

#### Trait Implementations

##### `impl Clone for Options`

- <span id="options-clone"></span>`fn clone(&self) -> Options` — [`Options`](#options)

##### `impl Copy for Options`

##### `impl Default for Options`

- <span id="options-default"></span>`fn default() -> Self`

##### `impl Eq for Options`

##### `impl PartialEq for Options`

- <span id="options-partialeq-eq"></span>`fn eq(&self, other: &Options) -> bool` — [`Options`](#options)

##### `impl StructuralPartialEq for Options`

## Functions

### `print_loc`

```rust
fn print_loc(w: &mut impl fmt::Write, loc: Option<&addr2line::Location<'_>>) -> Result<(), fmt::Error>
```

### `print_function`

```rust
fn print_function(w: &mut impl fmt::Write, name: Option<&str>, language: Option<gimli::DwLang>, demangle: bool) -> Result<(), fmt::Error>
```

### `symbolize`

```rust
fn symbolize(w: &mut impl fmt::Write, ctx: &addr2line::Context<impl gimli::Reader>, opts: &Options, addrs: impl Iterator<Item = u64>) -> Result<(), fmt::Error>
```

