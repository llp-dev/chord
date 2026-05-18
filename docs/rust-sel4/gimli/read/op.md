**gimli > read > op**

# Module: read::op

## Contents

**Structs**

- [`Evaluation`](#evaluation) - A DWARF expression evaluator.
- [`Expression`](#expression) - The bytecode for a DWARF expression or location description.
- [`OperationIter`](#operationiter) - An iterator for the operations in an expression.
- [`Piece`](#piece) - The description of a single piece of the result of a DWARF

**Enums**

- [`DieReference`](#diereference) - A reference to a DIE, either relative to the current CU or
- [`EvaluationResult`](#evaluationresult) - The state of an `Evaluation` after evaluating a DWARF expression.
- [`Location`](#location) - A single location of a piece of the result of a DWARF expression.
- [`Operation`](#operation) - A single decoded DWARF expression operation.

**Traits**

- [`EvaluationStorage`](#evaluationstorage) - Specification of what storage should be used for [`Evaluation`].

---

## gimli::read::op::DieReference

*Enum*

A reference to a DIE, either relative to the current CU or
relative to the section.

**Generic Parameters:**
- T

**Variants:**
- `UnitRef(crate::read::UnitOffset<T>)` - A CU-relative reference.
- `DebugInfoRef(crate::common::DebugInfoOffset<T>)` - A section-relative reference.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DieReference<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DieReference<T>`



## gimli::read::op::Evaluation

*Struct*

A DWARF expression evaluator.

# Usage
A DWARF expression may require additional data to produce a final result,
such as the value of a register or a memory location.  Once initial setup
is complete (i.e. `set_initial_value()`, `set_object_address()`) the
consumer calls the `evaluate()` method.  That returns an `EvaluationResult`,
which is either `EvaluationResult::Complete` or a value indicating what
data is needed to resume the `Evaluation`.  The consumer is responsible for
producing that data and resuming the computation with the correct method,
as documented for `EvaluationResult`.  Only once an `EvaluationResult::Complete`
is returned can the consumer call `result()`.

This design allows the consumer of `Evaluation` to decide how and when to
produce the required data and resume the computation.  The `Evaluation` can
be driven synchronously (as shown below) or by some asynchronous mechanism
such as futures.

# Examples
```rust,no_run
use gimli::{Evaluation, EvaluationResult, Expression};
# let bytecode = gimli::EndianSlice::new(&[], gimli::LittleEndian);
# let encoding = unimplemented!();
# let get_register_value = |_, _| gimli::Value::Generic(42);
# let get_frame_base = || 0xdeadbeef;

let mut eval = Evaluation::new(bytecode, encoding);
let mut result = eval.evaluate().unwrap();
while result != EvaluationResult::Complete {
  match result {
    EvaluationResult::RequiresRegister { register, base_type } => {
      let value = get_register_value(register, base_type);
      result = eval.resume_with_register(value).unwrap();
    },
    EvaluationResult::RequiresFrameBase => {
      let frame_base = get_frame_base();
      result = eval.resume_with_frame_base(frame_base).unwrap();
    },
    _ => unimplemented!(),
  };
}

let result = eval.result();
println!("{:?}", result);
```

**Generic Parameters:**
- R
- S

**Methods:**

- `fn new_in(bytecode: R, encoding: Encoding) -> Self` - Create a new DWARF expression evaluator.
- `fn set_initial_value(self: & mut Self, value: u64)` - Set an initial value to be pushed on the DWARF expression
- `fn set_object_address(self: & mut Self, value: u64)` - Set the enclosing object's address, as used by
- `fn set_max_iterations(self: & mut Self, value: u32)` - Set the maximum number of iterations to be allowed by the
- `fn value_result(self: &Self) -> Option<Value>` - Get the result if this is an evaluation for a value.
- `fn as_result(self: &Self) -> &[Piece<R>]` - Get the result of this `Evaluation`.
- `fn evaluate(self: & mut Self) -> Result<EvaluationResult<R>>` - Evaluate a DWARF expression.  This method should only ever be called
- `fn resume_with_memory(self: & mut Self, value: Value) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided memory `value`.  This will apply
- `fn resume_with_register(self: & mut Self, value: Value) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided `register` value.  This will apply
- `fn resume_with_frame_base(self: & mut Self, frame_base: u64) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided `frame_base`.  This will
- `fn resume_with_tls(self: & mut Self, value: u64) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided `value`.  This will apply
- `fn resume_with_call_frame_cfa(self: & mut Self, cfa: u64) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided `cfa`.  This will
- `fn resume_with_at_location(self: & mut Self, bytes: R) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided `bytes`.  This will
- `fn resume_with_entry_value(self: & mut Self, entry_value: Value) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided `entry_value`.  This will
- `fn resume_with_parameter_ref(self: & mut Self, parameter_value: u64) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided `parameter_value`.  This will
- `fn resume_with_relocated_address(self: & mut Self, address: u64) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided relocated `address`.  This will use the
- `fn resume_with_indexed_address(self: & mut Self, address: u64) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided indexed `address`.  This will use the
- `fn resume_with_base_type(self: & mut Self, base_type: ValueType) -> Result<EvaluationResult<R>>` - Resume the `Evaluation` with the provided `base_type`.  This will use the

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::op::EvaluationResult

*Enum*

The state of an `Evaluation` after evaluating a DWARF expression.
The evaluation is either `Complete`, or it requires more data
to continue, as described by the variant.

**Generic Parameters:**
- R

**Variants:**
- `Complete` - The `Evaluation` is complete, and `Evaluation::result()` can be called.
- `RequiresMemory{ address: u64, size: u8, space: Option<u64>, base_type: crate::read::UnitOffset<<R as >::Offset> }` - The `Evaluation` needs a value from memory to proceed further.  Once the
- `RequiresRegister{ register: crate::common::Register, base_type: crate::read::UnitOffset<<R as >::Offset> }` - The `Evaluation` needs a value from a register to proceed further.  Once
- `RequiresFrameBase` - The `Evaluation` needs the frame base address to proceed further.  Once
- `RequiresTls(u64)` - The `Evaluation` needs a value from TLS to proceed further.  Once the
- `RequiresCallFrameCfa` - The `Evaluation` needs the CFA to proceed further.  Once the caller
- `RequiresAtLocation(DieReference<<R as >::Offset>)` - The `Evaluation` needs the DWARF expression at the given location to
- `RequiresEntryValue(Expression<R>)` - The `Evaluation` needs the value produced by evaluating a DWARF
- `RequiresParameterRef(crate::read::UnitOffset<<R as >::Offset>)` - The `Evaluation` needs the value of the parameter at the given location
- `RequiresRelocatedAddress(u64)` - The `Evaluation` needs an address to be relocated to proceed further.
- `RequiresIndexedAddress{ index: crate::common::DebugAddrIndex<<R as >::Offset>, relocate: bool }` - The `Evaluation` needs an address from the `.debug_addr` section.
- `RequiresBaseType(crate::read::UnitOffset<<R as >::Offset>)` - The `Evaluation` needs the `ValueType` for the base type DIE at

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &EvaluationResult<R>) -> bool`



## gimli::read::op::EvaluationStorage

*Trait*

Specification of what storage should be used for [`Evaluation`].


If you need to avoid [`Evaluation`] from allocating memory, e.g. for signal safety,
you can provide you own storage specification:
```rust,no_run
# use gimli::*;
# let bytecode = EndianSlice::new(&[], LittleEndian);
# let encoding = unimplemented!();
# let get_register_value = |_, _| Value::Generic(42);
# let get_frame_base = || 0xdeadbeef;
#
struct StoreOnStack;

impl<R: Reader> EvaluationStorage<R> for StoreOnStack {
    type Stack = [Value; 64];
    type ExpressionStack = [(R, R); 4];
    type Result = [Piece<R>; 1];
}

let mut eval = Evaluation::<_, StoreOnStack>::new_in(bytecode, encoding);
let mut result = eval.evaluate().unwrap();
while result != EvaluationResult::Complete {
  match result {
    EvaluationResult::RequiresRegister { register, base_type } => {
      let value = get_register_value(register, base_type);
      result = eval.resume_with_register(value).unwrap();
    },
    EvaluationResult::RequiresFrameBase => {
      let frame_base = get_frame_base();
      result = eval.resume_with_frame_base(frame_base).unwrap();
    },
    _ => unimplemented!(),
  };
}

let result = eval.as_result();
println!("{:?}", result);
```

**Methods:**

- `Stack`: The storage used for the evaluation stack.
- `ExpressionStack`: The storage used for the expression stack.
- `Result`: The storage used for the results.



## gimli::read::op::Expression

*Struct*

The bytecode for a DWARF expression or location description.

**Generic Parameters:**
- R

**Tuple Struct**: `(R)`

**Methods:**

- `fn operations(self: Self, encoding: Encoding) -> OperationIter<R>` - Return an iterator for the operations in the expression.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Expression<R>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Expression<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::op::Location

*Enum*

A single location of a piece of the result of a DWARF expression.

**Generic Parameters:**
- R
- Offset

**Variants:**
- `Empty` - The piece is empty.  Ordinarily this means the piece has been
- `Register{ register: crate::common::Register }` - The piece is found in a register.
- `Address{ address: u64 }` - The piece is found in memory.
- `Value{ value: crate::read::Value }` - The piece has no location but its value is known.
- `Bytes{ value: R }` - The piece is represented by some constant bytes.
- `ImplicitPointer{ value: crate::common::DebugInfoOffset<Offset>, byte_offset: i64 }` - The piece is a pointer to a value which has no actual location.

**Methods:**

- `fn is_empty(self: &Self) -> bool` - Return true if the piece is empty.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Location<R, Offset>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Location<R, Offset>`



## gimli::read::op::Operation

*Enum*

A single decoded DWARF expression operation.

DWARF expression evaluation is done in two parts: first the raw
bytes of the next part of the expression are decoded; and then the
decoded operation is evaluated.  This approach lets other
consumers inspect the DWARF expression without reimplementing the
decoding operation.

Multiple DWARF opcodes may decode into a single `Operation`.  For
example, both `DW_OP_deref` and `DW_OP_xderef` are represented
using `Operation::Deref`.

**Generic Parameters:**
- R
- Offset

**Variants:**
- `Deref{ base_type: crate::read::UnitOffset<Offset>, size: u8, space: bool }` - Dereference the topmost value of the stack.
- `Drop` - Drop an item from the stack.
- `Pick{ index: u8 }` - Pick an item from the stack and push it on top of the stack.
- `Swap` - Swap the top two stack items.
- `Rot` - Rotate the top three stack items.
- `Abs` - Take the absolute value of the top of the stack.
- `And` - Bitwise `and` of the top two values on the stack.
- `Div` - Divide the top two values on the stack.
- `Minus` - Subtract the top two values on the stack.
- `Mod` - Modulus of the top two values on the stack.
- `Mul` - Multiply the top two values on the stack.
- `Neg` - Negate the top of the stack.
- `Not` - Bitwise `not` of the top of the stack.
- `Or` - Bitwise `or` of the top two values on the stack.
- `Plus` - Add the top two values on the stack.
- `PlusConstant{ value: u64 }` - Add a constant to the topmost value on the stack.
- `Shl` - Logical left shift of the 2nd value on the stack by the number
- `Shr` - Right shift of the 2nd value on the stack by the number of
- `Shra` - Arithmetic left shift of the 2nd value on the stack by the
- `Xor` - Bitwise `xor` of the top two values on the stack.
- `Bra{ target: i16 }` - Branch to the target location if the top of stack is nonzero.
- `Eq` - Compare the top two stack values for equality.
- `Ge` - Compare the top two stack values using `>=`.
- `Gt` - Compare the top two stack values using `>`.
- `Le` - Compare the top two stack values using `<=`.
- `Lt` - Compare the top two stack values using `<`.
- `Ne` - Compare the top two stack values using `!=`.
- `Skip{ target: i16 }` - Unconditional branch to the target location.
- `UnsignedConstant{ value: u64 }` - Push an unsigned constant value on the stack.  This handles multiple
- `SignedConstant{ value: i64 }` - Push a signed constant value on the stack.  This handles multiple
- `Register{ register: crate::common::Register }` - Indicate that this piece's location is in the given register.
- `RegisterOffset{ register: crate::common::Register, offset: i64, base_type: crate::read::UnitOffset<Offset> }` - Find the value of the given register, add the offset, and then
- `FrameOffset{ offset: i64 }` - Compute the frame base (using `DW_AT_frame_base`), add the
- `Nop` - No operation.
- `PushObjectAddress` - Push the object address on the stack.
- `Call{ offset: DieReference<Offset> }` - Evaluate a DWARF expression as a subroutine.  The expression
- `TLS` - Compute the address of a thread-local variable and push it on
- `CallFrameCFA` - Compute the call frame CFA and push it on the stack.
- `Piece{ size_in_bits: u64, bit_offset: Option<u64> }` - Terminate a piece.
- `ImplicitValue{ data: R }` - The object has no location, but has a known constant value.
- `StackValue` - The object has no location, but its value is at the top of the stack.
- `ImplicitPointer{ value: crate::common::DebugInfoOffset<Offset>, byte_offset: i64 }` - The object is a pointer to a value which has no actual location,
- `EntryValue{ expression: R }` - Evaluate an expression at the entry to the current subprogram, and push it on the stack.
- `ParameterRef{ offset: crate::read::UnitOffset<Offset> }` - This represents a parameter that was optimized out.
- `Address{ address: u64 }` - Relocate the address if needed, and push it on the stack.
- `AddressIndex{ index: crate::common::DebugAddrIndex<Offset> }` - Read the address at the given index in `.debug_addr, relocate the address if needed,
- `ConstantIndex{ index: crate::common::DebugAddrIndex<Offset> }` - Read the address at the given index in `.debug_addr, and push it on the stack.
- `TypedLiteral{ base_type: crate::read::UnitOffset<Offset>, value: R }` - Interpret the value bytes as a constant of a given type, and push it on the stack.
- `Convert{ base_type: crate::read::UnitOffset<Offset> }` - Pop the top stack entry, convert it to a different type, and push it on the stack.
- `Reinterpret{ base_type: crate::read::UnitOffset<Offset> }` - Pop the top stack entry, reinterpret the bits in its value as a different type,
- `WasmLocal{ index: u32 }` - The index of a local in the currently executing function.
- `WasmGlobal{ index: u32 }` - The index of a global.
- `WasmStack{ index: u32 }` - The index of an item on the operand stack.

**Methods:**

- `fn parse(bytes: & mut R, encoding: Encoding) -> Result<Operation<R, Offset>>` - Parse a single DWARF expression operation.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Operation<R, Offset>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Operation<R, Offset>`



## gimli::read::op::OperationIter

*Struct*

An iterator for the operations in an expression.

**Generic Parameters:**
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<Operation<R>>>` - Read the next operation in an expression.
- `fn offset_from(self: &Self, expression: &Expression<R>) -> <R as >::Offset` - Return the current byte offset of the iterator.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OperationIter<R>`



## gimli::read::op::Piece

*Struct*

The description of a single piece of the result of a DWARF
expression.

**Generic Parameters:**
- R
- Offset

**Fields:**
- `size_in_bits: Option<u64>` - If given, the size of the piece in bits.  If `None`, there
- `bit_offset: Option<u64>` - If given, the bit offset of the piece within the location.
- `location: Location<R, Offset>` - Where this piece is to be found.

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Piece<R, Offset>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Piece<R, Offset>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



