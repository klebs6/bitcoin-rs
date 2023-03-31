## bitcoinsecp256k1-scalar

### Manipulate Scalar Values on the Bitcoin secp256k1 Curve

This Rust crate provides functions for
manipulating scalar values on the secp256k1
elliptic curve, which is used in the Bitcoin
system. Scalars are 256-bit integers that are used
in the arithmetic operations of the elliptic curve
cryptography, such as scalar multiplication.

The following functions and structs are included
in this crate:

- `Scalar`: a struct representing a scalar value
  on the secp256k1 curve, with functions for
  performing arithmetic operations and other
  manipulations.

- `scalar_add`: a function that adds two scalar
  values together.

- `scalar_cadd_bit`: a function that conditionally
  adds a bit to a scalar value.

- `scalar_check_overflow`: a function that checks
  whether a scalar value is greater than or equal
  to the curve order.

- `scalar_clear`: a function that clears the value
  of a scalar.

- `scalar_cmov`: a function that conditionally
  moves a scalar value.

- `scalar_cond_negate`: a function that
  conditionally negates a scalar value.

- `scalar_eq`: a function that checks whether two
  scalar values are equal.

- `scalar_get_b32`: a function that retrieves the
  32-byte representation of a scalar value.

- `scalar_get_bits`: a function that retrieves the
  least significant `n` bits of a scalar value.

- `scalar_get_bits_var`: a function that retrieves
  a variable number of bits from a scalar value.

- `scalar_inverse`: a function that computes the
  modular inverse of a scalar value.

- `scalar_inverse_var`: a function that computes
  the modular inverse of a scalar value with
  a variable number of bits.

- `scalar_is_even`: a function that checks whether
  a scalar value is even.

- `scalar_is_high`: a function that checks whether
  a scalar value is greater than or equal to
  2^255.

- `scalar_is_one`: a function that checks whether
  a scalar value is equal to 1.

- `scalar_is_zero`: a function that checks whether
  a scalar value is equal to 0.

- `scalar_mul`: a function that multiplies two
  scalar values together.

- `scalar_mul_shift_var`: a function that
  multiplies a scalar value by a variable shift.

- `scalar_negate`: a function that negates
  a scalar value.

- `scalar_set_b32`: a function that sets the value
  of a scalar from a 32-byte array.

- `scalar_set_int`: a function that sets the value
  of a scalar from an integer.

- `scalar_shr_int`: a function that shifts
  a scalar value right by a fixed number of bits.

- `scalar_split_128`: a function that splits
  a scalar value into two 128-bit halves.

- `scalar_split_lambda`: a function that splits
  a scalar value into two 128-bit halves for use
  in the `lambda` parameter of the MuSig protocol.

- `scalar_split_lambda_verify`: a function that
  splits a scalar value into two 128-bit halves
  for use in the `lambda` parameter of the MuSig
  protocol, and verifies that the value is less
  than the curve order.

Scalars are used extensively in the arithmetic
operations of elliptic curve cryptography, and
efficient scalar arithmetic is essential for the
performance of these operations. The
`bitcoinsecp256k1-scalar` crate provides a fast
and efficient implementation of scalar arithmetic
on the secp256k1 curve, using optimized C code
from the `bitcoin-secp256k1` library.

Notice: This crate is part of a direct translation
from C++ to Rust of the Bitcoin core. As such,
some of the function bodies may still be in the
process of translation. Please note that this
system will become testable once the translation
process is

10> could you teach me about scalars in the
context of secp256k1?

In the context of secp256k1, a scalar is a 256-bit
integer used for scalar multiplication with a base
point on the elliptic curve. Scalar multiplication
is a core operation in elliptic curve
cryptography, and is used for tasks such as key
generation, signature generation, and signature
verification.

In secp256k1, the scalar values are used to
multiply the base point `G` of the curve to
generate a public key, or to generate a shared
secret in key agreement protocols. Specifically,
given a scalar value `k`, the corresponding public
key `P` is computed as:

```
P = k * G
```

where `G` is the base point of the curve, and `*`
denotes scalar multiplication. The scalar values
are also used in the computation of ECDSA
signatures and in the MuSig protocol.

