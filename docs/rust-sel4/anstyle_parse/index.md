# anstyle_parse

Parser for implementing virtual terminal emulators

[`Parser`] is implemented according to [Paul Williams' ANSI parser
state machine]. The state machine doesn't assign meaning to the parsed data
and is thus not itself sufficient for writing a terminal emulator. Instead,
it is expected that an implementation of [`Perform`] is provided which does
something useful with the parsed data. The [`Parser`] handles the book
keeping, and the [`Perform`] gets to simply handle actions.

# Examples

For an example of using the [`Parser`] please see the examples folder. The example included
there simply logs all the actions [`Perform`] does. One quick thing to see it in action is to
pipe `vim` into it

```sh
cargo build --release --example parselog
vim | target/release/examples/parselog
```

Just type `:q` to exit.

# Differences from original state machine description

* UTF-8 Support for Input
* OSC Strings can be terminated by 0x07
* Only supports 7-bit codes. Some 8-bit codes are still supported, but they no longer work in
  all states.

[Paul Williams' ANSI parser state machine]: https://vt100.net/emu/dec_ansi_parser

## Modules

### [`anstyle_parse`](anstyle_parse.md)

*1 module, 1 type alias, 2 traits, 3 structs*

### [`params`](params.md)

*2 structs*

### [`state`](state.md)

*1 function*

### [`state::definitions`](state/definitions.md)

*2 enums*

