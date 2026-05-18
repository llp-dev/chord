# anstream

**Auto-adapting [`stdout`] / [`stderr`] streams**

*A portmanteau of "ansi stream"*

[`AutoStream`] always accepts [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code),
[adapting to the user's terminal's capabilities][AutoStream].

Benefits
- Allows the caller to not be concerned with the terminal's capabilities
- Semver safe way of passing styled text between crates as ANSI escape codes offer more
  compatibility than most crate APIs.

Available styling crates:
- [anstyle](https://docs.rs/anstyle) for minimal runtime styling, designed to go in public APIs
- [owo-colors](https://docs.rs/owo-colors) for feature-rich runtime styling
- [color-print](https://docs.rs/color-print) for feature-rich compile-time styling

# Example

```
#  #[cfg(feature = "auto")] {
use anstream::println;
use owo_colors::OwoColorize as _;

// Foreground colors
println!("My number is {:#x}!", 10.green());
// Background colors
println!("My number is not {}!", 4.on_red());
# }
```

And this will correctly handle piping to a file, etc

## Modules

### [`anstream`](anstream.md)

*2 functions, 2 modules, 2 type aliases, 5 macros*

### [`adapter::strip`](adapter/strip.md)

*2 functions, 6 structs*

### [`adapter::wincon`](adapter/wincon.md)

*2 structs*

### [`auto`](auto.md)

*1 struct*

### [`stream`](stream.md)

*3 traits*

### [`stream::private`](stream/private.md)

*1 trait*

### [`strip`](strip.md)

*1 struct*

