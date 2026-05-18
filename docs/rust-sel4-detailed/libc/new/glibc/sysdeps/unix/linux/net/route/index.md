*[libc](../../../../../../../index.md) / [new](../../../../../../index.md) / [glibc](../../../../../index.md) / [sysdeps](../../../../index.md) / [unix](../../../index.md) / [linux](../../index.md) / [net](../index.md) / [route](index.md)*

---

# Module `route`

Header: `net/route.h`

Source header: `sysdeps/unix/sysv/linux/net/route.h`
<https://github.com/bminor/glibc/blob/master/sysdeps/unix/sysv/linux/net/route.h>

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`rtentry`](#rtentry) | struct |  |

## Structs

### `rtentry`

```rust
struct rtentry {
    pub rt_pad1: c_ulong,
    pub rt_dst: crate::sockaddr,
    pub rt_gateway: crate::sockaddr,
    pub rt_genmask: crate::sockaddr,
    pub rt_flags: c_ushort,
    pub rt_pad2: c_short,
    pub rt_pad3: c_ulong,
    pub rt_tos: c_uchar,
    pub rt_class: c_uchar,
    pub rt_pad4: [c_short; 3],
    pub rt_metric: c_short,
    pub rt_dev: *mut c_char,
    pub rt_mtu: c_ulong,
    pub rt_window: c_ulong,
    pub rt_irtt: c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for rtentry`

- <span id="rtentry-clone"></span>`fn clone(&self) -> rtentry` — [`rtentry`](../../../../../../index.md#rtentry)

##### `impl Copy for rtentry`

##### `impl Debug for rtentry`

- <span id="rtentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

