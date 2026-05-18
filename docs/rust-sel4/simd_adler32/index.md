# simd_adler32

# simd-adler32

A SIMD-accelerated Adler-32 hash algorithm implementation.

## Features

- No dependencies
- Support `no_std` (with `default-features = false`)
- Runtime CPU feature detection (when `std` enabled)
- Blazing fast performance on as many targets as possible (currently only x86 and x86_64)
- Default to scalar implementation when simd not available

## Quick start

> Cargo.toml

```toml
[dependencies]
simd-adler32 = "*"
```

> example.rs

```rust
use simd_adler32::Adler32;

let mut adler = Adler32::new();
adler.write(b"rust is pretty cool, man");
let hash = adler.finish();

println!("{}", hash);
// 1921255656
```

## Feature flags

* `std` - Enabled by default

Enables std support, see [CPU Feature Detection](#cpu-feature-detection) for runtime
detection support.
* `nightly`

Enables nightly features required for avx512 support.

* `const-generics` - Enabled by default

Enables const-generics support allowing for user-defined array hashing by value.  See
[`Adler32Hash`] for details.

## Support

**CPU Features**

| impl | arch             | feature |
| ---- | ---------------- | ------- |
| ✅   | `x86`, `x86_64`  | avx512  |
| ✅   | `x86`, `x86_64`  | avx2    |
| ✅   | `x86`, `x86_64`  | ssse3   |
| ✅   | `x86`, `x86_64`  | sse2    |
| 🚧   | `arm`, `aarch64` | neon    |
|      | `wasm32`         | simd128 |

**MSRV** `1.36.0`\*\*

Minimum supported rust version is tested before a new version is published. [**] Feature
`const-generics` needs to disabled to build on rustc versions `<1.51` which can be done
by updating your dependency definition to the following.

## CPU Feature Detection
simd-adler32 supports both runtime and compile time CPU feature detection using the
`std::is_x86_feature_detected` macro when the `Adler32` struct is instantiated with
the `new` fn.

Without `std` feature enabled simd-adler32 falls back to compile time feature detection
using `target-feature` or `target-cpu` flags supplied to rustc. See [https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html](https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html)
for more information.

Feature detection tries to use the fastest supported feature first.

## Modules

### [`simd_adler32`](simd_adler32.md)

*1 function, 1 struct, 1 trait*

