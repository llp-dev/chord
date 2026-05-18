# futures_channel

Asynchronous channels.

Like threads, concurrent tasks sometimes need to communicate with each
other. This module contains two basic abstractions for doing so:

- [oneshot], a way of sending a single value from one task to another.
- [mpsc], a multi-producer, single-consumer channel for sending values
  between tasks, analogous to the similarly-named structure in the standard
  library.

All items are only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

## Modules

### [`futures_channel`](futures_channel.md)

*1 module*

### [`oneshot`](oneshot.md)

*1 function, 4 structs*

