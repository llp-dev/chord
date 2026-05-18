# Crate `clap_lex`

Minimal, flexible command-line parser

As opposed to a declarative parser, this processes arguments as a stream of tokens.  As lexing
a command-line is not context-free, we rely on the caller to decide how to interpret the
arguments.

# Examples

```rust
use std::path::PathBuf;
use std::ffi::OsStr;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
struct Args {
    paths: Vec<PathBuf>,
    color: Color,
    verbosity: usize,
}

#[derive(Debug)]
enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    fn parse(s: Option<&OsStr>) -> Result<Self, BoxedError> {
        let s = s.map(|s| s.to_str().ok_or(s));
        match s {
            Some(Ok("always")) | Some(Ok("")) | None => {
                Ok(Color::Always)
            }
            Some(Ok("auto")) => {
                Ok(Color::Auto)
            }
            Some(Ok("never")) => {
                Ok(Color::Never)
            }
            Some(invalid) => {
                Err(format!("Invalid value for `--color`, {invalid:?}").into())
            }
        }
    }
}

fn parse_args(
    raw: impl IntoIterator<Item=impl Into<std::ffi::OsString>>
) -> Result<Args, BoxedError> {
    let mut args = Args {
        paths: Vec::new(),
        color: Color::Auto,
        verbosity: 0,
    };

    let raw = clap_lex::RawArgs::new(raw);
    let mut cursor = raw.cursor();
    raw.next(&mut cursor);  // Skip the bin
    while let Some(arg) = raw.next(&mut cursor) {
        if arg.is_escape() {
            args.paths.extend(raw.remaining(&mut cursor).map(PathBuf::from));
        } else if arg.is_stdio() {
            args.paths.push(PathBuf::from("-"));
        } else if let Some((long, value)) = arg.to_long() {
            match long {
                Ok("verbose") => {
                    if let Some(value) = value {
                        return Err(format!("`--verbose` does not take a value, got `{value:?}`").into());
                    }
                    args.verbosity += 1;
                }
                Ok("color") => {
                    args.color = Color::parse(value)?;
                }
                _ => {
                    return Err(
                        format!("Unexpected flag: --{}", arg.display()).into()
                    );
                }
            }
        } else if let Some(mut shorts) = arg.to_short() {
            while let Some(short) = shorts.next_flag() {
                match short {
                    Ok('v') => {
                        args.verbosity += 1;
                    }
                    Ok('c') => {
                        let value = shorts.next_value_os();
                        args.color = Color::parse(value)?;
                    }
                    Ok(c) => {
                        return Err(format!("Unexpected flag: -{c}").into());
                    }
                    Err(e) => {
                        return Err(format!("Unexpected flag: -{}", e.to_string_lossy()).into());
                    }
                }
            }
        } else {
            args.paths.push(PathBuf::from(arg.to_value_os().to_owned()));
        }
    }

    Ok(args)
}

let args = parse_args(["bin", "--hello", "world"]);
println!("{args:?}");
```

## Contents

