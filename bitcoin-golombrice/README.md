# bitcoin-golombrice

The `bitcoin-golombrice` crate is a Rust
implementation of the Golomb-Rice encoding
algorithm used by the Bitcoin system. It is part
of a direct translation of the Bitcoin codebase
from C++ to Rust, with some function bodies still
being translated.

Golomb-Rice encoding is a variable-length encoding
scheme that is commonly used for integer data. It
is particularly well-suited for encoding
non-negative integers with a geometric
distribution, i.e., where small values are much
more common than large ones. This is often the
case in the context of Bitcoin, where the sizes of
data structures such as transactions and blocks
are highly variable.

The `bitcoin-golombrice` crate includes the
`Element`, `ElementSet`, and `Params` types for
working with Golomb-Rice encoded data. It also
includes functions for building hashed sets,
encoding and decoding using Golomb-Rice, and
matching elements against a set.

Some of the mathematical concepts used in this
crate include:

- **Geometric distribution**: A discrete
  probability distribution that models the number
  of Bernoulli trials needed to get one success,
  where the probability of success is constant
  across trials. It has a parameter `p` that
  specifies the probability of success. The
  probability mass function of a geometric
  distribution is given by `P(X = k) = (1
  - p)^(k-1) * p`, where `X` is a random variable
  representing the number of trials needed to get
  one success.

- **Hash function**: A function that maps data of
  arbitrary size to a fixed-size output (often
  a fixed number of bits). Ideally, a hash
  function should be deterministic, meaning that
  the same input always produces the same output,
  and should have the property that it is
  computationally infeasible to find two inputs
  that produce the same output (i.e.,
  a collision).

- **Golomb-Rice encoding**: A variable-length
  encoding scheme that encodes non-negative
  integers with a geometric distribution. It first
  determines the quotient and remainder of the
  integer divided by a parameter `m`, which
  determines the tradeoff between the average bit
  length and the maximum bit length of the encoded
  integers. The quotient is then encoded using
  a unary code (i.e., `k` 1's followed by a 0,
  where `k` is the quotient), and the remainder is
  encoded using a truncated binary encoding with
  `ceil(log2(m))` bits. The resulting bitstream is
  concatenated to form the encoded value.

- **Golomb-Rice decoding**: The inverse of
  Golomb-Rice encoding, which recovers the
  original integer value from its Golomb-Rice
  encoded bitstream. It first reads a unary code
  to determine the quotient, and then reads
  a truncated binary encoding of the
  remainder. The original value is then recovered
  by multiplying the quotient by `m` and adding
  the remainder.

Note that the `bitcoin-golombrice` crate is
intended for use in the context of the Bitcoin
system and may not be suitable for other
applications.

193> thanks -- could you describe the golomb rice
algorithm in more depth?

Sure, the Golomb-Rice algorithm is a method of
compressing non-negative integers into a binary
format. The algorithm was first introduced by
Solomon W. Golomb in 1966 and later extended by
David Rice in 1979.

The basic idea of the Golomb-Rice algorithm is to
split a given integer into two parts: a quotient
and a remainder. The quotient is encoded using
unary coding, while the remainder is encoded using
binary coding. Unary coding represents a number
using a sequence of 1s followed by a 0. For
example, the number 3 is represented as 1110 in
unary coding. Binary coding represents a number
using a sequence of 0s and 1s.

The Golomb-Rice algorithm uses a parameter 'm'
which determines the split point between the
quotient and the remainder. Specifically, the
quotient is equal to the integer divided by 2^m,
rounded down, while the remainder is equal to the
integer modulo 2^m.

To encode an integer 'x', the algorithm first
computes the quotient 'q' and the remainder
'r'. The quotient is then encoded using unary
coding with 'q' 1s followed by a 0. The remainder
is encoded using binary coding with 'm' bits. The
resulting unary and binary codes are concatenated
to form the final compressed representation of
'x'.

To decode a compressed integer 'c', the algorithm
first reads the unary code to determine the
quotient 'q'. It then reads the next 'm' bits to
determine the remainder 'r'. The integer 'x' is
then reconstructed as x = q*2^m + r.

The Golomb-Rice algorithm is often used in
applications where space efficiency is a concern,
such as in data compression, cryptography, and
probabilistic data structures. In the context of
Bitcoin, the Golomb-Rice algorithm is used to
encode and decode integers in the GCSFilter, which
is a probabilistic data structure used for
compactly representing sets of Bitcoin
scriptPubKeys.

194> could you show me how it is used in data
compression, cryptography, and probabilistic data
structures?

Sure! Here are some examples of how the
Golomb-Rice algorithm is used in various
applications:

**Data compression:**

The Golomb-Rice algorithm is often used for
variable-length integer compression in data
compression algorithms. In particular, it is
commonly used in lossless compression of integer
sequences, such as in the case of gzip and
bzip2. The basic idea is that each integer is
first encoded using Golomb coding, which produces
a variable-length code that is shorter for smaller
integers and longer for larger integers. This is
then further compressed using other compression
techniques, such as Huffman coding or run-length
encoding. The use of Golomb-Rice coding can lead
to significant compression gains, particularly for
integer sequences with a skewed distribution.