The secp256k1 curve has a specific order `n`,
which is a large prime number that defines the
size of the scalar field. Scalars are used modulo
`n`, which means that any scalar value greater
than or equal to `n` is equivalent to a smaller
value modulo `n`. In other words, scalar values
are reduced modulo `n` to ensure that they are
within the scalar field.

In addition, it is important to ensure that scalar
values are generated using a secure random number
generator, since the security of the entire
cryptographic system depends on the randomness of
the scalar values. Leakage of the scalar value or
the private key can result in the compromise of
the entire system.

Overall, scalars are a critical component of
elliptic curve cryptography, and efficient scalar
arithmetic is essential for the performance and
security of the system.

11> what else should we keep in mind while using
them? are there any caveats?

When working with scalar values in secp256k1,
there are a few caveats to keep in mind:

1. Scalar values must be chosen uniformly at
   random to ensure the security of the
   cryptographic system. The use of a flawed
   random number generator can result in the
   compromise of the entire system. Therefore, it
   is important to use a secure random number
   generator to generate scalar values.

2. Scalar multiplication is a computationally
   expensive operation, and may take a significant
   amount of time to compute for large scalar
   values. In particular, the generation of public
   keys and signatures may require multiple scalar
   multiplications, and the efficiency of these
   operations is critical for the performance of
   the system.

3. Scalar values must be reduced modulo the curve
   order `n` to ensure that they are within the
   scalar field. If a scalar value is not reduced
   modulo `n`, it may result in an incorrect
   computation and potentially a security
   vulnerability.

4. The secp256k1 curve has a cofactor of 1, which
   means that every point on the curve is
   a multiple of the base point `G`. However, not
   every scalar value corresponds to a valid point
   on the curve, and some scalar values may result
   in an invalid point. Therefore, it is important
   to check that the scalar value corresponds to
   a valid point on the curve before using it in
   cryptographic operations.

In addition, it is important to follow best
practices for secure coding and to regularly
review and update the code to address any
potential security issues. The
`bitcoinsecp256k1-scalar` crate provides a fast
and efficient implementation of scalar arithmetic
on the secp256k1 curve, using optimized C code
from the `bitcoin-secp256k1` library, and has
undergone extensive security audits. However, as
with any cryptographic implementation, it is
important to follow best practices for secure
coding and to regularly review and update the code
to address any potential security issues.

12> does the cofactor of 1 parametrize the
secp256k1 curve?

No, the cofactor of 1 does not parametrize the
secp256k1 curve.

The secp256k1 curve is defined by a set of domain
parameters, including the prime modulus `p`, the
curve coefficients `a` and `b`, the base point
`G`, and the curve order `n`. These domain
parameters are fixed and do not depend on the
cofactor.

The cofactor of a curve is the ratio of the order
of the elliptic curve group to the order of the
subgroup generated by the base point `G`. For the
secp256k1 curve, the curve order `n` is a large
prime number that defines the size of the scalar
field, while the order of the subgroup generated
by `G` is equal to `n` (since the curve has
a cofactor of 1). This means that every point on
the curve is a multiple of `G`.

The cofactor of 1 has important implications for
the security of the curve, since it ensures that
there are no small subgroups that can be exploited
in attacks. In particular, the use of a curve with
a cofactor greater than 1 can result in
vulnerabilities in the Diffie-Hellman key exchange
protocol and other cryptographic schemes.

However, the cofactor does not parametrize the
curve itself, since it is simply a property of the
group generated by the base point `G`. The domain
parameters of the secp256k1 curve, including `p`,
`a`, `b`, `G`, and `n`, are fixed and
standardized, and are used in all implementations
of the curve.

13> so, is it correct to say that when we have
a large prime number as the curve order, we have
a cofactor of 1?

No, it is not necessarily true that a curve with
a large prime order has a cofactor of 1. While
a curve with a large prime order is more likely to
have a cofactor of 1, this is not a guarantee.

The cofactor of a curve is determined by the order
of the elliptic curve group and the order of the
subgroup generated by the base point. If the order
of the subgroup is equal to the order of the curve
group, then the cofactor is 1. However, if the
order of the subgroup is smaller than the order of
the curve group, then the cofactor is greater than
1.

