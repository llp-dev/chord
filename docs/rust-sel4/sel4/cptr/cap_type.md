**sel4 > cptr > cap_type**

# Module: cptr::cap_type

## Contents

**Structs**

- [`AsidControl`](#asidcontrol) - Corresponds to `seL4_ASIDControl`.
- [`AsidPool`](#asidpool) - Corresponds to `seL4_ASIDPool`.
- [`CNode`](#cnode) - Corresponds to `seL4_CNode`.
- [`DomainSet`](#domainset) - Corresponds to `seL4_DomainSet`
- [`Endpoint`](#endpoint) - Corresponds to the endpoint capability type.
- [`IrqControl`](#irqcontrol) - Corresponds to `seL4_IRQControl`.
- [`IrqHandler`](#irqhandler) - Corresponds to `seL4_IRQHandler`.
- [`Notification`](#notification) - Corresponds to the notification capability type.
- [`Null`](#null) - Corresponds to the null capability.
- [`Tcb`](#tcb) - Corresponds to `seL4_TCB`.
- [`Unspecified`](#unspecified) - Any capability.
- [`UnspecifiedIntermediateTranslationTable`](#unspecifiedintermediatetranslationtable) - Any intermediate translation table capability.
- [`UnspecifiedPage`](#unspecifiedpage) - Any page capability.
- [`Untyped`](#untyped) - Corresponds to `seL4_Untyped`.

---

## sel4::cptr::cap_type::AsidControl

*Struct*

Corresponds to `seL4_ASIDControl`.

**Unit Struct**

**Traits:** Copy, Eq, CapType

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &AsidControl) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> AsidControl`
- **Ord**
  - `fn cmp(self: &Self, other: &AsidControl) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &AsidControl) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4::cptr::cap_type::AsidPool

*Struct*

Corresponds to `seL4_ASIDPool`.

**Unit Struct**

**Traits:** Copy, Eq, CapType

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &AsidPool) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> AsidPool`
- **Ord**
  - `fn cmp(self: &Self, other: &AsidPool) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &AsidPool) -> bool`



## sel4::cptr::cap_type::CNode

*Struct*

Corresponds to `seL4_CNode`.

**Unit Struct**

**Traits:** Copy, Eq, CapType

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CNode`
- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &CNode) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &CNode) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfVariableSize**
  - `fn object_blueprint(size_bits: usize) -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &CNode) -> $crate::option::Option<$crate::cmp::Ordering>`



## sel4::cptr::cap_type::DomainSet

*Struct*

Corresponds to `seL4_DomainSet`

**Unit Struct**

**Traits:** Copy, Eq, CapType

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DomainSet) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DomainSet) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> DomainSet`
- **Ord**
  - `fn cmp(self: &Self, other: &DomainSet) -> $crate::cmp::Ordering`



## sel4::cptr::cap_type::Endpoint

*Struct*

Corresponds to the endpoint capability type.

**Unit Struct**

**Traits:** Copy, IpcCapType, Eq, CapType

**Trait Implementations:**

- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &Endpoint) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Endpoint) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Endpoint) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Endpoint`



## sel4::cptr::cap_type::IrqControl

*Struct*

Corresponds to `seL4_IRQControl`.

**Unit Struct**

**Traits:** Eq, CapType, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IrqControl) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &IrqControl) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> IrqControl`
- **Ord**
  - `fn cmp(self: &Self, other: &IrqControl) -> $crate::cmp::Ordering`



## sel4::cptr::cap_type::IrqHandler

*Struct*

Corresponds to `seL4_IRQHandler`.

**Unit Struct**

**Traits:** Copy, Eq, CapType

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &IrqHandler) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &IrqHandler) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &IrqHandler) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> IrqHandler`



## sel4::cptr::cap_type::Notification

*Struct*

Corresponds to the notification capability type.

**Unit Struct**

**Traits:** Copy, IpcCapType, Eq, CapType

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Notification) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Notification`
- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &Notification) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Notification) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4::cptr::cap_type::Null

*Struct*

Corresponds to the null capability.

**Unit Struct**

**Traits:** Copy, Eq, CapType

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Null) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Null`
- **Ord**
  - `fn cmp(self: &Self, other: &Null) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Null) -> bool`



## sel4::cptr::cap_type::Tcb

*Struct*

Corresponds to `seL4_TCB`.

**Unit Struct**

**Traits:** Copy, Eq, CapType

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Tcb) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Tcb`
- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &Tcb) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Tcb) -> bool`



## sel4::cptr::cap_type::Unspecified

*Struct*

Any capability.

**Unit Struct**

**Traits:** CapType, IpcCapType, Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Unspecified) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Unspecified) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Unspecified`
- **Ord**
  - `fn cmp(self: &Self, other: &Unspecified) -> $crate::cmp::Ordering`



## sel4::cptr::cap_type::UnspecifiedIntermediateTranslationTable

*Struct*

Any intermediate translation table capability.

**Unit Struct**

**Traits:** Copy, Eq, CapType

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &UnspecifiedIntermediateTranslationTable) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &UnspecifiedIntermediateTranslationTable) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> UnspecifiedIntermediateTranslationTable`
- **Ord**
  - `fn cmp(self: &Self, other: &UnspecifiedIntermediateTranslationTable) -> $crate::cmp::Ordering`



## sel4::cptr::cap_type::UnspecifiedPage

*Struct*

Any page capability.

**Unit Struct**

**Traits:** Copy, Eq, CapType, CapTypeForFrameObject

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &UnspecifiedPage) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> UnspecifiedPage`
- **Ord**
  - `fn cmp(self: &Self, other: &UnspecifiedPage) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnspecifiedPage) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4::cptr::cap_type::Untyped

*Struct*

Corresponds to `seL4_Untyped`.

**Unit Struct**

**Traits:** Copy, Eq, CapType

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Untyped) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfVariableSize**
  - `fn object_blueprint(size_bits: usize) -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Untyped) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Untyped`
- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &Untyped) -> $crate::cmp::Ordering`



