**pin_project_lite**

# Module: pin_project_lite

## Contents

**Macros**

- [`pin_project`](#pin_project) - A macro that creates a projection type covering all the fields of struct.

---

## pin_project_lite::pin_project

*Declarative Macro*

A macro that creates a projection type covering all the fields of struct.

This macro creates a projection type according to the following rules:

- For the field that uses `#[pin]` attribute, makes the pinned reference to the field.
- For the other fields, makes the unpinned reference to the field.

And the following methods are implemented on the original type:

```
# use std::pin::Pin;
# type Projection<'a> = &'a ();
# type ProjectionRef<'a> = &'a ();
# trait Dox {
fn project(self: Pin<&mut Self>) -> Projection<'_>;
fn project_ref(self: Pin<&Self>) -> ProjectionRef<'_>;
# }
```

By passing an attribute with the same name as the method to the macro,
you can name the projection type returned from the method. This allows you
to use pattern matching on the projected types.

```
# use pin_project_lite::pin_project;
# use std::pin::Pin;
pin_project! {
    #[project = EnumProj]
    enum Enum<T> {
        Variant { #[pin] field: T },
    }
}

impl<T> Enum<T> {
    fn method(self: Pin<&mut Self>) {
        let this: EnumProj<'_, T> = self.project();
        match this {
            EnumProj::Variant { field } => {
                let _: Pin<&mut T> = field;
            }
        }
    }
}
```

By passing the `#[project_replace = MyProjReplace]` attribute you may create an additional
method which allows the contents of `Pin<&mut Self>` to be replaced while simultaneously moving
out all unpinned fields in `Self`.

```
# use std::pin::Pin;
# type MyProjReplace = ();
# trait Dox {
fn project_replace(self: Pin<&mut Self>, replacement: Self) -> MyProjReplace;
# }
```

Also, note that the projection types returned by `project` and `project_ref` have
an additional lifetime at the beginning of generics.

```text
let this: EnumProj<'_, T> = self.project();
                   ^^
```

The visibility of the projected types and projection methods is based on the
original type. However, if the visibility of the original type is `pub`, the
visibility of the projected types and the projection methods is downgraded
to `pub(crate)`.

# Safety

`pin_project!` macro guarantees safety in much the same way as [pin-project] crate.
Both are completely safe unless you write other unsafe code.

See [pin-project] crate for more details.

# Examples

```
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    struct Struct<T, U> {
        #[pin]
        pinned: T,
        unpinned: U,
    }
}

impl<T, U> Struct<T, U> {
    fn method(self: Pin<&mut Self>) {
        let this = self.project();
        let _: Pin<&mut T> = this.pinned; // Pinned reference to the field
        let _: &mut U = this.unpinned; // Normal reference to the field
    }
}
```

To use `pin_project!` on enums, you need to name the projection type
returned from the method.

```
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    #[project = EnumProj]
    enum Enum<T> {
        Struct {
            #[pin]
            field: T,
        },
        Unit,
    }
}

impl<T> Enum<T> {
    fn method(self: Pin<&mut Self>) {
        match self.project() {
            EnumProj::Struct { field } => {
                let _: Pin<&mut T> = field;
            }
            EnumProj::Unit => {}
        }
    }
}
```

If you want to call the `project()` method multiple times or later use the
original [`Pin`] type, it needs to use [`.as_mut()`][`Pin::as_mut`] to avoid
consuming the [`Pin`].

```
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    struct Struct<T> {
        #[pin]
        field: T,
    }
}

impl<T> Struct<T> {
    fn call_project_twice(mut self: Pin<&mut Self>) {
        // `project` consumes `self`, so reborrow the `Pin<&mut Self>` via `as_mut`.
        self.as_mut().project();
        self.as_mut().project();
    }
}
```

# `!Unpin`

If you want to make sure `Unpin` is not implemented, use the `#[project(!Unpin)]`
attribute.

```
use pin_project_lite::pin_project;

pin_project! {
     #[project(!Unpin)]
     struct Struct<T> {
         #[pin]
         field: T,
     }
}
```

This is equivalent to using `#[pin]` attribute for a [`PhantomPinned`] field.

```
use std::marker::PhantomPinned;

use pin_project_lite::pin_project;

pin_project! {
    struct Struct<T> {
        field: T,
        #[pin]
        _pin: PhantomPinned,
    }
}
```

Note that using [`PhantomPinned`] without `#[pin]` or `#[project(!Unpin)]`
attribute has no effect.

# Pinned Drop

In order to correctly implement pin projections, a type's [`Drop`] impl must not move out of any
structurally pinned fields. Unfortunately, [`Drop::drop`] takes `&mut Self`, not `Pin<&mut Self>`.

To implement [`Drop`] for type that has pin, add an `impl PinnedDrop` block at the end of the
[`pin_project`] macro block. PinnedDrop has the following interface:

```
# use std::pin::Pin;
trait PinnedDrop {
    fn drop(this: Pin<&mut Self>);
}
```

Note that the argument to `PinnedDrop::drop` cannot be named `self`.

`pin_project!` implements the actual [`Drop`] trait via PinnedDrop you implemented. To
explicitly drop a type that implements PinnedDrop, use the [drop] function just like dropping a
type that directly implements [`Drop`].

`PinnedDrop::drop` will never be called more than once, just like [`Drop::drop`].

```
use pin_project_lite::pin_project;

pin_project! {
    pub struct Struct<'a> {
        was_dropped: &'a mut bool,
        #[pin]
        field: u8,
    }

    impl PinnedDrop for Struct<'_> {
        fn drop(this: Pin<&mut Self>) { // <----- NOTE: this is not `self`
            **this.project().was_dropped = true;
        }
    }
}

let mut was_dropped = false;
drop(Struct { was_dropped: &mut was_dropped, field: 42 });
assert!(was_dropped);
```

[`PhantomPinned`]: core::marker::PhantomPinned
[`Pin::as_mut`]: core::pin::Pin::as_mut
[`Pin`]: core::pin::Pin
[pin-project]: https://github.com/taiki-e/pin-project

```rust
macro_rules! pin_project {
    ($($tt:tt)*) => { ... };
}
```



