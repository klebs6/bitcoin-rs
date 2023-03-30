Short description tag: `bitcoinsecp256k1-group`

The `bitcoinsecp256k1-group` crate is a Rust
implementation of the group operations on the
secp256k1 elliptic curve used in the Bitcoin
protocol. This crate provides functions for point
addition, point multiplication, point negation,
point equality checks, and other operations
necessary for working with elliptic curve points
in the secp256k1 group.

Notice: This crate is part of a direct translation
from C++ to Rust of the bitcoin core. As such,
some of the function bodies may still be in the
process of translation. Please note that this
system will become testable once the translation
process is complete.

The main data structure in this crate is the `Ge`
type, which represents a point on the secp256k1
curve. The `GeStorage` type is used for storage of
a point in compressed format, and `Gej` represents
a point in Jacobian coordinates.

The mathematical operations performed on points in
this group are based on elliptic curve addition
and multiplication, which are defined as follows:

- Elliptic curve addition: Given two points `P`
  and `Q`, we can find a third point `R` such that
  `R = P + Q` if and only if the line connecting
  `P` and `Q` intersects the curve at a third
  point `R'`. `R` is then the reflection of `R'`
  across the x-axis.

- Elliptic curve multiplication: Given a point `P`
  and a scalar `k`, we can find a second point `Q`
  such that `Q = kP`, where `k` is an integer.

In addition to these basic operations, this crate
also provides functions for converting points
between different formats, checking if a point is
in the correct subgroup, and rescaling points.

Overall, the `bitcoinsecp256k1-group` crate
provides a robust set of tools for working with
elliptic curve points in the secp256k1 group,
making it a useful component in building
cryptographic protocols that rely on this curve.

141> how do  point addition, point multiplication,
point negation, point equality checks work?

Point addition, multiplication, negation, and
equality checks are all operations that can be
performed on points on an elliptic curve.

Point addition:

Given two points P and Q on an elliptic curve,
their sum R is computed as follows: draw a line
through P and Q, find the third point where the
line intersects the curve, and reflect this point
across the x-axis. This reflected point is
-R. Mathematically, this can be represented as
R = P + Q and -R = R.neg(), where neg() represents
the negation of the point.

Point multiplication:

Given a point P and an integer n, the point
multiplication nP is computed as follows: add P to
itself n-1 times. This operation can be made more
efficient using techniques such as the
double-and-add algorithm, which computes the
result in logarithmic time
complexity. Mathematically, this can be
represented as nP = P + P + ... + P (n times).

Point negation:

The negation of a point P is simply the point with
the same x-coordinate but a negated
y-coordinate. Mathematically, this can be
represented as -P = (x,-y).

Point equality checks:

To check if two points P and Q are equal, we
compare their x and y coordinates. If both
coordinates are the same, the points are
considered equal. However, because we are working
with finite fields, it is also necessary to check
that the points are not the point at infinity (the
identity element of the group), which is
represented by a special value. This value is
typically denoted as O.

142> why do we need to represent points in
jacobian coordinates? what exactly does this mean

In elliptic curve cryptography, it is often more
efficient to perform operations on points in
a specific coordinate system known as Jacobian
coordinates. This is because performing point
addition and scalar multiplication operations on
points in the standard affine coordinate system
can be computationally expensive.

Jacobian coordinates are a way of representing
elliptic curve points in a three-dimensional
space. In this coordinate system, a point on the
elliptic curve is represented by the coordinates
(X,Y,Z), where X, Y, and Z are elements of the
underlying field.

The advantage of using Jacobian coordinates is
that the formulas for point addition and scalar
multiplication become simpler and more
efficient. For example, when performing point
addition in Jacobian coordinates, the number of
field multiplications required is reduced from 12
to 11. Similarly, when performing scalar
multiplication, the number of field
multiplications required is reduced from 16 to 12.

Furthermore, using Jacobian coordinates allows us
to represent the point at infinity in a finite
way, which is useful for certain algorithms. In
affine coordinates, the point at infinity is
represented as an ideal point that is not part of
the underlying field, making it difficult to work
with.

