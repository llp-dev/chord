**clap_builder > builder > styled_str**

# Module: builder::styled_str

## Contents

**Structs**

- [`StyledStr`](#styledstr) - Terminal-styling container

---

## clap_builder::builder::styled_str::StyledStr

*Struct*

Terminal-styling container

Styling may be encoded as [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)

# Examples

```rust
# use clap_builder as clap;
// `cstr!` converts tags to ANSI codes
let after_help: &'static str = color_print::cstr!(
r#"<bold><underline>Examples</underline></bold>

  <dim>$</dim> <bold>mybin --input file.toml</bold>
"#);

let cmd = clap::Command::new("mybin")
    .after_help(after_help)  // The `&str` gets converted into a `StyledStr`
    // ...
#   ;
```

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self` - Create an empty buffer
- `fn ansi(self: &Self) -> impl Trait` - Display using [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code) styling
- `fn push_str(self: & mut Self, msg: &str)` - Appends a given string slice onto the end of this `StyledStr`.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &StyledStr) -> bool`
- **From**
  - `fn from(name: &String) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &StyledStr) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Default**
  - `fn default() -> StyledStr`
- **From**
  - `fn from(name: String) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> StyledStr`
- **From**
  - `fn from(name: &&'static str) -> Self`
- **From**
  - `fn from(cow: Cow<'static, str>) -> Self`
- **Write**
  - `fn write_str(self: & mut Self, s: &str) -> Result<(), std::fmt::Error>`
  - `fn write_char(self: & mut Self, c: char) -> Result<(), std::fmt::Error>`
- **From**
  - `fn from(name: &'static str) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &StyledStr) -> $crate::cmp::Ordering`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



