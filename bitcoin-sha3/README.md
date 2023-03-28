# `bitcoin-sha3` crate

The `bitcoin-sha3` crate is a Rust implementation
of SHA3-256, one of the SHA3 family of hash
functions selected as a standard by NIST. This
crate is part of a direct translation of the
Bitcoin codebase from C++ to Rust, and is
currently in the process of translation. Some
function bodies may still be undergoing
translation.

The SHA3-256 hash function is based on the Keccak
sponge function, which uses a permutation function
called `keccakf`. The `bitcoin-sha3` crate
includes an implementation of the `keccakf`
function, as well as functions for resetting the
hash state, finalizing the hash and writing input
data.

While there may not be any specific mathematical
equations or concepts involved in the
`bitcoin-sha3` crate, it is important to note that
SHA3-256 is a cryptographic hash function designed
to provide collision resistance, preimage
resistance, and second preimage resistance. These
properties make it suitable for use in
applications such as digital signatures, password
storage, and blockchain technology.

Overall, the `bitcoin-sha3` crate provides a Rust
implementation of the SHA3-256 hash function,
allowing for efficient and secure computation of
message digests in Bitcoin-related applications.

117> thanks -- could you describe this keccak
sponge function and its permutation in more depth?

Certainly! The Keccak sponge function is a family
of hash functions that use the sponge construction
to process input data. The sponge construction was
first introduced in 2007 by Bertoni, Daemen,
Peeters, and Van Assche, and was later chosen as
thek basis for the SHA-3 hash function, which uses
Keccak as its underlying sponge function.

At a high level, the sponge function works by
repeatedly applying a permutation function to
a state array, which alternates between absorbing
input data and squeezing out output data. The
permutation function used in Keccak is known as
the Keccak-f permutation, which is a family of
permutations parameterized by its capacity and
bitrate. The permutation operates on a state
array, which consists of a number of lanes, each
of which is a fixed number of bits (in the case of
SHA3-256, each lane is 64 bits).

The Keccak-f permutation consists of a number of
rounds, where each round consists of five stages:
theta, rho, pi, chi, and iota. The theta stage is
used to add diffusion to the state by xoring each
lane with a linear combination of the lanes in its
surrounding columns. The rho stage is used to
perform bit rotation operations on each lane,
which helps to spread information throughout the
state. The pi stage is used to permute the lanes
in a particular way. The chi stage is used to
apply a nonlinear transformation to each lane,
which helps to increase the complexity of the
permutation. Finally, the iota stage is used to
xor a particular round constant with a single
lane, which helps to ensure that each round is
different.

The number of rounds and the values of the round
constants used in Keccak-f depend on the capacity
and bitrate parameters, which determine the
overall security level of the permutation. In the
case of SHA3-256, the Keccak-f permutation is used
with a capacity of 512 bits and a bitrate of 1088
bits, and consists of 24 rounds.

Overall, the Keccak sponge function and its
permutation are designed to provide a high level
of security while remaining efficient and easy to
implement in hardware and software. The use of the
sponge construction allows for flexible processing
of input data, and the Keccak-f permutation
provides a strong level of diffusion and confusion
to ensure the security of the hash function.

118> thanks -- could you show me a block diagram
for it?

Sure! Here is a simplified block diagram of the
Keccak sponge function:

```
       Message   Sponge   Output
         / \       |        |
         | |       |        |
         | |       |        |
         | |       |        |
        [+]--------|       [+]------+
         | |       |        |       |
         | |       |        |       |
         | |       |        |       |
         | |       |        |       |
        [F] <-----[XOR]-----[F]     |
         | |       |        |       |
         | |       |        |       |
         | |       |        |       |
         | |       |        |       |
        [+]--------|       [+]-----+
         | |       |        |
         | |       |        |
         | |       |        |
         | |       |        |
        [F] <-----[XOR]-----[F]
         | |       |        |
         | |       |        |
         | |       |        |
         | |       |        |
        [+]--------|       [+]-----+
         |         |        |       |
         |         |        |       |
         |         |        |       |
         |         |        |       |
        [F] <-----[XOR]-----[F]     |
         |         |        |       |
         |         |        |       |
         |         |        |       |
         |         |        |       |
        [+]--------|       [+]------+
         |         |        |
         |         |        |
         |         |        |
         |         |        |
        [F] <-----[XOR]-----[F]
         |         |        |
         |         |        |
         |         |        |
         |         |        |
        [+]--------|       [+]-----+
         |         |        |       |
         |         |        |       |
         |         |        |       |
         |         |        |       |
        [F] <-----[XOR]-----[F]     |
         |         |        |       |
         |         |        |       |
         |         |        |       |
         |         |        |       |
        [+]--------|       [+]------+
         |         |        |
         |         |        |
         |         |        |
         |         |        |
        [F] <-----[XOR]-----[F]
         |         |        |
         |         |        |
         |         |        |
         |         |        |
        [+]--------|       [+]-----+
         |         |        |       |
         |         |        |       |
         |         |        |       |
         |         |        |       |
        [F] <-----[XOR]-----[F]     |
         |         |        |       |
         |         |        |       |
         |         |        |       |
         |         |        |       |
        [+]--------|       [+]------+
         |         |        |
         |         |        |
         |         |        |
         |         |        |
        [F] <-----[XOR]-----[F]
         |         |        |
         |         |        |
         |         |        |
         |         |        |
        [+]--------|       [+]-----+
         |         |        |       |
         |         |        |       |
         |         |        |       |
         |         |        |       |
        [F] <-----[XOR
```

119> thanks -- keep going

The Keccak sponge function can be thought of as
a black box with two inputs and two outputs. The
inputs are the message to be hashed and the length
of the digest to be produced. The outputs are the
hash value and the padding that was added to the
message to bring it up to the nearest multiple of
the block size.

The sponge function operates in two phases: the
absorbing phase and the squeezing phase. In the
absorbing phase, the message is processed block by
block. Each block is XORed with the internal
state, and the resulting value is fed into the
permutation function. After all the blocks have
been processed, the state is in a "saturated"
state, meaning that it has absorbed as much
information as it can from the input message.

In the squeezing phase, the sponge function
generates the output hash value by repeatedly
applying the permutation function to the internal
state and extracting the output bits. This process
continues until the required number of output bits
has been generated.

The permutation function itself is a key part of
the Keccak sponge function. It is a non-linear
function that operates on the internal state of
the sponge. The permutation function consists of
a sequence of "rounds", each of which applies
a combination of linear and non-linear operations
to the state. The number of rounds and the
specific operations used depend on the block size
and the desired level of security.

Overall, the Keccak sponge function and its
permutation offer a secure and flexible way to
generate hash values of varying lengths.

120> how do we measure diffusion and confusion?

In cryptography, diffusion and confusion are two
important properties that ensure the security of
the encryption scheme.

Diffusion refers to the idea that changing one bit
of the plaintext should cause multiple bits of the
ciphertext to change. This makes it difficult for
an attacker to deduce any information about the
plaintext from the ciphertext.

Confusion refers to the idea that the relationship
between the plaintext and the key should be so
complex that it is difficult to determine the key
from the ciphertext.

These properties can be measured mathematically
using various techniques, including Shannon
entropy and avalanche effect. The Shannon entropy
measures the amount of randomness in a system,
while the avalanche effect measures the amount of
change in the output when the input is changed.

A good cryptographic algorithm should exhibit high
levels of both diffusion and confusion, making it
difficult for an attacker to reverse-engineer the
key or deduce any information about the plaintext
from the ciphertext.

121> what is the avalanche effect in more depth?
what about shannon entropy?

The avalanche effect is an important property of
cryptographic algorithms that describes how small
changes in the input can result in large,
unpredictable changes in the output. In other
words, if we make a small change to the input of
a cryptographic algorithm, the output should
change significantly, and in an unpredictable way.

The avalanche effect is important because it helps
to ensure that an attacker cannot easily find
a correlation between the input and the output of
the algorithm. If an algorithm does not exhibit
a strong avalanche effect, an attacker might be
able to make small changes to the input and
observe how the output changes, which could help
the attacker to deduce information about the
input.

Shannon entropy is a measure of the amount of
uncertainty or randomness in a system. In the
context of cryptography, Shannon entropy is often
used to measure the unpredictability of
a cryptographic key or other secret value. If
a key has high entropy, it means that there is
a lot of uncertainty about the key, and it ise
difficult for an attacker to guess the key by
brute force.