Overall, Jacobian coordinates provide a more
efficient and convenient way to perform
cryptographic operations on elliptic curve points.

143> which different point formats do we convert
between? how do we know if a point is in the
correct subgroup? why do we rescale points and how
do we do it?

In elliptic curve cryptography, we commonly work
with points on the curve in different formats such
as affine coordinates, projective coordinates, and
Jacobian coordinates.

Affine coordinates represent a point as (x,y),
where x and y are elements in the underlying
field. Projective coordinates represent a point as
(X,Y,Z), where X, Y, and Z are elements in the
underlying field, and the point is (X/Z, Y/Z) in
affine coordinates.

Jacobian coordinates represent a point as (X,Y,Z),
where X, Y, and Z are elements in the underlying
field, and the point is (X/Z^2, Y/Z^3) in affine
coordinates. The reason we use Jacobian
coordinates is that they allow for faster point
addition and doubling operations, as well as
faster point multiplication using the
double-and-add algorithm.

To ensure that a point is in the correct subgroup,
we can check that its coordinates satisfy the
curve equation and that the order of the point is
equal to the order of the curve. If the order of
the point is not equal to the order of the curve,
then we can rescale the point by multiplying it by
the cofactor (the ratio of the order of the curve
to the order of the subgroup generated by the base
point).

To rescale a point in Jacobian coordinates, we can
simply multiply its Z coordinate by the cofactor,
which has the effect of scaling down the X and
Y coordinates. Then, we can convert the point back
to affine coordinates by computing (X/Z^2, Y/Z^3).

144> could you describe the definition of jacobian
coordinates i more detail?

Jacobian coordinates are a way of representing
points on an elliptic curve in three dimensions,
instead of the two dimensions used in affine
coordinates.

A point on an elliptic curve can be represented in
Jacobian coordinates as a tuple (X,Y,Z), where
X and Y are the coordinates of the point in affine
space, and Z is an additional coordinate that
allows for easy point addition and scalar
multiplication.

Mathematically, a point P in Jacobian coordinates
is equal to the affine point Q=(x,y) multiplied by
a scaling factor Z:

P = (X, Y, Z) = (x/Z^2, y/Z^3, Z)

This scaling factor Z allows for easy and
efficient point addition and scalar multiplication
operations, which are critical in cryptographic
applications.

It is important to note that not all points (X, Y,
Z) correspond to valid points on the elliptic
curve. To ensure that a point is valid, it must
satisfy the curve equation and be in the correct
subgroup of the curve.

145> could you compare and contrast  affine
coordinates, projective coordinates, and Jacobian
coordinates?

Affine, projective, and Jacobian coordinates are
all ways to represent points on an elliptic
curve. Each has its own advantages and
disadvantages, and they are used in different ways
depending on the application.

Affine coordinates are the most intuitive way to
represent a point on an elliptic curve. An affine
coordinate point is represented by two integers,
(x, y), which are the x and y coordinates of the
point, respectively. The disadvantage of affine
coordinates is that they can be difficult to work
with computationally, particularly when performing
point multiplication.

Projective coordinates provide a way to work with
points on an elliptic curve that is more efficient
for some calculations. In projective coordinates,
a point is represented by three coordinates, (X,
Y, Z), where the relationship between the
coordinates is given by X = x/Z and Y = y/Z. The
third coordinate, Z, is called the projective
coordinate. Projective coordinates make it easier
to perform point addition and point doubling
operations, which are fundamental operations in
elliptic curve cryptography.

Jacobian coordinates are similar to projective
coordinates, but have an additional parameter that
makes point multiplication more efficient. In
Jacobian coordinates, a point is represented by
three coordinates, (X, Y, Z), where the
relationship between the coordinates is given by
X = x/Z^2, Y = y/Z^3. The third coordinate, Z, is
the same as in projective coordinates, but the
additional exponent in the definition of X and
Y makes point multiplication faster. Jacobian
coordinates are the most commonly used coordinate
system in elliptic curve cryptography.

