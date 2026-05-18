# lru

An implementation of a LRU cache. The cache supports `get`, `get_mut`, `put`,
and `pop` operations, all of which are O(1). This crate was heavily influenced
by the [LRU Cache implementation in an earlier version of Rust's std::collections crate](https://doc.rust-lang.org/0.12.0/std/collections/lru_cache/struct.LruCache.html).

## Example

```rust
extern crate lru;

use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
        let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
        cache.put("apple", 3);
        cache.put("banana", 2);

        assert_eq!(*cache.get(&"apple").unwrap(), 3);
        assert_eq!(*cache.get(&"banana").unwrap(), 2);
        assert!(cache.get(&"pear").is_none());

        assert_eq!(cache.put("banana", 4), Some(2));
        assert_eq!(cache.put("pear", 5), None);

        assert_eq!(*cache.get(&"pear").unwrap(), 5);
        assert_eq!(*cache.get(&"banana").unwrap(), 4);
        assert!(cache.get(&"apple").is_none());

        {
            let v = cache.get_mut(&"banana").unwrap();
            *v = 6;
        }

        assert_eq!(*cache.get(&"banana").unwrap(), 6);
}
```

## Modules

### [`lru`](lru.md)

*1 type alias, 4 structs*

