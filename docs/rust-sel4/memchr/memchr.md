**memchr**

# Module: memchr

## Contents

**Modules**

- [`arch`](#arch) - A module with low-level architecture dependent routines.
- [`memmem`](#memmem) - This module provides forward and reverse substring search routines.

---

## Module: arch

A module with low-level architecture dependent routines.

These routines are useful as primitives for tasks not covered by the higher
level crate API.



## Module: memmem

This module provides forward and reverse substring search routines.

Unlike the standard library's substring search routines, these work on
arbitrary bytes. For all non-empty needles, these routines will report exactly
the same values as the corresponding routines in the standard library. For
the empty needle, the standard library reports matches only at valid UTF-8
boundaries, where as these routines will report matches at every position.

Other than being able to work on arbitrary bytes, the primary reason to prefer
these routines over the standard library routines is that these will generally
be faster. In some cases, significantly so.

# Example: iterating over substring matches

This example shows how to use [`find_iter`] to find occurrences of a substring
in a haystack.

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::find_iter(haystack, "foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

# Example: iterating over substring matches in reverse

This example shows how to use [`rfind_iter`] to find occurrences of a substring
in a haystack starting from the end of the haystack.

**NOTE:** This module does not implement double ended iterators, so reverse
searches aren't done by calling `rev` on a forward iterator.

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::rfind_iter(haystack, "foo");
assert_eq!(Some(16), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(0), it.next());
assert_eq!(None, it.next());
```

# Example: repeating a search for the same needle

It may be possible for the overhead of constructing a substring searcher to be
measurable in some workloads. In cases where the same needle is used to search
many haystacks, it is possible to do construction once and thus to avoid it for
subsequent searches. This can be done with a [`Finder`] (or a [`FinderRev`] for
reverse searches).

```
use memchr::memmem;

let finder = memmem::Finder::new("foo");

assert_eq!(Some(4), finder.find(b"baz foo quux"));
assert_eq!(None, finder.find(b"quux baz bar"));
```



