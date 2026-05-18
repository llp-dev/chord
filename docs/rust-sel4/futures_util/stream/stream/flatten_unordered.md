**futures_util > stream > stream > flatten_unordered**

# Module: stream::stream::flatten_unordered

## Contents

**Structs**

- [`FlattenUnorderedWithFlowController`](#flattenunorderedwithflowcontroller) - Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)

**Enums**

- [`FlowStep`](#flowstep) - Describes the next flow step.

**Traits**

- [`FlowController`](#flowcontroller) - Returns the next flow step based on the received item.

**Type Aliases**

- [`FlattenUnordered`](#flattenunordered) - Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)

---

## futures_util::stream::stream::flatten_unordered::FlattenUnordered

*Type Alias*: `FlattenUnorderedWithFlowController<St, ()>`

Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)
method.



## futures_util::stream::stream::flatten_unordered::FlattenUnorderedWithFlowController

*Struct*

Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)
method with ability to specify flow controller.

**Generic Parameters:**
- St
- Fc



## futures_util::stream::stream::flatten_unordered::FlowController

*Trait*

Returns the next flow step based on the received item.

**Methods:**

- `next_step`: Handles an item producing `FlowStep` describing the next flow step.



## futures_util::stream::stream::flatten_unordered::FlowStep

*Enum*

Describes the next flow step.

**Generic Parameters:**
- C
- R

**Variants:**
- `Continue(C)` - Just yields an item and continues standard flow.
- `Return(R)` - Immediately returns an underlying item from the function.



