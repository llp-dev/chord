# Crate `ring`

Safe, fast, small crypto using Rust with BoringSSL's cryptography
primitives.

# Feature Flags

<table>
<tr><th>Feature
    <th>Description
<tr><td><code>alloc (default)</code>
    <td>Enable features that require use of the heap, RSA in particular.
<tr><td><code>less-safe-getrandom-custom-or-rdrand</code>
    <td>Treat user-provided ("custom") and RDRAND-based <code>getrandom</code>
        implementations as secure random number generators (see
        <code>SecureRandom</code>). This feature only works with
        <code>os = "none"</code> targets. See
        <a href="https://docs.rs/getrandom/0.2.10/getrandom/macro.register_custom_getrandom.html">
            <code>register_custom_getrandom</code>
        </a> and <a href="https://docs.rs/getrandom/0.2.10/getrandom/#rdrand-on-x86">
            RDRAND on x86
        </a> for additional details.
<tr><td><code>std</code>
    <td>Enable features that use libstd, in particular
        <code>std::error::Error</code> integration. Implies `alloc`.
<tr><td><code>wasm32_unknown_unknown_js</code>
    <td>When this feature is enabled, for the wasm32-unknown-unknown target,
        Web APIs will be used to implement features like `ring::rand` that
        require an operating environment of some kind. This has no effect
        for any other target. This enables the `getrandom` crate's `js`
        feature.
</table>

## Contents

- [Modules](#modules)
  - [`debug`](#debug)
  - [`prefixed`](#prefixed)
  - [`test`](#test)
  - [`arithmetic`](#arithmetic)
  - [`bssl`](#bssl)
  - [`polyfill`](#polyfill)
  - [`aead`](#aead)
  - [`agreement`](#agreement)
  - [`bits`](#bits)
  - [`c`](#c)
  - [`constant_time`](#constant-time)
  - [`io`](#io)
  - [`cpu`](#cpu)
  - [`digest`](#digest)
  - [`ec`](#ec)
  - [`endian`](#endian)
  - [`error`](#error)
  - [`hkdf`](#hkdf)
  - [`hmac`](#hmac)
  - [`limb`](#limb)
  - [`pbkdf2`](#pbkdf2)
  - [`pkcs8`](#pkcs8)
  - [`rand`](#rand)
  - [`rsa`](#rsa)
  - [`signature`](#signature)
  - [`sealed`](#sealed)
- [Macros](#macros)
  - [`test_file!`](#test-file)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`debug`](#debug) | mod |  |
| [`prefixed`](#prefixed) | mod |  |
| [`test`](#test) | mod | Testing framework. |
| [`arithmetic`](#arithmetic) | mod |  |
| [`bssl`](#bssl) | mod |  |
| [`polyfill`](#polyfill) | mod | Polyfills for functionality that will (hopefully) be added to Rust's standard library soon. |
| [`aead`](#aead) | mod | Authenticated Encryption with Associated Data (AEAD). |
| [`agreement`](#agreement) | mod | Key Agreement: ECDH, including X25519. |
| [`bits`](#bits) | mod | Bit lengths. |
| [`c`](#c) | mod | C types. |
| [`constant_time`](#constant-time) | mod | Constant-time operations. |
| [`io`](#io) | mod | Serialization and deserialization. |
| [`cpu`](#cpu) | mod |  |
| [`digest`](#digest) | mod | SHA-2 and the legacy SHA-1 digest algorithm. |
| [`ec`](#ec) | mod |  |
| [`endian`](#endian) | mod |  |
| [`error`](#error) | mod | Error reporting. |
| [`hkdf`](#hkdf) | mod | HMAC-based Extract-and-Expand Key Derivation Function. |
| [`hmac`](#hmac) | mod | HMAC is specified in [RFC 2104]. |
| [`limb`](#limb) | mod | Unsigned multi-precision integer arithmetic. |
| [`pbkdf2`](#pbkdf2) | mod | PBKDF2 derivation and verification. |
| [`pkcs8`](#pkcs8) | mod | PKCS#8 is specified in [RFC 5958]. |
| [`rand`](#rand) | mod | Cryptographic pseudo-random number generation. |
| [`rsa`](#rsa) | mod | RSA. |
| [`signature`](#signature) | mod | Public key signatures: signing and verification. |
| [`sealed`](#sealed) | mod |  |
| [`test_file!`](#test-file) | macro | References a test input file. |

## Modules

- [`debug`](debug/index.md)
- [`prefixed`](prefixed/index.md)
- [`test`](test/index.md) — Testing framework.
- [`arithmetic`](arithmetic/index.md)
- [`bssl`](bssl/index.md)
- [`polyfill`](polyfill/index.md) — Polyfills for functionality that will (hopefully) be added to Rust's
- [`aead`](aead/index.md) — Authenticated Encryption with Associated Data (AEAD).
- [`agreement`](agreement/index.md) — Key Agreement: ECDH, including X25519.
- [`bits`](bits/index.md) — Bit lengths.
- [`c`](c/index.md) — C types.
- [`constant_time`](constant_time/index.md) — Constant-time operations.
- [`io`](io/index.md) — Serialization and deserialization.
- [`cpu`](cpu/index.md)
- [`digest`](digest/index.md) — SHA-2 and the legacy SHA-1 digest algorithm.
- [`ec`](ec/index.md)
- [`endian`](endian/index.md)
- [`error`](error/index.md) — Error reporting.
- [`hkdf`](hkdf/index.md) — HMAC-based Extract-and-Expand Key Derivation Function.
- [`hmac`](hmac/index.md) — HMAC is specified in [RFC 2104].
- [`limb`](limb/index.md) — Unsigned multi-precision integer arithmetic.
- [`pbkdf2`](pbkdf2/index.md) — PBKDF2 derivation and verification.
- [`pkcs8`](pkcs8/index.md) — PKCS#8 is specified in [RFC 5958].
- [`rand`](rand/index.md) — Cryptographic pseudo-random number generation.
- [`rsa`](rsa/index.md) — RSA.
- [`signature`](signature/index.md) — Public key signatures: signing and verification.
- [`sealed`](sealed/index.md)

## Macros

### `test_file!`

References a test input file.

