**sel4_test_harness > for_generated_code > types**

# Module: for_generated_code::types

## Contents

**Structs**

- [`TestDesc`](#testdesc)
- [`TestDescAndFn`](#testdescandfn)
- [`TestId`](#testid)

**Enums**

- [`NamePadding`](#namepadding)
- [`ShouldPanic`](#shouldpanic) - Whether test is expected to panic or not
- [`TestFn`](#testfn)
- [`TestName`](#testname)
- [`TestType`](#testtype)

---

## sel4_test_harness::for_generated_code::types::NamePadding

*Enum*

**Variants:**
- `PadNone`
- `PadOnRight`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &NamePadding) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NamePadding`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4_test_harness::for_generated_code::types::ShouldPanic

*Enum*

Whether test is expected to panic or not

**Variants:**
- `No`
- `Yes`
- `YesWithMessage(&'static str)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &ShouldPanic) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ShouldPanic`



## sel4_test_harness::for_generated_code::types::TestDesc

*Struct*

**Fields:**
- `name: TestName`
- `ignore: bool`
- `ignore_message: Option<&'static str>`
- `source_file: &'static str`
- `start_line: usize`
- `start_col: usize`
- `end_line: usize`
- `end_col: usize`
- `should_panic: ShouldPanic`
- `compile_fail: bool`
- `no_run: bool`
- `test_type: TestType`

**Methods:**

- `fn padded_name(self: &Self, column_count: usize, align: NamePadding) -> String`
- `fn test_mode(self: &Self) -> Option<&'static str>` - Returns None for ignored test or tests that are just run, otherwise returns a description of the type of test.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TestDesc`



## sel4_test_harness::for_generated_code::types::TestDescAndFn

*Struct*

**Fields:**
- `desc: TestDesc`
- `testfn: TestFn`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_test_harness::for_generated_code::types::TestFn

*Enum*

**Variants:**
- `StaticTestFn(fn(...))`

**Methods:**

- `fn padding(self: &Self) -> NamePadding`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## sel4_test_harness::for_generated_code::types::TestId

*Struct*

**Tuple Struct**: `(usize)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &TestId) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TestId`



## sel4_test_harness::for_generated_code::types::TestName

*Enum*

**Variants:**
- `StaticTestName(&'static str)`
- `DynTestName(alloc::string::String)`
- `AlignedTestName(alloc::borrow::Cow<'static, str>, NamePadding)`

**Methods:**

- `fn as_slice(self: &Self) -> &str`
- `fn padding(self: &Self) -> NamePadding`
- `fn with_padding(self: &Self, padding: NamePadding) -> TestName`

**Traits:** Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &TestName) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TestName`



## sel4_test_harness::for_generated_code::types::TestType

*Enum*

**Variants:**
- `UnitTest`
- `IntegrationTest`
- `DocTest`
- `Unknown`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &TestType) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TestType`



