# `bitcoin-sha512` Crate

The `bitcoin-sha512` crate is a direct translation
of the SHA-512 hash function used in the Bitcoin
system from C++ to Rust. The crate is currently in
the process of translation and it is possible that
some function bodies are still being worked on.

The SHA-512 hash function takes an input message
of any length and produces a fixed-length output
of 512 bits. It uses a series of modular
arithmetic operations and logical operations to
transform the input message into a digest.

This crate provides functions for initializing the
hash function, feeding data into the hash
function, and finalizing the hash function to
produce the output digest. It also includes
functions for the HMAC-SHA512 message
authentication code, which uses the SHA-512 hash
function as a building block.

The mathematical concepts involved in this crate
include modular arithmetic and logical
operations. The SHA-512 hash function uses
a series of constants and functions, including
`ch`, `maj`, `sigma0`, and `sigma1`, to perform
the transformations on the input message.

The `bitcoin-sha512` crate includes functions for
initializing and resetting the SHA-512 hash
function, feeding data into the function, and
finalizing the function to produce the output
digest. It also includes functions for the
HMAC-SHA512 message authentication code, which
uses the SHA-512 hash function as a building
block.

Please note that this crate is a work in progress
and some function bodies may still be in the
process of being translated.

## Example Usage

```rust
use bitcoin_sha512::{Sha512, HmacSha512};

let mut sha512 = Sha512::default();
sha512.feed_data_in(b"test");
sha512.feed_data_in(b" message");
let output = sha512.finalize();
assert_eq!(output[..], [0x43, 0xf4, 0xdb, 0xf4, 0x9b, 0x2d, 0xaa, 0x15,
                        0x34, 0x78, 0x9c, 0x0d, 0x60, 0x89, 0xc6, 0x61,
                        0x98, 0x6b, 0x3c, 0x1e, 0x11, 0x11, 0x9a, 0x1c,
                        0x0f, 0xc2, 0x5d, 0x5d, 0x5d, 0xd8, 0xba, 0x14,
                        0xf6, 0x37, 0x3d, 0xe6, 0x2a, 0x3f, 0xa8, 0x36,
                        0x70, 0xa8, 0x1c, 0x37, 0xb4, 0xd4, 0x92, 0x9e,
                        0x4e, 0x6d, 0xd4, 0x88, 0xba, 0xb1, 0x2c, 0x31,
                        0x56, 0x15, 0x52, 0x60, 0x5c, 0x46, 0x12, 0x21][..]);

let mut hmac_sha512 = HmacSha512::default();
hmac_sha512.feed_key(b"key");
hmac_sha512.feed_data_in(b"test
```

135> thanks -- how is it different from sha-256?

SHA-512 is a variant of the SHA-2 family of
cryptographic hash functions, just like
SHA-256. However, SHA-512 uses 64-bit words and 80
rounds, as opposed to SHA-256 which uses 32-bit
words and 64 rounds. This results in a longer
output of 512 bits instead of 256 bits, making
SHA-512 a more secure option than
SHA-256. Additionally, SHA-512 has higher
computational complexity, making it slower than
SHA-256, but more resistant to brute-force
attacks.

136> are these the only differences?

No, there are a few other differences between
SHA-256 and SHA-512. One of the main differences
is that SHA-512 operates on 1024-bit blocks,
whereas SHA-256 operates on 512-bit blocks. This
means that SHA-512 can process more data per
block, but it also requires more processing power.

Another difference is that SHA-512 uses larger
constants in its internal functions, which
provides greater security against certain types of
attacks. Additionally, SHA-512 has a larger
internal state, which provides greater resistance
against collision attacks.

Overall, SHA-512 is considered to be a stronger
hash function than SHA-256, but it also requires
more resources to run. In the context of Bitcoin,
SHA-256 is still the preferred hash function for
most purposes, but SHA-512 could potentially be
used in the future if there is a need for stronger
security.

137> is the overall architecture the same from
a network topological structure?