For example, consider the curve defined by the
Weierstrass equation `y^2 = x^3 + ax + b` over the
prime field `F_p`, where `a` and `b` are constants
and `p` is a prime number. If the order of the
curve group is a large prime number, then it is
more likely that the curve has a cofactor of
1. However, it is still possible for the curve to
have a smaller subgroup order and a larger
cofactor.

In the case of the secp256k1 curve, the curve
order `n` is a large prime number, and the curve
has a cofactor of 1. This is one of the reasons
why the secp256k1 curve is considered to be
a strong choice for elliptic curve
cryptography. However, the cofactor of a curve is
a separate parameter from the order, and must be
taken into account separately in cryptographic
applications.

14> ok thanks. it sounds like an important concept
to understand when working with elliptic
curves. how do we define the subgroup generated by
the base point?

The subgroup generated by the base point `G` of an
elliptic curve is the set of all points on the
curve that can be obtained by adding `G` to itself
a certain number of times. In other words, the
subgroup is the set of all scalar multiples of
`G`, where the scalar values are integers modulo
the curve order `n`.

To define the subgroup generated by `G`, we can
start by computing the order of `G`, which is the
smallest positive integer `k` such that `k
* G = O`, where `O` is the point at infinity. This
can be done using an efficient algorithm such as
Pollard's rho algorithm or Schoof's algorithm.

Once we have computed the order of `G`, we know
that the subgroup generated by `G` has `k`
elements, which are all the scalar multiples of
`G` up to `(k-1) * G`. In other words, the
subgroup generated by `G` is the set:

{ G, 2G, 3G, ..., (k-1)G }

where each element in the set is a point on the
curve. The order of the subgroup generated by `G`
is equal to the order of `G`, which is `k`.

It is important to note that not every point on
the curve is a member of the subgroup generated by
`G`. In fact, only points that are scalar
multiples of `G` are members of the subgroup. The
cofactor of the curve is the ratio of the order of
the curve group to the order of the subgroup
generated by `G`, and gives the number of cosets
of the subgroup in the curve group.

15> what is schoof's algorithm?

Schoof's algorithm is an efficient algorithm for
computing the number of points on an elliptic
curve over a finite field of characteristic
`p`. The algorithm was developed by the
mathematician Hendrik Lenstra Jr. and published by
his PhD student Arnold Schoof in 1985.

The basic idea of Schoof's algorithm is to compute
the number of points on the curve by reducing the
problem to a series of simpler problems involving
elliptic curves over smaller finite
fields. Specifically, the algorithm involves the
following steps:

1. Compute the curve discriminant `Delta` and the
   constant `B` such that `4B^3 + 27Delta ≠ 0`.

2. Choose a prime `l` such that `l > 2sqrt(p)` and
   `l` does not divide `Delta`.

3. Compute the number of points on the curve
   modulo `l` using Schoof's lemma, which states
   that the number of points on the curve modulo
   `l` is congruent to `p+1 - t` modulo `l`, where
   `t` is the trace of the Frobenius endomorphism.

4. Repeat steps 2-3 for a sequence of primes `l_i`
   such that `∏l_i > 4sqrt(p)`.

5. Use the Chinese Remainder Theorem to combine
   the results from step 4 and compute the number
   of points on the curve modulo `p`.

The key insight behind Schoof's algorithm is that
the trace of the Frobenius endomorphism can be
computed using an efficient dynamic programming
algorithm that reduces the problem to simpler
elliptic curves over smaller finite fields. By
choosing a sequence of primes that is large enough
to cover all possible values of the trace,
Schoof's algorithm can compute the number of
points on the curve over the entire finite field
with a complexity of `O(log^8 p)` operations in
the finite field.

Schoof's algorithm is an important tool in
computational number theory and has applications
in cryptography, including the computation of
discrete logarithms in elliptic curve
cryptography. However, more recent algorithms such
as the Schoof-Elkies-Atkin algorithm and the SEA
algorithm can be faster for certain types of
curves and finite fields.

16> what is the frobenius endomorphism?

