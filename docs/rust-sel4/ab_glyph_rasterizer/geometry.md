**ab_glyph_rasterizer > geometry**

# Module: geometry

## Contents

**Structs**

- [`Point`](#point) - An (x, y) coordinate.

**Functions**

- [`point`](#point) - [`Point`] constructor.

---

## ab_glyph_rasterizer::geometry::Point

*Struct*

An (x, y) coordinate.

# Example
```
use ab_glyph_rasterizer::{point, Point};
let p: Point = point(0.1, 23.2);
```

**Fields:**
- `x: f32`
- `y: f32`

**Traits:** Copy

**Trait Implementations:**

- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - ```
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Point) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from([x, y]: [F; 2]) -> Self` - ```
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Point`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: Self)` - ```
- **From**
  - `fn from((x, y): (F, F)) -> Self` - ```
- **Sub**
  - `fn sub(self: Self, rhs: Point) -> Point` - Subtract rhs.x from x, rhs.y from y.
- **Default**
  - `fn default() -> Point`
- **PartialEq**
  - `fn eq(self: &Self, other: &Point) -> bool`
- **Add**
  - `fn add(self: Self, rhs: Point) -> Point` - Add rhs.x to x, rhs.y to y.



## ab_glyph_rasterizer::geometry::point

*Function*

[`Point`] constructor.

# Example
```
# use ab_glyph_rasterizer::{point, Point};
let p = point(0.1, 23.2);
```

```rust
fn point(x: f32, y: f32) -> Point
```



