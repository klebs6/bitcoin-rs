## bitcoinsecp256k1-modinv

This Rust crate is part of the Bitcoin system and
is a direct translation from the C++ codebase. The
primary purpose of this crate is to provide
efficient modular inversion operations for the
finite field used in the secp256k1 elliptic
curve. The functions provided in this crate are
optimized for 64-bit and 32-bit architectures,
with the `ModInv64` and `ModInv32` structs
respectively.

The `modinv` operation is used to calculate the
modular inverse of a number `a` with respect to
a modulus `m`. That is, to find a number `b` such
that `a * b ≡ 1 (mod m)`. The modular inverse is
an important operation in public-key cryptography,
where it is used to compute private keys from
public keys, and to verify digital signatures.

The crate provides various implementations of the
`modinv` operation, optimized for different
architectures. The algorithms implemented include
the classical binary extended Euclidean algorithm,
as well as more efficient variants such as
Montgomery's ladder and sliding window algorithms.

In addition to the `modinv` operation, the crate
also provides other modular arithmetic functions
such as multiplication and normalization. These
functions are used internally by the modular
inversion algorithms.

Note that since modular inversion is
a computationally intensive operation, it is
important to use efficient implementations in
order to achieve acceptable performance in
cryptographic applications. The functions provided
by this crate are optimized for speed and should
be used whenever modular inversion is required in
a secp256k1-based system.

Notice: This crate is part of a direct translation
from C++ to Rust of the bitcoin core. As such,
some of the function bodies may still be in the
process of translation. Please note that this
system will become testable once the translation
process is complete.

163> could you please explain the classical binary
extended euclidean algorithm? also please explain
montgomery's ladder and the sliding window
algorithms

Sure, here's an explanation of the classical
binary extended Euclidean algorithm, Montgomery's
ladder algorithm, and the sliding window
algorithm.

Classical Binary Extended Euclidean Algorithm:

The extended Euclidean algorithm is used to
compute the greatest common divisor (GCD) of two
integers, along with two coefficients that can be
used to express the GCD as a linear combination of
the two integers. The binary extended Euclidean
algorithm is an optimized version of the extended
Euclidean algorithm that operates on binary
inputs, which is particularly useful in
cryptography.

In the binary extended Euclidean algorithm, we
represent the input integers as binary numbers and
apply a series of binary operations to compute the
GCD and coefficients. Specifically, we repeatedly
divide the larger number by 2 until both numbers
become odd, and then apply a series of binary
subtraction and addition operations to compute the
GCD and coefficients. This algorithm is more
efficient than the classical extended Euclidean
algorithm because it operates on binary inputs,
which allows us to perform binary shifts and
bit-wise operations instead of division and
multiplication.

Montgomery's ladder algorithm:

Montgomery's ladder algorithm is a scalar
multiplication algorithm that is used to compute
the scalar multiplication of a point on an
elliptic curve. Scalar multiplication is the
operation of multiplying a point by a scalar (a
non-negative integer) in order to generate a new
point on the curve.

The Montgomery ladder algorithm is particularly
useful in elliptic curve cryptography because it
is resistant to side-channel attacks. These
attacks are a type of attack that exploit
information leaked by the system (such as timing,
power consumption, or electromagnetic radiation)
to gain information about the secret key used in
the system. The Montgomery ladder algorithm is
designed to be constant-time, meaning that it
takes the same amount of time to execute
regardless of the input, which makes it resistant
to these types of attacks.

The Montgomery ladder algorithm operates by
iteratively applying two operations to a point on
the elliptic curve. First, it adds the point to
itself (doubling the point) if the next bit in the
scalar is a 0, and second, it adds the point to
another point (adding the two points together) if
the next bit in the scalar is a 1. The algorithm
proceeds in this way until all bits in the scalar
have been processed, at which point the final
point is returned as the result of the scalar
multiplication.

Sliding window algorithm:

The sliding window algorithm is another scalar
multiplication algorithm that is used to compute
the scalar multiplication of a point on an
elliptic curve. Like Montgomery's ladder
algorithm, it is also resistant to side-channel
attacks.

The sliding window algorithm operates by
precomputing a table of points on the elliptic
curve that are multiples of a small number of
points. This table is then used to speed up the
computation of scalar
multiplication. Specifically, the scalar is
represented as a binary number and is processed in
a sliding window of a fixed number of bits. The
algorithm then uses this sliding window to compute
the scalar multiplication by performing a series
of additions and doublings on the points in the
precomputed table. By using a sliding window, the
algorithm can take advantage of the fact that most
bits in the scalar are 0 and skip over these bits
in the computation.

Overall, the classical binary extended Euclidean
algorithm, Montgomery's ladder algorithm, and the
sliding window algorithm are all useful tools in
elliptic curve cryptography. The binary extended
Euclidean algorithm is used to compute the
greatest common divisor of two integers, while
Montgomery's ladder algorithm and the sliding
window algorithm are used to compute the scalar
multiplication of a point on an elliptic curve.

