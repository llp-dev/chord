**rkyv > tuple**

# Module: tuple

## Contents

**Structs**

- [`ArchivedTuple1`](#archivedtuple1) - An archived tuple with 1 elements
- [`ArchivedTuple10`](#archivedtuple10) - An archived tuple with 10 elements
- [`ArchivedTuple11`](#archivedtuple11) - An archived tuple with 11 elements
- [`ArchivedTuple12`](#archivedtuple12) - An archived tuple with 12 elements
- [`ArchivedTuple13`](#archivedtuple13) - An archived tuple with 13 elements
- [`ArchivedTuple2`](#archivedtuple2) - An archived tuple with 2 elements
- [`ArchivedTuple3`](#archivedtuple3) - An archived tuple with 3 elements
- [`ArchivedTuple4`](#archivedtuple4) - An archived tuple with 4 elements
- [`ArchivedTuple5`](#archivedtuple5) - An archived tuple with 5 elements
- [`ArchivedTuple6`](#archivedtuple6) - An archived tuple with 6 elements
- [`ArchivedTuple7`](#archivedtuple7) - An archived tuple with 7 elements
- [`ArchivedTuple8`](#archivedtuple8) - An archived tuple with 8 elements
- [`ArchivedTuple9`](#archivedtuple9) - An archived tuple with 9 elements

---

## rkyv::tuple::ArchivedTuple1

*Struct*

An archived tuple with 1 elements

**Generic Parameters:**
- T0

**Tuple Struct**: `(T0)`

**Traits:** Portable, Eq

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0), <D as >::Error>`
- **Default**
  - `fn default() -> ArchivedTuple1<T0>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0)) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple1<T0>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple1<T0>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple1<T0>) -> bool`



## rkyv::tuple::ArchivedTuple10

*Struct*

An archived tuple with 10 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3
- T4
- T5
- T6
- T7
- T8
- T9

**Tuple Struct**: `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)`

**Traits:** Eq, Portable

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), <D as >::Error>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>) -> bool`
- **Default**
  - `fn default() -> ArchivedTuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)) -> bool`



## rkyv::tuple::ArchivedTuple11

*Struct*

An archived tuple with 11 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3
- T4
- T5
- T6
- T7
- T8
- T9
- T10

**Tuple Struct**: `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)`

**Traits:** Eq, Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>) -> bool`
- **Default**
  - `fn default() -> ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), <D as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>) -> $crate::option::Option<$crate::cmp::Ordering>`



## rkyv::tuple::ArchivedTuple12

*Struct*

An archived tuple with 12 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3
- T4
- T5
- T6
- T7
- T8
- T9
- T10
- T11

**Tuple Struct**: `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)`

**Traits:** Portable, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), <D as >::Error>`
- **Default**
  - `fn default() -> ArchivedTuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rkyv::tuple::ArchivedTuple13

*Struct*

An archived tuple with 13 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3
- T4
- T5
- T6
- T7
- T8
- T9
- T10
- T11
- T12

**Tuple Struct**: `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)`

**Traits:** Portable, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12)) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), <D as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>) -> bool`



## rkyv::tuple::ArchivedTuple2

*Struct*

An archived tuple with 2 elements

**Generic Parameters:**
- T0
- T1

**Tuple Struct**: `(T0, T1)`

**Traits:** Eq, Portable

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple2<T0, T1>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple2<T0, T1>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple2<T0, T1>) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1), <D as >::Error>`
- **Default**
  - `fn default() -> ArchivedTuple2<T0, T1>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1)) -> bool`



## rkyv::tuple::ArchivedTuple3

*Struct*

An archived tuple with 3 elements

**Generic Parameters:**
- T0
- T1
- T2

**Tuple Struct**: `(T0, T1, T2)`

**Traits:** Eq, Portable

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple3<T0, T1, T2>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2), <D as >::Error>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple3<T0, T1, T2>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple3<T0, T1, T2>) -> bool`
- **Default**
  - `fn default() -> ArchivedTuple3<T0, T1, T2>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2)) -> bool`



## rkyv::tuple::ArchivedTuple4

*Struct*

An archived tuple with 4 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3

**Tuple Struct**: `(T0, T1, T2, T3)`

**Traits:** Portable, Eq

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3), <D as >::Error>`
- **Default**
  - `fn default() -> ArchivedTuple4<T0, T1, T2, T3>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3)) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple4<T0, T1, T2, T3>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple4<T0, T1, T2, T3>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple4<T0, T1, T2, T3>) -> bool`



## rkyv::tuple::ArchivedTuple5

*Struct*

An archived tuple with 5 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3
- T4

**Tuple Struct**: `(T0, T1, T2, T3, T4)`

**Traits:** Eq, Portable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3, U4)) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple5<T0, T1, T2, T3, T4>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple5<T0, T1, T2, T3, T4>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple5<T0, T1, T2, T3, T4>) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3, T4), <D as >::Error>`
- **Default**
  - `fn default() -> ArchivedTuple5<T0, T1, T2, T3, T4>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## rkyv::tuple::ArchivedTuple6

*Struct*

An archived tuple with 6 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3
- T4
- T5

**Tuple Struct**: `(T0, T1, T2, T3, T4, T5)`

**Traits:** Eq, Portable

**Trait Implementations:**

- **Default**
  - `fn default() -> ArchivedTuple6<T0, T1, T2, T3, T4, T5>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5)) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple6<T0, T1, T2, T3, T4, T5>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3, T4, T5), <D as >::Error>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple6<T0, T1, T2, T3, T4, T5>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple6<T0, T1, T2, T3, T4, T5>) -> bool`



## rkyv::tuple::ArchivedTuple7

*Struct*

An archived tuple with 7 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3
- T4
- T5
- T6

**Tuple Struct**: `(T0, T1, T2, T3, T4, T5, T6)`

**Traits:** Portable, Eq

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6>) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6), <D as >::Error>`
- **Default**
  - `fn default() -> ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6)) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple7<T0, T1, T2, T3, T4, T5, T6>) -> $crate::option::Option<$crate::cmp::Ordering>`



## rkyv::tuple::ArchivedTuple8

*Struct*

An archived tuple with 8 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3
- T4
- T5
- T6
- T7

**Tuple Struct**: `(T0, T1, T2, T3, T4, T5, T6, T7)`

**Traits:** Eq, Portable

**Trait Implementations:**

- **Default**
  - `fn default() -> ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7)) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7), <D as >::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple8<T0, T1, T2, T3, T4, T5, T6, T7>) -> bool`



## rkyv::tuple::ArchivedTuple9

*Struct*

An archived tuple with 9 elements

**Generic Parameters:**
- T0
- T1
- T2
- T3
- T4
- T5
- T6
- T7
- T8

**Tuple Struct**: `(T0, T1, T2, T3, T4, T5, T6, T7, T8)`

**Traits:** Portable, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8>) -> bool`
- **Default**
  - `fn default() -> ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &(U0, U1, U2, U3, U4, U5, U6, U7, U8)) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), <D as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedTuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8>) -> $crate::cmp::Ordering`



