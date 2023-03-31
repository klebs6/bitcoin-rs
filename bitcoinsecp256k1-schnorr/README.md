# bitcoinsecp256k1-schnorr

This Rust crate is a direct translation from C++
to Rust of the Schnorr signature implementation
used in the Bitcoin system.

## Notice

This crate is part of a direct translation from
C++ to Rust of the Bitcoin Core. As such, some of
the function bodies may still be in the process of
translation. Please note that this system will
become testable once the translation process is
complete.

## Mathematical Ideas

Schnorr signatures are a type of digital signature
that are based on the discrete logarithm
problem. The signature scheme was first proposed
by Claus-Peter Schnorr in 1989, and has since
become an important tool in cryptography,
including its use in the Bitcoin system.

The Schnorr signature scheme is based on the
security of the discrete logarithm problem in
a finite field. The scheme works by having the
signer generate a random secret key, compute
a public key using that secret key, and then sign
a message using a combination of the secret key
and the message. The signature can then be
verified by anyone who knows the public key.

The tokens in this crate implement various
functions related to the Schnorr signature scheme,
including the computation of the nonce used in the
signing process, the signing and verification of
messages, and various tests to ensure the
correctness of the implementation.

Overall, the Schnorr signature scheme is an
important component of the Bitcoin system, and
understanding its implementation is crucial for
anyone working with the system. The implementation
of this crate in Rust provides a fast and
efficient implementation of the Schnorr signature
scheme, and can be used as a building block for
other cryptographic applications.

22> thanks -- could you please describe the
schnorr signature scheme in mathematical detail?
what are its mathematical properties?

Sure! The Schnorr signature scheme is a digital
signature scheme based on the difficulty of
computing discrete logarithms in finite fields.

The scheme consists of three steps: key
generation, signature generation, and signature
verification.

### Key Generation

In this step, the signer generates
a public-private key pair. Specifically, the
signer chooses a secret integer $x$ (the private
key) uniformly at random from the interval $[1,
n-1]$, where $n$ is the order of the base point
$G$ of the elliptic curve being used. The signer
then computes the corresponding public key $P
= xG$, where $G$ is the base point.

The public key $P$ is then shared with others who
wish to verify signatures from the signer.

### Signature Generation

To sign a message $m$, the signer follows the
following steps:

1. Choose a random integer $k$ uniformly at random
   from the interval $[1, n-1]$.

2. Compute the point $R = kG$.

3. Compute the value $e = H(m \| R)$, where $H$ is
   a cryptographic hash function.

4. Compute the value $s = k + xe \mod n$.

The signature for the message $m$ is the pair $(R,
s)$.

### Signature Verification

To verify a signature $(R, s)$ for a message $m$,
the verifier computes:

1. The value $e = H(m \| R)$.
2. The point $P' = sG - eR$.
3. If $P' = P$, the signature is valid. Otherwise,
   the signature is invalid.

The security of the Schnorr signature scheme is
based on the difficulty of computing discrete
logarithms in the underlying elliptic curve. In
particular, the security of the scheme relies on
the difficulty of computing the private key $x$
given the public key $P$, and the difficulty of
computing the value $k$ used in the signature
generation step given the signature $(R, s)$ and
the message $m$.

One of the notable properties of the Schnorr
signature scheme is that it is provably secure in
the random oracle model, under the assumption that
the discrete logarithm problem is hard in the
underlying elliptic curve. Additionally, the
Schnorr signature scheme has several advantages
over other signature schemes, including its
simplicity and efficiency, and the ability to
aggregate multiple signatures into a single
signature, which can be useful in certain
applications.

23> what is the random oracle model? how do we use
it to aggregate multiple signatures into a single
signature? is it correct to say that schnorr
scheme encapsulates the elliptic curve mathematics
we were discussing before?

The random oracle model is a theoretical model
that is often used to analyze the security of
cryptographic protocols. In this model, a random
function is assumed to exist, called the "random
oracle," which maps inputs to random outputs. This
random oracle is used as a substitute for
a cryptographic hash function, which is used in
practice.

The random oracle model is used to analyze the
security of the Schnorr signature scheme because
it relies on a hash function to compute the value
$e$ used in the signature generation and
verification steps. In practice, a cryptographic
hash function such as SHA-256 is used as the hash
function, but in the random oracle model, a random
oracle is used instead.

One of the advantages of the Schnorr signature
scheme is that it can be used to aggregate
multiple signatures into a single signature, which
can be useful in certain applications. This is
achieved through a technique called "batch
verification," where multiple signatures are
verified simultaneously using a single
computation.

To perform batch verification, the verifier
collects all of the signatures and messages to be
verified, and computes a single value $e' = H(m_1
\| R_1) + \cdots + H(m_k \| R_k)$, where $k$ is
the number of signatures, $m_i$ is the $i$-th
message, and $R_i$ is the $i$-th point in the
signature. The verifier then computes the value
$P' = s_1 G - e' R_1 + \cdots + s_k G - e' R_k$,
where $s_i$ is the $i$-th value in the
signature. If $P' = P$, where $P$ is the public
key of the signer, then all of the signatures are
valid.

Yes, it is correct to say that the Schnorr
signature scheme encapsulates the elliptic curve
mathematics we were discussing before, as it is
a digital signature scheme based on the discrete
logarithm problem in an elliptic curve.

24> what else should we know about it? any
caveats?

One caveat with the Schnorr signature scheme is
that it is subject to the same attacks as other
digital signature schemes based on the discrete
logarithm problem. In particular, if the discrete
logarithm problem is ever solved for the elliptic
curve used in the Schnorr scheme, then the
security of the scheme would be compromised.

Another potential caveat is that the Schnorr
scheme is not yet widely adopted in practice, and
thus may not be as well-vetted as other signature
schemes such as the widely used ECDSA. However, it
is gaining traction in the Bitcoin community due
to its potential advantages over ECDSA, such as
its ability to be used for signature aggregation
and its resistance to certain types of attacks.
