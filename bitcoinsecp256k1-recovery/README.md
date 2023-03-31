## bitcoinsecp256k1-recovery

### Recover Bitcoin secp256k1 ECDSA Signatures

This Rust crate provides functions for recovering
the public key used to sign a message with an
ECDSA signature on the secp256k1 elliptic curve,
which is used in the Bitcoin system.

The following functions and structs are included
in this crate:

- `EcdsaRecoverableSignature`: a struct
  representing a recoverable ECDSA signature,
  which includes the signature itself and
  a recovery ID that indicates which of the
  possible public keys to recover.

- `ecdsa_recover`: a function that recovers the
  public key from a non-recoverable ECDSA
  signature and the corresponding message hash.

- `ecdsa_recoverable_signature_convert`:
  a function that converts a recoverable ECDSA
  signature to a non-recoverable signature.

- `ecdsa_recoverable_signature_load`: a function
  that loads a recoverable ECDSA signature from
  a byte slice.

- `ecdsa_recoverable_signature_parse_compact`:
  a function that parses a compact byte
  representation of a recoverable ECDSA signature.

- `ecdsa_recoverable_signature_save`: a function
  that saves a recoverable ECDSA signature to
  a byte slice.

- `ecdsa_recoverable_signature_serialize_compact`:
  a function that serializes a recoverable ECDSA
  signature to a compact byte representation.

- `ecdsa_sig_recover`: a function that recovers
  the public key from a non-recoverable ECDSA
  signature and the corresponding message hash
  (alias for `ecdsa_recover`).

- `ecdsa_sign_recoverable`: a function that signs
  a message with a recoverable ECDSA signature.

- `is_recovery_enabled`: a function that returns
  true if the library was compiled with support
  for ECDSA signature recovery.

- `recovery_test_nonce_function`: a function that
  tests the nonces used in ECDSA signature
  recovery.

- `run_recovery_tests`: a function that runs a set
  of tests for ECDSA signature recovery.

- `test_ecdsa_recovery_api`: a function that tests
  the basic functionality of the ECDSA signature
  recovery API.

- `test_ecdsa_recovery_edge_cases`: a function
  that tests edge cases in the ECDSA signature
  recovery API.

- `test_ecdsa_recovery_end_to_end`: a function
  that tests the end-to-end functionality of the
  ECDSA signature recovery API.

- `test_exhaustive_recovery`: a function that
  tests the exhaustive recovery of public keys
  from ECDSA signatures.

- `test_exhaustive_recovery_sign`: a function that
  tests the exhaustive signing of messages with
  recoverable ECDSA signatures.

- `test_exhaustive_recovery_verify`: a function
  that tests the exhaustive verification of
  messages with recoverable ECDSA signatures.

The ECDSA signature recovery algorithm is based on
the mathematical properties of the secp256k1
elliptic curve, specifically the fact that
a message signed with a private key and
a recoverable signature can be used to uniquely
determine the corresponding public key. The
recovery process involves guessing the value of
the nonce used in the signature generation and
using it to compute the public key from the
signature and the message hash. The recovery ID in
the signature indicates which of the possible
public keys is the correct one.

Notice: This crate is part of a direct translation
from C++ to Rust of the Bitcoin core. As such,
some of the function bodies may still be in the
process of translation. Please note that this
system will become testable once the translation
process is complete.

7> what exactly is ECDSA signature recovery?

ECDSA signature recovery is the process of
recovering the public key used to sign a message
with an ECDSA signature, when the signature is in
a recoverable format. In other words, given
a message `m` and a recoverable signature `(r, s, v)`, 
where `r` and `s` are the signature components and
`v` is a recovery ID, the goal is to find the
public key `P` such that `r` and `s` were computed
using the private key corresponding to `P`.

The signature recovery process involves solving
for the public key `P` using the elliptic curve
equation and the message hash `h
= hash(m)`. Specifically, the following steps are
performed:

1. Compute the curve point `R = r^-1 * (sG - hP)`,
   where `G` is the base point of the curve and
   `^-1` denotes the multiplicative inverse modulo
   the order of the curve.

2. Compute the recovery ID `v` as the least
   integer such that `R + vG` is a valid curve
   point (i.e., satisfies the curve equation).

3. Compute the public key `P` as `P = R + vG`.

The recovery ID `v` indicates which of the four
possible solutions for `P` is the correct one,
since each `v` corresponds to a different point on
the curve that satisfies the equation.

ECDSA signature recovery can be useful in several
contexts, such as recovering the public key of
a Bitcoin address from a transaction signature,
without needing to know the public key beforehand.

8> thanks -- what else should we know when working
with it? are there any caveats?

When working with ECDSA signature recovery, there
are a few caveats to keep in mind:

1. Not all ECDSA signatures are recoverable. Only
   signatures that include the recovery ID `v` are
   recoverable. Non-recoverable signatures cannot
   be used for signature recovery.

2. Signature recovery is an iterative process that
   involves guessing the value of the nonce used
   in the signature generation. In rare cases, the
   guess may be incorrect, leading to an incorrect
   public key recovery. However, the probability
   of this happening is very small (less than
   2^-127 for secp256k1), so it is not usually
   a concern in practice.

3. ECDSA signature recovery is computationally
   expensive, especially when using a brute-force
   approach to guess the nonce value. In some
   cases, it may be more efficient to use
   non-recoverable signatures or to use other
   techniques for deriving the public key from
   a signature, such as key recovery based on the
   signer's identity or key-agreement protocols.

In addition, it is important to ensure that the
implementation of ECDSA signature recovery is
secure and does not introduce any vulnerabilities
in the system.