In summary, affine coordinates are the most
intuitive way to represent a point on an elliptic
curve, but can be difficult to work with
computationally. Projective coordinates provide
a more efficient way to perform point addition and
point doubling operations, while Jacobian
coordinates provide an even more efficient way to
perform point multiplication.

146> what are groups? what are their mathematical
properties? why do we use them in elliptic curve
cryptography?

In abstract algebra, a group is a set of elements
with an operation that combines any two elements
to form a third element, where the operation
satisfies certain properties. Specifically,
a group is defined as a set G together with
a binary operation ∗ that combines any two
elements x and y to form a third element denoted
x ∗ y, such that the following properties hold:

1. Closure: For all x, y in G, the result of the
   operation x ∗ y is also in G.

2. Associativity: For all x, y, and z in G, the
   equation (x ∗ y) ∗ z = x ∗ (y ∗ z) holds.

3. Identity element: There exists an element e in
   G, called the identity element, such that for
   all x in G, the equation
   e ∗ x = x ∗ e = x holds.

4. Inverse element: For each x in G, there exists
   an element y in G, called the inverse of x,
   such that x ∗ y = y ∗ x = e.

Groups have many mathematical properties that make
them useful in cryptography. For example, groups
can be used to create one-way functions that are
easy to compute in one direction but difficult to
invert in the other direction, which is the basis
for many cryptographic protocols. Groups can also
be used to generate random numbers, which are
essential for many cryptographic operations. In
elliptic curve cryptography, groups are used to
define the set of points that form the curve and
to define the operations of point addition and
point multiplication.

Affine coordinates, projective coordinates, and
Jacobian coordinates are different ways of
representing points on an elliptic curve. Affine
coordinates are the most common way of
representing points, where each point is
represented by its x and y coordinates. Projective
coordinates are a way of representing points using
three coordinates (X, Y, Z), where the x and
y coordinates of the point are (X/Z,
Y/Z). Jacobian coordinates are a variation of
projective coordinates, where each point is
represented by three coordinates (X, Y, Z), but
the values of X and Y are multiplied by Z^2 and
Z^3, respectively.

The use of different coordinate systems is mainly
to simplify the computations involved in point
addition and point multiplication. Specifically,
projective and Jacobian coordinates can speed up
the computation of point addition and point
doubling operations. Additionally, the use of
projective or Jacobian coordinates can make it
easier to ensure that points are in the correct
subgroup and can simplify the implementation of
algorithms such as point compression.

147> what are some things we should watch out for
when using groups?

When working with groups, there are a few things
to watch out for:

1. Make sure the group is the correct size and has
   the desired properties. For example, in
   elliptic curve cryptography, it is important to
   use a group that is sufficiently large and has
   properties such as being prime-order or having
   no small subgroups.

2. Be careful with group operations. In some
   cases, performing a group operation incorrectly
   or with incorrect inputs can lead to security
   vulnerabilities. For example, in elliptic curve
   cryptography, using an incorrect point addition
   algorithm or multiplying a scalar by a secret
   key that is too small can lead to attacks.

3. Watch out for attacks that exploit group
   properties. For example, some attacks may rely
   on the fact that certain elements in the group
   have small order or that certain group
   operations are not computationally
   difficult. It is important to be aware of these
   attacks and to use appropriate countermeasures.

4. Be careful with random number generation in the
   group. In many cases, random number generation
   is used to generate secret keys or other
   cryptographic parameters. If the random number
   generation is flawed or predictable, it can
   lead to security vulnerabilities.

5. Be aware of implementation issues. Group
   operations can be computationally intensive, so
   it is important to use efficient algorithms and
   implementations. Additionally, side-channel
   attacks and other implementation-specific
   vulnerabilities can arise if the implementation
   is not carefully designed and tested.

148> what would happen if a group used in elliptic
curve cryptography had a small subgroup?

