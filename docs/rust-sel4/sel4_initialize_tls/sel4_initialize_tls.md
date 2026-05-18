**sel4_initialize_tls**

# Module: sel4_initialize_tls

## Contents

**Structs**

- [`InvalidTlsImageError`](#invalidtlsimageerror)
- [`Region`](#region)
- [`RegionLayoutError`](#regionlayouterror)
- [`TlsImage`](#tlsimage)
- [`TlsReservationLayout`](#tlsreservationlayout)
- [`UncheckedTlsImage`](#uncheckedtlsimage)

---

## sel4_initialize_tls::InvalidTlsImageError

*Struct*

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &InvalidTlsImageError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> InvalidTlsImageError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_initialize_tls::Region

*Struct*

**Methods:**

- `fn new(start: *mut u8, size: usize) -> Self`
- `fn start(self: &Self) -> *mut u8`
- `fn size(self: &Self) -> usize`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Region`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Region) -> bool`



## sel4_initialize_tls::RegionLayoutError

*Struct*

**Tuple Struct**: `()`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RegionLayoutError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RegionLayoutError`



## sel4_initialize_tls::TlsImage

*Struct*

**Methods:**

- `fn reservation_layout(self: &Self) -> TlsReservationLayout`
- `fn initialize_reservation(self: &Self, reservation_start: *mut u8) -> usize`
- `fn initialize_exact_reservation_region(self: &Self, exact_reservation: &Region) -> Result<usize, RegionLayoutError>`
- `fn initialize_inexact_reservation_region(self: &Self, inexact_reservation: &Region) -> Result<usize, RegionLayoutError>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TlsImage) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TlsImage`



## sel4_initialize_tls::TlsReservationLayout

*Struct*

**Methods:**

- `fn footprint(self: &Self) -> Layout`
- `fn segment_offset(self: &Self) -> usize`
- `fn thread_pointer_offset(self: &Self) -> usize`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &TlsReservationLayout) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TlsReservationLayout`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_initialize_tls::UncheckedTlsImage

*Struct*

**Fields:**
- `vaddr: usize`
- `filesz: usize`
- `memsz: usize`
- `align: usize`

**Methods:**

- `fn check(self: &Self) -> Result<TlsImage, InvalidTlsImageError>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UncheckedTlsImage) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UncheckedTlsImage`



