**owned_ttf_parser > owned**

# Module: owned

## Contents

**Structs**

- [`OwnedFace`](#ownedface) - An owned version of font [`Face`](struct.Face.html).

---

## owned_ttf_parser::owned::OwnedFace

*Struct*

An owned version of font [`Face`](struct.Face.html).

**Tuple Struct**: `()`

**Methods:**

- `fn from_vec(data: Vec<u8>, index: u32) -> Result<Self, ttf_parser::FaceParsingError>` - Creates an `OwnedFace` from owned data.
- `fn as_slice(self: &Self) -> &[u8]` - Extracts a slice containing the data passed into [`OwnedFace::from_vec`].
- `fn into_vec(self: Self) -> Vec<u8>` - Unwraps the data passed into [`OwnedFace::from_vec`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsFaceRef**
  - `fn as_face_ref(self: &Self) -> &ttf_parser::Face`
- **FaceMut**
  - `fn set_variation(self: & mut Self, axis: ttf_parser::Tag, value: f32) -> Option<()>`



