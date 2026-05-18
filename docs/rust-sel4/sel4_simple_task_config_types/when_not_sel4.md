**sel4_simple_task_config_types > when_not_sel4**

# Module: when_not_sel4

## Contents

**Structs**

- [`AsidControl`](#asidcontrol)
- [`AsidPool`](#asidpool)
- [`CNode`](#cnode)
- [`CPtr`](#cptr)
- [`Endpoint`](#endpoint)
- [`HugePage`](#hugepage)
- [`IrqControl`](#irqcontrol)
- [`IrqHandler`](#irqhandler)
- [`LargePage`](#largepage)
- [`Notification`](#notification)
- [`Null`](#null)
- [`PT`](#pt)
- [`SmallPage`](#smallpage)
- [`StaticThread`](#staticthread)
- [`Tcb`](#tcb)
- [`Unspecified`](#unspecified)
- [`Untyped`](#untyped)
- [`VCpu`](#vcpu)
- [`VSpace`](#vspace)

**Type Aliases**

- [`RawConfigWord`](#rawconfigword)

---

## sel4_simple_task_config_types::when_not_sel4::AsidControl

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &AsidControl) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AsidControl`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &AsidControl) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &AsidControl) -> $crate::option::Option<$crate::cmp::Ordering>`



## sel4_simple_task_config_types::when_not_sel4::AsidPool

*Struct*

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &AsidPool) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AsidPool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &AsidPool) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &AsidPool) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::CNode

*Struct*

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CNode) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CNode`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &CNode) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &CNode) -> $crate::option::Option<$crate::cmp::Ordering>`



## sel4_simple_task_config_types::when_not_sel4::CPtr

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CPtr`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &CPtr) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &CPtr) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &CPtr) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::Endpoint

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Endpoint`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Endpoint) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Endpoint) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Endpoint) -> bool`



## sel4_simple_task_config_types::when_not_sel4::HugePage

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &HugePage) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> HugePage`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &HugePage) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &HugePage) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::IrqControl

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &IrqControl) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &IrqControl) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IrqControl) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> IrqControl`



## sel4_simple_task_config_types::when_not_sel4::IrqHandler

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &IrqHandler) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &IrqHandler) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IrqHandler) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> IrqHandler`



## sel4_simple_task_config_types::when_not_sel4::LargePage

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &LargePage) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> LargePage`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &LargePage) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &LargePage) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::Notification

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Notification) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Notification) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Notification) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Notification`



## sel4_simple_task_config_types::when_not_sel4::Null

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Null) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Null`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Null) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Null) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::PT

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &PT) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> PT`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &PT) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PT) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::RawConfigWord

*Type Alias*: `u64`



## sel4_simple_task_config_types::when_not_sel4::SmallPage

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SmallPage) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SmallPage`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &SmallPage) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SmallPage) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::StaticThread

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &StaticThread) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> StaticThread`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &StaticThread) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &StaticThread) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::Tcb

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Tcb) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Tcb) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Tcb) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Tcb`



## sel4_simple_task_config_types::when_not_sel4::Unspecified

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Unspecified) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Unspecified`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Unspecified) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Unspecified) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::Untyped

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Untyped) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Untyped`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Untyped) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Untyped) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_config_types::when_not_sel4::VCpu

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &VCpu) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VCpu) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> VCpu`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &VCpu) -> $crate::cmp::Ordering`



## sel4_simple_task_config_types::when_not_sel4::VSpace

*Struct*

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VSpace`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &VSpace) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &VSpace) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VSpace) -> bool`



