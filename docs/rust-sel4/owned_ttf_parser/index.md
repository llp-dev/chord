# owned_ttf_parser

Extends [ttf_parser](https://docs.rs/ttf-parser) with owned version of
[`Face`](struct.Face.html): [`OwnedFace`](struct.OwnedFace.html).

Re-exports `ttf_parser::*`.

# Example
```
use owned_ttf_parser::{AsFaceRef, Face, OwnedFace};

# let owned_font_data = include_bytes!("../fonts/font.ttf").to_vec();
let owned_face = OwnedFace::from_vec(owned_font_data, 0).unwrap();
let face_ref: &Face<'_> = owned_face.as_face_ref();

assert_eq!(face_ref.ascender(), 2254);
```

## Modules

### [`convert`](convert.md)

*2 traits*

### [`owned`](owned.md)

*1 struct*

### [`preparse`](preparse.md)

*1 struct*

