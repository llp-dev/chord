**lru**

# Module: lru

## Contents

**Structs**

- [`IntoIter`](#intoiter) - An iterator that moves out of a `LruCache`.
- [`Iter`](#iter) - An iterator over the entries of a `LruCache`.
- [`IterMut`](#itermut) - An iterator over mutables entries of a `LruCache`.
- [`LruCache`](#lrucache) - An LRU Cache

**Type Aliases**

- [`DefaultHasher`](#defaulthasher)

---

## lru::DefaultHasher

*Type Alias*: `hashbrown::DefaultHashBuilder`



## lru::IntoIter

*Struct*

An iterator that moves out of a `LruCache`.

This `struct` is created by the [`into_iter`] method on [`LruCache`][`LruCache`]. See its
documentation for more.

[`into_iter`]: struct.LruCache.html#method.into_iter
[`LruCache`]: struct.LruCache.html

**Generic Parameters:**
- K
- V

**Traits:** FusedIterator, ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<(K, V)>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn count(self: Self) -> usize`



## lru::Iter

*Struct*

An iterator over the entries of a `LruCache`.

This `struct` is created by the [`iter`] method on [`LruCache`][`LruCache`]. See its
documentation for more.

[`iter`]: struct.LruCache.html#method.iter
[`LruCache`]: struct.LruCache.html

**Generic Parameters:**
- 'a
- K
- V

**Traits:** FusedIterator, ExactSizeIterator, Sync, Send

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<(&'a K, &'a V)>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<(&'a K, &'a V)>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn count(self: Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> Iter<'a, K, V>`



## lru::IterMut

*Struct*

An iterator over mutables entries of a `LruCache`.

This `struct` is created by the [`iter_mut`] method on [`LruCache`][`LruCache`]. See its
documentation for more.

[`iter_mut`]: struct.LruCache.html#method.iter_mut
[`LruCache`]: struct.LruCache.html

**Generic Parameters:**
- 'a
- K
- V

**Traits:** Sync, Send, FusedIterator, ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<(&'a K, &'a  mut V)>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn count(self: Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<(&'a K, &'a  mut V)>`



## lru::LruCache

*Struct*

An LRU Cache

**Generic Parameters:**
- K
- V
- S

**Methods:**

- `fn new(cap: NonZeroUsize) -> LruCache<K, V>` - Creates a new LRU Cache that holds at most `cap` items.
- `fn unbounded() -> LruCache<K, V>` - Creates a new LRU Cache that never automatically evicts items.
- `fn with_hasher(cap: NonZeroUsize, hash_builder: S) -> LruCache<K, V, S>` - Creates a new LRU Cache that holds at most `cap` items and
- `fn unbounded_with_hasher(hash_builder: S) -> LruCache<K, V, S>` - Creates a new LRU Cache that never automatically evicts items and
- `fn put(self: & mut Self, k: K, v: V) -> Option<V>` - Puts a key-value pair into cache. If the key already exists in the cache, then it updates
- `fn push(self: & mut Self, k: K, v: V) -> Option<(K, V)>` - Pushes a key-value pair into the cache. If an entry with key `k` already exists in
- `fn get<'a, Q>(self: &'a  mut Self, k: &Q) -> Option<&'a V>` - Returns a reference to the value of the key in the cache or `None` if it is not
- `fn get_mut<'a, Q>(self: &'a  mut Self, k: &Q) -> Option<&'a  mut V>` - Returns a mutable reference to the value of the key in the cache or `None` if it
- `fn get_key_value<'a, Q>(self: &'a  mut Self, k: &Q) -> Option<(&'a K, &'a V)>` - Returns a key-value references pair of the key in the cache or `None` if it is not
- `fn get_key_value_mut<'a, Q>(self: &'a  mut Self, k: &Q) -> Option<(&'a K, &'a  mut V)>` - Returns a key-value references pair of the key in the cache or `None` if it is not
- `fn get_or_insert<F>(self: & mut Self, k: K, f: F) -> &V` - Returns a reference to the value of the key in the cache if it is
- `fn get_or_insert_ref<'a, Q, F>(self: &'a  mut Self, k: &Q, f: F) -> &'a V` - Returns a reference to the value of the key in the cache if it is
- `fn try_get_or_insert<F, E>(self: & mut Self, k: K, f: F) -> Result<&V, E>` - Returns a reference to the value of the key in the cache if it is
- `fn try_get_or_insert_ref<'a, Q, F, E>(self: &'a  mut Self, k: &Q, f: F) -> Result<&'a V, E>` - Returns a reference to the value of the key in the cache if it is
- `fn get_or_insert_mut<F>(self: & mut Self, k: K, f: F) -> & mut V` - Returns a mutable reference to the value of the key in the cache if it is
- `fn get_or_insert_mut_ref<'a, Q, F>(self: & mut Self, k: &Q, f: F) -> &'a  mut V` - Returns a mutable reference to the value of the key in the cache if it is
- `fn try_get_or_insert_mut<F, E>(self: & mut Self, k: K, f: F) -> Result<& mut V, E>` - Returns a mutable reference to the value of the key in the cache if it is
- `fn try_get_or_insert_mut_ref<'a, Q, F, E>(self: &'a  mut Self, k: &Q, f: F) -> Result<&'a  mut V, E>` - Returns a mutable reference to the value of the key in the cache if it is
- `fn peek<'a, Q>(self: &'a Self, k: &Q) -> Option<&'a V>` - Returns a reference to the value corresponding to the key in the cache or `None` if it is
- `fn peek_mut<'a, Q>(self: &'a  mut Self, k: &Q) -> Option<&'a  mut V>` - Returns a mutable reference to the value corresponding to the key in the cache or `None`
- `fn peek_lru(self: &Self) -> Option<(&K, &V)>` - Returns the value corresponding to the least recently used item or `None` if the
- `fn peek_mru(self: &Self) -> Option<(&K, &V)>` - Returns the value corresponding to the most recently used item or `None` if the
- `fn contains<Q>(self: &Self, k: &Q) -> bool` - Returns a bool indicating whether the given key is in the cache. Does not update the
- `fn pop<Q>(self: & mut Self, k: &Q) -> Option<V>` - Removes and returns the value corresponding to the key from the cache or
- `fn pop_entry<Q>(self: & mut Self, k: &Q) -> Option<(K, V)>` - Removes and returns the key and the value corresponding to the key from the cache or
- `fn pop_lru(self: & mut Self) -> Option<(K, V)>` - Removes and returns the key and value corresponding to the least recently
- `fn pop_mru(self: & mut Self) -> Option<(K, V)>` - Removes and returns the key and value corresponding to the most recently
- `fn promote<Q>(self: & mut Self, k: &Q) -> bool` - Marks the key as the most recently used one. Returns true if the key
- `fn demote<Q>(self: & mut Self, k: &Q) -> bool` - Marks the key as the least recently used one. Returns true if the key was demoted
- `fn len(self: &Self) -> usize` - Returns the number of key-value pairs that are currently in the the cache.
- `fn is_empty(self: &Self) -> bool` - Returns a bool indicating whether the cache is empty or not.
- `fn cap(self: &Self) -> NonZeroUsize` - Returns the maximum number of key-value pairs the cache can hold.
- `fn resize(self: & mut Self, cap: NonZeroUsize)` - Resizes the cache. If the new capacity is smaller than the size of the current
- `fn clear(self: & mut Self)` - Clears the contents of the cache.
- `fn iter(self: &Self) -> Iter<K, V>` - An iterator visiting all entries in most-recently used order. The iterator element type is
- `fn iter_mut(self: & mut Self) -> IterMut<K, V>` - An iterator visiting all entries in most-recently-used order, giving a mutable reference on

**Traits:** Sync, Send

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **IntoIterator**
  - `fn into_iter(self: Self) -> IntoIter<K, V>`



