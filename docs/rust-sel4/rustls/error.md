**rustls > error**

# Module: error

## Contents

**Enums**

- [`CertRevocationListError`](#certrevocationlisterror) - The ways in which a certificate revocation list (CRL) can be invalid.
- [`CertificateError`](#certificateerror) - The ways in which certificate validators can express errors.
- [`EncryptedClientHelloError`](#encryptedclienthelloerror) - An error that occurred while handling Encrypted Client Hello (ECH).
- [`Error`](#error) - rustls reports protocol errors using this type.
- [`ExtendedKeyPurpose`](#extendedkeypurpose) - Extended Key Usage (EKU) purpose values.
- [`InconsistentKeys`](#inconsistentkeys) - Specific failure cases from [`keys_match`] or a [`crate::crypto::signer::SigningKey`] that cannot produce a corresponding public key.
- [`InvalidMessage`](#invalidmessage) - A corrupt TLS message payload that resulted in an error.
- [`PeerIncompatible`](#peerincompatible) - The set of cases where we failed to make a connection because a peer
- [`PeerMisbehaved`](#peermisbehaved) - The set of cases where we failed to make a connection because we thought

---

## rustls::error::CertRevocationListError

*Enum*

The ways in which a certificate revocation list (CRL) can be invalid.

**Variants:**
- `BadSignature` - The CRL had a bad signature from its issuer.
- `UnsupportedSignatureAlgorithm` - The CRL had an unsupported signature from its issuer.
- `UnsupportedSignatureAlgorithmContext{ signature_algorithm_id: alloc::vec::Vec<u8>, supported_algorithms: alloc::vec::Vec<pki_types::AlgorithmIdentifier> }` - A signature inside a certificate or on a handshake was made with an unsupported algorithm.
- `UnsupportedSignatureAlgorithmForPublicKeyContext{ signature_algorithm_id: alloc::vec::Vec<u8>, public_key_algorithm_id: alloc::vec::Vec<u8> }` - A signature was made with an algorithm that doesn't match the relevant public key.
- `InvalidCrlNumber` - The CRL contained an invalid CRL number.
- `InvalidRevokedCertSerialNumber` - The CRL contained a revoked certificate with an invalid serial number.
- `IssuerInvalidForCrl` - The CRL issuer does not specify the cRLSign key usage.
- `Other(OtherError)` - The CRL is invalid for some other reason.
- `ParseError` - The CRL is not correctly encoded.
- `UnsupportedCrlVersion` - The CRL is not a v2 X.509 CRL.
- `UnsupportedCriticalExtension` - The CRL, or a revoked certificate in the CRL, contained an unsupported critical extension.
- `UnsupportedDeltaCrl` - The CRL is an unsupported delta CRL, containing only changes relative to another CRL.
- `UnsupportedIndirectCrl` - The CRL is an unsupported indirect CRL, containing revoked certificates issued by a CA
- `UnsupportedRevocationReason` - The CRL contained a revoked certificate with an unsupported revocation reason.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CertRevocationListError`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::error::CertificateError

*Enum*

The ways in which certificate validators can express errors.

Note that the rustls TLS protocol code interprets specifically these
error codes to send specific TLS alerts.  Therefore, if a
custom certificate validator uses incorrect errors the library as
a whole will send alerts that do not match the standard (this is usually
a minor issue, but could be misleading).

**Variants:**
- `BadEncoding` - The certificate is not correctly encoded.
- `Expired` - The current time is after the `notAfter` time in the certificate.
- `ExpiredContext{ time: pki_types::UnixTime, not_after: pki_types::UnixTime }` - The current time is after the `notAfter` time in the certificate.
- `NotValidYet` - The current time is before the `notBefore` time in the certificate.
- `NotValidYetContext{ time: pki_types::UnixTime, not_before: pki_types::UnixTime }` - The current time is before the `notBefore` time in the certificate.
- `Revoked` - The certificate has been revoked.
- `UnhandledCriticalExtension` - The certificate contains an extension marked critical, but it was
- `UnknownIssuer` - The certificate chain is not issued by a known root certificate.
- `UnknownRevocationStatus` - The certificate's revocation status could not be determined.
- `ExpiredRevocationList` - The certificate's revocation status could not be determined, because the CRL is expired.
- `ExpiredRevocationListContext{ time: pki_types::UnixTime, next_update: pki_types::UnixTime }` - The certificate's revocation status could not be determined, because the CRL is expired.
- `BadSignature` - A certificate is not correctly signed by the key of its alleged
- `UnsupportedSignatureAlgorithm` - A signature inside a certificate or on a handshake was made with an unsupported algorithm.
- `UnsupportedSignatureAlgorithmContext{ signature_algorithm_id: alloc::vec::Vec<u8>, supported_algorithms: alloc::vec::Vec<pki_types::AlgorithmIdentifier> }` - A signature inside a certificate or on a handshake was made with an unsupported algorithm.
- `UnsupportedSignatureAlgorithmForPublicKeyContext{ signature_algorithm_id: alloc::vec::Vec<u8>, public_key_algorithm_id: alloc::vec::Vec<u8> }` - A signature was made with an algorithm that doesn't match the relevant public key.
- `NotValidForName` - The subject names in an end-entity certificate do not include
- `NotValidForNameContext{ expected: pki_types::ServerName<'static>, presented: alloc::vec::Vec<alloc::string::String> }` - The subject names in an end-entity certificate do not include
- `InvalidPurpose` - The certificate is being used for a different purpose than allowed.
- `InvalidPurposeContext{ required: ExtendedKeyPurpose, presented: alloc::vec::Vec<ExtendedKeyPurpose> }` - The certificate is being used for a different purpose than allowed.
- `InvalidOcspResponse` - The OCSP response provided to the verifier was invalid.
- `ApplicationVerificationFailure` - The certificate is valid, but the handshake is rejected for other
- `Other(OtherError)` - Any other error.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CertificateError`



## rustls::error::EncryptedClientHelloError

*Enum*

An error that occurred while handling Encrypted Client Hello (ECH).

**Variants:**
- `InvalidConfigList` - The provided ECH configuration list was invalid.
- `NoCompatibleConfig` - No compatible ECH configuration.
- `SniRequired` - The client configuration has server name indication (SNI) disabled.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> EncryptedClientHelloError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &EncryptedClientHelloError) -> bool`



## rustls::error::Error

*Enum*

rustls reports protocol errors using this type.

**Variants:**
- `InappropriateMessage{ expect_types: alloc::vec::Vec<crate::enums::ContentType>, got_type: crate::enums::ContentType }` - We received a TLS message that isn't valid right now.
- `InappropriateHandshakeMessage{ expect_types: alloc::vec::Vec<crate::enums::HandshakeType>, got_type: crate::enums::HandshakeType }` - We received a TLS handshake message that isn't valid right now.
- `InvalidEncryptedClientHello(EncryptedClientHelloError)` - An error occurred while handling Encrypted Client Hello (ECH).
- `InvalidMessage(InvalidMessage)` - The peer sent us a TLS message with invalid contents.
- `NoCertificatesPresented` - The peer didn't give us any certificates.
- `UnsupportedNameType` - The certificate verifier doesn't support the given type of name.
- `DecryptError` - We couldn't decrypt a message.  This is invariably fatal.
- `EncryptError` - We couldn't encrypt a message because it was larger than the allowed message size.
- `PeerIncompatible(PeerIncompatible)` - The peer doesn't support a protocol version/feature we require.
- `PeerMisbehaved(PeerMisbehaved)` - The peer deviated from the standard TLS protocol.
- `AlertReceived(crate::enums::AlertDescription)` - We received a fatal alert.  This means the peer is unhappy.
- `InvalidCertificate(CertificateError)` - We saw an invalid certificate.
- `InvalidCertRevocationList(CertRevocationListError)` - A provided certificate revocation list (CRL) was invalid.
- `General(alloc::string::String)` - A catch-all error for unlikely errors.
- `FailedToGetCurrentTime` - We failed to figure out what time it currently is.
- `FailedToGetRandomBytes` - We failed to acquire random bytes from the system.
- `HandshakeNotComplete` - This function doesn't work until the TLS handshake
- `PeerSentOversizedRecord` - The peer sent an oversized record/fragment.
- `NoApplicationProtocol` - An incoming connection did not support any known application protocol.
- `BadMaxFragmentSize` - The `max_fragment_size` value supplied in configuration was too small,
- `InconsistentKeys(InconsistentKeys)` - Specific failure cases from [`keys_match`] or a [`crate::crypto::signer::SigningKey`] that cannot produce a corresponding public key.
- `Other(OtherError)` - Any other error.

**Trait Implementations:**

- **From**
  - `fn from(e: CertificateError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **From**
  - `fn from(e: PeerMisbehaved) -> Self`
- **From**
  - `fn from(e: CertRevocationListError) -> Self`
- **From**
  - `fn from(_: rand::GetRandomFailed) -> Self`
- **From**
  - `fn from(e: InconsistentKeys) -> Self`
- **From**
  - `fn from(e: PeerIncompatible) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(e: EncryptedClientHelloError) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **From**
  - `fn from(value: UnsupportedOperationError) -> Self`
- **From**
  - `fn from(value: OtherError) -> Self`
- **From**
  - `fn from(e: InvalidMessage) -> Self`



## rustls::error::ExtendedKeyPurpose

*Enum*

Extended Key Usage (EKU) purpose values.

These are usually represented as OID values in the certificate's extension (if present), but
we represent the values that are most relevant to rustls as named enum variants.

**Variants:**
- `ClientAuth` - Client authentication
- `ServerAuth` - Server authentication
- `Other(alloc::vec::Vec<usize>)` - Other EKU values

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ExtendedKeyPurpose`
- **PartialEq**
  - `fn eq(self: &Self, other: &ExtendedKeyPurpose) -> bool`



## rustls::error::InconsistentKeys

*Enum*

Specific failure cases from [`keys_match`] or a [`crate::crypto::signer::SigningKey`] that cannot produce a corresponding public key.

[`keys_match`]: crate::crypto::signer::CertifiedKey::keys_match

**Variants:**
- `KeyMismatch` - The public key returned by the [`SigningKey`] does not match the public key information in the certificate.
- `Unknown` - The [`SigningKey`] cannot produce its corresponding public key.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &InconsistentKeys) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> InconsistentKeys`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::error::InvalidMessage

*Enum*

A corrupt TLS message payload that resulted in an error.

**Variants:**
- `CertificatePayloadTooLarge` - A certificate payload exceeded rustls's 64KB limit
- `HandshakePayloadTooLarge` - An advertised message was larger then expected.
- `InvalidCcs` - The peer sent us a syntactically incorrect ChangeCipherSpec payload.
- `InvalidContentType` - An unknown content type was encountered during message decoding.
- `InvalidCertificateStatusType` - A peer sent an invalid certificate status type
- `InvalidCertRequest` - Context was incorrectly attached to a certificate request during a handshake.
- `InvalidDhParams` - A peer's DH params could not be decoded
- `InvalidEmptyPayload` - A message was zero-length when its record kind forbids it.
- `InvalidKeyUpdate` - A peer sent an unexpected key update request.
- `InvalidServerName` - A peer's server name could not be decoded
- `MessageTooLarge` - A TLS message payload was larger then allowed by the specification.
- `MessageTooShort` - Message is shorter than the expected length
- `MissingData(&'static str)` - Missing data for the named handshake payload value
- `MissingKeyExchange` - A peer did not advertise its supported key exchange groups.
- `NoSignatureSchemes` - A peer sent an empty list of signature schemes
- `TrailingData(&'static str)` - Trailing data found for the named handshake payload value
- `UnexpectedMessage(&'static str)` - A peer sent an unexpected message type.
- `UnknownProtocolVersion` - An unknown TLS protocol was encountered during message decoding.
- `UnsupportedCompression` - A peer sent a non-null compression method.
- `UnsupportedCurveType` - A peer sent an unknown elliptic curve type.
- `UnsupportedKeyExchangeAlgorithm(crate::msgs::handshake::KeyExchangeAlgorithm)` - A peer sent an unsupported key exchange algorithm.
- `EmptyTicketValue` - A server sent an empty ticket
- `IllegalEmptyList(&'static str)` - A peer sent an empty list of items, but a non-empty list is required.
- `IllegalEmptyValue` - A peer sent an empty value, but a non-empty value is required.
- `DuplicateExtension(u16)` - A peer sent a message where a given extension type was repeated
- `PreSharedKeyIsNotFinalExtension` - A peer sent a message with a PSK offer extension in wrong position
- `UnknownHelloRetryRequestExtension` - A server sent a HelloRetryRequest with an unknown extension
- `UnknownCertificateExtension` - The peer sent a TLS1.3 Certificate with an unknown extension

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> InvalidMessage`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &InvalidMessage) -> bool`



## rustls::error::PeerIncompatible

*Enum*

The set of cases where we failed to make a connection because a peer
doesn't support a TLS version/feature we require.

This is `non_exhaustive`: we might add or stop using items here in minor
versions.

**Variants:**
- `EcPointsExtensionRequired`
- `ExtendedMasterSecretExtensionRequired`
- `IncorrectCertificateTypeExtension`
- `KeyShareExtensionRequired`
- `NamedGroupsExtensionRequired`
- `NoCertificateRequestSignatureSchemesInCommon`
- `NoCipherSuitesInCommon`
- `NoEcPointFormatsInCommon`
- `NoKxGroupsInCommon`
- `NoSignatureSchemesInCommon`
- `NullCompressionRequired`
- `ServerDoesNotSupportTls12Or13`
- `ServerSentHelloRetryRequestWithUnknownExtension`
- `ServerTlsVersionIsDisabledByOurConfig`
- `SignatureAlgorithmsExtensionRequired`
- `SupportedVersionsExtensionRequired`
- `Tls12NotOffered`
- `Tls12NotOfferedOrEnabled`
- `Tls13RequiredForQuic`
- `UncompressedEcPointsRequired`
- `UnsolicitedCertificateTypeExtension`
- `ServerRejectedEncryptedClientHello(Option<alloc::vec::Vec<crate::msgs::handshake::EchConfigPayload>>)`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PeerIncompatible) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> PeerIncompatible`



## rustls::error::PeerMisbehaved

*Enum*

The set of cases where we failed to make a connection because we thought
the peer was misbehaving.

This is `non_exhaustive`: we might add or stop using items here in minor
versions.  We also don't document what they mean.  Generally a user of
rustls shouldn't vary its behaviour on these error codes, and there is
nothing it can do to improve matters.

Please file a bug against rustls if you see `Error::PeerMisbehaved` in
the wild.

**Variants:**
- `AttemptedDowngradeToTls12WhenTls13IsSupported`
- `BadCertChainExtensions`
- `DisallowedEncryptedExtension`
- `DuplicateClientHelloExtensions`
- `DuplicateEncryptedExtensions`
- `DuplicateHelloRetryRequestExtensions`
- `DuplicateNewSessionTicketExtensions`
- `DuplicateServerHelloExtensions`
- `DuplicateServerNameTypes`
- `EarlyDataAttemptedInSecondClientHello`
- `EarlyDataExtensionWithoutResumption`
- `EarlyDataOfferedWithVariedCipherSuite`
- `HandshakeHashVariedAfterRetry`
- `IllegalHelloRetryRequestWithEmptyCookie`
- `IllegalHelloRetryRequestWithNoChanges`
- `IllegalHelloRetryRequestWithOfferedGroup`
- `IllegalHelloRetryRequestWithUnofferedCipherSuite`
- `IllegalHelloRetryRequestWithUnofferedNamedGroup`
- `IllegalHelloRetryRequestWithUnsupportedVersion`
- `IllegalHelloRetryRequestWithWrongSessionId`
- `IllegalHelloRetryRequestWithInvalidEch`
- `IllegalMiddleboxChangeCipherSpec`
- `IllegalTlsInnerPlaintext`
- `IncorrectBinder`
- `InvalidCertCompression`
- `InvalidMaxEarlyDataSize`
- `InvalidKeyShare`
- `KeyEpochWithPendingFragment`
- `KeyUpdateReceivedInQuicConnection`
- `MessageInterleavedWithHandshakeMessage`
- `MissingBinderInPskExtension`
- `MissingKeyShare`
- `MissingPskModesExtension`
- `MissingQuicTransportParameters`
- `OfferedDuplicateCertificateCompressions`
- `OfferedDuplicateKeyShares`
- `OfferedEarlyDataWithOldProtocolVersion`
- `OfferedEmptyApplicationProtocol`
- `OfferedIncorrectCompressions`
- `PskExtensionMustBeLast`
- `PskExtensionWithMismatchedIdsAndBinders`
- `RefusedToFollowHelloRetryRequest`
- `RejectedEarlyDataInterleavedWithHandshakeMessage`
- `ResumptionAttemptedWithVariedEms`
- `ResumptionOfferedWithVariedCipherSuite`
- `ResumptionOfferedWithVariedEms`
- `ResumptionOfferedWithIncompatibleCipherSuite`
- `SelectedDifferentCipherSuiteAfterRetry`
- `SelectedInvalidPsk`
- `SelectedTls12UsingTls13VersionExtension`
- `SelectedUnofferedApplicationProtocol`
- `SelectedUnofferedCertCompression`
- `SelectedUnofferedCipherSuite`
- `SelectedUnofferedCompression`
- `SelectedUnofferedKxGroup`
- `SelectedUnofferedPsk`
- `SelectedUnusableCipherSuiteForVersion`
- `ServerEchoedCompatibilitySessionId`
- `ServerHelloMustOfferUncompressedEcPoints`
- `ServerNameDifferedOnRetry`
- `ServerNameMustContainOneHostName`
- `SignedKxWithWrongAlgorithm`
- `SignedHandshakeWithUnadvertisedSigScheme`
- `TooManyEmptyFragments`
- `TooManyKeyUpdateRequests`
- `TooManyRenegotiationRequests`
- `TooManyWarningAlertsReceived`
- `TooMuchEarlyDataReceived`
- `UnexpectedCleartextExtension`
- `UnsolicitedCertExtension`
- `UnsolicitedEncryptedExtension`
- `UnsolicitedSctList`
- `UnsolicitedServerHelloExtension`
- `WrongGroupForKeyShare`
- `UnsolicitedEchExtension`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PeerMisbehaved`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PeerMisbehaved) -> bool`



