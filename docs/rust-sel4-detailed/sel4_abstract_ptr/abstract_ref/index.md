*[sel4_abstract_ptr](../index.md) / [abstract_ref](index.md)*

---

# Module `abstract_ref`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AbstractRef`](#abstractref) | struct |  |

## Structs

### `AbstractRef<'a, M, T, A>`

```rust
struct AbstractRef<'a, M, T, A>
where
    T: ?Sized {
    pointer: core::ptr::NonNull<T>,
    memory_type: core::marker::PhantomData<M>,
    reference: core::marker::PhantomData<&'a T>,
    access: core::marker::PhantomData<A>,
}
```

#### Implementations

- <span id="abstractref-new"></span>`unsafe fn new(pointer: NonNull<T>) -> Self`

- <span id="abstractref-new-read-only"></span>`const unsafe fn new_read_only(pointer: NonNull<T>) -> AbstractRef<'a, M, T, ReadOnly>` — [`AbstractRef`](#abstractref), [`ReadOnly`](../access/index.md#readonly)

- <span id="abstractref-new-restricted"></span>`const unsafe fn new_restricted<A>(access: A, pointer: NonNull<T>) -> AbstractRef<'a, M, T, A>` — [`AbstractRef`](#abstractref)

- <span id="abstractref-from-ref"></span>`fn from_ref(reference: &'a T) -> AbstractRef<'a, M, T, ReadOnly>` — [`AbstractRef`](#abstractref), [`ReadOnly`](../access/index.md#readonly)

- <span id="abstractref-from-mut-ref"></span>`fn from_mut_ref(reference: &'a mut T) -> Self`

- <span id="abstractref-new-generic"></span>`const unsafe fn new_generic<A>(pointer: NonNull<T>) -> AbstractRef<'a, M, T, A>` — [`AbstractRef`](#abstractref)

#### Trait Implementations

##### `impl<M, T, A> Clone for AbstractRef<'_, M, T, A>`

- <span id="abstractref-clone"></span>`fn clone(&self) -> Self`

##### `impl<M, T, A> Copy for AbstractRef<'_, M, T, A>`

##### `impl<M, T, A> Debug for AbstractRef<'_, M, T, A>`

- <span id="abstractref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M, T, A> Eq for AbstractRef<'_, M, T, A>`

##### `impl<M, T, A> Hash for AbstractRef<'_, M, T, A>`

- <span id="abstractref-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl<M, T, A> Ord for AbstractRef<'_, M, T, A>`

- <span id="abstractref-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<M, T, A> PartialEq for AbstractRef<'_, M, T, A>`

- <span id="abstractref-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<M, T, A> PartialOrd for AbstractRef<'_, M, T, A>`

- <span id="abstractref-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> Pointee for AbstractRef<'a, M, T, A>`

- <span id="abstractref-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl<M, T, A> Pointer for AbstractRef<'_, M, T, A>`

- <span id="abstractref-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M, T, A> Send for AbstractRef<'_, M, T, A>`

##### `impl<M, T, A> Sync for AbstractRef<'_, M, T, A>`

