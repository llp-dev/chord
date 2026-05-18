**anstream > auto**

# Module: auto

## Contents

**Structs**

- [`AutoStream`](#autostream) - [`std::io::Write`] that adapts ANSI escape codes to the underlying `Write`s capabilities

---

## anstream::auto::AutoStream

*Struct*

[`std::io::Write`] that adapts ANSI escape codes to the underlying `Write`s capabilities

This includes
- Stripping colors for non-terminals
- Respecting env variables like [NO_COLOR](https://no-color.org/) or [CLICOLOR](https://bixense.com/clicolors/)
- *(windows)* Falling back to the wincon API where [ENABLE_VIRTUAL_TERMINAL_PROCESSING](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences) is unsupported

You can customize auto-detection by calling into
[anstyle_query](https://docs.rs/anstyle-query/latest/anstyle_query/)
to get a [`ColorChoice`] and then calling [`AutoStream::new(stream, choice)`].

**Generic Parameters:**
- S

**Methods:**

- `fn lock(self: Self) -> AutoStream<std::io::StderrLock<'static>>` - Get exclusive access to the `AutoStream`
- `fn lock(self: Self) -> AutoStream<std::io::StdoutLock<'static>>` - Get exclusive access to the `AutoStream`
- `fn new(raw: S, choice: ColorChoice) -> Self` - Runtime control over styling behavior
- `fn auto(raw: S) -> Self` - Auto-adapt for the stream's capabilities
- `fn choice(raw: &S) -> ColorChoice` - Report the desired choice for the given stream
- `fn always_ansi(raw: S) -> Self` - Force ANSI escape codes to be passed through as-is, no matter what the inner `Write`
- `fn always(raw: S) -> Self` - Force color, no matter what the inner `Write` supports.
- `fn never(raw: S) -> Self` - Only pass printable data to the inner `Write`.
- `fn into_inner(self: Self) -> S` - Get the wrapped [`RawStream`]
- `fn as_inner(self: &Self) -> &S` - Get the wrapped [`RawStream`]
- `fn is_terminal(self: &Self) -> bool` - Returns `true` if the descriptor/handle refers to a terminal/tty.
- `fn current_choice(self: &Self) -> ColorChoice` - Prefer [`AutoStream::choice`]

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> std::io::Result<usize>`
  - `fn write_vectored(self: & mut Self, bufs: &[std::io::IoSlice]) -> std::io::Result<usize>`
  - `fn flush(self: & mut Self) -> std::io::Result<()>`
  - `fn write_all(self: & mut Self, buf: &[u8]) -> std::io::Result<()>`
  - `fn write_fmt(self: & mut Self, args: std::fmt::Arguments) -> std::io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



