*[sel4](../index.md) / [message_info](index.md)*

---

# Module `message_info`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MessageInfo`](#messageinfo) | struct | Corresponds to `seL4_MessageInfo_t`. |
| [`MessageInfoBuilder`](#messageinfobuilder) | struct | Helper for constructing [`MessageInfo`]. |

## Structs

### `MessageInfo`

```rust
struct MessageInfo(sys::seL4_MessageInfo);
```

Corresponds to `seL4_MessageInfo_t`.

#### Implementations

- <span id="messageinfo-from-inner"></span>`const fn from_inner(inner: sys::seL4_MessageInfo) -> Self`

- <span id="messageinfo-into-inner"></span>`const fn into_inner(self) -> sys::seL4_MessageInfo`

- <span id="messageinfo-inner"></span>`const fn inner(&self) -> &sys::seL4_MessageInfo`

- <span id="messageinfo-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_MessageInfo`

- <span id="messageinfo-new"></span>`fn new(label: Word, caps_unwrapped: usize, extra_caps: usize, length: usize) -> Self` — [`Word`](../index.md#word)

- <span id="messageinfo-label"></span>`fn label(&self) -> Word` — [`Word`](../index.md#word)

- <span id="messageinfo-label-width"></span>`const fn label_width() -> usize`

- <span id="messageinfo-caps-unwrapped"></span>`fn caps_unwrapped(&self) -> usize`

- <span id="messageinfo-extra-caps"></span>`fn extra_caps(&self) -> usize`

- <span id="messageinfo-length"></span>`fn length(&self) -> usize`

#### Trait Implementations

##### `impl Clone for MessageInfo`

- <span id="messageinfo-clone"></span>`fn clone(&self) -> MessageInfo` — [`MessageInfo`](#messageinfo)

##### `impl Debug for MessageInfo`

- <span id="messageinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MessageInfo`

##### `impl PartialEq for MessageInfo`

- <span id="messageinfo-partialeq-eq"></span>`fn eq(&self, other: &MessageInfo) -> bool` — [`MessageInfo`](#messageinfo)

##### `impl StructuralPartialEq for MessageInfo`

### `MessageInfoBuilder`

```rust
struct MessageInfoBuilder {
    label: crate::Word,
    caps_unwrapped: usize,
    extra_caps: usize,
    length: usize,
}
```

Helper for constructing [`MessageInfo`](#messageinfo).

#### Implementations

- <span id="messageinfobuilder-build"></span>`fn build(self) -> MessageInfo` — [`MessageInfo`](#messageinfo)

- <span id="messageinfobuilder-label"></span>`fn label(self, label: Word) -> Self` — [`Word`](../index.md#word)

- <span id="messageinfobuilder-caps-unwrapped"></span>`fn caps_unwrapped(self, caps_unwrapped: usize) -> Self`

- <span id="messageinfobuilder-extra-caps"></span>`fn extra_caps(self, extra_caps: usize) -> Self`

- <span id="messageinfobuilder-length"></span>`fn length(self, length: usize) -> Self`

#### Trait Implementations

##### `impl Clone for MessageInfoBuilder`

- <span id="messageinfobuilder-clone"></span>`fn clone(&self) -> MessageInfoBuilder` — [`MessageInfoBuilder`](#messageinfobuilder)

##### `impl Copy for MessageInfoBuilder`

##### `impl Debug for MessageInfoBuilder`

- <span id="messageinfobuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MessageInfoBuilder`

- <span id="messageinfobuilder-default"></span>`fn default() -> MessageInfoBuilder` — [`MessageInfoBuilder`](#messageinfobuilder)

##### `impl Eq for MessageInfoBuilder`

##### `impl PartialEq for MessageInfoBuilder`

- <span id="messageinfobuilder-partialeq-eq"></span>`fn eq(&self, other: &MessageInfoBuilder) -> bool` — [`MessageInfoBuilder`](#messageinfobuilder)

##### `impl StructuralPartialEq for MessageInfoBuilder`