The Frobenius endomorphism is a mathematical
function that is defined on an elliptic curve over
a finite field. Specifically, if `E` is an
elliptic curve defined over a finite field `F_q`,
then the Frobenius endomorphism `phi` is a map
from `E` to itself that sends a point `P = (x,y)`
to a new point `phi(P) = (x^q, y^q)`, where `q` is
the order of the field `F_q`.

The Frobenius endomorphism has a number of
interesting properties that make it important in
the study of elliptic curves over finite
fields. One of the most important properties is
that it is a ring homomorphism, which means that
it preserves the algebraic structure of the
curve. Specifically, if `P` and `Q` are points on
the curve and `a` and `b` are scalars, then:

- `phi(P + Q) = phi(P) + phi(Q)`

- `phi(aP) = a^q * phi(P)`

- `phi(P) = P` if and only if `P` is defined over
  the base field `F_q`.

The trace of the Frobenius endomorphism `t` is
a number that encodes information about the number
of points on the curve over the finite field
`F_q`. Specifically, the trace is defined as `t
= q + 1 - #E(F_q)`, where `#E(F_q)` is the number
of points on the curve over `F_q`. The trace is an
important parameter in many algorithms for
computing discrete logarithms in elliptic curve
cryptography, since it can be used to compute the
number of points on the curve over larger finite
fields using techniques such as Schoof's
algorithm.

The Frobenius endomorphism also plays a key role
in the theory of complex multiplication, which
relates the theory of elliptic curves to the
theory of algebraic number fields. In this
context, the Frobenius endomorphism is used to
define the endomorphism ring of an elliptic curve,
which is a ring of algebraic integers that
contains the Frobenius endomorphism and other
endomorphisms of the curve. The endomorphism ring
is a powerful tool in the study of elliptic
curves, and has important applications in
cryptography, including the construction of
pairing-based cryptographic protocols.

17> what are the mathematical properties of the
endomorphism ring of an elliptic curve? what are
pairing-based cryptographic protocols?

The endomorphism ring of an elliptic curve `E` is
the ring of all endomorphisms of `E` that are
defined over the base field of the curve. An
endomorphism is a map from `E` to itself that
preserves the group structure of the curve. The
endomorphism ring of `E` is denoted by `End(E)`.

The endomorphism ring has a number of important
mathematical properties that make it useful in the
study of elliptic curves. One of the most
important properties is that it is a commutative
ring with identity, which means that it has many
of the same algebraic properties as the ring of
integers. In particular, the endomorphism ring has
a unique factorization property, which means that
every element can be factored into a product of
irreducible elements in a unique way. This
property is essential in many algorithms for
computing discrete logarithms in elliptic curve
cryptography.

Another important property of the endomorphism
ring is that it can be used to define an isogeny,
which is a map from one elliptic curve to another
that preserves the group structure. The isogeny is
defined by an element of the endomorphism ring,
and the degree of the isogeny is equal to the norm
of the endomorphism.

Pairing-based cryptographic protocols are a class
of cryptographic protocols that rely on the
properties of the endomorphism ring of an elliptic
curve. A pairing is a bilinear map that takes two
points on an elliptic curve and produces a value
in a finite field. The pairing is defined using
elements of the endomorphism ring, and has the
property that it is non-degenerate, which means
that it is non-zero for all pairs of distinct
points on the curve.

Pairings have a number of important cryptographic
applications, including identity-based encryption,
group signatures, and non-interactive
zero-knowledge proofs. The security of
pairing-based protocols is based on the difficulty
of computing discrete logarithms in the
endomorphism ring of the curve. In particular, the
hardness of the discrete logarithm problem in the
endomorphism ring is often assumed to be at least
as hard as the discrete logarithm problem in the
base field of the curve.

18> what are the mathematical properties of
isogeny?

Isogenies are maps between elliptic curves that
preserve the group structure of the
curves. Specifically, an isogeny between two
elliptic curves `E` and `E'` is a non-constant map
`ϕ: E → E'` that satisfies the following
properties:

1. `ϕ(O_E) = O_E'`, where `O_E` and `O_E'` are the
   identity elements of the curves.

2. `ϕ(P + Q) = ϕ(P) + ϕ(Q)` for all `P` and `Q` in `E`.

