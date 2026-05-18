**object > write > macho**

# Module: write::macho

## Contents

**Structs**

- [`MachOBuildVersion`](#machobuildversion) - The customizable portion of a [`macho::BuildVersionCommand`].

---

## object::write::macho::MachOBuildVersion

*Struct*

The customizable portion of a [`macho::BuildVersionCommand`].

**Fields:**
- `platform: u32` - One of the `PLATFORM_` constants (for example,
- `minos: u32` - The minimum OS version, where `X.Y.Z` is encoded in nibbles as
- `sdk: u32` - The SDK version as `X.Y.Z`, where `X.Y.Z` is encoded in nibbles as

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> MachOBuildVersion`
- **Clone**
  - `fn clone(self: &Self) -> MachOBuildVersion`



