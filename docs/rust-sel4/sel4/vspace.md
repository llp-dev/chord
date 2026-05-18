**sel4 > vspace**

# Module: vspace

## Contents

**Modules**

- [`vspace_levels`](#vspace_levels) - Items describing the layout of address translation structures for this kernel configuration.

**Traits**

- [`CapTypeForFrameObject`](#captypeforframeobject) - Trait for [`CapType`]s which correspond to frame objects.
- [`CapTypeForFrameObjectOfFixedSize`](#captypeforframeobjectoffixedsize) - Trait for [`CapType`]s which correspond to frame objects of fixed size.
- [`CapTypeForTranslationTableObject`](#captypefortranslationtableobject) - Trait for [`CapType`]s which correspond to translation table objects.

---

## sel4::vspace::CapTypeForFrameObject

*Trait*

Trait for [`CapType`]s which correspond to frame objects.



## sel4::vspace::CapTypeForFrameObjectOfFixedSize

*Trait*

Trait for [`CapType`]s which correspond to frame objects of fixed size.

**Methods:**

- `FRAME_OBJECT_TYPE`



## sel4::vspace::CapTypeForTranslationTableObject

*Trait*

Trait for [`CapType`]s which correspond to translation table objects.

**Methods:**

- `TRANSLATION_TABLE_OBJECT_TYPE`



## Module: vspace_levels

Items describing the layout of address translation structures for this kernel configuration.