If a group used in elliptic curve cryptography had
a small subgroup, it would be vulnerable to the
so-called "small subgroup attacks". In particular,
an attacker could compute the discrete logarithm
of a point by exploiting the fact that the point
is in a small subgroup of the curve. This would
allow the attacker to recover the private key of
a user, which is intended to be kept secret.

To prevent small subgroup attacks, it is important
to choose a curve that has a large prime order
subgroup. This can be achieved by selecting
a curve with a large prime order or by using
a technique called "point compression" which maps
each point to a unique representative in the prime
order subgroup. It is also important to check that
the curve has no other small subgroups.

149> how can an attack rely on the fact that
certain elements in the group have small order?
how can an attack rely on the fact that certain
group operations are not computationally
difficult?

If certain elements in the group have a small
order, then an attacker can use a technique called
the "Pohlig-Hellman algorithm" to compute the
discrete logarithm of any element in the
group. This algorithm relies on the Chinese
Remainder Theorem to break the computation into
several smaller computations that can be solved
efficiently by exploiting the small order of the
group elements. This attack is called
a "small-subgroup attack".

If certain group operations are not
computationally difficult, then an attacker may be
able to use a technique called the "Pollard's rho
algorithm" to compute the discrete logarithm of
any element in the group. This algorithm relies on
the birthday paradox and the existence of
a collision-finding algorithm to generate
a sequence of group elements that eventually leads
to the solution of the discrete logarithm
problem. This attack is called a "generic
algorithm" or a "generic attack" and is applicable
to any group in which the group operations are not
computationally difficult.

In summary, groups used in elliptic curve
cryptography must be carefully chosen to avoid
small subgroups and to ensure that the group
operations are computationally difficult. Failure
to do so may result in the security of the system
being compromised.

150> how does point compression work?

Point compression is a technique used in elliptic
curve cryptography to represent a point on a curve
using a shorter binary string. It works by
exploiting the fact that in many elliptic curves,
the x-coordinate uniquely determines the
y-coordinate up to a sign.

In uncompressed form, a point on an elliptic curve
is represented as a pair of coordinates (x, y),
where x and y are elements of the field over which
the curve is defined. In compressed form, a point
is represented as a single binary string that
includes the x-coordinate and a bit indicating
whether y is even or odd.

To compress a point, one simply takes the
x-coordinate and the least significant bit of the
y-coordinate and concatenates them. To decompress
a point, one uses the x-coordinate and the bit
indicating whether y is even or odd to compute the
two possible values of y and then checks which one
satisfies the equation of the curve.

Point compression is useful because it reduces the
amount of data that needs to be transmitted or
stored when working with elliptic curve
cryptography. However, it does come with some
computational overhead due to the need to
decompress points before using them in
computations.

151> what is the Pohlig-Hellman algorithm?

The Pohlig-Hellman algorithm is a technique for
solving the discrete logarithm problem in a finite
cyclic group. The discrete logarithm problem is
the problem of finding the exponent $x$ such that
$g^x=h$ for a given generator $g$ and an element
$h$ in the group. The Pohlig-Hellman algorithm
exploits the fact that if the order of the group
is the product of small primes, then the discrete
logarithm problem can be reduced to a set of
smaller subproblems that are easier to solve.

The algorithm works as follows. Suppose we are
given a generator $g$ and an element $h$ in
a finite cyclic group $G$ of order $n$. Let
$n=p_1^{e_1}p_2^{e_2}\cdots p_k^{e_k}$ be the
prime factorization of $n$. For each prime factor
$p_i$, we define $n_i=\frac{n}{p_i^{e_i}}$, and we
compute $g_i=g^{n_i}$ and $h_i=h^{n_i}$.

Now we solve the discrete logarithm problem for
each of the groups $G_i=\langle g_i\rangle$, which
have order $p_i^{e_i}$. For each $i$, we compute
$x_i=\log_{g_i}(h_i)$ using any algorithm for the
discrete logarithm problem in a cyclic group of
prime order. Then we use the Chinese Remainder
Theorem to combine the solutions $x_i$ and obtain
the solution $x$ to the original discrete
logarithm problem, which satisfies $x\equiv
x_i\pmod{p_i^{e_i}}$ for each $i$.

