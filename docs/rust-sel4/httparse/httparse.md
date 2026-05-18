**httparse**

# Module: httparse

## Contents

**Structs**

- [`Header`](#header) - Represents a parsed header.
- [`InvalidChunkSize`](#invalidchunksize) - An error in parsing a chunk size.
- [`ParserConfig`](#parserconfig) - Parser configuration.
- [`Request`](#request) - A parsed Request.
- [`Response`](#response) - A parsed Response.

**Enums**

- [`Error`](#error) - An error in parsing.
- [`Status`](#status) - The result of a successful parse pass.

**Functions**

- [`parse_chunk_size`](#parse_chunk_size) - Parse a buffer of bytes as a chunk size.
- [`parse_headers`](#parse_headers) - Parse a buffer of bytes as headers.

**Constants**

- [`EMPTY_HEADER`](#empty_header) - An empty header, useful for constructing a `Header` array to pass in for

**Type Aliases**

- [`Result`](#result) - A Result of any parsing action.

---

## httparse::EMPTY_HEADER

*Constant*: `Header<'static>`

An empty header, useful for constructing a `Header` array to pass in for
parsing.

# Example

```
let headers = [httparse::EMPTY_HEADER; 64];
```



## httparse::Error

*Enum*

An error in parsing.

**Variants:**
- `HeaderName` - Invalid byte in header name.
- `HeaderValue` - Invalid byte in header value.
- `NewLine` - Invalid byte in new line.
- `Status` - Invalid byte in Response status.
- `Token` - Invalid byte where token is required.
- `TooManyHeaders` - Parsed more headers than provided buffer can contain.
- `Version` - Invalid byte in HTTP version.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## httparse::Header

*Struct*

Represents a parsed header.

**Generic Parameters:**
- 'a

**Fields:**
- `name: &'a str` - The name portion of a header.
- `value: &'a [u8]` - The value portion of a header.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Header<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Header<'a>`



## httparse::InvalidChunkSize

*Struct*

An error in parsing a chunk size.

**Unit Struct**

**Traits:** Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &InvalidChunkSize) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## httparse::ParserConfig

*Struct*

Parser configuration.

**Methods:**

- `fn allow_spaces_after_header_name_in_responses(self: & mut Self, value: bool) -> & mut Self` - Sets whether spaces and tabs should be allowed after header names in responses.
- `fn allow_multiple_spaces_in_request_line_delimiters(self: & mut Self, value: bool) -> & mut Self` - Sets whether multiple spaces are allowed as delimiters in request lines.
- `fn multiple_spaces_in_request_line_delimiters_are_allowed(self: &Self) -> bool` - Whether multiple spaces are allowed as delimiters in request lines.
- `fn allow_multiple_spaces_in_response_status_delimiters(self: & mut Self, value: bool) -> & mut Self` - Sets whether multiple spaces are allowed as delimiters in response status lines.
- `fn multiple_spaces_in_response_status_delimiters_are_allowed(self: &Self) -> bool` - Whether multiple spaces are allowed as delimiters in response status lines.
- `fn allow_obsolete_multiline_headers_in_responses(self: & mut Self, value: bool) -> & mut Self` - Sets whether obsolete multiline headers should be allowed.
- `fn obsolete_multiline_headers_in_responses_are_allowed(self: &Self) -> bool` - Whether obsolete multiline headers should be allowed.
- `fn allow_space_before_first_header_name(self: & mut Self, value: bool) -> & mut Self` - Sets whether white space before the first header is allowed
- `fn space_before_first_header_name_are_allowed(self: &Self) -> bool` - Whether white space before first header is allowed or not
- `fn parse_request<'buf>(self: &Self, request: & mut Request<'buf>, buf: &'buf [u8]) -> Result<usize>` - Parses a request with the given config.
- `fn parse_request_with_uninit_headers<'headers, 'buf>(self: &Self, request: & mut Request<'headers, 'buf>, buf: &'buf [u8], headers: &'headers  mut [MaybeUninit<Header<'buf>>]) -> Result<usize>` - Parses a request with the given config and buffer for headers
- `fn ignore_invalid_headers_in_responses(self: & mut Self, value: bool) -> & mut Self` - Sets whether invalid header lines should be silently ignored in responses.
- `fn ignore_invalid_headers_in_requests(self: & mut Self, value: bool) -> & mut Self` - Sets whether invalid header lines should be silently ignored in requests.
- `fn parse_response<'buf>(self: &Self, response: & mut Response<'buf>, buf: &'buf [u8]) -> Result<usize>` - Parses a response with the given config.
- `fn parse_response_with_uninit_headers<'headers, 'buf>(self: &Self, response: & mut Response<'headers, 'buf>, buf: &'buf [u8], headers: &'headers  mut [MaybeUninit<Header<'buf>>]) -> Result<usize>` - Parses a response with the given config and buffer for headers

**Trait Implementations:**

- **Default**
  - `fn default() -> ParserConfig`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ParserConfig`



## httparse::Request

*Struct*

A parsed Request.

The optional values will be `None` if a parse was not complete, and did not
parse the associated property. This allows you to inspect the parts that
could be parsed, before reading more, in case you wish to exit early.

# Example

```no_run
let buf = b"GET /404 HTTP/1.1\r\nHost:";
let mut headers = [httparse::EMPTY_HEADER; 16];
let mut req = httparse::Request::new(&mut headers);
let res = req.parse(buf).unwrap();
if res.is_partial() {
    match req.path {
        Some(ref path) => {
            // check router for path.
            // /404 doesn't exist? we could stop parsing
        },
        None => {
            // must read more and parse again
        }
    }
}
```

**Generic Parameters:**
- 'headers
- 'buf

**Fields:**
- `method: Option<&'buf str>` - The request method, such as `GET`.
- `path: Option<&'buf str>` - The request path, such as `/about-us`.
- `version: Option<u8>` - The request minor version, such as `1` for `HTTP/1.1`.
- `headers: &'headers  mut [Header<'buf>]` - The request headers.

**Methods:**

- `fn new(headers: &'h  mut [Header<'b>]) -> Request<'h, 'b>` - Creates a new Request, using a slice of headers you allocate.
- `fn parse_with_uninit_headers(self: & mut Self, buf: &'b [u8], headers: &'h  mut [MaybeUninit<Header<'b>>]) -> Result<usize>` - Try to parse a buffer of bytes into the Request,
- `fn parse(self: & mut Self, buf: &'b [u8]) -> Result<usize>` - Try to parse a buffer of bytes into the Request.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Request<'headers, 'buf>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## httparse::Response

*Struct*

A parsed Response.

See `Request` docs for explanation of optional values.

**Generic Parameters:**
- 'headers
- 'buf

**Fields:**
- `version: Option<u8>` - The response minor version, such as `1` for `HTTP/1.1`.
- `code: Option<u16>` - The response code, such as `200`.
- `reason: Option<&'buf str>` - The response reason-phrase, such as `OK`.
- `headers: &'headers  mut [Header<'buf>]` - The response headers.

**Methods:**

- `fn new(headers: &'h  mut [Header<'b>]) -> Response<'h, 'b>` - Creates a new `Response` using a slice of `Header`s you have allocated.
- `fn parse(self: & mut Self, buf: &'b [u8]) -> Result<usize>` - Try to parse a buffer of bytes into this `Response`.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Response<'headers, 'buf>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## httparse::Result

*Type Alias*: `result::Result<Status<T>, Error>`

A Result of any parsing action.

If the input is invalid, an `Error` will be returned. Note that incomplete
data is not considered invalid, and so will not return an error, but rather
a `Ok(Status::Partial)`.



## httparse::Status

*Enum*

The result of a successful parse pass.

`Complete` is used when the buffer contained the complete value.
`Partial` is used when parsing did not reach the end of the expected value,
but no invalid data was found.

**Generic Parameters:**
- T

**Variants:**
- `Complete(T)` - The completed result.
- `Partial` - A partial result.

**Methods:**

- `fn is_complete(self: &Self) -> bool` - Convenience method to check if status is complete.
- `fn is_partial(self: &Self) -> bool` - Convenience method to check if status is partial.
- `fn unwrap(self: Self) -> T` - Convenience method to unwrap a Complete value. Panics if the status is

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Status<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Status<T>`



## httparse::parse_chunk_size

*Function*

Parse a buffer of bytes as a chunk size.

The return value, if complete and successful, includes the index of the
buffer that parsing stopped at, and the size of the following chunk.

# Example

```
let buf = b"4\r\nRust\r\n0\r\n\r\n";
assert_eq!(httparse::parse_chunk_size(buf),
           Ok(httparse::Status::Complete((3, 4))));
```

```rust
fn parse_chunk_size(buf: &[u8]) -> result::Result<Status<(usize, u64)>, InvalidChunkSize>
```



## httparse::parse_headers

*Function*

Parse a buffer of bytes as headers.

The return value, if complete and successful, includes the index of the
buffer that parsing stopped at, and a sliced reference to the parsed
headers. The length of the slice will be equal to the number of properly
parsed headers.

# Example

```
let buf = b"Host: foo.bar\nAccept: */*\n\nblah blah";
let mut headers = [httparse::EMPTY_HEADER; 4];
assert_eq!(httparse::parse_headers(buf, &mut headers),
           Ok(httparse::Status::Complete((27, &[
               httparse::Header { name: "Host", value: b"foo.bar" },
               httparse::Header { name: "Accept", value: b"*/*" }
           ][..]))));
```

```rust
fn parse_headers<'b, 'h>(src: &'b [u8], dst: &'h  mut [Header<'b>]) -> Result<(usize, &'h [Header<'b>])>
```