**Cryptography:**

The Golomb-Rice algorithm is also used in some
cryptographic applications, such as in the
construction of hash functions and signature
schemes. For example, the RSA-PSS (Probabilistic
Signature Scheme) algorithm uses Golomb-Rice
coding to encode the hash value of a message,
which is then used as input to the RSA signature
algorithm. The Golomb-Rice coding helps to ensure
that the hash value is uniformly distributed and
avoids certain vulnerabilities that can arise in
other encoding schemes.

**Probabilistic data structures:**

Finally, the Golomb-Rice algorithm is used in some
probabilistic data structures, such as Bloom
filters and Golomb-coded sets. In these
structures, the Golomb-Rice coding is used to
encode the indices of the elements that are
present in the set or filter. This allows for
efficient storage and lookup of elements, and can
lead to significant memory savings over
traditional set or filter data structures. The use
of Golomb-Rice coding in these structures can also
help to ensure that the indices are uniformly
distributed, which is important for maintaining
the accuracy and false positive rate of the
filter.

195> what are its performance characteristics?

The Golomb-Rice algorithm has some unique
performance characteristics that make it
well-suited for certain applications.

In terms of time complexity, encoding and decoding
an integer takes O(log n) time, where n is the
value being encoded or decoded. This is because
the algorithm involves dividing the integer into
two parts: one part is used to represent the
quotient, while the other part is used to
represent the remainder using a unary code. The
quotient is then encoded using a binary code.

In terms of space complexity, the Golomb-Rice
algorithm can achieve compression ratios close to
the entropy limit for certain types of data
distributions. For data with a high degree of
skewness, where a small number of values occur
frequently, the algorithm can achieve very high
compression ratios. However, for data with a more
uniform distribution, the compression ratio may be
lower.

In cryptographic applications, the Golomb-Rice
algorithm is often used as a component of other
cryptographic protocols, such as hash-based
message authentication codes (HMACs) and digital
signatures. Because of its simplicity and speed,
it is often used in situations where other more
complex cryptographic algorithms would be
impractical.

In probabilistic data structures, the Golomb-Rice
algorithm is often used as a component of Bloom
filters and other types of filters that are used
to efficiently test membership of an element in
a set. The algorithm is used to map elements to
a bit array in a way that minimizes the number of
false positives.

Overall, the Golomb-Rice algorithm is a useful and
versatile tool that can be applied in a wide range
of applications, but its performance
characteristics should be carefully evaluated for
each specific use case.

196> what do you mean by the "entropy limit"?

The "entropy limit" refers to the maximum level of
compression that can be achieved by any lossless
compression algorithm. It is determined by the
entropy of the data being compressed, which is
a measure of the amount of information contained
in the data. Entropy is typically measured in bits
per symbol, and the entropy limit is defined as
the number of bits per symbol required to
represent the data without loss.

For example, if a data source has an entropy of
8 bits per symbol, the entropy limit for that data
is 8 bits per symbol. This means that no lossless
compression algorithm can achieve a compression
ratio better than 8:1 for that data source. In
practice, most data sources have an entropy limit
that is much lower than 8 bits per symbol, which
means that there is significant room for
compression. However, there are some data sources
(such as random data) that have an entropy limit
that is close to or equal to the number of bits
per symbol, making compression difficult or
impossible.

197> how do we determine this limit
mathematically?

The entropy limit of a Golomb-Rice code can be
calculated using the formula:

```
limit = ceil(log2(m)) + ceil(log2(k))
```

where `m` is the size of the modulus `M` used in
the code, and `k` is the Rice parameter.

The `ceil` function rounds up the result to the
nearest integer.

The idea behind this formula is that the maximum
number of bits required to represent any value in
the range `[0, M)` is `ceil(log2(m))`, and the
number of bits required to represent the Rice code
for any value is `q + 1`, where `q` is the
quotient obtained by dividing the value by
`k`. Thus, the maximum number of bits required to
represent any value using a Golomb-Rice code with
parameter `k` is `ceil(log2(m)) + ceil(log2(k))`.

This formula can be used to choose appropriate
values for `m` and `k` when designing
a Golomb-Rice code for a specific application. In
general, larger values of `m` and smaller values
of `k` will result in more efficient coding, but
will also require more computation to encode and
decode the data.

198> what are the other mathematical properties of
the golomb rice code and the surrounding ideas?

There are a few additional mathematical properties
of Golomb-Rice codes and related ideas:

1. **Prefix property:** Each code in a Golomb-Rice
   code is a prefix of any longer code in the
   code. This property ensures that the decoding
   process is unambiguous.

2. **Variable-length codes:** Golomb-Rice codes
   are a type of variable-length code, where each
   symbol is represented by a different number of
   bits. This is in contrast to fixed-length
   codes, where each symbol is represented by the
   same number of bits.

