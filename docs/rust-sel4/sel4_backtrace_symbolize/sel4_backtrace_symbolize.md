**sel4_backtrace_symbolize**

# Module: sel4_backtrace_symbolize

## Contents

**Structs**

- [`Options`](#options)

**Functions**

- [`symbolize`](#symbolize)

---

## sel4_backtrace_symbolize::Options

*Struct*

**Fields:**
- `do_functions: bool`
- `do_inlines: bool`
- `demangle: bool`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Options`
- **PartialEq**
  - `fn eq(self: &Self, other: &Options) -> bool`



## sel4_backtrace_symbolize::symbolize

*Function*

```rust
fn symbolize<impl fmt::Write, impl gimli::Reader, impl Iterator<Item = u64>>(w: & mut impl Trait, ctx: &addr2line::Context<impl Trait>, opts: &Options, addrs: impl Trait) -> Result<(), fmt::Error>
```