Yes, the overall network topological structure of
Bitcoin is the same regardless of the hashing
algorithm used (SHA-256 or SHA-512). The Bitcoin
network is a decentralized peer-to-peer network
where nodes communicate with each other to
maintain a shared ledger of transactions. The
nodes are connected to each other in a network
topology that can be described as a mesh or
overlay network. Nodes are not required to trust
each other and can independently verify the
validity of transactions and blocks using
cryptographic proofs. The consensus mechanism used
by the Bitcoin network, known as Proof of Work
(PoW), also remains the same regardless of the
hashing algorithm used.

138> no, I meant the network topological structure
of the sha256 and sha512 algorithms.  is it the
same network flow but with different size
d components? or are the topological structures of
the two algorithms different?

The network topological structures of the SHA-256
and SHA-512 algorithms are different. SHA-256 uses
a Merkle-Damgard construction, which is based on
a sequential iteration of compression
functions. In contrast, SHA-512 uses a different
construction called the Miyaguchi-Preneel
construction, which is based on a parallel
iteration of compression functions. This means
that SHA-512 is designed to be more efficient on
64-bit processors and can process messages in
128-bit blocks, whereas SHA-256 is more efficient
on 32-bit processors and can only process messages
in 64-bit blocks. Overall, the network topological
structures of the two algorithms are different due
to their different constructions and block sizes.

139> thanks -- could you teach me the
Merkle-Damgard construction?

Sure, the Merkle-Damgard construction is a method
for creating a hash function based on
a compression function. It is named after Ralph
Merkle and Ivan Damgard who independently proposed
the construction in the late 1970s.

The basic idea behind the construction is to break
up a message into blocks, process each block
through a compression function, and then combine
the output of the compression functions to produce
a final hash value. The compression function takes
as input a fixed-size block of data and outputs
a fixed-size digest, which is then fed back into
the function along with the next block of data.

Here is a simplified diagram of the Merkle-Damgard
construction:

```
+-----------+       +-----------+       +-----------+
|  Message  |  -->  |  Block 1  |  -->  |  Digest 1 |
+-----------+       +-----------+       +-----------+
                      |          |       +-----------+
                      |          |  -->  |  Digest 2 |
                      |          |       +-----------+
                      |          |               :
                      |          |               :
                      |          |       +-----------+
                      |          |  -->  |  Digest N |
                      |          |       +-----------+
                      +-----------+
```

The Merkle-Damgard construction is used by many
popular hash functions, including MD5, SHA-1, and
SHA-2. However, it is susceptible to length
extension attacks, which is why the SHA-3
algorithm uses a different construction based on
the sponge function.

140> thanks -- what about the Miyaguchi-Preneel ?

The Miyaguchi-Preneel (MP) construction is a hash
function construction that is similar to the
Merkle-Damgard construction, but uses a different
approach to address some of the weaknesses of the
Merkle-Damgard construction. The MP construction
was proposed in 1990 by Tetsu Iwata and Kaoru
Kurosawa.

Like the Merkle-Damgard construction, the MP
construction takes an input message and processes
it in a series of fixed-length blocks. However,
instead of using a compression function to process
each block, the MP construction uses a block
cipher to mix the input block with the previous
output block before feeding it into the next
round. The output of the final block is then
hashed using a separate compression function.

The use of a block cipher in the MP construction
provides an additional layer of security, as it
makes it more difficult for an attacker to
manipulate the input message in a way that would
result in a collision in the output
hash. Additionally, the use of a separate
compression function for the final block provides
an added level of security, as it makes it more
difficult for an attacker to manipulate the final
block to produce a collision.

However, the MP construction is generally
considered to be slower than the Merkle-Damgard
construction, due to the additional overhead
required to run the block cipher for each input
block.

Overall, while the MP construction offers some
improvements overk the Merkle-Damgard construction
in terms of security, it is not as widely used in
practice, due to its slower speed and more complex
implementation.
