**ttf_parser > tables > cff > charstring**

# Module: tables::cff::charstring

## Contents

**Structs**

- [`CharStringParser`](#charstringparser)

---

## ttf_parser::tables::cff::charstring::CharStringParser

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `stack: super::argstack::ArgumentsStack<'a>`
- `builder: &'a  mut super::Builder<'a>`
- `x: f32`
- `y: f32`
- `has_move_to: bool`
- `is_first_move_to: bool`
- `width_only: bool`

**Methods:**

- `fn parse_move_to(self: & mut Self, offset: usize) -> Result<(), CFFError>`
- `fn parse_horizontal_move_to(self: & mut Self, offset: usize) -> Result<(), CFFError>`
- `fn parse_vertical_move_to(self: & mut Self, offset: usize) -> Result<(), CFFError>`
- `fn parse_line_to(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_horizontal_line_to(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_vertical_line_to(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_curve_to(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_curve_line(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_line_curve(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_hh_curve_to(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_vv_curve_to(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_hv_curve_to(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_vh_curve_to(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_flex(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_flex1(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_hflex(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_hflex1(self: & mut Self) -> Result<(), CFFError>`
- `fn parse_int1(self: & mut Self, op: u8) -> Result<(), CFFError>`
- `fn parse_int2(self: & mut Self, op: u8, s: & mut Stream) -> Result<(), CFFError>`
- `fn parse_int3(self: & mut Self, op: u8, s: & mut Stream) -> Result<(), CFFError>`
- `fn parse_fixed(self: & mut Self, s: & mut Stream) -> Result<(), CFFError>`