The Pohlig-Hellman algorithm can be useful for
attacking cryptosystems based on the discrete
logarithm problem, especially when the prime
factorization of the group order is known or can
be efficiently computed. However, it is less
effective when the prime factors are large or when
the group order is not smooth, meaning it has
large prime factors or other structure that makes
it difficult to apply the algorithm.

152> how does the  Chinese Remainder Theorem work?
how does it apply to elliptic curve cryptography
and groups in the context of bitcoin?

The Chinese Remainder Theorem (CRT) is
a mathematical theorem that provides a way to
solve a system of linear congruences. In essence,
the CRT states that if we have a system of
equations of the form:

```
x ≡ a1 (mod m1)
x ≡ a2 (mod m2)
...
x ≡ ak (mod mk)
```

where `a1, a2, ..., ak` and `m1, m2, ..., mk` are
integers, and the `mi` are pairwise coprime (i.e.,
they have no common divisors except 1), then there
exists a unique integer `x` modulo `m1 * m2
* ... * mk` that satisfies all the congruences.

In elliptic curve cryptography, the CRT can be
used to speed up certain computations involving
point multiplication. Specifically, suppose we
have an elliptic curve `E` defined over a finite
field `Fp` of order `p`, and we want to compute `k
* P`, where `P` is a point on `E` and `k` is
a scalar (i.e., an integer). We can write `k` in
binary form as:

```
k = k0 + k1 * 2^w + k2 * 2^(2w) + ... + km * 2^(mw),
```

where `w` is some positive integer and `k0, k1,
..., km` are integers less than `2^w`. We can then
compute `k * P` using the following algorithm:

1. Compute `P, 2P, 4P, ..., 2^(mw)P` using
   repeated point doubling, storing each result in
   a table.

2. For each `i` from `m` to `0`, compute `ki * 2^(iw)P` 
    using the table and add the result
    to a running sum.

3. Return the final sum.

This algorithm requires `O(m)` point doublings and
`O(mw)` point additions. However, if we choose `w`
such that each `ki` is relatively small (say, less
than `2^(w/2)`), we can use the CRT to speed up
the point additions. Specifically, we can choose
pairwise coprime integers `mi` such that `mi
= 1 (mod 2^w)` and `mi = 0 (mod 2^j)` for `j
≠ i`. Then, we can compute `ki * 2^(iw)P` mod
`miP` for each `i` using the table, and use the
CRT to combine the results into a single point
multiplication.

In the context of bitcoin, the CRT is used in the
implementation of the secp256k1 elliptic curve,
which is used to generate and verify digital
signatures. The secp256k1 curve has order `n`,
where `n` is a large prime, and signature
generation involves computing `k^-1 (h + r*d)` mod
`n`, where `k` is a random scalar, `h` is a hash
value, `r` is the x-coordinate of a point on the
curve, `d` is the private key, and `^-1` denotes
multiplicative inverse mod `n`. By using the CRT
to speed up the computation of `k^-1`, the
secp256k1 curve can generate signatures more
efficiently, making it suitable for use in
a blockchain network like bitcoin.

153> what is the Pollard's rho algorithm?

Pollard's rho algorithm is a probabilistic
algorithm for solving the discrete logarithm
problem (DLP) in a finite group. The discrete
logarithm problem is the problem of finding an
integer `k` such that `g^k = h` in a cyclic group
generated by `g`. It is believed to be
computationally infeasible for large groups, and
thus provides a basis for many cryptographic
protocols, including elliptic curve cryptography.

Pollard's rho algorithm was first proposed by John
Pollard in 1978. The algorithm uses a random walk
on the group to find a collision between two
points, which can then be used to recover the
discrete logarithm.

