**rustls > msgs > ffdhe_groups**

# Module: msgs::ffdhe_groups

## Contents

**Structs**

- [`FfdheGroup`](#ffdhegroup) - Parameters of an FFDHE group, with Big-endian byte order

**Constants**

- [`FFDHE2048`](#ffdhe2048) - FFDHE2048 group defined in [RFC 7919 Appendix A.1]
- [`FFDHE3072`](#ffdhe3072) - FFDHE3072 group defined in [RFC 7919 Appendix A.2]
- [`FFDHE4096`](#ffdhe4096) - FFDHE4096 group defined in [RFC 7919 Appendix A.3]
- [`FFDHE6144`](#ffdhe6144) - FFDHE6144 group defined in [RFC 7919 Appendix A.4]
- [`FFDHE8192`](#ffdhe8192) - FFDHE8192 group defined in [RFC 7919 Appendix A.5]

---

## rustls::msgs::ffdhe_groups::FFDHE2048

*Constant*: `FfdheGroup<'static>`

FFDHE2048 group defined in [RFC 7919 Appendix A.1]

[RFC 7919 Appendix A.1]: https://datatracker.ietf.org/doc/html/rfc7919#appendix-A.1



## rustls::msgs::ffdhe_groups::FFDHE3072

*Constant*: `FfdheGroup<'static>`

FFDHE3072 group defined in [RFC 7919 Appendix A.2]

[RFC 7919 Appendix A.2]: https://datatracker.ietf.org/doc/html/rfc7919#appendix-A.2



## rustls::msgs::ffdhe_groups::FFDHE4096

*Constant*: `FfdheGroup<'static>`

FFDHE4096 group defined in [RFC 7919 Appendix A.3]

[RFC 7919 Appendix A.3]: https://datatracker.ietf.org/doc/html/rfc7919#appendix-A.3



## rustls::msgs::ffdhe_groups::FFDHE6144

*Constant*: `FfdheGroup<'static>`

FFDHE6144 group defined in [RFC 7919 Appendix A.4]

[RFC 7919 Appendix A.4]: https://datatracker.ietf.org/doc/html/rfc7919#appendix-A.4



## rustls::msgs::ffdhe_groups::FFDHE8192

*Constant*: `FfdheGroup<'static>`

FFDHE8192 group defined in [RFC 7919 Appendix A.5]

[RFC 7919 Appendix A.5]: https://datatracker.ietf.org/doc/html/rfc7919#appendix-A.5



## rustls::msgs::ffdhe_groups::FfdheGroup

*Struct*

Parameters of an FFDHE group, with Big-endian byte order

**Generic Parameters:**
- 'a

**Fields:**
- `p: &'a [u8]`
- `g: &'a [u8]`

**Methods:**

- `fn from_named_group(named_group: NamedGroup) -> Option<Self>` - Return the `FfdheGroup` corresponding to the provided `NamedGroup`
- `fn named_group(self: &Self) -> Option<NamedGroup>` - Return the `NamedGroup` for the `FfdheGroup` if it represents one.
- `fn from_params_trimming_leading_zeros(p: &'a [u8], g: &'a [u8]) -> Self` - Construct an `FfdheGroup` from the given `p` and `g`, trimming any potential leading zeros.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FfdheGroup<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FfdheGroup<'a>) -> bool`