3. `ϕ(-P) = -ϕ(P)` for all `P` in `E`.

The degree of an isogeny `ϕ` is the number of
points in the kernel of the map `ϕ`. In other
words, the degree is the size of the set of points
in `E` that are mapped to the identity element
`O_E'` of `E'`.

Isogenies have a number of interesting
mathematical properties that make them useful in
the study of elliptic curves. One of the most
important properties is that they form a group
under composition. That is, if `ϕ` and `ψ` are two
isogenies between elliptic curves, then their
composition `ψ ∘ ϕ` is also an isogeny.

Another important property of isogenies is that
they preserve certain algebraic structures. For
example, if `E` is an elliptic curve over a field
`K`, then an isogeny `ϕ: E → E'` between `E` and
another elliptic curve `E'` over the same field
`K` preserves the field structure of `K`. That is,
if `P` is a point on `E` with coordinates in `K`,
then the coordinates of `ϕ(P)` are also in `K`.

Isogenies also play an important role in
cryptography, including in the construction of
post-quantum cryptographic systems. One example of
a cryptographic system based on isogenies is the
Supersingular Isogeny Diffie-Hellman (SIDH)
protocol, which is a key exchange protocol that
relies on the difficulty of computing isogenies
between supersingular elliptic curves.

19> what is the supersingular isogeny
diffie-hellman protocol? what are supersingular
elliptic curves? what distinguishes them from
regular elliptic curves?

The Supersingular Isogeny Diffie-Hellman (SIDH)
protocol is a post-quantum cryptographic key
exchange protocol that is based on the mathematics
of supersingular elliptic curves and isogenies.

Supersingular elliptic curves are a special class
of elliptic curves with certain mathematical
properties that make them useful in
cryptography. Unlike regular elliptic curves,
which have a unique tangent line at each point,
supersingular elliptic curves have a tangent line
that is defined over the base field of the
curve. This property makes them easier to work
with mathematically, and makes it possible to
define a group structure on the set of isogenies
between supersingular elliptic curves.

The SIDH protocol works by Alice and Bob each
generating a random private key, and then
exchanging public keys based on their private keys
using a series of isogenies. The protocol is
designed so that an eavesdropper who intercepts
the public keys will not be able to compute the
shared secret key without solving a hard
mathematical problem.

The security of the SIDH protocol is based on the
difficulty of computing isogenies between
supersingular elliptic curves. In particular, the
security of the protocol is based on the fact that
there are no known efficient algorithms for
computing isogenies between supersingular elliptic
curves, even using a quantum computer.

One of the key advantages of the SIDH protocol is
that it is resistant to attacks by quantum
computers, which are expected to be able to break
many existing cryptographic systems. This makes it
an important candidate for use in post-quantum
cryptography, which is an area of research focused
on developing cryptographic systems that are
secure against attacks by quantum computers.

20> why is this problem difficult? will we need to
ultimately implement this for bitcoin? is it
possible to implement the protocol on a classical
computer?

Computing isogenies between supersingular elliptic
curves is difficult because there are no known
efficient algorithms for doing so, even on
a quantum computer. The best known classical
algorithm for computing isogenies has a runtime
that is exponential in the size of the prime field
over which the elliptic curves are defined. This
means that the problem becomes exponentially
harder as the size of the field increases, making
it infeasible for use in cryptographic
applications.

The SIDH protocol is not currently used in
Bitcoin, but it is a candidate for use in
post-quantum cryptography, which is an area of
research focused on developing cryptographic
systems that are secure against attacks by quantum
computers. Bitcoin currently uses elliptic curve
cryptography based on the secp256k1 curve, which
is believed to be secure against classical
attacks, but may be vulnerable to attacks by
quantum computers.

It is possible to implement the SIDH protocol on
a classical computer, but the key sizes needed for
security are much larger than those used in
classical cryptography. This means that the
protocol requires more computational resources
than classical key exchange protocols, and may not
be as practical for use in all applications.

Overall, the SIDH protocol is an important area of
research in post-quantum cryptography, and may
become more important as the development of
quantum computers continues. While it is not
currently used in Bitcoin, it is possible that it
could be used in the future as part of efforts to
improve the security of the Bitcoin system.
