**rkyv > niche > niching**

# Module: niche::niching

## Contents

**Structs**

- [`Bool`](#bool) - [`Niching`] for booleans.
- [`DefaultNiche`](#defaultniche) - Default [`Niching`] for various types.
- [`NaN`](#nan) - [`Niching`] for NaN-niched values.
- [`Null`](#null) - [`Niching`] for null-pointer-niched values.
- [`Zero`](#zero) - [`Niching`] for zero-niched values.

**Traits**

- [`Niching`](#niching) - A type that can be used to niche a value with [`NicheInto`].
- [`SharedNiching`](#sharedniching) - Trait to allow `NichedOption<Self, N1>` to be niched further by `N2`.

---

## rkyv::niche::niching::Bool

*Struct*

[`Niching`] for booleans.

**Unit Struct**

**Trait Implementations:**

- **Niching**
  - `fn is_niched(niched: *const bool) -> bool`
  - `fn resolve_niched(out: Place<bool>)`



## rkyv::niche::niching::DefaultNiche

*Struct*

Default [`Niching`] for various types.

Also serves as with-wrapper by being shorthand for
`NicheInto<DefaultNiche>`.

**Unit Struct**

**Trait Implementations:**

- **Niching**
  - `fn is_niched(niched: *const ArchivedBox<T>) -> bool`
  - `fn resolve_niched(out: Place<ArchivedBox<T>>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroU64) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroU64>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroI128) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroI128>)`
- **Niching**
  - `fn is_niched(niched: *const NonZeroI8) -> bool`
  - `fn resolve_niched(out: Place<NonZeroI8>)`
- **ArchiveWith**
  - `fn resolve_with(field: &Option<T>, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`
- **DeserializeWith**
  - `fn deserialize_with(field: &NichedOption<<T as >::Archived, Self>, deserializer: & mut D) -> Result<Option<T>, <D as >::Error>`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroU32) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroU32>)`
- **Niching**
  - `fn is_niched(niched: *const bool) -> bool`
  - `fn resolve_niched(out: Place<bool>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroI64) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroI64>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroU16) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroU16>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroI32) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroI32>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroU128) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroU128>)`
- **Niching**
  - `fn is_niched(niched: *const NonZeroU8) -> bool`
  - `fn resolve_niched(out: Place<NonZeroU8>)`
- **SerializeWith**
  - `fn serialize_with(field: &Option<T>, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroI16) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroI16>)`



## rkyv::niche::niching::NaN

*Struct*

[`Niching`] for NaN-niched values.

**Unit Struct**

**Trait Implementations:**

- **Niching**
  - `fn is_niched(niched: *const ArchivedF32) -> bool`
  - `fn resolve_niched(out: Place<ArchivedF32>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedF64) -> bool`
  - `fn resolve_niched(out: Place<ArchivedF64>)`



## rkyv::niche::niching::Niching

*Trait*

A type that can be used to niche a value with [`NicheInto`].

# Example

```
use rkyv::{
    niche::niching::Niching, primitive::ArchivedU32, with::NicheInto,
    Archive, Archived, Place, Serialize,
};

// Let's niche `Option<u32>` by using odd values
struct NeverOdd;

impl Niching<ArchivedU32> for NeverOdd {
    unsafe fn is_niched(niched: *const ArchivedU32) -> bool {
        // Interprete odd values as "niched"
        unsafe { *niched % 2 == 1 }
    }

    fn resolve_niched(out: Place<ArchivedU32>) {
        // To niche, we use the value `1`
        out.write(ArchivedU32::from_native(1))
    }
}

#[derive(Archive)]
struct Basic {
    field: Option<u32>,
}

#[derive(Archive, Serialize)]
struct Niched {
    #[rkyv(with = NicheInto<NeverOdd>)]
    field: Option<u32>,
}

# fn main() -> Result<(), rkyv::rancor::Error> {
// Indeed, we have a smaller archived representation
assert!(size_of::<ArchivedNiched>() < size_of::<ArchivedBasic>());

let values: Vec<Niched> =
    (0..4).map(|n| Niched { field: Some(n) }).collect();

let bytes = rkyv::to_bytes(&values)?;
let archived = rkyv::access::<Archived<Vec<Niched>>, _>(&bytes)?;
assert_eq!(archived[0].field.as_ref(), Some(&0.into()));
assert_eq!(archived[1].field.as_ref(), None);
assert_eq!(archived[2].field.as_ref(), Some(&2.into()));
assert_eq!(archived[3].field.as_ref(), None);
# Ok(()) }
```

[`NicheInto`]: crate::with::NicheInto

**Methods:**

- `is_niched`: Returns whether the given value has been niched.
- `resolve_niched`: Writes data to `out` indicating that a `T` is niched.



## rkyv::niche::niching::Null

*Struct*

[`Niching`] for null-pointer-niched values.

**Unit Struct**

**Trait Implementations:**

- **Niching**
  - `fn is_niched(niched: *const ArchivedBox<T>) -> bool`
  - `fn resolve_niched(out: Place<ArchivedBox<T>>)`



## rkyv::niche::niching::SharedNiching

*Trait*

Trait to allow `NichedOption<Self, N1>` to be niched further by `N2`.

# Safety

Implementors must ensure that the memory regions within `Self` that are used
for [`Niching`] impls of `N1` and `N2` are mutually exclusive.



## rkyv::niche::niching::Zero

*Struct*

[`Niching`] for zero-niched values.

**Unit Struct**

**Trait Implementations:**

- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroI64) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroI64>)`
- **Niching**
  - `fn is_niched(niched: *const NonZeroU8) -> bool`
  - `fn resolve_niched(out: Place<NonZeroU8>)`
- **Niching**
  - `fn is_niched(niched: *const NonZeroI8) -> bool`
  - `fn resolve_niched(out: Place<NonZeroI8>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroU16) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroU16>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroI32) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroI32>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroU128) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroU128>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroI16) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroI16>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroU64) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroU64>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroI128) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroI128>)`
- **Niching**
  - `fn is_niched(niched: *const ArchivedNonZeroU32) -> bool`
  - `fn resolve_niched(out: Place<ArchivedNonZeroU32>)`



