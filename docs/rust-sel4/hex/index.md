# hex

Encoding and decoding hex strings.

For most cases, you can simply use the [`decode`], [`encode`] and
[`encode_upper`] functions. If you need a bit more control, use the traits
[`ToHex`] and [`FromHex`] instead.

# Example

```
# #[cfg(not(feature = "alloc"))]
# let mut output = [0; 0x18];
#
# #[cfg(not(feature = "alloc"))]
# hex::encode_to_slice(b"Hello world!", &mut output).unwrap();
#
# #[cfg(not(feature = "alloc"))]
# let hex_string = ::core::str::from_utf8(&output).unwrap();
#
# #[cfg(feature = "alloc")]
let hex_string = hex::encode("Hello world!");

println!("{}", hex_string); // Prints "48656c6c6f20776f726c6421"

# assert_eq!(hex_string, "48656c6c6f20776f726c6421");
```

## Modules

### [`hex`](hex.md)

*2 traits, 5 functions*

### [`error`](error.md)

*1 enum*

