**quote > ext**

# Module: ext

## Contents

**Traits**

- [`TokenStreamExt`](#tokenstreamext) - TokenStream extension trait with methods for appending tokens.

---

## quote::ext::TokenStreamExt

*Trait*

TokenStream extension trait with methods for appending tokens.

This trait is sealed and cannot be implemented outside of the `quote` crate.

**Methods:**

- `append`: For use by `ToTokens` implementations.
- `append_all`: For use by `ToTokens` implementations.
- `append_separated`: For use by `ToTokens` implementations.
- `append_terminated`: For use by `ToTokens` implementations.



