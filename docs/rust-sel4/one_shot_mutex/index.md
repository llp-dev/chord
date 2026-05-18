# one_shot_mutex

One-shot locks that panic instead of (dead)locking on contention.

These locks allow no contention and panic instead of blocking on `lock` if they are already locked.
This is useful in situations where contention would be a bug,
such as in single-threaded programs that would deadlock on contention.

See the [`sync::RawOneShotMutex`] and [`sync::RawOneShotRwLock`] types for more information.

## Modules

### [`one_shot_mutex`](one_shot_mutex.md)

*2 modules*

### [`sync::mutex`](sync/mutex.md)

*1 struct, 2 type aliases*

### [`sync::rwlock`](sync/rwlock.md)

*1 struct, 4 type aliases*

### [`unsync::mutex`](unsync/mutex.md)

*1 struct, 2 type aliases*

### [`unsync::rwlock`](unsync/rwlock.md)

*1 struct, 4 type aliases*