- [Modules](#modules)
  - [`ext`](#ext)
- [Structs](#structs)
  - [`RawArgs`](#rawargs)
  - [`ArgCursor`](#argcursor)
  - [`ParsedArg`](#parsedarg)
  - [`ShortFlags`](#shortflags)
- [Traits](#traits)
  - [`OsStrExt`](#osstrext)
- [Functions](#functions)
  - [`split_nonutf8_once`](#split-nonutf8-once)
  - [`is_number`](#is-number)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ext`](#ext) | mod |  |
| [`RawArgs`](#rawargs) | struct | Command-line arguments |
| [`ArgCursor`](#argcursor) | struct | Position within [`RawArgs`] |
| [`ParsedArg`](#parsedarg) | struct | Command-line Argument |
| [`ShortFlags`](#shortflags) | struct | Walk through short flags within a [`ParsedArg`] |
| [`OsStrExt`](#osstrext) | trait |  |
| [`split_nonutf8_once`](#split-nonutf8-once) | fn |  |
| [`is_number`](#is-number) | fn |  |

## Modules

- [`ext`](ext/index.md)

## Structs

### `RawArgs`

```rust
struct RawArgs {
    items: Vec<std::ffi::OsString>,
}
```

Command-line arguments

#### Implementations

- <span id="rawargs-from-args"></span>`fn from_args() -> Self`

  

  <div class="warning">

  

  **NOTE:** The argument returned will be the current binary.

  

  </div>

  

  # Example

  

  ```rust,no_run

  use std::path::PathBuf;

  let raw = clap_lex::RawArgs::from_args();

  let mut cursor = raw.cursor();

  let _bin = raw.next_os(&mut cursor);

  

  let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();

  println!("{paths:?}");

  ```

- <span id="rawargs-new"></span>`fn new(iter: impl IntoIterator<Item = impl Into<OsString>>) -> Self`

  

  # Example

  

  ```rust,no_run

  use std::path::PathBuf;

  let raw = clap_lex::RawArgs::new(["bin", "foo.txt"]);

  let mut cursor = raw.cursor();

  let _bin = raw.next_os(&mut cursor);

  

  let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();

  println!("{paths:?}");

  ```

- <span id="rawargs-cursor"></span>`fn cursor(&self) -> ArgCursor` — [`ArgCursor`](#argcursor)

  Create a cursor for walking the arguments

  

  # Example

  

  ```rust,no_run

  use std::path::PathBuf;

  let raw = clap_lex::RawArgs::new(["bin", "foo.txt"]);

  let mut cursor = raw.cursor();

  let _bin = raw.next_os(&mut cursor);

  

  let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();

  println!("{paths:?}");

  ```

- <span id="rawargs-next"></span>`fn next<'s>(self: &'s Self, cursor: &mut ArgCursor) -> Option<ParsedArg<'s>>` — [`ArgCursor`](#argcursor), [`ParsedArg`](#parsedarg)

  Advance the cursor, returning the next [`ParsedArg`](#parsedarg)

- <span id="rawargs-next-os"></span>`fn next_os<'s>(self: &'s Self, cursor: &mut ArgCursor) -> Option<&'s OsStr>` — [`ArgCursor`](#argcursor)

  Advance the cursor, returning a raw argument value.

- <span id="rawargs-peek"></span>`fn peek<'s>(self: &'s Self, cursor: &ArgCursor) -> Option<ParsedArg<'s>>` — [`ArgCursor`](#argcursor), [`ParsedArg`](#parsedarg)

  Return the next [`ParsedArg`](#parsedarg)

- <span id="rawargs-peek-os"></span>`fn peek_os<'s>(self: &'s Self, cursor: &ArgCursor) -> Option<&'s OsStr>` — [`ArgCursor`](#argcursor)

  Return a raw argument value.

- <span id="rawargs-remaining"></span>`fn remaining<'s>(self: &'s Self, cursor: &mut ArgCursor) -> impl Iterator<Item = &'s OsStr>` — [`ArgCursor`](#argcursor)

  Return all remaining raw arguments, advancing the cursor to the end

  

  # Example

  

  ```rust,no_run

  use std::path::PathBuf;

  let raw = clap_lex::RawArgs::new(["bin", "foo.txt"]);

  let mut cursor = raw.cursor();

  let _bin = raw.next_os(&mut cursor);

  

  let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();

  println!("{paths:?}");

  ```

- <span id="rawargs-seek"></span>`fn seek(&self, cursor: &mut ArgCursor, pos: SeekFrom)` — [`ArgCursor`](#argcursor), [`SeekFrom`](#seekfrom)

  Adjust the cursor's position

- <span id="rawargs-insert"></span>`fn insert(&mut self, cursor: &ArgCursor, insert_items: impl IntoIterator<Item = impl Into<OsString>>)` — [`ArgCursor`](#argcursor)

  Inject arguments before the `RawArgs::next`

- <span id="rawargs-is-end"></span>`fn is_end(&self, cursor: &ArgCursor) -> bool` — [`ArgCursor`](#argcursor)

  Any remaining args?

#### Trait Implementations

##### `impl Clone for RawArgs`

- <span id="rawargs-clone"></span>`fn clone(&self) -> RawArgs` — [`RawArgs`](#rawargs)

##### `impl Debug for RawArgs`

- <span id="rawargs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RawArgs`

- <span id="rawargs-default"></span>`fn default() -> RawArgs` — [`RawArgs`](#rawargs)

##### `impl Eq for RawArgs`

##### `impl PartialEq for RawArgs`

- <span id="rawargs-partialeq-eq"></span>`fn eq(&self, other: &RawArgs) -> bool` — [`RawArgs`](#rawargs)

##### `impl StructuralPartialEq for RawArgs`

### `ArgCursor`

```rust
struct ArgCursor {
    cursor: usize,
}
```

Position within [`RawArgs`](#rawargs)

#### Implementations

- <span id="argcursor-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for ArgCursor`

- <span id="argcursor-clone"></span>`fn clone(&self) -> ArgCursor` — [`ArgCursor`](#argcursor)

##### `impl Debug for ArgCursor`

- <span id="argcursor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArgCursor`

##### `impl Ord for ArgCursor`

- <span id="argcursor-ord-cmp"></span>`fn cmp(&self, other: &ArgCursor) -> cmp::Ordering` — [`ArgCursor`](#argcursor)

##### `impl PartialEq for ArgCursor`

- <span id="argcursor-partialeq-eq"></span>`fn eq(&self, other: &ArgCursor) -> bool` — [`ArgCursor`](#argcursor)

##### `impl PartialOrd for ArgCursor`

- <span id="argcursor-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ArgCursor) -> option::Option<cmp::Ordering>` — [`ArgCursor`](#argcursor)

##### `impl StructuralPartialEq for ArgCursor`

### `ParsedArg<'s>`

```rust
struct ParsedArg<'s> {
    inner: &'s std::ffi::OsStr,
}
```

Command-line Argument

#### Implementations

- <span id="parsedarg-new"></span>`fn new(inner: &'s OsStr) -> Self`

- <span id="parsedarg-is-empty"></span>`fn is_empty(&self) -> bool`

  Argument is length of 0

- <span id="parsedarg-is-stdio"></span>`fn is_stdio(&self) -> bool`

  Does the argument look like a stdio argument (`-`)

- <span id="parsedarg-is-escape"></span>`fn is_escape(&self) -> bool`

  Does the argument look like an argument escape (`--`)

- <span id="parsedarg-is-negative-number"></span>`fn is_negative_number(&self) -> bool`

  Does the argument look like a negative number?

  

  This won't parse the number in full but attempts to see if this looks

  like something along the lines of `-3`, `-0.3`, or `-33.03`

- <span id="parsedarg-to-long"></span>`fn to_long(&self) -> Option<(Result<&'s str, &'s OsStr>, Option<&'s OsStr>)>`

  Treat as a long-flag

- <span id="parsedarg-is-long"></span>`fn is_long(&self) -> bool`

  Can treat as a long-flag

- <span id="parsedarg-to-short"></span>`fn to_short(&self) -> Option<ShortFlags<'s>>` — [`ShortFlags`](#shortflags)

  Treat as a short-flag

- <span id="parsedarg-is-short"></span>`fn is_short(&self) -> bool`

  Can treat as a short-flag

- <span id="parsedarg-to-value-os"></span>`fn to_value_os(&self) -> &'s OsStr`

  Treat as a value

  

  <div class="warning">

  

  **NOTE:** May return a flag or an escape.

  

  </div>

- <span id="parsedarg-to-value"></span>`fn to_value(&self) -> Result<&'s str, &'s OsStr>`

  Treat as a value

  

  <div class="warning">

  

  **NOTE:** May return a flag or an escape.

  

  </div>

- <span id="parsedarg-display"></span>`fn display(&self) -> impl std::fmt::Display + 's`

  Safely print an argument that may contain non-UTF8 content

  

  This may perform lossy conversion, depending on the platform. If you would like an implementation which escapes the path please use Debug instead.

#### Trait Implementations

##### `impl Clone for ParsedArg<'s>`

- <span id="parsedarg-clone"></span>`fn clone(&self) -> ParsedArg<'s>` — [`ParsedArg`](#parsedarg)

##### `impl Debug for ParsedArg<'s>`

- <span id="parsedarg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParsedArg<'s>`

##### `impl Hash for ParsedArg<'s>`

- <span id="parsedarg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ParsedArg<'s>`

- <span id="parsedarg-ord-cmp"></span>`fn cmp(&self, other: &ParsedArg<'s>) -> cmp::Ordering` — [`ParsedArg`](#parsedarg)

##### `impl PartialEq for ParsedArg<'s>`

- <span id="parsedarg-partialeq-eq"></span>`fn eq(&self, other: &ParsedArg<'s>) -> bool` — [`ParsedArg`](#parsedarg)

##### `impl PartialOrd for ParsedArg<'s>`

- <span id="parsedarg-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ParsedArg<'s>) -> option::Option<cmp::Ordering>` — [`ParsedArg`](#parsedarg)

##### `impl StructuralPartialEq for ParsedArg<'s>`

### `ShortFlags<'s>`

```rust
struct ShortFlags<'s> {
    inner: &'s std::ffi::OsStr,
    utf8_prefix: std::str::CharIndices<'s>,
    invalid_suffix: Option<&'s std::ffi::OsStr>,
}
```

Walk through short flags within a [`ParsedArg`](#parsedarg)

#### Implementations

- <span id="shortflags-new"></span>`fn new(inner: &'s OsStr) -> Self`

- <span id="shortflags-advance-by"></span>`fn advance_by(&mut self, n: usize) -> Result<(), usize>`

  Move the iterator forward by `n` short flags

- <span id="shortflags-is-empty"></span>`fn is_empty(&self) -> bool`

  No short flags left

- <span id="shortflags-is-negative-number"></span>`fn is_negative_number(&self) -> bool`

  Does the short flag look like a number

  

  Ideally call this before doing any iterator

- <span id="shortflags-next-flag"></span>`fn next_flag(&mut self) -> Option<Result<char, &'s OsStr>>`

  Advance the iterator, returning the next short flag on success

  

  On error, returns the invalid-UTF8 value

- <span id="shortflags-next-value-os"></span>`fn next_value_os(&mut self) -> Option<&'s OsStr>`

  Advance the iterator, returning everything left as a value

#### Trait Implementations

##### `impl Clone for ShortFlags<'s>`

- <span id="shortflags-clone"></span>`fn clone(&self) -> ShortFlags<'s>` — [`ShortFlags`](#shortflags)

##### `impl Debug for ShortFlags<'s>`

- <span id="shortflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ShortFlags<'s>`

- <span id="shortflags-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="shortflags-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="shortflags-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ShortFlags<'s>`

- <span id="shortflags-iterator-type-item"></span>`type Item = Result<char, &'s OsStr>`

- <span id="shortflags-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Traits

### `OsStrExt`

```rust
trait OsStrExt: private::Sealed { ... }
```

String-like methods for [`OsStr`](../clap_builder/builder/os_str/index.md)

#### Required Methods

- `fn try_str(&self) -> Result<&str, std::str::Utf8Error>`

  Converts to a string slice.

- `fn contains(&self, needle: &str) -> bool`

  Returns `true` if the given pattern matches a sub-slice of

- `fn find(&self, needle: &str) -> Option<usize>`

  Returns the byte index of the first character of this string slice that

- `fn strip_prefix(&self, prefix: &str) -> Option<&OsStr>`

  Returns a string slice with the prefix removed.

- `fn starts_with(&self, prefix: &str) -> bool`

  Returns `true` if the given pattern matches a prefix of this

- `fn split<'s, 'n>(self: &'s Self, needle: &'n str) -> Split<'s, 'n>`

  An iterator over substrings of this string slice, separated by

- `fn split_once(&self, needle: &str) -> Option<(&OsStr, &OsStr)>`

  Splits the string on the first occurrence of the specified delimiter and

#### Implementors

- `std::ffi::OsStr`

## Functions

### `split_nonutf8_once`

```rust
fn split_nonutf8_once(b: &std::ffi::OsStr) -> (&str, Option<&std::ffi::OsStr>)
```

### `is_number`

```rust
fn is_number(arg: &str) -> bool
```

