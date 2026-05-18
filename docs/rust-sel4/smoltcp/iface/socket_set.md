**smoltcp > iface > socket_set**

# Module: iface::socket_set

## Contents

**Structs**

- [`SocketHandle`](#sockethandle) - A handle, identifying a socket in an Interface.
- [`SocketSet`](#socketset) - An extensible set of sockets.
- [`SocketStorage`](#socketstorage) - Opaque struct with space for storing one socket.

---

## smoltcp::iface::socket_set::SocketHandle

*Struct*

A handle, identifying a socket in an Interface.

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SocketHandle`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SocketHandle) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &SocketHandle) -> $crate::cmp::Ordering`
- **Default**
  - `fn default() -> SocketHandle`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SocketHandle) -> bool`



## smoltcp::iface::socket_set::SocketSet

*Struct*

An extensible set of sockets.

The lifetime `'a` is used when storing a `Socket<'a>`.  If you're using
owned buffers for your sockets (passed in as `Vec`s) you can use
`SocketSet<'static>`.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<SocketsT>(sockets: SocketsT) -> SocketSet<'a>` - Create a socket set using the provided storage.
- `fn add<T>(self: & mut Self, socket: T) -> SocketHandle` - Add a socket to the set, and return its handle.
- `fn get<T>(self: &Self, handle: SocketHandle) -> &T` - Get a socket from the set by its handle, as mutable.
- `fn get_mut<T>(self: & mut Self, handle: SocketHandle) -> & mut T` - Get a mutable socket from the set by its handle, as mutable.
- `fn remove(self: & mut Self, handle: SocketHandle) -> Socket<'a>` - Remove a socket from the set, without changing its state.
- `fn iter(self: &Self) -> impl Trait` - Get an iterator to the inner sockets.
- `fn iter_mut(self: & mut Self) -> impl Trait` - Get a mutable iterator to the inner sockets.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::iface::socket_set::SocketStorage

*Struct*

Opaque struct with space for storing one socket.

This is public so you can use it to allocate space for storing
sockets when creating an Interface.

**Generic Parameters:**
- 'a

**Methods:**


**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> SocketStorage<'a>`



