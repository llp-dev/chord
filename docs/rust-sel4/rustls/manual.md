**rustls > manual**

# Module: manual

## Contents

**Modules**

- [`_01_impl_vulnerabilities`](#_01_impl_vulnerabilities) -  This section discusses vulnerabilities in other TLS implementations, theorising their
- [`_02_tls_vulnerabilities`](#_02_tls_vulnerabilities) -  This section discusses vulnerabilities and design errors in the TLS protocol.
- [`_03_howto`](#_03_howto) -  This section collects together goal-oriented documentation.
- [`_04_features`](#_04_features) -  This section documents rustls itself: what protocol features are and are not implemented.
- [`_05_defaults`](#_05_defaults) -  This section provides rationale for the defaults in rustls.
- [`_06_fips`](#_06_fips) -  This section provides guidance on using rustls with FIPS-approved cryptography.

---

## Module: _01_impl_vulnerabilities

 This section discusses vulnerabilities in other TLS implementations, theorising their
 root cause and how we aim to avoid them in rustls.
 # A review of TLS Implementation Vulnerabilities

An important part of engineering involves studying and learning from the mistakes of the past.
It would be tremendously unfortunate to spend effort re-discovering and re-fixing the same
vulnerabilities that were discovered in the past.

## Memory safety

Being written entirely in the safe-subset of Rust immediately offers us freedom from the entire
class of memory safety vulnerabilities.  There are too many to exhaustively list, and there will
certainly be more in the future.

Examples:

- Heartbleed [CVE-2014-0160](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2014-0160) (OpenSSL)
- Memory corruption in ASN.1 decoder [CVE-2016-2108](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2016-2108) (OpenSSL)
- Buffer overflow in read_server_hello [CVE-2014-3466](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2014-3466) (GnuTLS)

## `goto fail`

This is the name of a vulnerability in Apple Secure Transport [CVE-2014-1266](https://nvd.nist.gov/vuln/detail/CVE-2014-1266).
This boiled down to the following code, which validates the server's signature on the key exchange:

```c
    if ((err = SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
        goto fail;
    if ((err = SSLHashSHA1.update(&hashCtx, &signedParams)) != 0)
        goto fail;
>       goto fail;
    if ((err = SSLHashSHA1.final(&hashCtx, &hashOut)) != 0)
        goto fail;
```

The marked line was duplicated, likely accidentally during a merge.  This meant
the remaining part of the function (including the actual signature validation)
was unconditionally skipped.

Ultimately the one countermeasure to this type of bug is basic testing: that a
valid signature returns success, and that an invalid one does not.  rustls
has such testing, but this is really table stakes for security code.

Further than this, though, we could consider that the *lack* of an error from
this function is a poor indicator that the signature was valid.  rustls, instead,
has zero-size and non-copyable types that indicate a particular signature validation
has been performed.  These types can be thought of as *capabilities* originated only
by designated signature verification functions -- such functions can then be a focus
of manual code review.  Like capabilities, values of these types are otherwise unforgeable,
and are communicable only by Rust's move semantics.

Values of these types are threaded through the protocol state machine, leading to terminal
states that look like:

```ignore
struct ExpectTraffic {
   (...)
    _cert_verified: verify::ServerCertVerified,
    _sig_verified: verify::HandshakeSignatureValid,
    _fin_verified: verify::FinishedMessageVerified,
}
```

Since this state requires a value of these types, it will be a compile-time error to
reach that state without performing the requisite security-critical operations.

This approach is not infallible, but it has zero runtime cost.

## State machine attacks: EarlyCCS and SMACK/SKIP/FREAK

EarlyCCS [CVE-2014-0224](https://nvd.nist.gov/vuln/detail/CVE-2014-0224) was a vulnerability in OpenSSL
found in 2014.  The TLS `ChangeCipherSpec` message would be processed at inappropriate times, leading
to data being encrypted with the wrong keys (specifically, keys which were not secret).  This resulted
from OpenSSL taking a *reactive* strategy to incoming messages ("when I get a message X, I should do Y")
which allows it to diverge from the proper state machine under attacker control.

[SMACK](https://mitls.org/pages/attacks/SMACK) is a similar suite of vulnerabilities found in JSSE,
CyaSSL, OpenSSL, Mono and axTLS.  "SKIP-TLS" demonstrated that some implementations allowed handshake
messages (and in one case, the entire handshake!) to be skipped leading to breaks in security.  "FREAK"
found that some implementations incorrectly allowed export-only state transitions (i.e., transitions that
were only valid when an export ciphersuite was in use).

rustls represents its protocol state machine carefully to avoid these defects.  We model the handshake,
CCS and application data subprotocols in the same single state machine.  Each state in this machine is
represented with a single struct, and transitions are modelled as functions that consume the current state
plus one TLS message[^1] and return a struct representing the next state.  These functions fully validate
the message type before further operations.

A sample sequence for a full TLSv1.2 handshake by a client looks like:

- `hs::ExpectServerHello` (Note: ClientHello is logically sent before this state); transition to `tls12::ExpectCertificate`
- `tls12::ExpectCertificate`; transition to `tls12::ExpectServerKX`
- `tls12::ExpectServerKX`; transition to `tls12::ExpectServerDoneOrCertReq`
- `tls12::ExpectServerDoneOrCertReq`; delegates to `tls12::ExpectCertificateRequest` or `tls12::ExpectServerDone` depending on incoming message.
  - `tls12::ExpectServerDone`; transition to `tls12::ExpectCCS`
- `tls12::ExpectCCS`; transition to `tls12::ExpectFinished`
- `tls12::ExpectFinished`; transition to `tls12::ExpectTraffic`
- `tls12::ExpectTraffic`; terminal state; transitions to `tls12::ExpectTraffic`

In the future we plan to formally prove that all possible transitions modelled in this system of types
are correct with respect to the standard(s).  At the moment we rely merely on exhaustive testing.

[^1]: a logical TLS message: post-decryption, post-fragmentation.





## Module: _02_tls_vulnerabilities

 This section discusses vulnerabilities and design errors in the TLS protocol.
 # A review of protocol vulnerabilities

## CBC MAC-then-encrypt ciphersuites

Back in 2000 [Bellare and Namprempre](https://eprint.iacr.org/2000/025) discussed how to make authenticated
encryption by composing separate encryption and authentication primitives.  That paper included this table:

| Composition Method | Privacy | | | Integrity | |
|--------------------|---------|-|-|-----------|-|
|| IND-CPA | IND-CCA | NM-CPA | INT-PTXT | INT-CTXT |
| Encrypt-and-MAC | insecure | insecure | insecure | secure | insecure |
| MAC-then-encrypt | secure | insecure | insecure | secure | insecure |
| Encrypt-then-MAC | secure | secure | secure | secure | secure |

One may assume from this fairly clear result that encrypt-and-MAC and MAC-then-encrypt compositions would be quickly abandoned
in favour of the remaining proven-secure option.  But that didn't happen, not in TLSv1.1 (2006) nor in TLSv1.2 (2008).  Worse,
both RFCs included incorrect advice on countermeasures for implementers, suggesting that the flaw was "not believed to be large
enough to be exploitable".

[Lucky 13](http://www.isg.rhul.ac.uk/tls/Lucky13.html) (2013) exploited this flaw and affected all implementations, including
those written [after discovery](https://aws.amazon.com/blogs/security/s2n-and-lucky-13/). OpenSSL even had a
[memory safety vulnerability in the fix for Lucky 13](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2016-2107), which
gives a flavour of the kind of complexity required to remove the side channel.

rustls does not implement CBC MAC-then-encrypt ciphersuites for these reasons.  TLSv1.3 removed support for these
ciphersuites in 2018.

There are some further rejected options worth mentioning: [RFC7366](https://tools.ietf.org/html/rfc7366) defines
Encrypt-then-MAC for TLS, but unfortunately cannot be negotiated without also supporting MAC-then-encrypt
(clients cannot express "I offer CBC, but only EtM and not MtE").

## RSA PKCS#1 encryption

"RSA key exchange" in TLS involves the client choosing a large random value and encrypting it using the server's
public key.  This has two overall problems:

1. It provides no _forward secrecy_: later compromise of the server's private key breaks confidentiality of
   *all* past sessions using that key.  This is a crucial property in the presence of software that is often
   [poor at keeping a secret](http://heartbleed.com/).
2. The padding used in practice in TLS ("PKCS#1", or fully "RSAES-PKCS1-v1_5") has been known to be broken since
   [1998](http://archiv.infsec.ethz.ch/education/fs08/secsem/bleichenbacher98.pdf).

In a similar pattern to the MAC-then-encrypt problem discussed above, TLSv1.0 (1999), TLSv1.1 (2006) and TLSv1.2 (2008)
continued to specify use of PKCS#1 encryption, again with incrementally more complex and incorrect advice on countermeasures.

[ROBOT](https://robotattack.org/) (2018) showed that implementations were still vulnerable to these attacks twenty years later.
[The Marvin Attack](https://people.redhat.com/~hkario/marvin/) (2023) demonstrated the same a further five years later.

rustls does not support RSA key exchange.  TLSv1.3 also removed support.

## BEAST

[BEAST](https://vnhacker.blogspot.com/2011/09/beast.html) ([CVE-2011-3389](https://nvd.nist.gov/vuln/detail/CVE-2011-3389))
was demonstrated in 2011 by Thai Duong and Juliano Rizzo,
and was another vulnerability in CBC-based ciphersuites in SSLv3.0 and TLSv1.0.  CBC mode is vulnerable to adaptive
chosen-plaintext attacks if the IV is predictable.  In the case of these protocol versions, the IV was the previous
block of ciphertext (as if the entire TLS session was one CBC ciphertext, albeit revealed incrementally).  This was
obviously predictable, since it was published on the wire.

OpenSSL contained a countermeasure for this problem from 2002 onwards: it encrypts an empty message before each real
one, so that the IV used in the real message is unpredictable.  This was turned off by default due to bugs in IE6.

TLSv1.1 fix this vulnerability, but not any of the other deficiencies of CBC mode (see above).

rustls does not support these ciphersuites.

## CRIME

In 2002 [John Kelsey](https://www.iacr.org/cryptodb/archive/2002/FSE/3091/3091.pdf) discussed the length side channel
as applied to compression of combined secret and attacker-chosen strings.

Compression continued to be an option in TLSv1.1 (2006) and in TLSv1.2 (2008).  Support in libraries was widespread.

[CRIME](https://en.wikipedia.org/wiki/CRIME) ([CVE-2012-4929](https://nvd.nist.gov/vuln/detail/CVE-2012-4929))
was demonstrated in 2012, again by Thai Duong and Juliano Rizzo.  It attacked several protocols offering transparent
compression of application data, allowing quick adaptive chosen-plaintext attacks against secret values like cookies.

rustls does not implement compression.  TLSv1.3 also removed support.

## Logjam / FREAK

Way back when SSL was first being born, circa 1995, the US government considered cryptography a munition requiring
export control.  SSL contained specific ciphersuites with dramatically small key sizes that were not subject
to export control.  These controls were dropped in 2000.

Since the "export-grade" ciphersuites no longer fulfilled any purpose, and because they were actively harmful to users,
one may have expected software support to disappear quickly. This did not happen.

In 2015 [the FREAK attack](https://mitls.org/pages/attacks/SMACK#freak) ([CVE-2015-0204](https://nvd.nist.gov/vuln/detail/CVE-2015-0204))
and [the Logjam attack](https://weakdh.org/) ([CVE-2015-4000](https://nvd.nist.gov/vuln/detail/CVE-2015-4000)) both
demonstrated total breaks of security in the presence of servers that accepted export ciphersuites.  FREAK factored
512-bit RSA keys, while Logjam optimised solving discrete logs in the 512-bit group used by many different servers.

Naturally, rustls does not implement any of these ciphersuites.

## SWEET32

Block ciphers are vulnerable to birthday attacks, where the probability of repeating a block increases dramatically
once a particular key has been used for many blocks.  For block ciphers with 64-bit blocks, this becomes probable
once a given key encrypts the order of 32GB of data.

[Sweet32](https://sweet32.info/) ([CVE-2016-2183](https://nvd.nist.gov/vuln/detail/CVE-2016-2183)) attacked this fact
in the context of TLS support for 3DES, breaking confidentiality by analysing a large amount of attacker-induced traffic
in one session.

rustls does not support any 64-bit block ciphers.

## DROWN

[DROWN](https://drownattack.com/) ([CVE-2016-0800](https://nvd.nist.gov/vuln/detail/CVE-2016-0800)) is a cross-protocol
attack that breaks the security of TLSv1.2 and earlier (when used with RSA key exchange) by using SSLv2.  It is required
that the server uses the same key for both protocol versions.

rustls naturally does not support SSLv2, but most importantly does not support RSA key exchange for TLSv1.2.

## Poodle

[POODLE](https://cdn1.vox-cdn.com/uploads/chorus_asset/file/2354994/ssl-poodle.0.pdf) ([CVE-2014-3566](https://nvd.nist.gov/vuln/detail/CVE-2014-3566))
is an attack against CBC mode ciphersuites in SSLv3.  This was possible in most cases because some clients willingly
downgraded to SSLv3 after failed handshakes for later versions.

rustls does not support CBC mode ciphersuites, or SSLv3.  Note that rustls does not need to implement `TLS_FALLBACK_SCSV`
introduced as a countermeasure because it contains no ability to downgrade from TLS 1.2 to earlier protocol versions,
and TLS 1.3 has protocol-level downgrade protection based on the [ServerHello server random value](https://www.rfc-editor.org/rfc/rfc8446#section-4.1.3).

## GCM nonces

[RFC5288](https://tools.ietf.org/html/rfc5288) introduced GCM-based ciphersuites for use in TLS.  Unfortunately
the design was poor; it reused design for an unrelated security setting proposed in RFC5116.

GCM is a typical nonce-based AEAD: it requires a unique (but not necessarily unpredictable) 96-bit nonce for each encryption
with a given key.  The design specified by RFC5288 left two-thirds of the nonce construction up to implementations:

- wasting 8 bytes per TLS ciphertext,
- meaning correct operation cannot be tested for (e.g., in protocol-level test vectors).

There were no trade-offs here: TLS has a 64-bit sequence number that is not allowed to wrap and would make an ideal nonce.

As a result, a [2016 study](https://eprint.iacr.org/2016/475.pdf) found:

- implementations from IBM, A10 and Citrix used randomly-chosen nonces, which are unlikely to be unique over long connections,
- an implementation from Radware used the same nonce for the first two messages.

rustls uses a counter from a random starting point for GCM nonces.  TLSv1.3 and the Chacha20-Poly1305 TLSv1.2 ciphersuite
standardise this method.

## Renegotiation

In 2009 Marsh Ray and Steve Dispensa [discovered](https://kryptera.se/Renegotiating%20TLS.pdf) that the renegotiation
feature of all versions of TLS allows a MitM to splice a request of their choice onto the front of the client's real HTTP
request.  A countermeasure was proposed and widely implemented to bind renegotiations to their previous negotiations;
unfortunately this was insufficient.

rustls does not support renegotiation in TLSv1.2.  TLSv1.3 also no longer supports renegotiation.

## 3SHAKE

[3SHAKE](https://www.mitls.org/pages/attacks/3SHAKE) (2014) described a complex attack that broke the "Secure Renegotiation" extension
introduced as a countermeasure to the previous protocol flaw.

rustls does not support renegotiation for TLSv1.2 connections, or RSA key exchange, and both are required for this attack
to work.  rustls implements the "Extended Master Secret" (RFC7627) extension for TLSv1.2 which was standardised as a countermeasure.

TLSv1.3 no longer supports renegotiation and RSA key exchange.  It also effectively incorporates the improvements made in RFC7627.

## KCI

[This vulnerability](https://kcitls.org/) makes use of TLS ciphersuites (those offering static DH) which were standardised
yet not widely used. However, they were implemented by libraries, and as a result enabled for various clients.  It coupled
this with misconfigured certificates (on services including facebook.com) which allowed their misuse to MitM connections.

rustls does not support static DH/EC-DH ciphersuites.  We assert that it is misissuance to sign an EC certificate
with the keyUsage extension allowing both signatures and key exchange.  That it isn't is probably a failure
of CAB Forum baseline requirements.



## Module: _03_howto

 This section collects together goal-oriented documentation.
 # Customising private key usage

By default rustls supports PKCS#8-format[^1] RSA or ECDSA keys, plus PKCS#1-format RSA keys.

However, if your private key resides in a HSM, or in another process, or perhaps
another machine, rustls has some extension points to support this:

The main trait you must implement is [`sign::SigningKey`][signing_key]. The primary method here
is [`choose_scheme()`][choose_scheme] where you are given a set of [`SignatureScheme`s][sig_scheme] the client says
it supports: you must choose one (or return `None` -- this aborts the handshake). Having
done that, you return an implementation of the [`sign::Signer`][signer] trait.
The [`sign()`][sign_method] performs the signature and returns it.

(Unfortunately this is currently designed for keys with low latency access, like in a
PKCS#11 provider, Microsoft CryptoAPI, etc. so is blocking rather than asynchronous.
It's a TODO to make these and other extension points async.)

Once you have these two pieces, configuring a server to use them involves, briefly:

- packaging your [`sign::SigningKey`][signing_key] with the matching certificate chain into a [`sign::CertifiedKey`][certified_key]
- making a [`ResolvesServerCertUsingSni`][cert_using_sni] and feeding in your [`sign::CertifiedKey`][certified_key] for all SNI hostnames you want to use it for,
- setting that as your `ServerConfig`'s [`cert_resolver`][cert_resolver]

For a complete example of implementing a custom [`sign::SigningKey`][signing_key] and
[`sign::Signer`][signer] see the [`signer` module in the `rustls-cng` crate][rustls-cng-signer].

[signing_key]: crate::crypto::signer::SigningKey
[choose_scheme]: crate::crypto::signer::SigningKey::choose_scheme
[sig_scheme]: crate::SignatureScheme
[signer]: crate::crypto::signer::Signer
[sign_method]: crate::crypto::signer::Signer::sign
[certified_key]: crate::crypto::signer::CertifiedKey
[cert_using_sni]: crate::server::ResolvesServerCertUsingSni
[cert_resolver]: crate::ServerConfig::cert_resolver
[rustls-cng-signer]: https://github.com/rustls/rustls-cng/blob/dev/src/signer.rs

[^1]: For PKCS#8 it does not support password encryption -- there's not a meaningful threat
      model addressed by this, and the encryption supported is typically extremely poor.

# Unexpected EOF

TLS has a `close_notify` mechanism to prevent truncation attacks[^2].
According to the TLS RFCs, each party is required to send a `close_notify` message before
closing the write side of the connection. However, some implementations don't send it.
So long as the application layer protocol (for instance HTTP/2) has message length framing
and can reject truncated messages, this is not a security problem.

Rustls treats an EOF without `close_notify` as an error of type `std::io::Error` with
`ErrorKind::UnexpectedEof`. In some situations it's appropriate for the application to handle
this error the same way it would handle a normal EOF (a read returning `Ok(0)`). In particular
if `UnexpectedEof` occurs on an idle connection it is appropriate to treat it the same way as a
clean shutdown. And if an application always uses messages with length framing (in other words,
messages are never delimited by the close of the TCP connection), it can unconditionally
ignore `UnexpectedEof` errors from rustls.

[^2]: <https://datatracker.ietf.org/doc/html/rfc8446#section-6.1>

# Debugging

If you encounter a bug with Rustls it can be helpful to collect up as much diagnostic
information as possible.

## Collecting logs

If your bug reproduces with one of the [Rustls examples] you can use the
[`RUST_LOG`] environment variable to increase the log verbosity. If you're using
your own application, you may need to configure it with a logging backend
like `env_logger`.

Consider reproducing your bug with `RUST_LOG=rustls=trace` and sharing the result
in a [GitHub gist].

[Rustls examples]: https://github.com/rustls/rustls/tree/main/examples
[`RUST_LOG`]: https://docs.rs/env_logger/latest/env_logger/#enabling-logging
[`env_logger`]: https://docs.rs/env_logger/
[GitHub gist]: https://docs.github.com/en/get-started/writing-on-github/editing-and-sharing-content-with-gists/creating-gists

## Taking a packet capture

When logs aren't enough taking a packet capture ("pcap") is another helpful tool.
The details of how to accomplish this vary by operating system/context.

### tcpdump

As one example, on Linux using [`tcpdump`] is often easiest.

If you know the IP address of the remote server your bug demonstrates with you
could take a short packet capture with this command (after replacing
`XX.XX.XX.XX` with the correct IP address):

```bash
sudo tcpdump -i any tcp and dst host XX.XX.XX.XX -C5 -w rustls.pcap
```

The `-i any` captures on any network interface. The `tcp and dst host XX.XX.XX.XX`
portion target the capture to TCP traffic to the specified IP address. The `-C5`
argument limits the capture to at most 5MB. Lastly the `-w` argument writes the
capture to `rustls.pcap`.

Another approach is to use `tcp and port XXXX` instead of `tcp and dst host XX.XX.XX.XX`
to capture all traffic to a specific port instead of a specific host server.

[`tcpdump`]: https://www.redhat.com/en/blog/introduction-using-tcpdump-linux-command-line

### SSLKEYLOGFILE

If the bug you are reporting happens after data is encrypted you may also wish to
share the secret keys required to decrypt the post-handshake traffic.

If you're using one of the [Rustls examples] you can set the `SSLKEYLOGFILE` environment
variable to a path where secrets will be written. E.g. `SSLKEYLOGFILE=rustls.pcap.keys`.

If you're using your own application you may need to customize the Rustls `ClientConfig`
or `ServerConfig`'s `key_log` setting like the example applications do.

With the file from `SSLKEYLOGFILE` it is possible to use [Wireshark] or another tool to
decrypt the post-handshake messages, following [these instructions][curl-sslkeylogfile].

Remember this allows plaintext decryption and should only be done in testing contexts
where no sensitive data (API keys, etc) are being shared.

[Wireshark]: https://www.wireshark.org/download.html
[curl-sslkeylogfile]: https://everything.curl.dev/usingcurl/tls/sslkeylogfile.html



## Module: _04_features

 This section documents rustls itself: what protocol features are and are not implemented.

The below list reflects the support provided with the default crate features.
Items marked with an asterisk `*` can be extended or altered via public
APIs ([`CryptoProvider`] for example).

[`CryptoProvider`]: crate::crypto::CryptoProvider

## Current features

* TLS1.2 and TLS1.3
* ECDSA, Ed25519 or RSA server authentication by clients `*`
* ECDSA, Ed25519[^1] or RSA server authentication by servers `*`
* Forward secrecy using ECDHE; with curve25519, nistp256 or nistp384 curves `*`
* Post-quantum hybrid key exchange with [X25519MLKEM768](https://datatracker.ietf.org/doc/draft-ietf-tls-ecdhe-mlkem/) [^2] `*`
* AES128-GCM and AES256-GCM bulk encryption, with safe nonces `*`
* ChaCha20-Poly1305 bulk encryption ([RFC7905](https://tools.ietf.org/html/rfc7905)) `*`
* ALPN support
* SNI support
* Tunable fragment size to make TLS messages match size of underlying transport
* Optional use of vectored IO to minimise system calls
* TLS1.2 session resumption
* TLS1.2 resumption via tickets ([RFC5077](https://tools.ietf.org/html/rfc5077))
* TLS1.3 resumption via tickets or session storage
* TLS1.3 0-RTT data
* Server and optional client authentication
* Extended master secret support ([RFC7627](https://tools.ietf.org/html/rfc7627))
* Exporters ([RFC5705](https://tools.ietf.org/html/rfc5705))
* OCSP stapling by servers
* [RFC7250](https://tools.ietf.org/html/rfc7250) raw public keys for TLS1.3
* [RFC8879](https://tools.ietf.org/html/rfc8879) certificate compression by clients
  and servers `*`
* Client-side Encrypted client hello (ECH)
   ([draft-ietf-tls-esni](https://datatracker.ietf.org/doc/draft-ietf-tls-esni/)).

[^1]: Note that, at the time of writing, Ed25519 does not have wide support
      in browsers.  It is also not supported by the WebPKI, because the
      CA/Browser Forum Baseline Requirements do not support it for publicly
      trusted certificates.
[^2]: See [the documentation][crate::manual::_05_defaults#about-the-post-quantum-secure-key-exchange-x25519mlkem768]

## Non-features

For reasons explained in the other sections of this manual, rustls does not
and will not support:

* SSL1, SSL2, SSL3, TLS1 or TLS1.1
* RC4
* DES or triple DES
* EXPORT ciphersuites
* MAC-then-encrypt ciphersuites
* Ciphersuites without forward secrecy
* Renegotiation
* Kerberos
* TLS 1.2 protocol compression
* Discrete-log Diffie-Hellman `*`
* Automatic protocol version downgrade
* Using CA certificates directly to authenticate a server/client (often called "self-signed
  certificates"). _Rustls' default certificate verifier does not support using a trust anchor as
  both a CA certificate and an end-entity certificate in order to limit complexity and risk in
  path building. While dangerous, all authentication can be turned off if required --
  see the [example code](https://github.com/rustls/rustls/blob/v/0.23.23/examples/src/bin/tlsclient-mio.rs#L338)_ `*`

### About "custom extensions"

OpenSSL allows an application to add arbitrary TLS extensions (via
the `SSL_CTX_add_custom_ext` function and associated APIs).  We don't
support this, with the following rationale:

Such an API is limited to extensions that are quite narrow in scope:
they cannot change the meaning of standard messages, or introduce new
messages, or make any changes to the connection's cryptography.

However, there is no reasonable way to technically limit that API to
that set of extensions.  That makes the API pretty unsafe (in the
TLS and cryptography sense, not memory safety sense).  This could
cause security or interop failures.

Instead, we suggest that potential users of that API consider:

- whether their use can fit in standard extensions such as ALPN,
  or [ALPS][alps][^3].
- if not, whether they can fit in a more general extension, and define
  and standardize that in the [IETF TLSWG][tlswg].

Note the above is not a guarantee or offer that rustls will implement
any specific extensions that are standardized by the IETF TLSWG.
It is a non-goal of this project to implement absolutely everything.

For experimentation and pre-standardization testing, we suggest
forking rustls.

See also: [Go's position on such an API][golang].

[alps]: https://datatracker.ietf.org/doc/html/draft-vvv-tls-alps
[golang]: https://github.com/golang/go/issues/51497
[tlswg]: https://datatracker.ietf.org/wg/tls/charter/
[^3]: rustls does not currently implement ALPS, but it is something we
  would consider once standardised and deployed.



## Module: _05_defaults

 This section provides rationale for the defaults in rustls.

## Rationale for defaults

### Why is AES-256 preferred over AES-128?

This is a trade-off between:

1. classical security level: searching a 2^128 key space is as implausible as 2^256.
2. post-quantum security level: the difference is more meaningful, and AES-256 seems like the conservative choice.
3. performance: AES-256 is around 40% slower than AES-128, though hardware acceleration typically narrows this gap.

The choice is frankly quite marginal.

### Why is AES-GCM preferred over chacha20-poly1305?

Hardware support for accelerating AES-GCM is widespread, and hardware-accelerated AES-GCM
is quicker than un-accelerated chacha20-poly1305.

However, if you know your application will run on a platform without that, you should
_definitely_ change the default order to prefer chacha20-poly1305: both the performance and
the implementation security will be improved.  We think this is an uncommon case.

### Why is x25519 preferred for key exchange over nistp256?

Both provide roughly the same classical security level, but x25519 has better performance and
it's _much_ more likely that both peers will have good quality implementations.

### About the post-quantum-secure key exchange `X25519MLKEM768`

[`X25519MLKEM768`] -- a hybrid[^1], post-quantum-secure[^2] key exchange
algorithm -- is available when using the aws-lc-rs provider.

The `prefer-post-quantum` crate feature makes `X25519MLKEM768` the
highest-priority key exchange algorithm.  Otherwise, it is available but
not highest-priority.

[X25519MLKEM768] is pre-standardization, but is now widely deployed,
for example, by [Chrome] and [Cloudflare].

You may see unexpected connection failures (such as [tldr.fail])
-- [please report these to us][interop-bug].

The two components of this key exchange are well regarded:
X25519 alone is already used by default by rustls, and tends to have
higher quality implementations than other elliptic curves.
ML-KEM-768 was standardized by NIST in [FIPS203].

[`MLKEM768`] is available separately, but is not currently enabled
by default out of conservatism.

[^1]: meaning: a construction that runs a classical and post-quantum
      key exchange, and uses the output of both together.  This is a hedge
      against the post-quantum half being broken.

[^2]: a "post-quantum-secure" algorithm is one posited to be invulnerable
      to attack using a cryptographically-relevant quantum computer.  In contrast,
      classical algorithms would be broken by such a computer.  Note that such computers
      do not currently exist, and may never exist, but current traffic could be captured
      now and attacked later.

[X25519MLKEM768]: <https://datatracker.ietf.org/doc/draft-ietf-tls-ecdhe-mlkem/>
[`X25519MLKEM768`]: crate::crypto::aws_lc_rs::kx_group::X25519MLKEM768
[`MLKEM768`]: crate::crypto::aws_lc_rs::kx_group::MLKEM768
[FIPS203]: <https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf>
[Chrome]: <https://security.googleblog.com/2024/09/a-new-path-for-kyber-on-web.html>
[Cloudflare]: <https://blog.cloudflare.com/pq-2024/#ml-kem-768-and-x25519>
[interop-bug]: <https://github.com/rustls/rustls/issues/new?assignees=&labels=&projects=&template=bug_report.md&title=>
[tldr.fail]: <https://tldr.fail/>



## Module: _06_fips

 This section provides guidance on using rustls with FIPS-approved cryptography.
 # Using rustls with FIPS-approved cryptography

To use FIPS-approved cryptography with rustls, you should take
these actions:

## 1. Enable the `fips` crate feature for rustls.

Use:

```toml
rustls = { version = "0.23", features = [ "fips" ] }
```

## 2. Use the FIPS `CryptoProvider`

This is [`default_fips_provider()`]:

```rust,ignore
rustls::crypto::default_fips_provider()
    .install_default()
    .expect("default provider already set elsewhere");
```

This snippet makes use of the process-default provider,
and that assumes all your uses of rustls use that.
See [`CryptoProvider`] documentation for other ways to
specify which `CryptoProvider` to use.

## 3. Validate the FIPS status of your `ClientConfig`/`ServerConfig` at run-time

See [`ClientConfig::fips()`] or [`ServerConfig::fips()`].

You could, for example:

```rust,ignore
# let client_config = unreachable!();
assert!(client_config.fips());
```

But maybe your application has an error handling
or health-check strategy better than panicking.

# aws-lc-rs FIPS approval status

This is covered by [FIPS 140-3 certificate #4816][cert-4816].
See [the security policy][policy-4816] for precisely which
environments and functions this certificate covers.

Later releases of aws-lc-rs may be covered by later certificates,
or be pending certification.

For the most up-to-date details see the latest documentation
for the [`aws-lc-fips-sys`] crate.

[`aws-lc-fips-sys`]: https://crates.io/crates/aws-lc-fips-sys
[`default_fips_provider()`]: crate::crypto::default_fips_provider
[`CryptoProvider`]: crate::crypto::CryptoProvider
[`ClientConfig::fips()`]: crate::client::ClientConfig::fips
[`ServerConfig::fips()`]: crate::server::ServerConfig::fips
[cert-4816]: https://csrc.nist.gov/projects/cryptographic-module-validation-program/certificate/4816
[policy-4816]: https://csrc.nist.gov/CSRC/media/projects/cryptographic-module-validation-program/documents/security-policies/140sp4816.pdf



