# rancor

# rancor

rancor provides scalable and efficient error handling without using type
composition. This makes it best-suited for situations where:

- Programmatic error introspection is not useful
- Functions may error, but succeed most of the time
- Errors should provide as much useful detail as possible when emitted
- Use cases include both `no_std` and targets with support for `std`

## Features

- `alloc`: Provides the [`BoxedError`] type. Enabled by default.

## Modules

### [`rancor`](rancor.md)

*1 enum, 1 function, 1 macro, 3 structs, 6 traits*

### [`boxed_error`](boxed_error.md)

*1 struct*

