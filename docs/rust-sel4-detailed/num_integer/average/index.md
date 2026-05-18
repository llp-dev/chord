*[num_integer](../index.md) / [average](index.md)*

---

# Module `average`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Average`](#average) | trait | Provides methods to compute the average of two integers, without overflows. |
| [`average_floor`](#average-floor) | fn | Returns the floor value of the average of `x` and `y` -- see [Average::average_floor](trait.Average.html#tymethod.average_floor). |
| [`average_ceil`](#average-ceil) | fn | Returns the ceiling value of the average of `x` and `y` -- see [Average::average_ceil](trait.Average.html#tymethod.average_ceil). |

## Traits

### `Average`

```rust
trait Average: Integer { ... }
```

Provides methods to compute the average of two integers, without overflows.

#### Required Methods

- `fn average_ceil(&self, other: &Self) -> Self`

  Returns the ceiling value of the average of `self` and `other`.

- `fn average_floor(&self, other: &Self) -> Self`

  Returns the floor value of the average of `self` and `other`.

#### Implementors

- `I`

## Functions

### `average_floor`

```rust
fn average_floor<T: Average>(x: T, y: T) -> T
```

Returns the floor value of the average of `x` and `y` --
see [Average::average_floor](#average-average-floor).

### `average_ceil`

```rust
fn average_ceil<T: Average>(x: T, y: T) -> T
```

Returns the ceiling value of the average of `x` and `y` --
see [Average::average_ceil](#average-average-ceil).

