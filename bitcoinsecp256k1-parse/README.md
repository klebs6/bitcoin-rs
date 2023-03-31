## bitcoinsecp256k1-parse

### Parse Bitcoin secp256k1 Private Keys and Signatures from DER Encodings

This Rust crate provides functions for parsing
DER-encoded private keys and ECDSA signatures
using the secp256k1 elliptic curve, which is used
in the Bitcoin system.

The following functions are included in this
crate:

- `ec_privkey_export_der`: exports a private key
  in DER format

- `ec_privkey_import_der`: imports a private key
  from DER format

- `ecdsa_signature_parse_der_lax`: parses an ECDSA
  signature in DER format with lax encoding rules

The secp256k1 elliptic curve is defined by the
equation:

y^2 = x^3 + 7 (mod p)

where `p` is a prime number, specifically:

p = 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1

This curve is used in the Bitcoin system for
generating public and private keys, as well as for
signing and verifying transactions.

Notice: This crate is part of a direct translation
from C++ to Rust of the Bitcoin core. As such,
some of the function bodies may still be in the
process of translation. Please note that this
system will become testable once the translation
process is complete.

2> how is the DER-encoded private key specified?

The DER-encoded private key is specified according
to the following ASN.1 structure:

```
ECPrivateKey ::= SEQUENCE {
        version INTEGER { ecPrivkeyVer1(1) } (ecPrivkeyVer1),
        privateKey OCTET STRING,
        parameters [0] EXPLICIT ECDomainParameters OPTIONAL,
        publicKey [1] EXPLICIT BIT STRING OPTIONAL
}
```

Here, `version` is the version number of the
private key format (currently only version 1 is
supported), `privateKey` is the octet string
representing the private key, and `parameters` and
`publicKey` are optional fields specifying the
elliptic curve domain parameters and the
corresponding public key, respectively.

The `privateKey` octet string is simply
a big-endian byte string representing the integer
value of the private key.

3> what are elliptic curve domain parameters?

Elliptic curve domain parameters are a set of
values that define the properties of an elliptic
curve used in cryptographic applications, such as
key generation and digital signatures. These
parameters include the coefficients of the
equation defining the curve, the prime modulus of
the underlying finite field, and the base point (a
fixed point on the curve) used to generate the
public keys.

For the secp256k1 elliptic curve used in the
Bitcoin system, the domain parameters are:

- The prime modulus `p`:

  p = 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1

- The coefficients `a` and `b` of the curve equation:

  y^2 = x^3 + 7 (mod p)

  where `a` = 0 and `b` = 7.

- The base point `G` of the curve:

  G = (x, y)

  where 
  `x` = 0x79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798 and 
  `y` = 0x483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8.

These parameters are usually fixed and publicly
known, and are used by all parties in the
cryptographic system to generate their own public
and private keys. The security of the system
depends on the difficulty of computing discrete
logarithms on the elliptic curve, which is
believed to be a hard problem.

4> what is the base point of the curve?

The base point of an elliptic curve is a fixed
point on the curve that is used as a generator for
the group of points on the curve. In other words,
any other point on the curve can be obtained by
adding a multiple of the base point to itself,
where the multiple is an integer.

For the secp256k1 elliptic curve used in the
Bitcoin system, the base point is denoted by `G`
and has the following coordinates (in hexadecimal
notation):

```
x = 0x79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798
y = 0x483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8
```

The base point `G` has a very large order
(2^256-1), which means that any multiple of `G`
(except for the point at infinity) generates
a different point on the curve. This property is
used in the secp256k1-based cryptographic systems,
such as Bitcoin, for generating public and private
keys and for signing and verifying transactions.

5> thanks! what else should we know about this
crate?

In addition to the functions for parsing
DER-encoded private keys and ECDSA signatures, the
`bitcoinsecp256k1-parse` crate also provides
a `PublicKey` struct representing a public key on
the secp256k1 elliptic curve. The struct has
a `from_slice` function for constructing a public
key from a byte slice and a `serialize` function
for exporting the public key in compressed or
uncompressed format.

The crate uses the `bitcoin-secp256k1` library for
implementing the secp256k1 elliptic curve
operations, which provides optimized C code for
the most computationally intensive operations. The
`bitcoin-secp256k1` library is included as
a submodule of the `bitcoinsecp256k1-parse` crate
and is automatically compiled during the crate
build process.

This crate is part of a direct translation from
C++ to Rust of the Bitcoin core. As such, some of
the function bodies may still be in the process of
translation. Please note that this system will
become testable once the translation process is
complete.

Overall, the `bitcoinsecp256k1-parse` crate
provides a useful tool for parsing and
manipulating secp256k1-based cryptographic objects
in Rust, particularly in the context of the
Bitcoin system.
