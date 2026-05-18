# fallible_iterator

"Fallible" iterators.

The iterator APIs in the Rust standard library do not support iteration
that can fail in a first class manner. These iterators are typically modeled
as iterating over `Result<T, E>` values; for example, the `Lines` iterator
returns `io::Result<String>`s. When simply iterating over these types, the
value being iterated over must be unwrapped in some way before it can be
used:

```ignore
for line in reader.lines() {
    let line = line?;
    // work with line
}
```

In addition, many of the additional methods on the `Iterator` trait will
not behave properly in the presence of errors when working with these kinds
of iterators. For example, if one wanted to count the number of lines of
text in a `Read`er, this might be a way to go about it:

```ignore
let count = reader.lines().count();
```

This will return the proper value when the reader operates successfully, but
if it encounters an IO error, the result will either be slightly higher than
expected if the error is transient, or it may run forever if the error is
returned repeatedly!

In contrast, a fallible iterator is built around the concept that a call to
`next` can fail. The trait has an additional `Error` associated type in
addition to the `Item` type, and `next` returns `Result<Option<Self::Item>,
Self::Error>` rather than `Option<Self::Item>`. Methods like `count` return
`Result`s as well.

This does mean that fallible iterators are incompatible with Rust's `for`
loop syntax, but `while let` loops offer a similar level of ergonomics:

```ignore
while let Some(item) = iter.next()? {
    // work with item
}
```

## Fallible closure arguments

Like `Iterator`, many `FallibleIterator` methods take closures as arguments.
These use the same signatures as their `Iterator` counterparts, except that
`FallibleIterator` expects the closures to be fallible: they return
`Result<T, Self::Error>` instead of simply `T`.

For example, the standard library's `Iterator::filter` adapter method
filters the underlying iterator according to a predicate provided by the
user, whose return type is `bool`. In `FallibleIterator::filter`, however,
the predicate returns `Result<bool, Self::Error>`:

```
# use std::error::Error;
# use std::str::FromStr;
# use fallible_iterator::{convert, FallibleIterator};
let numbers = convert("100\n200\nfern\n400".lines().map(Ok::<&str, Box<Error>>));
let big_numbers = numbers.filter(|n| Ok(u64::from_str(n)? > 100));
assert!(big_numbers.count().is_err());
```

## Modules

### [`fallible_iterator`](fallible_iterator.md)

*31 structs, 4 traits, 7 functions*

