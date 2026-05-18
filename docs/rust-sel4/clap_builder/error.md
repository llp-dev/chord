**clap_builder > error**

# Module: error

## Contents

**Structs**

- [`Error`](#error) - Command Line Argument Parser Error

**Type Aliases**

- [`Result`](#result) - Short hand for [`Result`] type

---

## clap_builder::error::Error

*Struct*

Command Line Argument Parser Error

See [`Command::error`] to create an error.

[`Command::error`]: crate::Command::error

**Generic Parameters:**
- F

**Methods:**

- `fn raw<impl Display>(kind: ErrorKind, message: impl Trait) -> Self` - Create an unformatted error
- `fn format(self: Self, cmd: & mut Command) -> Self` - Format the existing message with the Command's context
- `fn new(kind: ErrorKind) -> Self` - Create an error with a pre-defined message
- `fn with_cmd(self: Self, cmd: &Command) -> Self` - Apply [`Command`]'s formatting to the error
- `fn apply<EF>(self: Self) -> Error<EF>` - Apply an alternative formatter to the error
- `fn kind(self: &Self) -> ErrorKind` - Type of error for programmatic processing
- `fn context(self: &Self) -> impl Trait` - Additional information to further qualify the error
- `fn get(self: &Self, kind: ContextKind) -> Option<&ContextValue>` - Lookup a piece of context
- `fn insert(self: & mut Self, kind: ContextKind, value: ContextValue) -> Option<ContextValue>` - Insert a piece of context
- `fn remove(self: & mut Self, kind: ContextKind) -> Option<ContextValue>` - Remove a piece of context, return the old value if any
- `fn use_stderr(self: &Self) -> bool` - Should the message be written to `stdout` or not?
- `fn exit_code(self: &Self) -> i32` - Returns the exit code that `.exit` will exit the process with.
- `fn exit(self: &Self) -> never` - Prints the error and exits.
- `fn print(self: &Self) -> io::Result<()>` - Prints formatted and colored error to `stdout` or `stderr` according to its error kind
- `fn render(self: &Self) -> StyledStr` - Render the error message to a [`StyledStr`].

**Trait Implementations:**

- **Error**
  - `fn source(self: &Self) -> Option<&dyn error::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> Result<(), fmt::Error>`
- **From**
  - `fn from(e: fmt::Error) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **From**
  - `fn from(e: io::Error) -> Self`



## clap_builder::error::Result

*Type Alias*: `std::result::Result<T, E>`

Short hand for [`Result`] type

[`Result`]: std::result::Result



