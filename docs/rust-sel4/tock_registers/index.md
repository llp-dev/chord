# tock_registers

Tock Register Interface

Provides efficient mechanisms to express and use type-checked memory mapped
registers and bitfields.

```rust
# fn main() {}

use tock_registers::registers::{ReadOnly, ReadWrite};
use tock_registers::register_bitfields;

// Register maps are specified like this:
#[repr(C)]
struct Registers {
    // Control register: read-write
    cr: ReadWrite<u32, Control::Register>,
    // Status register: read-only
    s: ReadOnly<u32, Status::Register>,
}

// Register fields and definitions look like this:
register_bitfields![u32,
    // Simpler bitfields are expressed concisely:
    Control [
        /// Stop the Current Transfer
        STOP 8,
        /// Software Reset
        SWRST 7,
        /// Master Disable
        MDIS 1,
        /// Master Enable
        MEN 0
    ],

    // More complex registers can express subtypes:
    Status [
        TXCOMPLETE  OFFSET(0) NUMBITS(1) [],
        TXINTERRUPT OFFSET(1) NUMBITS(1) [],
        RXCOMPLETE  OFFSET(2) NUMBITS(1) [],
        RXINTERRUPT OFFSET(3) NUMBITS(1) [],
        MODE        OFFSET(4) NUMBITS(3) [
            FullDuplex = 0,
            HalfDuplex = 1,
            Loopback = 2,
            Disabled = 3
        ],
        ERRORCOUNT OFFSET(6) NUMBITS(3) []
    ]
];
```

Author
------
- Shane Leonard <shanel@stanford.edu>

## Modules

### [`tock_registers`](tock_registers.md)

*2 traits, 6 modules, 7 macros*

### [`debug`](debug.md)

*1 struct, 2 enums, 2 traits*

### [`fields`](fields.md)

*1 trait, 2 macros, 2 structs*

### [`interfaces`](interfaces.md)

*4 traits*

### [`local_register`](local_register.md)

*1 macro, 1 struct*

### [`registers`](registers.md)

*5 structs*

