**ttf_parser > tables**

# Module: tables

## Contents

**Modules**

- [`cbdt`](#cbdt) - A [Color Bitmap Data Table](
- [`cblc`](#cblc) - A [Color Bitmap Location Table](
- [`cff`](#cff)
- [`cmap`](#cmap) - A [Character to Glyph Index Mapping Table](
- [`colr`](#colr) - A [Color Table](
- [`cpal`](#cpal) - A [Color Palette Table](
- [`glyf`](#glyf) - A [Glyph Data Table](
- [`head`](#head) - A [Font Header Table](
- [`hhea`](#hhea) - A [Horizontal Header Table](
- [`hmtx`](#hmtx) - A [Horizontal/Vertical Metrics Table](
- [`kern`](#kern) - A [Kerning Table](
- [`loca`](#loca) - An [Index to Location Table](https://docs.microsoft.com/en-us/typography/opentype/spec/loca)
- [`maxp`](#maxp) - A [Maximum Profile Table](
- [`name`](#name) - A [Naming Table](
- [`os2`](#os2) - A [OS/2 and Windows Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/os2)
- [`post`](#post) - A [PostScript Table](
- [`sbix`](#sbix) - A [Standard Bitmap Graphics Table](
- [`stat`](#stat) - A [Style Attributes Table](https://docs.microsoft.com/en-us/typography/opentype/spec/stat) implementation.
- [`svg`](#svg) - An [SVG Table](https://docs.microsoft.com/en-us/typography/opentype/spec/svg) implementation.
- [`vhea`](#vhea) - A [Vertical Header Table](
- [`vorg`](#vorg) - A [Vertical Origin Table](

---

## Module: cbdt

A [Color Bitmap Data Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cbdt) implementation.



## Module: cblc

A [Color Bitmap Location Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cblc) implementation.



## Module: cff



## Module: cmap

A [Character to Glyph Index Mapping Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cmap) implementation.

This module provides a low-level alternative to
[`Face::glyph_index`](../struct.Face.html#method.glyph_index) and
[`Face::glyph_variation_index`](../struct.Face.html#method.glyph_variation_index)
methods.



## Module: colr

A [Color Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/colr) implementation.



## Module: cpal

A [Color Palette Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cpal) implementation.



## Module: glyf

A [Glyph Data Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/glyf) implementation.



## Module: head

A [Font Header Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/head) implementation.



## Module: hhea

A [Horizontal Header Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/hhea) implementation.



## Module: hmtx

A [Horizontal/Vertical Metrics Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx) implementation.



## Module: kern

A [Kerning Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/kern) implementation.

Supports both
[OpenType](https://docs.microsoft.com/en-us/typography/opentype/spec/kern)
and
[Apple Advanced Typography](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6kern.html)
variants.

Since there is no single correct way to process a kerning data,
we have to provide an access to kerning subtables, so a caller can implement
a kerning algorithm manually.
But we still try to keep the API as high-level as possible.



## Module: loca

An [Index to Location Table](https://docs.microsoft.com/en-us/typography/opentype/spec/loca)
implementation.



## Module: maxp

A [Maximum Profile Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/maxp) implementation.



## Module: name

A [Naming Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/name) implementation.



## Module: os2

A [OS/2 and Windows Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/os2)
implementation.



## Module: post

A [PostScript Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/post) implementation.



## Module: sbix

A [Standard Bitmap Graphics Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/sbix) implementation.



## Module: stat

A [Style Attributes Table](https://docs.microsoft.com/en-us/typography/opentype/spec/stat) implementation.



## Module: svg

An [SVG Table](https://docs.microsoft.com/en-us/typography/opentype/spec/svg) implementation.



## Module: vhea

A [Vertical Header Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/vhea) implementation.



## Module: vorg

A [Vertical Origin Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/vorg) implementation.