The basic idea of the algorithm is to generate
a sequence of points `x_1, x_2, ...` in the group
using a random function. At each step, two points
`x_i` and `x_{2i}` are computed. If `x_i` and
`x_{2i}` are equal, then a collision has been
found and the algorithm can be used to recover the
discrete logarithm. If not, the algorithm
continues by generating a new sequence of points
starting from `x_{2i}`.

The algorithm can be made more efficient by using
a technique called "parallel collision search". In
this technique, multiple random walks are
performed simultaneously, and collisions between
any pair of walks are checked. This can greatly
reduce the expected running time of the algorithm.

In the context of elliptic curve cryptography,
Pollard's rho algorithm can be used to attack the
discrete logarithm problem on elliptic
curves. This can be used to break various
cryptographic protocols, including key exchange
protocols and digital signature schemes. To
prevent such attacks, elliptic curve parameters
should be chosen carefully, and cryptographic
protocols should be designed to be resistant to
Pollard's rho algorithm and other known attacks.

154> how are groups specified in computer code?
which information needs to be stored? how are
groups specified and stored in computer memory ?

In computer code, groups are typically specified
by defining a set of operations that satisfy the
properties of a group. In the context of elliptic
curve cryptography, the group is defined by
specifying the curve equation and the base point,
which generates the group via repeated addition.

To store a group in computer memory, we need to
store several pieces of information. First, we
need to store the curve parameters, including the
coefficients of the equation defining the
curve. We also need to store the base point of the
group, which is typically represented as a point
on the curve in projective or Jacobian
coordinates. In addition, we may need to store
auxiliary data structures, such as precomputed
tables used in point multiplication or point
addition algorithms.

The specific details of how groups are stored in
memory depend on the implementation and the
programming language being used. In some cases,
the entire group may be stored as a single object
or data structure, while in other cases, the curve
parameters and base point may be stored
separately. The representation of group elements,
such as points on the curve, also depends on the
implementation and may be represented in affine,
projective, or Jacobian coordinates.

155> what else should we know about groups in the
context of secp256k1 and bitcoin?

In the context of secp256k1 and Bitcoin, it is
important to note that the group used is
specifically chosen to have certain properties
that make it suitable for use in cryptography. In
particular, the secp256k1 group is chosen to have
a large prime order, which ensures that it has no
small subgroups and that the discrete logarithm
problem is hard to solve.

Additionally, the parameters of the secp256k1
group are chosen in a way that provides certain
efficiency benefits. For example, the field
modulus used is a 256-bit prime number, which
allows arithmetic operations on field elements to
be implemented efficiently using 64-bit and 32-bit
integers.

It is also worth noting that the secp256k1 group
is not the only group used in Bitcoin. Other
cryptographic schemes in Bitcoin, such as the
BIP32 hierarchical deterministic wallet scheme,
use different groups with different properties
tailored to their specific cryptographic needs.

156> is the large prime for secp256k1 fixed?

Yes, the large prime for secp256k1 is fixed and
publicly known. It is:

p = 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1

This fixed prime is an important aspect of the
secp256k1 elliptic curve and is used in all
computations involving points on the curve.

157> why was it chosen, specifically?

The choice of the prime for secp256k1, which is
$2^{256}-2^{32}-2^9-2^8-2^7-2^6-2^4-1$, was based
on a combination of factors. One important
consideration was that it is a prime that is close
to a power of 2, which makes certain mathematical
operations, such as modular reduction, more
efficient. Additionally, it has certain
mathematical properties that make it suitable for
use in elliptic curve cryptography, including
being a safe prime (a prime of the form $2p+1$
where $p$ is also prime) and having a large
multiplicative subgroup of prime order. The latter
property is particularly important for the
security of the curve, as it ensures that certain
attacks based on small subgroups or weak group
structures are not feasible. Finally, the choice
of secp256k1 was also influenced by its
compatibility with existing Bitcoin software and
infrastructure, as well as its relatively
efficient implementation on a wide range of
hardware platforms.