Shannon entropy is usually measured in bits, and
it is calculated based on the probability
distribution of the possible values that a random
variable can take. For example, if we have
a 128-bit key and all possible 128-bit values are
equally likely, then the Shannon entropy of the
key is 128 bits. This means that there are 2^128
possible keys, and an attacker would need to try
an average of half of them in order to find the
correct key by brute force.

122> does this have to do with chaos?

The avalanche effect and Shannon entropy are not
directly related to chaos theory. Chaos theory
deals with the study of nonlinear, dynamic systems
that are highly sensitive to initial conditions,
whereas the avalanche effect and Shannon entropy
are concepts in cryptography that relate to the
randomness and unpredictability of the output of
a cryptographic function or algorithm. However,
there are some connections between chaos theory
and cryptography, particularly in the area of
chaos-based cryptography, which uses chaotic
dynamical systems to generate keys or encrypt
data.

123> it sounds like chaos theory is similar to the
avalanche effect

Yes, there is a connection between the avalanche
effect and chaos theory. Chaos theory deals with
the behavior of dynamic systems that are highly
sensitive to initial conditions, and small
differences in initial conditions can result in
vastly different outcomes. This sensitivity to
initial conditions can lead to a system that
exhibits the avalanche effect, where small changes
in the input to the system can result in a large
change in the output. The avalanche effect in
cryptography is desirable because it provides
a high degree of diffusion and makes it difficult
to predict the output of a cryptographic function
even if you know the input.

124> what are some other ideas from chaos theory
which relate to cryptography?

Chaos theory has many applications in
cryptography, some of which include:

1. Pseudo-random number generation: Chaos theory
   can be used to generate unpredictable sequences
   of random numbers that are useful for
   encryption purposes. One way to achieve this is
   by using chaotic maps, which are non-linear
   functions that exhibit chaotic behavior. By
   iterating a chaotic map with an initial value
   (seed), a sequence of seemingly random numbers
   can be generated.

2. Cryptographic key generation: Chaotic systems
   can also be used to generate cryptographic
   keys. By taking advantage of the chaotic
   behavior of a system, it is possible to
   generate a sequence of bits that can be used as
   a key. One way to achieve this is by using
   a chaotic system as a source of entropy, which
   can be used to seed a cryptographically secure
   pseudo-random number generator (CSPRNG).

3. Cryptanalysis: Chaos theory can also be used to
   analyze the security of cryptographic
   algorithms. By studying the dynamics of
   a chaotic system, it is possible to identify
   vulnerabilities in a cryptographic algorithm
   and develop attacks to exploit them.

4. Secure communications: Chaotic systems can be
   used to secure communications by encoding
   messages using chaotic signals. By transmitting
   a signal that is synchronized with the
   receiver, it is possible to encode a message in
   a way that is difficult to intercept or
   decipher.

Overall, the chaotic behavior of dynamic systems
provides a rich source of randomness and
unpredictability that can be harnessed for use in
cryptography.

125> thanks -- what else should we know about
sha3?

SHA-3 was chosen as the winner of the NIST hash
function competition in 2012, and it was formally
standardized in 2015 as FIPS PUB 202. It is
designed to provide stronger security than SHA-2,
particularly in the context of so-called
"length-extension attacks". In addition to the
SHA3-256 variant, there are also SHA3-224,
SHA3-384, and SHA3-512 variants with different
output sizes.

One interesting property of SHA-3 is that it is
based on a different internal structure than SHA-2
or SHA-1, called the "sponge construction". This
structure has a number of benefits, including the
ability to support variable-length input and
output sizes, and a design that is more resistant
to side-channel attacks.

While SHA-3 has not yet been widely adopted in the
same way as SHA-256, it is increasingly being used
in various applications, including
cryptocurrencies and blockchain systems, where
security is of critical importance.

126> why is it called the sponge construction?

The sponge construction is so named because it
absorbs ("sponges up") the input message data and
then squeezes out the output hash values. This
process is analogous to a physical sponge, which
absorbs liquid and then can be squeezed to release
that liquid again. The sponge construction was
first introduced in the context of hash functions
by J.-P. Aumasson, O. Bernhard, and S. Neves in
their 2009 paper "Sponge Functions."

127> why has sha3 not been as widely adopted as
sha256?

