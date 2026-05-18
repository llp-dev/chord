**bitflags**

# Module: bitflags

## Contents

**Macros**

- [`bitflags`](#bitflags) - The macro used to generate the flag structures.

---

## bitflags::bitflags

*Declarative Macro*

The macro used to generate the flag structures.

See the [crate level docs](../bitflags/index.html) for complete documentation.

# Example

```
use bitflags::bitflags;

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

fn main() {
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), Flags::ABC);   // union
    assert_eq!((e1 & e2), Flags::C);     // intersection
    assert_eq!((e1 - e2), Flags::A);     // set difference
    assert_eq!(!e2, Flags::A);           // set complement
}
```

The generated `struct`s can also be extended with type and trait
implementations:

```
use std::fmt;

use bitflags::bitflags;

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
    }
}

impl Flags {
    pub fn clear(&mut self) {
        self.bits = 0;  // The `bits` field can be accessed from within the
                        // same module where the `bitflags!` macro was invoked.
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hi!")
    }
}

fn main() {
    let mut flags = Flags::A | Flags::B;
    flags.clear();
    assert!(flags.is_empty());
    assert_eq!(format!("{}", flags), "hi!");
    assert_eq!(format!("{:?}", Flags::A | Flags::B), "A | B");
    assert_eq!(format!("{:?}", Flags::B), "B");
}
```

```rust
macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )*
        }

        $($t:tt)*
    ) => { ... };
    () => { ... };
}
```



