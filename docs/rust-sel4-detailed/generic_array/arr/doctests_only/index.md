*[generic_array](../../index.md) / [arr](../index.md) / [doctests_only](index.md)*

---

# Module `doctests_only`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DocTests`](#doctests) | enum | # With ellision |

## Enums

### `DocTests`

```rust
enum DocTests {
}
```


# With ellision

Testing that lifetimes aren't transmuted when they're ellided.

```compile_fail
#[macro_use] extern crate generic_array;
fn main() {
   fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
       arr![&A; a][0]
   }
}
```

```rust
#[macro_use] extern crate generic_array;
fn main() {
   fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'a A {
       arr![&A; a][0]
   }
}
```

# Without ellision

Testing that lifetimes aren't transmuted when they're specified explicitly.

```compile_fail
#[macro_use] extern crate generic_array;
fn main() {
   fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
       arr![&'a A; a][0]
   }
}
```

```compile_fail
#[macro_use] extern crate generic_array;
fn main() {
   fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
       arr![&'static A; a][0]
   }
}
```

```rust
#[macro_use] extern crate generic_array;
fn main() {
   fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'a A {
       arr![&'a A; a][0]
   }
}
```

#### Trait Implementations

##### `impl Same for DocTests`

- <span id="doctests-same-type-output"></span>`type Output = T`

