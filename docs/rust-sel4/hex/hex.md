**hex**

# Module: hex

## Contents

**Functions**

- [`decode`](#decode) - Decodes a hex string into raw bytes.
- [`decode_to_slice`](#decode_to_slice) - Decode a hex string into a mutable bytes slice.
- [`encode`](#encode) - Encodes `data` as hex string using lowercase characters.
- [`encode_to_slice`](#encode_to_slice) - Encodes some bytes into a mutable slice of bytes.
- [`encode_upper`](#encode_upper) - Encodes `data` as hex string using uppercase characters.

**Traits**

- [`FromHex`](#fromhex) - Types that can be decoded from a hex string.
- [`ToHex`](#tohex) - Encoding values as hex string.

---

## hex::FromHex

*Trait*

Types that can be decoded from a hex string.

This trait is implemented for `Vec<u8>` and small `u8`-arrays.

# Example

```
use core::str;
use hex::FromHex;

let buffer = <[u8; 12]>::from_hex("48656c6c6f20776f726c6421")?;
let string = str::from_utf8(&buffer).expect("invalid buffer length");

println!("{}", string); // prints "Hello world!"
# assert_eq!("Hello world!", string);
# Ok::<(), hex::FromHexError>(())
```

**Methods:**

- `Error`
- `from_hex`: Creates an instance of type `Self` from the given hex string, or fails



## hex::ToHex

*Trait*

Encoding values as hex string.

This trait is implemented for all `T` which implement `AsRef<[u8]>`. This
includes `String`, `str`, `Vec<u8>` and `[u8]`.

# Example

```
use hex::ToHex;

println!("{}", "Hello world!".encode_hex::<String>());
# assert_eq!("Hello world!".encode_hex::<String>(), "48656c6c6f20776f726c6421".to_string());
```

*Note*: instead of using this trait, you might want to use [`encode()`].

**Methods:**

- `encode_hex`: Encode the hex strict representing `self` into the result. Lower case
- `encode_hex_upper`: Encode the hex strict representing `self` into the result. Upper case



## hex::decode

*Function*

Decodes a hex string into raw bytes.

Both, upper and lower case characters are valid in the input string and can
even be mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings).

# Example

```
assert_eq!(
    hex::decode("48656c6c6f20776f726c6421"),
    Ok("Hello world!".to_owned().into_bytes())
);

assert_eq!(hex::decode("123"), Err(hex::FromHexError::OddLength));
assert!(hex::decode("foo").is_err());
```

```rust
fn decode<T>(data: T) -> Result<alloc::vec::Vec<u8>, FromHexError>
```



## hex::decode_to_slice

*Function*

Decode a hex string into a mutable bytes slice.

Both, upper and lower case characters are valid in the input string and can
even be mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings).

# Example

```
let mut bytes = [0u8; 4];
assert_eq!(hex::decode_to_slice("6b697769", &mut bytes as &mut [u8]), Ok(()));
assert_eq!(&bytes, b"kiwi");
```

```rust
fn decode_to_slice<T>(data: T, out: & mut [u8]) -> Result<(), FromHexError>
```



## hex::encode

*Function*

Encodes `data` as hex string using lowercase characters.

Lowercase characters are used (e.g. `f9b4ca`). The resulting string's
length is always even, each byte in `data` is always encoded using two hex
digits. Thus, the resulting string contains exactly twice as many bytes as
the input data.

# Example

```
assert_eq!(hex::encode("Hello world!"), "48656c6c6f20776f726c6421");
assert_eq!(hex::encode(vec![1, 2, 3, 15, 16]), "0102030f10");
```

```rust
fn encode<T>(data: T) -> alloc::string::String
```



## hex::encode_to_slice

*Function*

Encodes some bytes into a mutable slice of bytes.

The output buffer, has to be able to hold at least `input.len() * 2` bytes,
otherwise this function will return an error.

# Example

```
# use hex::FromHexError;
# fn main() -> Result<(), FromHexError> {
let mut bytes = [0u8; 4 * 2];

hex::encode_to_slice(b"kiwi", &mut bytes)?;
assert_eq!(&bytes, b"6b697769");
# Ok(())
# }
```

```rust
fn encode_to_slice<T>(input: T, output: & mut [u8]) -> Result<(), FromHexError>
```



## hex::encode_upper

*Function*

Encodes `data` as hex string using uppercase characters.

Apart from the characters' casing, this works exactly like `encode()`.

# Example

```
assert_eq!(hex::encode_upper("Hello world!"), "48656C6C6F20776F726C6421");
assert_eq!(hex::encode_upper(vec![1, 2, 3, 15, 16]), "0102030F10");
```

```rust
fn encode_upper<T>(data: T) -> alloc::string::String
```