164> could you teach me more about the modular
inversion concept?

Certainly! In modular arithmetic, the modular
inverse of an integer `a` with respect to
a modulus `m` is another integer `b` such that the
product of `a` and `b` modulo `m` equals 1. In
other words, `b` is the "multiplicative inverse"
of `a` modulo `m`.

The modular inverse is useful in many areas of
cryptography, such as key generation, digital
signatures, and authentication protocols.

There are various algorithms to compute modular
inverses, such as the extended Euclidean
algorithm, binary extended Euclidean algorithm,
Montgomery inversion, and others. These algorithms
differ in their performance characteristics and
suitability for different scenarios.

For example, the extended Euclidean algorithm is
a general-purpose algorithm that can compute
modular inverses for any modulus, but it may not
be the most efficient algorithm in certain
situations. On the other hand, Montgomery
inversion is a specialized algorithm that is
particularly efficient for modular inverses in
prime fields, such as those used in elliptic curve
cryptography.

Overall, modular inversion is an important concept
in cryptography and efficient algorithms for
computing modular inverses are crucial for the
performance and security of cryptographic systems.

165> why do we need it in the context of ecdh?

In the context of Elliptic Curve Diffie-Hellman
(ECDH), modular inversion is used to compute the
shared secret between two parties.

In ECDH, each party generates a private key and
a corresponding public key on the chosen elliptic
curve. The two parties then exchange their public
keys and use them to compute a shared secret.

To compute the shared secret, one party multiplies
their private key with the other party's public
key, and the other party multiplies their private
key with the first party's public key. This
results in two points on the elliptic curve, and
these points are equal to each other up to
a multiple of the generator point (which is
publicly known). The shared secret is the
x-coordinate of either of these points.

However, computing this shared secret requires
modular inversion in the underlying finite field
of the elliptic curve. Specifically, the private
key of each party is a random integer modulo the
order of the generator point, and computing the
shared secret involves dividing by this integer
modulo the order of the generator point. This is
equivalent to computing the modular inverse of the
private key modulo the order of the generator
point.

Therefore, modular inversion is a crucial step in
the computation of the shared secret in ECDH.

166> what are its mathematical properties?

Modular inversion is a mathematical operation
performed on two integers, where one integer is
divided by another integer in a modular arithmetic
system. The result of the operation is another
integer that satisfies a congruence relationship
with respect to a given modulus.

In general, modular inversion can be represented
as:

a⁻¹ ≡ b (mod m)

where a, b, and m are integers, and a and m are
coprime (i.e., they have no common divisors other
than 1). The modular inverse a⁻¹ is the integer
such that a times a⁻¹ is congruent to 1 modulo m,
i.e., a × a⁻¹ ≡ 1 (mod m).

Modular inversion has several important
properties, including:

1. Unique inverse: If a and m are coprime, then
   the modular inverse of a modulo m is unique.

2. Associativity: The modular inverse of a product
   of integers modulo m is equal to the product of
   the modular inverses of the individual integers
   modulo m, i.e., (ab)⁻¹ ≡ a⁻¹b⁻¹ (mod m).

3. Distributivity: The modular inverse of a sum of
   integers modulo m is generally not equal to the
   sum of the modular inverses of the individual
   integers modulo m, but the following
   distributive law holds: (a+b)⁻¹ ≡ a⁻¹ + b⁻¹
   - kab⁻¹ (mod m), where k is an integer such
   that k(a+b) ≡ 1 (mod m).

Modular inversion is a fundamental operation in
elliptic curve cryptography, and is used in many
cryptographic protocols and algorithms, including
ECDH and ECDSA. In ECDH, modular inversion is used
to compute the shared secret key between two
parties, while in ECDSA, it is used to compute the
signature of a message.

167> what should we watch out for when using this
property in code?

When implementing modular inversion in code, it is
important to watch out for edge cases and
potential errors. Here are some things to keep in
mind:

1. Division by zero: Modular inversion requires
   division by a number which may be zero. In this
   case, the result is undefined, so it's
   important to check for this condition and
   handle it appropriately.

2. Non-invertible elements: Not all elements have
   a modular inverse. For example, if the modulus
   and the element have a common factor, then the
   element will not have an inverse. It is
   important to check for this condition before
   attempting to compute the inverse, otherwise
   the program may crash or return an incorrect
   result.

3. Numerical precision: When working with large
   numbers, it is important to ensure that the
   program has sufficient numerical precision to
   avoid overflow or underflow errors. For
   example, if the modulus is very large, then
   computing the inverse using the extended
   Euclidean algorithm may require many
   iterations, which can be computationally
   expensive and can lead to numerical errors if
   the intermediate results are not handled
   carefully.

4. Timing attacks: The time taken to compute the
   modular inverse can reveal information about
   the input data, which can be exploited by an
   attacker. To prevent timing attacks, it is
   important to use constant-time algorithms that
   do not depend on the input data, and to avoid
   branching or other operations that can leak
   information about the computation.
