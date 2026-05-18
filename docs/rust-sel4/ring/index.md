# ring

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

## Modules

### [`ring`](ring.md)

*1 macro, 14 modules*

### [`aead`](aead.md)

*1 constant, 2 modules, 2 traits, 3 structs*

### [`aead::aes_gcm`](aead/aes_gcm.md)

*2 statics*

### [`aead::chacha20_poly1305`](aead/chacha20_poly1305.md)

*1 static*

### [`aead::chacha20_poly1305_openssh`](aead/chacha20_poly1305_openssh.md)

*2 structs, 3 constants*

### [`aead::less_safe_key`](aead/less_safe_key.md)

*1 struct*

### [`aead::nonce`](aead/nonce.md)

*1 constant, 1 struct*

### [`aead::opening_key`](aead/opening_key.md)

*1 struct*

### [`aead::quic`](aead/quic.md)

*1 type alias, 2 structs, 3 statics*

### [`aead::sealing_key`](aead/sealing_key.md)

*1 struct*

### [`aead::unbound_key`](aead/unbound_key.md)

*1 struct*

### [`agreement`](agreement.md)

*1 function, 4 structs*

### [`bits`](bits.md)

*1 struct*

### [`bssl`](bssl.md)

*1 struct*

### [`constant_time`](constant_time.md)

*1 function*

### [`digest`](digest.md)

*1 function, 3 structs, 5 statics, 8 constants*

### [`ec::curve25519::ed25519`](ec/curve25519/ed25519.md)

*1 constant*

### [`ec::curve25519::ed25519::signing`](ec/curve25519/ed25519/signing.md)

*2 structs*

### [`ec::curve25519::ed25519::verification`](ec/curve25519/ed25519/verification.md)

*1 static, 1 struct*

### [`ec::curve25519::x25519`](ec/curve25519/x25519.md)

*1 static*

### [`ec::suite_b::ecdh`](ec/suite_b/ecdh.md)

*2 statics*

### [`ec::suite_b::ecdsa::signing`](ec/suite_b/ecdsa/signing.md)

*3 structs, 4 statics*

### [`ec::suite_b::ecdsa::verification`](ec/suite_b/ecdsa/verification.md)

*1 struct, 6 statics*

### [`endian`](endian.md)

*1 struct*

### [`error`](error.md)

*2 structs*

### [`hkdf`](hkdf.md)

*1 trait, 4 statics, 4 structs*

### [`hmac`](hmac.md)

*2 functions, 4 statics, 4 structs*

### [`io::positive`](io/positive.md)

*1 struct*

### [`pbkdf2`](pbkdf2.md)

*1 struct, 2 functions, 4 statics*

### [`pkcs8`](pkcs8.md)

*1 struct*

### [`rand`](rand.md)

*1 function, 2 structs, 2 traits*

### [`rand::sealed`](rand/sealed.md)

*2 traits*

### [`rsa`](rsa.md)

*1 struct*

### [`rsa::keypair`](rsa/keypair.md)

*1 struct*

### [`rsa::keypair_components`](rsa/keypair_components.md)

*1 struct*

### [`rsa::padding`](rsa/padding.md)

*2 traits*

### [`rsa::padding::pkcs1`](rsa/padding/pkcs1.md)

*1 struct, 3 statics*

### [`rsa::padding::pss`](rsa/padding/pss.md)

*1 struct, 3 statics*

### [`rsa::public_key`](rsa/public_key.md)

*1 struct*

### [`rsa::public_key_components`](rsa/public_key_components.md)

*1 struct*

### [`rsa::verification`](rsa/verification.md)

*11 statics*

### [`sealed`](sealed.md)

*1 trait*

### [`signature`](signature.md)

*1 type alias, 2 structs, 2 traits*

### [`test`](test.md)

*2 structs, 7 functions*

