*[wasmparser](../index.md) / [features](index.md)*

---

# Module `features`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WasmFeatures`](#wasmfeatures) | struct | Enabled WebAssembly proposals and features. |
| [`define_wasm_features!`](#define-wasm-features) | macro |  |
| [`foreach_wasm_feature!`](#foreach-wasm-feature) | macro |  |

## Structs

### `WasmFeatures`

```rust
struct WasmFeatures {
    _priv: (),
}
```

Enabled WebAssembly proposals and features.

This is the disabled zero-size version of this structure because the
`features` feature was disabled at compile time of this crate.

#### Implementations

- <span id="wasmfeatures-mutable-global"></span>`fn mutable_global(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-saturating-float-to-int"></span>`fn saturating_float_to_int(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-sign-extension"></span>`fn sign_extension(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-reference-types"></span>`fn reference_types(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-multi-value"></span>`fn multi_value(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-bulk-memory"></span>`fn bulk_memory(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-simd"></span>`fn simd(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-relaxed-simd"></span>`fn relaxed_simd(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-threads"></span>`fn threads(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-shared-everything-threads"></span>`fn shared_everything_threads(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-tail-call"></span>`fn tail_call(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-floats"></span>`fn floats(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-multi-memory"></span>`fn multi_memory(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-exceptions"></span>`fn exceptions(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-memory64"></span>`fn memory64(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-extended-const"></span>`fn extended_const(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-component-model"></span>`fn component_model(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-function-references"></span>`fn function_references(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-memory-control"></span>`fn memory_control(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-gc"></span>`fn gc(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-custom-page-sizes"></span>`fn custom_page_sizes(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-legacy-exceptions"></span>`fn legacy_exceptions(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-gc-types"></span>`fn gc_types(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-stack-switching"></span>`fn stack_switching(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-wide-arithmetic"></span>`fn wide_arithmetic(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-values"></span>`fn cm_values(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-nested-names"></span>`fn cm_nested_names(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-async"></span>`fn cm_async(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-async-stackful"></span>`fn cm_async_stackful(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-async-builtins"></span>`fn cm_async_builtins(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-threading"></span>`fn cm_threading(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-error-context"></span>`fn cm_error_context(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-fixed-size-list"></span>`fn cm_fixed_size_list(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-gc"></span>`fn cm_gc(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-call-indirect-overlong"></span>`fn call_indirect_overlong(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-bulk-memory-opt"></span>`fn bulk_memory_opt(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-custom-descriptors"></span>`fn custom_descriptors(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

#### Trait Implementations

##### `impl Clone for WasmFeatures`

- <span id="wasmfeatures-clone"></span>`fn clone(&self) -> WasmFeatures` — [`WasmFeatures`](../index.md#wasmfeatures)

##### `impl Copy for WasmFeatures`

##### `impl Debug for WasmFeatures`

- <span id="wasmfeatures-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WasmFeatures`

- <span id="wasmfeatures-default"></span>`fn default() -> WasmFeatures` — [`WasmFeatures`](../index.md#wasmfeatures)

##### `impl Hash for WasmFeatures`

- <span id="wasmfeatures-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

## Macros

### `define_wasm_features!`

### `foreach_wasm_feature!`

