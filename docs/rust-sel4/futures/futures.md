**futures**

# Module: futures

## Contents

**Modules**

- [`prelude`](#prelude) - A "prelude" for crates using the `futures` crate.

---

## Module: prelude

A "prelude" for crates using the `futures` crate.

This prelude is similar to the standard library's prelude in that you'll
almost always want to import its entire contents, but unlike the
standard library's prelude you'll have to do so manually:

```
# #[allow(unused_imports)]
use futures::prelude::*;
```

The prelude may grow over time as additional items see ubiquitous use.



