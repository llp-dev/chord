**sel4_capdl_initializer_types > spec**

# Module: spec

## Contents

**Modules**

- [`cap`](#cap)
- [`object`](#object)

**Structs**

- [`ArchivedCapSlot`](#archivedcapslot) - An archived [`CapSlot`]
- [`ArchivedCapTableEntry`](#archivedcaptableentry) - An archived [`CapTableEntry`]
- [`ArchivedDomainSchedEntry`](#archiveddomainschedentry) - An archived [`DomainSchedEntry`]
- [`ArchivedIrqEntry`](#archivedirqentry) - An archived [`IrqEntry`]
- [`ArchivedNamedObject`](#archivednamedobject) - An archived [`NamedObject`]
- [`ArchivedObjectId`](#archivedobjectid) - An archived [`ObjectId`]
- [`ArchivedOrigCapSlots`](#archivedorigcapslots) - An archived [`OrigCapSlots`]
- [`ArchivedRights`](#archivedrights) - An archived [`Rights`]
- [`ArchivedSpec`](#archivedspec) - An archived [`Spec`]
- [`ArchivedUntypedCover`](#archiveduntypedcover) - An archived [`UntypedCover`]
- [`ArchivedWord`](#archivedword) - An archived [`Word`]
- [`CapSlot`](#capslot)
- [`CapSlotResolver`](#capslotresolver) - The resolver for an archived [`CapSlot`]
- [`CapTableEntry`](#captableentry)
- [`CapTableEntryResolver`](#captableentryresolver) - The resolver for an archived [`CapTableEntry`]
- [`DomainSchedEntry`](#domainschedentry)
- [`DomainSchedEntryResolver`](#domainschedentryresolver) - The resolver for an archived [`DomainSchedEntry`]
- [`IrqEntry`](#irqentry)
- [`IrqEntryResolver`](#irqentryresolver) - The resolver for an archived [`IrqEntry`]
- [`NamedObject`](#namedobject)
- [`NamedObjectResolver`](#namedobjectresolver) - The resolver for an archived [`NamedObject`]
- [`ObjectId`](#objectid)
- [`ObjectIdResolver`](#objectidresolver) - The resolver for an archived [`ObjectId`]
- [`OrigCapSlots`](#origcapslots)
- [`OrigCapSlotsResolver`](#origcapslotsresolver) - The resolver for an archived [`OrigCapSlots`]
- [`Rights`](#rights)
- [`RightsResolver`](#rightsresolver) - The resolver for an archived [`Rights`]
- [`Spec`](#spec)
- [`SpecResolver`](#specresolver) - The resolver for an archived [`Spec`]
- [`UntypedCover`](#untypedcover)
- [`UntypedCoverResolver`](#untypedcoverresolver) - The resolver for an archived [`UntypedCover`]
- [`Word`](#word)
- [`WordResolver`](#wordresolver) - The resolver for an archived [`Word`]

**Enums**

- [`ArchivedCap`](#archivedcap) - An archived [`Cap`]
- [`ArchivedObject`](#archivedobject) - An archived [`Object`]
- [`Cap`](#cap)
- [`CapResolver`](#capresolver) - The resolver for an archived [`Cap`]
- [`Object`](#object)
- [`ObjectResolver`](#objectresolver) - The resolver for an archived [`Object`]
- [`PageTableEntry`](#pagetableentry)

**Traits**

- [`IsArchivedCap`](#isarchivedcap)
- [`IsArchivedObject`](#isarchivedobject)
- [`IsCap`](#iscap)
- [`IsObject`](#isobject)

**Type Aliases**

- [`AsidSlotEntry`](#asidslotentry)

---

## sel4_capdl_initializer_types::spec::ArchivedCap

*Enum*

An archived [`Cap`]

**Variants:**
- `Untyped(<cap::Untyped as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::Untyped`]
- `Endpoint(<cap::Endpoint as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::Endpoint`]
- `Notification(<cap::Notification as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::Notification`]
- `CNode(<cap::CNode as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::CNode`]
- `Tcb(<cap::Tcb as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::Tcb`]
- `IrqHandler(<cap::IrqHandler as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::IrqHandler`]
- `VCpu(<cap::VCpu as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::VCpu`]
- `Frame(<cap::Frame as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::Frame`]
- `PageTable(<cap::PageTable as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::PageTable`]
- `AsidPool(<cap::AsidPool as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::AsidPool`]
- `ArmIrqHandler(<cap::ArmIrqHandler as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::ArmIrqHandler`]
- `IrqMsiHandler(<cap::IrqMsiHandler as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::IrqMsiHandler`]
- `IrqIOApicHandler(<cap::IrqIOApicHandler as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::IrqIOApicHandler`]
- `RiscvIrqHandler(<cap::RiscvIrqHandler as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::RiscvIrqHandler`]
- `IOPorts(<cap::IOPorts as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::IOPorts`]
- `SchedContext(<cap::SchedContext as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::SchedContext`]
- `Reply(<cap::Reply as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::Reply`]
- `ArmSmc(<cap::ArmSmc as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::ArmSmc`]
- `DomainSet(<cap::DomainSet as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Cap::DomainSet`]

**Methods:**

- `fn as_<T>(self: &Self) -> Option<&T>`
- `fn obj(self: &Self) -> ArchivedObjectId`

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedCapSlot

*Struct*

An archived [`CapSlot`]

**Tuple Struct**: `(<u32 as ::rkyv::Archive>::Archived)`

**Traits:** Eq, Copy, Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedCapSlot) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedCapSlot`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(x: usize) -> ArchivedCapSlot`



## sel4_capdl_initializer_types::spec::ArchivedCapTableEntry

*Struct*

An archived [`CapTableEntry`]

**Fields:**
- `slot: <CapSlot as ::rkyv::Archive>::Archived` - The archived counterpart of [`CapTableEntry::slot`]
- `cap: <Cap as ::rkyv::Archive>::Archived` - The archived counterpart of [`CapTableEntry::cap`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedDomainSchedEntry

*Struct*

An archived [`DomainSchedEntry`]

**Fields:**
- `id: <u8 as ::rkyv::Archive>::Archived` - The archived counterpart of [`DomainSchedEntry::id`]
- `time: <u64 as ::rkyv::Archive>::Archived` - The archived counterpart of [`DomainSchedEntry::time`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedIrqEntry

*Struct*

An archived [`IrqEntry`]

**Fields:**
- `irq: <Word as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqEntry::irq`]
- `handler: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`IrqEntry::handler`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedNamedObject

*Struct*

An archived [`NamedObject`]

**Generic Parameters:**
- D

**Fields:**
- `name: <Option<alloc::string::String> as ::rkyv::Archive>::Archived` - The archived counterpart of [`NamedObject::name`]
- `object: <Object<D> as ::rkyv::Archive>::Archived` - The archived counterpart of [`NamedObject::object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedObject

*Enum*

An archived [`Object`]

**Generic Parameters:**
- D

**Variants:**
- `Untyped(<object::Untyped as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::Untyped`]
- `Endpoint` - The archived counterpart of [`Object::Endpoint`]
- `Notification` - The archived counterpart of [`Object::Notification`]
- `CNode(<object::CNode as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::CNode`]
- `Tcb(<object::Tcb as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::Tcb`]
- `Irq(<object::Irq as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::Irq`]
- `VCpu` - The archived counterpart of [`Object::VCpu`]
- `Frame(<object::Frame<D> as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::Frame`]
- `PageTable(<object::PageTable as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::PageTable`]
- `AsidPool(<object::AsidPool as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::AsidPool`]
- `ArmIrq(<object::ArmIrq as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::ArmIrq`]
- `IrqMsi(<object::IrqMsi as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::IrqMsi`]
- `IrqIOApic(<object::IrqIOApic as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::IrqIOApic`]
- `RiscvIrq(<object::RiscvIrq as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::RiscvIrq`]
- `IOPorts(<object::IOPorts as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::IOPorts`]
- `SchedContext(<object::SchedContext as ::rkyv::Archive>::Archived)` - The archived counterpart of [`Object::SchedContext`]
- `Reply` - The archived counterpart of [`Object::Reply`]
- `ArmSmc` - The archived counterpart of [`Object::ArmSmc`]
- `DomainSet` - The archived counterpart of [`Object::DomainSet`]

**Methods:**

- `fn as_<T>(self: &Self) -> Option<&T>`
- `fn paddr(self: &Self) -> ArchivedOption<ArchivedWord>`

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedObjectId

*Struct*

An archived [`ObjectId`]

**Tuple Struct**: `(<u32 as ::rkyv::Archive>::Archived)`

**Methods:**

- `fn into_usize_range(range: &Range<ArchivedObjectId>) -> Range<usize>`

**Traits:** Eq, Copy, Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedObjectId) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedObjectId`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(x: usize) -> ArchivedObjectId`



## sel4_capdl_initializer_types::spec::ArchivedOrigCapSlots

*Struct*

An archived [`OrigCapSlots`]

**Fields:**
- `num_occupied: <u32 as ::rkyv::Archive>::Archived` - The archived counterpart of [`OrigCapSlots::num_occupied`]
- `offsets_by_object: <alloc::vec::Vec<Option<u32>> as ::rkyv::Archive>::Archived` - The archived counterpart of [`OrigCapSlots::offsets_by_object`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedRights

*Struct*

An archived [`Rights`]

**Fields:**
- `read: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`Rights::read`]
- `write: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`Rights::write`]
- `grant: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`Rights::grant`]
- `grant_reply: <bool as ::rkyv::Archive>::Archived` - The archived counterpart of [`Rights::grant_reply`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedSpec

*Struct*

An archived [`Spec`]

**Generic Parameters:**
- D

**Fields:**
- `objects: <alloc::vec::Vec<NamedObject<D>> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::objects`]
- `irqs: <alloc::vec::Vec<IrqEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::irqs`]
- `domain_schedule: <Option<alloc::vec::Vec<DomainSchedEntry>> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::domain_schedule`]
- `domain_set_start: <Option<Word> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::domain_set_start`]
- `domain_idx_shift: <Option<Word> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::domain_idx_shift`]
- `asid_slots: <alloc::vec::Vec<AsidSlotEntry> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::asid_slots`]
- `root_objects: <core::ops::Range<ObjectId> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::root_objects`]
- `untyped_covers: <alloc::vec::Vec<UntypedCover> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::untyped_covers`]
- `cached_orig_cap_slots: <Option<OrigCapSlots> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::cached_orig_cap_slots`]
- `log_level: <Option<u8> as ::rkyv::Archive>::Archived` - The archived counterpart of [`Spec::log_level`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedUntypedCover

*Struct*

An archived [`UntypedCover`]

**Fields:**
- `parent: <ObjectId as ::rkyv::Archive>::Archived` - The archived counterpart of [`UntypedCover::parent`]
- `children: <core::ops::Range<ObjectId> as ::rkyv::Archive>::Archived` - The archived counterpart of [`UntypedCover::children`]

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::ArchivedWord

*Struct*

An archived [`Word`]

**Tuple Struct**: `(<u64 as ::rkyv::Archive>::Archived)`

**Traits:** Eq, Copy, Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::rkyv::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedWord) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedWord`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_capdl_initializer_types::spec::AsidSlotEntry

*Type Alias*: `ObjectId`



## sel4_capdl_initializer_types::spec::Cap

*Enum*

**Variants:**
- `Untyped(cap::Untyped)`
- `Endpoint(cap::Endpoint)`
- `Notification(cap::Notification)`
- `CNode(cap::CNode)`
- `Tcb(cap::Tcb)`
- `IrqHandler(cap::IrqHandler)`
- `VCpu(cap::VCpu)`
- `Frame(cap::Frame)`
- `PageTable(cap::PageTable)`
- `AsidPool(cap::AsidPool)`
- `ArmIrqHandler(cap::ArmIrqHandler)`
- `IrqMsiHandler(cap::IrqMsiHandler)`
- `IrqIOApicHandler(cap::IrqIOApicHandler)`
- `RiscvIrqHandler(cap::RiscvIrqHandler)`
- `IOPorts(cap::IOPorts)`
- `SchedContext(cap::SchedContext)`
- `Reply(cap::Reply)`
- `ArmSmc(cap::ArmSmc)`
- `DomainSet(cap::DomainSet)`

**Methods:**

- `fn as_<T>(self: &Self) -> Option<&T>`
- `fn obj(self: &Self) -> ObjectId`
- `fn set_obj(self: & mut Self, object: ObjectId)`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as ::rkyv::Archive>::Resolver, out: ::rkyv::Place<<Self as ::rkyv::Archive>::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Cap) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Cap`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::CapResolver

*Enum*

The resolver for an archived [`Cap`]

**Variants:**
- `Untyped(<cap::Untyped as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::Untyped`]
- `Endpoint(<cap::Endpoint as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::Endpoint`]
- `Notification(<cap::Notification as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::Notification`]
- `CNode(<cap::CNode as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::CNode`]
- `Tcb(<cap::Tcb as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::Tcb`]
- `IrqHandler(<cap::IrqHandler as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::IrqHandler`]
- `VCpu(<cap::VCpu as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::VCpu`]
- `Frame(<cap::Frame as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::Frame`]
- `PageTable(<cap::PageTable as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::PageTable`]
- `AsidPool(<cap::AsidPool as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::AsidPool`]
- `ArmIrqHandler(<cap::ArmIrqHandler as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::ArmIrqHandler`]
- `IrqMsiHandler(<cap::IrqMsiHandler as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::IrqMsiHandler`]
- `IrqIOApicHandler(<cap::IrqIOApicHandler as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::IrqIOApicHandler`]
- `RiscvIrqHandler(<cap::RiscvIrqHandler as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::RiscvIrqHandler`]
- `IOPorts(<cap::IOPorts as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::IOPorts`]
- `SchedContext(<cap::SchedContext as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::SchedContext`]
- `Reply(<cap::Reply as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::Reply`]
- `ArmSmc(<cap::ArmSmc as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::ArmSmc`]
- `DomainSet(<cap::DomainSet as ::rkyv::Archive>::Resolver)` - The resolver for [`Cap::DomainSet`]



## sel4_capdl_initializer_types::spec::CapSlot

*Struct*

**Tuple Struct**: `(u32)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &CapSlot) -> bool`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CapSlot`
- **From**
  - `fn from(x: usize) -> CapSlot`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::CapSlotResolver

*Struct*

The resolver for an archived [`CapSlot`]

**Tuple Struct**: `()`



## sel4_capdl_initializer_types::spec::CapTableEntry

*Struct*

**Fields:**
- `slot: CapSlot`
- `cap: Cap`

**Traits:** Eq

**Trait Implementations:**

- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CapTableEntry) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CapTableEntry`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::CapTableEntryResolver

*Struct*

The resolver for an archived [`CapTableEntry`]



## sel4_capdl_initializer_types::spec::DomainSchedEntry

*Struct*

**Fields:**
- `id: u8`
- `time: u64`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &DomainSchedEntry) -> bool`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DomainSchedEntry`



## sel4_capdl_initializer_types::spec::DomainSchedEntryResolver

*Struct*

The resolver for an archived [`DomainSchedEntry`]



## sel4_capdl_initializer_types::spec::IrqEntry

*Struct*

**Fields:**
- `irq: Word`
- `handler: ObjectId`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IrqEntry) -> bool`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> IrqEntry`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`



## sel4_capdl_initializer_types::spec::IrqEntryResolver

*Struct*

The resolver for an archived [`IrqEntry`]



## sel4_capdl_initializer_types::spec::IsArchivedCap

*Trait*

**Methods:**

- `try_from_cap`



## sel4_capdl_initializer_types::spec::IsArchivedObject

*Trait*

**Methods:**

- `try_from_object`



## sel4_capdl_initializer_types::spec::IsCap

*Trait*

**Methods:**

- `into_cap`
- `try_from_cap`



## sel4_capdl_initializer_types::spec::IsObject

*Trait*

**Methods:**

- `into_object`
- `try_from_object`



## sel4_capdl_initializer_types::spec::NamedObject

*Struct*

**Generic Parameters:**
- D

**Fields:**
- `name: Option<alloc::string::String>`
- `object: Object<D>`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NamedObject<D>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NamedObject<D>`



## sel4_capdl_initializer_types::spec::NamedObjectResolver

*Struct*

The resolver for an archived [`NamedObject`]

**Generic Parameters:**
- D



## sel4_capdl_initializer_types::spec::Object

*Enum*

**Generic Parameters:**
- D

**Variants:**
- `Untyped(object::Untyped)`
- `Endpoint`
- `Notification`
- `CNode(object::CNode)`
- `Tcb(object::Tcb)`
- `Irq(object::Irq)`
- `VCpu`
- `Frame(object::Frame<D>)`
- `PageTable(object::PageTable)`
- `AsidPool(object::AsidPool)`
- `ArmIrq(object::ArmIrq)`
- `IrqMsi(object::IrqMsi)`
- `IrqIOApic(object::IrqIOApic)`
- `RiscvIrq(object::RiscvIrq)`
- `IOPorts(object::IOPorts)`
- `SchedContext(object::SchedContext)`
- `Reply`
- `ArmSmc`
- `DomainSet`

**Methods:**

- `fn as_<T>(self: &Self) -> Option<&T>`
- `fn paddr(self: &Self) -> Option<Word>`
- `fn slots(self: &Self) -> Option<&[CapTableEntry]>`
- `fn slots_mut(self: & mut Self) -> Option<& mut Vec<CapTableEntry>>`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as ::rkyv::Archive>::Resolver, out: ::rkyv::Place<<Self as ::rkyv::Archive>::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Object<D>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Object<D>`



## sel4_capdl_initializer_types::spec::ObjectId

*Struct*

**Tuple Struct**: `(u32)`

**Methods:**

- `fn into_usize_range(range: &Range<ObjectId>) -> Range<usize>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ObjectId`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectId) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **From**
  - `fn from(x: usize) -> ObjectId`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_capdl_initializer_types::spec::ObjectIdResolver

*Struct*

The resolver for an archived [`ObjectId`]

**Tuple Struct**: `()`



## sel4_capdl_initializer_types::spec::ObjectResolver

*Enum*

The resolver for an archived [`Object`]

**Generic Parameters:**
- D

**Variants:**
- `Untyped(<object::Untyped as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::Untyped`]
- `Endpoint` - The resolver for [`Object::Endpoint`]
- `Notification` - The resolver for [`Object::Notification`]
- `CNode(<object::CNode as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::CNode`]
- `Tcb(<object::Tcb as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::Tcb`]
- `Irq(<object::Irq as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::Irq`]
- `VCpu` - The resolver for [`Object::VCpu`]
- `Frame(<object::Frame<D> as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::Frame`]
- `PageTable(<object::PageTable as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::PageTable`]
- `AsidPool(<object::AsidPool as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::AsidPool`]
- `ArmIrq(<object::ArmIrq as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::ArmIrq`]
- `IrqMsi(<object::IrqMsi as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::IrqMsi`]
- `IrqIOApic(<object::IrqIOApic as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::IrqIOApic`]
- `RiscvIrq(<object::RiscvIrq as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::RiscvIrq`]
- `IOPorts(<object::IOPorts as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::IOPorts`]
- `SchedContext(<object::SchedContext as ::rkyv::Archive>::Resolver)` - The resolver for [`Object::SchedContext`]
- `Reply` - The resolver for [`Object::Reply`]
- `ArmSmc` - The resolver for [`Object::ArmSmc`]
- `DomainSet` - The resolver for [`Object::DomainSet`]



## sel4_capdl_initializer_types::spec::OrigCapSlots

*Struct*

**Fields:**
- `num_occupied: u32`
- `offsets_by_object: alloc::vec::Vec<Option<u32>>`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &OrigCapSlots) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> OrigCapSlots`



## sel4_capdl_initializer_types::spec::OrigCapSlotsResolver

*Struct*

The resolver for an archived [`OrigCapSlots`]



## sel4_capdl_initializer_types::spec::PageTableEntry

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `PageTable(&'a cap::ArchivedPageTable)`
- `Frame(&'a cap::ArchivedFrame)`



## sel4_capdl_initializer_types::spec::Rights

*Struct*

**Fields:**
- `read: bool`
- `write: bool`
- `grant: bool`
- `grant_reply: bool`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Rights) -> bool`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Rights`



## sel4_capdl_initializer_types::spec::RightsResolver

*Struct*

The resolver for an archived [`Rights`]



## sel4_capdl_initializer_types::spec::Spec

*Struct*

**Generic Parameters:**
- D

**Fields:**
- `objects: alloc::vec::Vec<NamedObject<D>>`
- `irqs: alloc::vec::Vec<IrqEntry>`
- `domain_schedule: Option<alloc::vec::Vec<DomainSchedEntry>>`
- `domain_set_start: Option<Word>`
- `domain_idx_shift: Option<Word>`
- `asid_slots: alloc::vec::Vec<AsidSlotEntry>`
- `root_objects: core::ops::Range<ObjectId>`
- `untyped_covers: alloc::vec::Vec<UntypedCover>`
- `cached_orig_cap_slots: Option<OrigCapSlots>`
- `log_level: Option<u8>`

**Methods:**

- `fn to_bytes(self: &Self) -> Result<AlignedVec, rancor::Error>`
- `fn access(buf: &[u8]) -> Result<&<Self as Archive>::Archived, rancor::Error>`
- `fn access_unchecked(buf: &[u8]) -> &<Self as Archive>::Archived`

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Spec<D>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Spec<D>`



## sel4_capdl_initializer_types::spec::SpecResolver

*Struct*

The resolver for an archived [`Spec`]

**Generic Parameters:**
- D



## sel4_capdl_initializer_types::spec::UntypedCover

*Struct*

**Fields:**
- `parent: ObjectId`
- `children: core::ops::Range<ObjectId>`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UntypedCover`
- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UntypedCover) -> bool`



## sel4_capdl_initializer_types::spec::UntypedCoverResolver

*Struct*

The resolver for an archived [`UntypedCover`]



## sel4_capdl_initializer_types::spec::Word

*Struct*

**Tuple Struct**: `(u64)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut __S) -> ::core::result::Result<<Self as ::rkyv::Archive>::Resolver, <__S as ::rkyv::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Word) -> bool`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: ::rkyv::Place<<Self as >::Archived>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Word`
- **From**
  - `fn from(x: u64) -> Word`



## sel4_capdl_initializer_types::spec::WordResolver

*Struct*

The resolver for an archived [`Word`]

**Tuple Struct**: `()`



## Module: cap



## Module: object