3. **Probabilistic data structures:** Golomb-Rice
   codes are used in various probabilistic data
   structures, such as Bloom filters and Count-Min
   Sketches, to efficiently represent sets and
   approximate frequencies of elements.

4. **Hash functions:** The hash function used to
   generate the hash values for the elements in
   a Golomb-Rice code can also affect the
   performance of the compression and
   decompression algorithms. One commonly used
   hash function is the MurmurHash function, which
   is designed to be fast and provide good random
   distribution of hash values.

5. **Entropy coding:** Golomb-Rice codes are also
   used as a type of entropy coding, where symbols
   are encoded based on their frequency of
   occurrence in the input data. In this context,
   Golomb-Rice codes are often used in conjunction
   with other entropy coding techniques, such as
   Huffman coding, to further improve compression
   performance.

Overall, the Golomb-Rice code and related ideas
have proven to be versatile and effective
techniques for data compression, cryptography, and
probabilistic data structures.

199> thanks -- what else should we know about
golomb rice in the context of bitcoin?

In the context of Bitcoin, Golomb-Rice coding is
used in various places for data compression and
probabilistic data structures. One example is in
the Compact Block protocol, where transactions and
their inputs are represented using Golomb-Rice
encoding to reduce the size of the data being
transmitted between nodes. Golomb-Rice coding is
also used in Bitcoin's bloom filters, which are
probabilistic data structures used to reduce the
number of false positives in transaction
filtering. Specifically, bloom filters use
a variant of Golomb-Rice coding known as
Golomb-coded sets to represent sets of indices
within a larger set. This allows bloom filters to
be highly space-efficient while still providing
a low rate of false positives.

Overall, Golomb-Rice coding is a powerful tool for
data compression and probabilistic data structures
in the context of Bitcoin, allowing the system to
operate efficiently and securely even in the face
of large amounts of data.

200> what are some other probabilistic data
structures and how do they compare to golomb rice?

There are many other probabilistic data
structures, each with their own strengths and
weaknesses. Here are a few examples:

1. Bloom filters: Bloom filters are a popular
   probabilistic data structure used to test
   whether an element is a member of a set. Bloom
   filters use multiple hash functions to map
   elements to a bit array, and then set the
   corresponding bits to 1. When checking for
   membership, the same hash functions are applied
   to the element, and if all the corresponding
   bits are 1, then the element is probably
   a member of the set. Bloom filters have
   constant time insertion and lookup, and use
   a fixed amount of memory, but they have a false
   positive rate that increases with the number of
   elements added to the filter.

2. Count-min sketch: Count-min sketch is another
   probabilistic data structure used to estimate
   the frequency of items in a data stream. It
   uses a set of hash functions to map items to
   a two-dimensional array of counters. When an
   item is encountered in the stream, the
   corresponding counters are incremented. To
   estimate the frequency of an item, the same
   hash functions are applied to the item, and the
   minimum count across all the corresponding
   counters is returned. Count-min sketch provides
   a more accurate estimate of frequencies than
   Bloom filters, but requires more memory.

3. HyperLogLog: HyperLogLog is a probabilistic
   data structure used to estimate the number of
   distinct elements in a data stream. It uses
   a set of hash functions to map items to a bit
   array, and then counts the number of leading
   zeros in the binary representation of the hash
   value. The estimate of the number of distinct
   elements is proportional to 2^r, where r is the
   number of leading zeros. HyperLogLog has
   a fixed memory requirement and a small error
   rate, but it can only be used to estimate the
   number of distinct elements, and not their
   identities.

Overall, the choice of probabilistic data
structure depends on the specific use case and the
desired trade-off between accuracy and memory
usage. In the context of Bitcoin, Golomb-Rice
coding is often used in combination with Bloom
filters to improve the efficiency of block
propagation and transaction filtering.

201> thanks -- what are the cryptographic
properties of golomb rice?

Golomb-Rice encoding by itself is not
a cryptographic primitive, but it is often used in
combination with other cryptographic algorithms to
provide space-efficient and secure data
structures.

In the context of Bitcoin, the Golomb-Rice
encoding is used in constructing the Golomb-Rice
Filter, which is a probabilistic data structure
used to test whether a particular element is
a member of a set. The Golomb-Rice Filter is
constructed from the Golomb-Rice encoding of a set
of elements, which can then be used to test
membership with high probability.

The cryptographic properties of the Golomb-Rice
Filter come from the fact that it is constructed
using cryptographic hash functions. Specifically,
it uses the SHA-256 hash function to map elements
to bit positions in a Bloom filter, which is then
compressed using Golomb-Rice encoding.

This construction provides collision resistance,
meaning that it is computationally infeasible to
find two distinct elements that hash to the same
bit position in the filter. It also provides the
property of false positive probability, which is
the probability that a non-member element is
incorrectly reported as a member of the set. The
false positive probability can be tuned by
adjusting the parameters of the filter, such as
the number of hash functions used and the size of
the filter.

Overall, the Golomb-Rice Filter provides
a space-efficient and secure way to test
membership in a set, making it useful in many
applications including the Bitcoin system.