There could be multiple reasons why SHA-3 has not
been as widely adopted as SHA-256. One reason is
that SHA-3 is a relatively new hash function, and
it takes time for new cryptographic algorithms to
gain widespread acceptance and
adoption. Additionally, SHA-256 is already widely
used in many applications, and there may not be
a compelling need to switch to SHA-3. Another
reason is that SHA-3 is a relatively slow
algorithm compared to SHA-256, and in some cases,
speed may be a more important consideration than
security. Finally, the current security of SHA-256
is considered sufficient for many applications,
and there may not be a need to switch to a more
secure hash function like SHA-3.

128> so, sha3 is on deck, ready to go in case
there is a problem with sha256.  is that correct?

Yes, that's correct. SHA-3 has been designed as
a backup option to SHA-2 (which includes SHA-256)
in case there is a need for a new hash function
due to any security issues or limitations
discovered in SHA-2.

129> okay good.  what should we know about it in
the context of the bitcoin system?

In the context of the Bitcoin system, SHA-3 is not
currently used as the hashing algorithm. As of
2023, the Bitcoin system still uses SHA-256 as the
primary hashing algorithm. However, SHA-3 may be
used as a backup or alternative option in the
future if SHA-256 is found to be vulnerable or for
other reasons. Additionally, SHA-3 may be used in
other applications related to the Bitcoin system,
such as in the development of other cryptographic
protocols or tools.

130> what about sha3 supports variable length
output sizes?  how does that work exactyl?

SHA3 supports variable length output sizes through
a parameter called "c". The value of "c"
determines the maximum output length of the hash
function, and can take on values of 224, 256, 384,
and 512.

Internally, the SHA3 function uses a sponge
construction that absorbs the input message and
then squeezes out the hash value. The sponge
construction works by partitioning the internal
state of the hash function into two parts: the
"capacity" and the "rate". The capacity is fixed
and determines the security of the hash function,
while the rate can be adjusted to determine the
output length.

To generate a hash value with a specific output
length, the sponge construction is initialized
with the appropriate capacity value and then
a message is absorbed using the rate value. The
hash function is then squeezed to produce an
output with the desired length.

For example, to generate a SHA3-256 hash with an
output length of 160 bits (20 bytes), the sponge
construction would be initialized with a capacity
of 256 bits and a rate of 448 bits (512 bits - 160
bits), and then a message would be absorbed using
the rate value. The hash function would then be
squeezed to produce a 160-bit output.

131> what other parameters does sha3 have other
than c?

In addition to the capacity `c`, SHA-3 also has
a variable parameter `r`, which represents the
bitrate. The rate and capacity together determine
the security level and the output size.

The SHA-3 standard defines the following values
for `r`:

- `r = 1152` for SHA-3-224
- `r = 1088` for SHA-3-256
- `r = 832` for SHA-3-384
- `r = 576` for SHA-3-512

These values are chosen such that the bitrate plus
the capacity equals the width of the internal
state, which is fixed at 1600 bits for SHA-3.

The SHA-3 standard also allows for a variable
length message input, which is padded with the
appropriate delimiter and padding before being
processed by the sponge construction.

132> how much slower is sha3 than sha256? what is
the instruction throughput?

The performance of SHA3 compared to SHA256 depends
on various factors such as the implementation,
platform, and the specific use case. In general,
SHA3 is expected to be slower than SHA256 due to
its larger state size and more complex operations.

The instruction throughput of SHA3 depends on the
platform and implementation. For example, on an
Intel Core i5-6600K processor running at 3.5 GHz,
the instruction throughput of SHA256 is around 8.7
cycles per byte, while that of SHA3-256 is around
24 cycles per byte. However, it's worth noting
that these numbers can vary significantly based on
the implementation details and the specific
platform.

133> what is the typical memory performance?

The memory performance of SHA-3 depends on the
size of the input data, the chosen hash output
size, and the hardware implementation of the
algorithm. Generally, SHA-3 is memory-bound,
meaning that its performance is limited by the
memory bandwidth available to the algorithm. The
throughput of SHA-3 can be improved by using
hardware acceleration, such as ASICs or FPGAs, or
by optimizing the software implementation. The
memory performance of SHA-3 can also be influenced
by the number of rounds used in the permutation
function and the size of the state. However, the
exact memory performance of SHA-3 can vary widely
depending on the specific implementation and the
hardware platform used.
