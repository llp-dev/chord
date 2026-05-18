# uguid

Library providing a GUID (Globally Unique Identifier) type. The
format is defined in [RFC 4122]. However, unlike "normal" UUIDs
(such as those provided by the [`uuid`] crate), the first three
fields are little-endian. See [Appendix A] of the UEFI
Specification. This format of GUID is also used in Microsoft
Windows.

[Appendix A]: https://uefi.org/specs/UEFI/2.10/Apx_A_GUID_and_Time_Formats.html
[RFC 4122]: https://datatracker.ietf.org/doc/html/rfc4122
[`uuid`]: https://docs.rs/uuid/latest/uuid

# Features

No features are enabled by default.

* `bytemuck`: Implements bytemuck's `Pod` and `Zeroable` traits for `Guid`.
* `serde`: Implements serde's `Serialize` and `Deserialize` traits for `Guid`.
* `std`: Currently has no effect.

# Examples

Construct a GUID at compile time with the `guid!` macro:

```
use uguid::guid;

let guid = guid!("01234567-89ab-cdef-0123-456789abcdef");
```

Parse a GUID at runtime from a string:

```
use uguid::Guid;

let guid: Guid = "01234567-89ab-cdef-0123-456789abcdef".parse().unwrap();
```

Construct a GUID from its components or a byte array:

```
use uguid::Guid;

##[rustfmt::skip]
let guid1 = Guid::from_bytes([
    0x01, 0x02, 0x03, 0x04,
    0x05, 0x06, 0x07, 0x08,
    0x09, 0x10, 0x11, 0x12,
    0x13, 0x14, 0x15, 0x16,
]);
let guid2 = Guid::new(
    [0x01, 0x02, 0x03, 0x04],
    [0x05, 0x06],
    [0x07, 0x08],
    0x09,
    0x10,
    [0x11, 0x12, 0x13, 0x14, 0x15, 0x16],
);
assert_eq!(guid1, guid2);
```

Convert to a string or a byte array:

```
use uguid::guid;

let guid = guid!("01234567-89ab-cdef-0123-456789abcdef");
assert_eq!(guid.to_string(), "01234567-89ab-cdef-0123-456789abcdef");
assert_eq!(
    guid.to_bytes(),
    [
        0x67, 0x45, 0x23, 0x01, 0xab, 0x89, 0xef, 0xcd, 0x01, 0x23, 0x45,
        0x67, 0x89, 0xab, 0xcd, 0xef
    ]
);
```

## Modules

### [`uguid`](uguid.md)

*2 macros, 3 modules*

### [`error`](error.md)

*1 enum*

### [`guid`](guid.md)

*1 enum, 1 struct*

### [`util`](util.md)

*4 functions*

