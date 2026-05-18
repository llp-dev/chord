# crc

# crc
Rust implementation of CRC.

### Examples
Using a well-known algorithm:
```rust
const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
assert_eq!(X25.checksum(b"123456789"), 0x906e);
```

Using a custom algorithm:
```rust
const CUSTOM_ALG: crc::Algorithm<u16> = crc::Algorithm {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xaee7,
    residue: 0x0000
};
let crc = crc::Crc::<u16>::new(&CUSTOM_ALG);
let mut digest = crc.digest();
digest.update(b"123456789");
assert_eq!(digest.finalize(), 0xaee7);
```

## Modules

### [`crc`](crc.md)

*1 trait, 1 type alias, 3 structs*

### [`private`](private.md)

*1 trait*

