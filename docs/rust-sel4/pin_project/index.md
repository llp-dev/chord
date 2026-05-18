# pin_project

<!-- Note: Document from sync-markdown-to-rustdoc:start through sync-markdown-to-rustdoc:end
     is synchronized from README.md. Any changes to that range are not preserved. -->
<!-- tidy:sync-markdown-to-rustdoc:start -->

A crate for safe and ergonomic [pin-projection].

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pin-project = "1"
```

## Examples

[`#[pin_project]`][`pin_project`] attribute creates projection types
covering all the fields of struct or enum.

```
use std::pin::Pin;

use pin_project::pin_project;

#[pin_project]
struct Struct<T, U> {
    #[pin]
    pinned: T,
    unpinned: U,
}

impl<T, U> Struct<T, U> {
    fn method(self: Pin<&mut Self>) {
        let this = self.project();
        let _: Pin<&mut T> = this.pinned; // Pinned reference to the field
        let _: &mut U = this.unpinned; // Normal reference to the field
    }
}
```

[*code like this will be generated*][struct-default-expanded]

To use `#[pin_project]` on enums, you need to name the projection type
returned from the method.

```
use std::pin::Pin;

use pin_project::pin_project;

#[pin_project(project = EnumProj)]
enum Enum<T, U> {
    Pinned(#[pin] T),
    Unpinned(U),
}

impl<T, U> Enum<T, U> {
    fn method(self: Pin<&mut Self>) {
        match self.project() {
            EnumProj::Pinned(x) => {
                let _: Pin<&mut T> = x;
            }
            EnumProj::Unpinned(y) => {
                let _: &mut U = y;
            }
        }
    }
}
```

[*code like this will be generated*][enum-default-expanded]

See [`#[pin_project]`][`pin_project`] attribute for more details, and
see [examples] directory for more examples and generated code.

## Related Projects

- [pin-project-lite]: A lightweight version of pin-project written with declarative macros.

[enum-default-expanded]: https://github.com/taiki-e/pin-project/blob/HEAD/examples/enum-default-expanded.rs
[examples]: https://github.com/taiki-e/pin-project/blob/HEAD/examples/README.md
[pin-project-lite]: https://github.com/taiki-e/pin-project-lite
[pin-projection]: https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning
[struct-default-expanded]: https://github.com/taiki-e/pin-project/blob/HEAD/examples/struct-default-expanded.rs

<!-- tidy:sync-markdown-to-rustdoc:end -->

## Modules

### [`pin_project`](pin_project.md)

*1 trait*

