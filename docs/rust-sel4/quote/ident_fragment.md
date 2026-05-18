**quote > ident_fragment**

# Module: ident_fragment

## Contents

**Traits**

- [`IdentFragment`](#identfragment) - Specialized formatting trait used by `format_ident!`.

---

## quote::ident_fragment::IdentFragment

*Trait*

Specialized formatting trait used by `format_ident!`.

[`Ident`] arguments formatted using this trait will have their `r#` prefix
stripped, if present.

See [`format_ident!`] for more information.

[`format_ident!`]: crate::format_ident

**Methods:**

- `fmt`: Format this value as an identifier fragment.
- `span`: Span associated with this `IdentFragment`.



