**addr2line > frame**

# Module: frame

## Contents

**Structs**

- [`Frame`](#frame) - A function frame.
- [`FrameIter`](#frameiter) - An iterator over function frames.
- [`FunctionName`](#functionname) - A function name.
- [`Location`](#location) - A source location.

**Functions**

- [`demangle`](#demangle) - Demangle a symbol name using the demangling scheme for the given language.
- [`demangle_auto`](#demangle_auto) - Apply 'best effort' demangling of a symbol name.

---

## addr2line::frame::Frame

*Struct*

A function frame.

**Generic Parameters:**
- 'ctx
- R

**Fields:**
- `dw_die_offset: Option<gimli::UnitOffset<<R as >::Offset>>` - The DWARF unit offset corresponding to the DIE of the function.
- `function: Option<FunctionName<R>>` - The name of the function.
- `location: Option<Location<'ctx>>` - The source location corresponding to this frame.



## addr2line::frame::FrameIter

*Struct*

An iterator over function frames.

**Generic Parameters:**
- 'ctx
- R

**Tuple Struct**: `()`

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>` - Advances the iterator and returns the next frame.

**Trait Implementations:**

- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>`



## addr2line::frame::FunctionName

*Struct*

A function name.

**Generic Parameters:**
- R

**Fields:**
- `name: R` - The name of the function.
- `language: Option<gimli::DwLang>` - The language of the compilation unit containing this function.

**Methods:**

- `fn raw_name(self: &Self) -> Result<Cow<str>, gimli::Error>` - The raw name of this function before demangling.
- `fn demangle(self: &Self) -> Result<Cow<str>, gimli::Error>` - The name of this function after demangling (if applicable).



## addr2line::frame::Location

*Struct*

A source location.

**Generic Parameters:**
- 'a

**Fields:**
- `file: Option<&'a str>` - The file name.
- `line: Option<u32>` - The line number.
- `column: Option<u32>` - The column number.



## addr2line::frame::demangle

*Function*

Demangle a symbol name using the demangling scheme for the given language.

Returns `None` if demangling failed or is not required.

```rust
fn demangle(name: &str, language: gimli::DwLang) -> Option<alloc::string::String>
```



## addr2line::frame::demangle_auto

*Function*

Apply 'best effort' demangling of a symbol name.

If `language` is given, then only the demangling scheme for that language
is used.

If `language` is `None`, then heuristics are used to determine how to
demangle the name. Currently, these heuristics are very basic.

If demangling fails or is not required, then `name` is returned unchanged.

```rust
fn demangle_auto(name: alloc::borrow::Cow<str>, language: Option<gimli::DwLang>) -> alloc::borrow::Cow<str>
```



