**verus_builtin_macros > topological_sort**

# Module: topological_sort

## Contents

**Structs**

- [`TopologicalSort`](#topologicalsort)

---

## verus_builtin_macros::topological_sort::TopologicalSort

*Struct*

**Generic Parameters:**
- T

**Fields:**
- `values: Vec<T>`
- `nodes: std::collections::HashMap<T, usize>`
- `edges_rev: Vec<Vec<usize>>`

**Methods:**

- `fn new() -> Self`
- `fn add_node(self: & mut Self, v: T)`
- `fn add_edge(self: & mut Self, a: &T, b: &T)`
- `fn compute_topological_sort(self: & mut Self) -> Option<Vec<T>>`
- `fn visit(self: & mut Self, res: & mut Vec<T>, visited: & mut Vec<bool>, in_stack: & mut Vec<bool>, i: usize) -> bool`



