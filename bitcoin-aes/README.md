# bitcoin-aes

This Rust crate is a subcomponent of the Bitcoin
system and is currently in the process of being
translated from C++ to Rust. Some function bodies
are still in the process of translation. The
`bitcoin-aes` crate provides functionality for AES
encryption and decryption, specifically for
AES-128, AES-192, and AES-256 in CBC mode.

The crate includes several functions such as
`AES256CBCDecrypt`, `AES256CBCEncrypt`,
`AES256Decrypt`, and `AES256Encrypt` which
implement the AES block cipher algorithm for
256-bit keys in CBC mode with the corresponding
key length. The CBC mode encryption and decryption
functions `cbc_encrypt` and `cbc_decrypt` utilize
the AES encryption and decryption functions along
with padding and initialization vector generation
to provide a secure method for encrypting and
decrypting messages.

In addition, the crate includes functions for AES
key setup, such as `aes_setup`, which initializes
an AES key context and sets up the key for use
with the encryption and decryption functions. The
key setup functions for AES-128, AES-192, and
AES-256 are `aes128_init`, `aes192_init`, and
`aes256_init`, respectively.

The crate also contains various utility functions
such as `from_hex`, `print_number`, and
`gettimedouble` for converting data types and
measuring performance.

The mathematical concepts utilized in this crate
include the AES block cipher algorithm, which is
based on substitution-permutation network (SPN)
and is defined as a set of mathematical operations
on bytes, including substitution, permutation, and
linear transformation. The AES algorithm also
involves a key schedule that generates round keys
from the initial cipher key. The CBC mode of
operation involves chaining previous ciphertext
blocks with the current plaintext block, along
with initialization vectors and padding, to
provide security against various attacks.

Overall, the `bitcoin-aes` crate provides a secure
and efficient implementation of AES encryption and
decryption in CBC mode, which is a crucial
component of the Bitcoin system.

*Note: This crate description was written by an AI
and is meant to serve as a guideline for what
might be contained within.*

2> thanks, could you please describe AES for me in
more detail? how is it parametrized? what are its
inputs and outputs? how does it work internally?

Certainly!

AES (Advanced Encryption Standard) is a symmetric
block cipher algorithm used to encrypt and decrypt
data. It is a widely used cryptographic algorithm
that is considered secure and has been adopted as
a standard by the U.S. government.

The AES algorithm operates on blocks of data,
where each block consists of 16 bytes (128
bits). The key length for AES can be 128, 192, or
256 bits. The key length determines the number of
rounds that are performed in the
algorithm. Specifically, the number of rounds for
AES-128 is 10, AES-192 is 12, and AES-256 is 14.

The inputs to AES are a plaintext block and
a key. The plaintext block is a 16-byte block of
data that is to be encrypted or decrypted. The key
is a sequence of bytes that is used to perform the
encryption or decryption. The length of the key
can be 128, 192, or 256 bits.

The output of AES is a ciphertext block, which is
also a 16-byte block of data. The ciphertext block
is the encrypted version of the plaintext block.

Internally, the AES algorithm is based on
a substitution-permutation network (SPN)
structure. The algorithm consists of several
rounds of operations, with each round consisting
of four operations: SubBytes, ShiftRows,
MixColumns, and AddRoundKey.

The SubBytes operation involves replacing each
byte of the input block with a corresponding byte
from a fixed table called the S-box. The ShiftRows
operation involves shifting the rows of the input
block by a certain number of bytes. The MixColumns
operation involves mixing the columns of the input
block using a matrix multiplication. The
AddRoundKey operation involves XORing the input
block with a round key that is generated from the
main key.

The key schedule for AES involves generating
a series of round keys from the main key. The
number of round keys that are generated depends on
the key length and the number of rounds for the
algorithm. Each round key is generated from the
previous round key using a combination of
operations such as rotating bytes, applying the
S-box, and performing XOR operations.

In summary, AES is a symmetric block cipher
algorithm that operates on blocks of data with
a fixed size of 16 bytes. It uses a key of length
128, 192, or 256 bits to perform encryption or
decryption. The algorithm consists of several
rounds of operations, including SubBytes,
ShiftRows, MixColumns, and AddRoundKey, and
utilizes a key schedule to generate round keys for
each round of operations.

3> could you describe in more detail the details
abou t key length and number of rounds?

Certainly!

The AES algorithm can be configured with different
key lengths, which determines the strength of the
encryption. There are three different key lengths
that are commonly used: 128-bit, 192-bit, and
256-bit.

The key length determines the number of rounds
that are performed in the algorithm. A round is
a sequence of operations that is performed on the
plaintext block using a different round key. The
number of rounds that are performed depends on the
key length, as follows:

- For a 128-bit key, the number of rounds is 10.
- For a 192-bit key, the number of rounds is 12.
- For a 256-bit key, the number of rounds is 14.

Each round involves a sequence of four operations:
SubBytes, ShiftRows, MixColumns, and
AddRoundKey. The first three operations are
non-linear and introduce confusion and diffusion
to the plaintext block. The AddRoundKey operation
is linear and adds the round key to the state of
the block.

The SubBytes operation involves applying
a substitution table called the S-box to each byte
of the state. The S-box is a fixed table that maps
each byte value to another byte value. This
operation is designed to provide confusion by
introducing non-linearity into the encryption
process.

The ShiftRows operation involves shifting the rows
of the state matrix by a certain number of
bytes. Specifically, the first row is not shifted,
the second row is shifted by one byte to the left,
the third row is shifted by two bytes to the left,
and the fourth row is shifted by three bytes to
the left. This operation is designed to provide
diffusion by spreading the byte values across the
state matrix.

The MixColumns operation involves mixing the
columns of the state matrix using a matrix
multiplication. This operation is designed to
provide diffusion by ensuring that each byte of
the output depends on all bytes of the input.

The AddRoundKey operation involves XORing the
state matrix with a round key that is derived from
the main key. The round key is generated using
a key schedule, which is a sequence of operations
that generates a set of round keys from the main
key. The key schedule is designed to ensure that
each round key is different from the previous
round key and that the keys are uniformly
distributed across the key space.

In summary, the key length and number of rounds
are important parameters for the AES algorithm, as
they determine the strength of the encryption and
the number of rounds that are performed in the
encryption process. The key length can be 128-bit,
192-bit, or 256-bit, while the number of rounds
depends on the key length and is 10, 12, or 14 for
128-bit, 192-bit, and 256-bit keys, respectively.

4> how do we measure the strength of encryption at
each setting?

The strength of encryption for each setting of the
AES algorithm can be measured by estimating the
amount of effort required to break the
encryption. This effort is typically measured in
terms of the number of operations that an attacker
would need to perform in order to recover the
plaintext from the ciphertext.

For example, the strength of AES-128 encryption
can be estimated by considering the best known
attacks on the algorithm and estimating the number
of operations that an attacker would need to
perform in order to recover the plaintext from the
ciphertext. The best known attacks on AES-128 are
brute-force attacks, which involve trying every
possible key until the correct one is found, and
related-key attacks, which exploit weaknesses in
the key schedule to recover the key. Brute-force
attacks on AES-128 require 2^128 operations on
average, which is considered computationally
infeasible given current technology. Related-key
attacks on AES-128 require significantly fewer
operations, but they are still considered
impractical in most scenarios.

Similarly, the strength of AES-192 and AES-256
encryption can be estimated by considering the
best known attacks on the algorithms and
estimating the number of operations that an
attacker would need to perform in order to recover
the plaintext from the ciphertext. These attacks
are typically more complex than attacks on AES-128
due to the larger key size and the increased
number of rounds.

Overall, the strength of AES encryption depends on
a variety of factors, including the key length,
the number of rounds, the quality of the key
schedule, and the specific attack scenarios that
are considered. Estimating the strength of
encryption for a given setting typically involves
analyzing the best known attacks on the algorithm
and considering the computational resources that
an attacker would need to perform these attacks.

5> could you please describe the key schedule?
what are the possible attack scenarios?

The key schedule is an algorithm used by AES to
expand a relatively short secret key into a longer
set of round keys that are used in the encryption
and decryption process. The key schedule is an
important component of AES because it ensures that
each round of the encryption process uses a unique
key.

The key schedule for AES-128 generates 11 round
keys, each of which is 128 bits long. The key
schedule for AES-192 generates 13 round keys, each
of which is 192 bits long. The key schedule for
AES-256 generates 15 round keys, each of which is
256 bits long.

The key schedule involves a combination of simple
bitwise operations, such as XOR and bit rotations,
as well as more complex operations such as the
SubBytes and MixColumns transformations. The exact
details of the key schedule algorithm vary
depending on the key size and the specific
implementation of the algorithm.

One possible attack scenario against AES involves
weaknesses in the key schedule algorithm that
allow an attacker to recover the secret key from
a set of known plaintext-ciphertext pairs. Known
as a related-key attack, this type of attack
exploits weaknesses in the key schedule to
generate related keys that can be used to recover
the secret key.

Another possible attack scenario against AES
involves weaknesses in the implementation of the
algorithm that allow an attacker to recover the
secret key through side-channel attacks. These
attacks exploit information leakage from the
implementation of the algorithm, such as power
consumption or electromagnetic radiation, to
recover the secret key.

Overall, the key schedule is an important
component of AES that plays a critical role in
ensuring the security of the algorithm. However,
weaknesses in the key schedule algorithm or the
implementation of the algorithm can potentially be
exploited by attackers to recover the secret key
and compromise the security of the encryption. As
a result, it is important to use a high-quality
implementation of AES and to follow best practices
for securing cryptographic keys.

6> how does AES relate to bitcoin? where does it
appear in the protocol?

AES encryption is used in several places within
the Bitcoin protocol to protect sensitive data and
provide security. One notable example is the use
of AES-256 encryption in Bitcoin wallet software
to protect the private keys that are used to sign
transactions and spend funds.

Private keys in Bitcoin are 256-bit numbers, and
are typically represented as a sequence of 64
hexadecimal digits. To protect the privacy and
security of these keys, Bitcoin wallet software
typically uses AES-256 encryption to encrypt the
private keys and store them on disk.

When a user wants to spend their Bitcoin funds,
they must first enter their password or passphrase
to decrypt the private key. The wallet software
then uses the decrypted private key to sign the
transaction and spend the funds.

AES encryption is also used in other parts of the
Bitcoin protocol, such as in the encryption of
network traffic between nodes and in the
encryption of data stored on disk by mining
software.

Overall, AES encryption plays a critical role in
the security of the Bitcoin protocol by protecting
sensitive data and providing a high level of
security for cryptographic keys.

7> what else should we know about AES?

Here are some additional things to know about AES:

- AES is a widely used and trusted encryption
  algorithm that is considered secure against
  known attacks when implemented correctly.

- AES is a symmetric key encryption algorithm,
  meaning that the same secret key is used for
  both encryption and decryption.

- AES is a block cipher, meaning that it encrypts
  data in fixed-size blocks. The block size for
  AES is 128 bits.

- AES supports key sizes of 128, 192, and 256
  bits. The larger the key size, the more secure
  the encryption.

- AES uses a fixed number of rounds for encryption
  and decryption, with the number of rounds
  depending on the key size. AES-128 uses 10
  rounds, AES-192 uses 12 rounds, and AES-256 uses
  14 rounds.

- AES is resistant to most known attacks,
  including brute-force attacks, differential and
  linear cryptanalysis, and most known
  side-channel attacks.

- AES has been standardized by several
  organizations, including the US National
  Institute of Standards and Technology (NIST),
  and is widely used in many applications,
  including electronic payment systems, virtual
  private networks (VPNs), and secure messaging
  applications.

Overall, AES is a well-studied and widely used
encryption algorithm that provides strong security
and is considered a fundamental building block of
modern cryptography.

8> how long would it take to brute force AES on
the different settings?

The time required to brute force AES encryption
depends on the key size and the attacker's
computing power. Brute force attacks involve
trying every possible key until the correct one is
found, and the number of possible keys increases
exponentially with the key size. As a result,
larger key sizes require exponentially more
computational power to brute force.

Here are some estimated times to brute force AES
encryption using current technology and assuming
a brute force attack that can check 1 billion keys
per second:

- AES-128: The key size is 128 bits, which means
  there are 2^128 possible keys. At 1 billion keys
  per second, it would take approximately 10^17
  years to try all possible keys.

- AES-192: The key size is 192 bits, which means
  there are 2^192 possible keys. At 1 billion keys
  per second, it would take approximately 10^31
  years to try all possible keys.

- AES-256: The key size is 256 bits, which means
  there are 2^256 possible keys. At 1 billion keys
  per second, it would take approximately 10^56
  years to try all possible keys.

These timescales are so large that brute force
attacks against AES are not considered a practical
threat, and the security of AES is based on the
assumption that it isk computationally infeasible
to break the encryption by brute force. However,
other types of attacks, such as side-channel
attacks or attacks on the implementation of the
algorithm, may be able to bypass the encryption
and are potential threats to its security.

9> what are side-channel attacks and how do we
protect against them?

Side-channel attacks are a type of attack that
exploit information leaked by a cryptographic
system, rather than directly attacking the
cryptographic algorithm or key. They take
advantage of physical characteristics of the
implementation, such as power consumption,
electromagnetic radiation, or timing, to extract
information about the secret key or plaintext.

Some examples of side-channel attacks include
power analysis, electromagnetic radiation
analysis, and timing analysis. In a power analysis
attack, the attacker measures the power consumed
by the device during cryptographic operations and
uses this information to deduce the value of the
secret key. In an electromagnetic radiation
analysis attack, the attacker measures the
electromagnetic radiation emitted by the device
during cryptographic operations and uses this
information to deduce the value of the secret
key. In a timing analysis attack, the attacker
measures the time taken by the device to perform
cryptographic operations and uses this information
to deduce the value of the secret key.

To protect against side-channel attacks, it is
important to use cryptographic implementations
that are designed to resist these attacks. This
can be achieved by using techniques such as
masking, which involves adding random values to
the secret key or plaintext to make it more
difficult for an attacker to extract useful
information from the device's behavior. Other
techniques include constant-time implementations,
which ensure that the time taken by the device to
perform cryptographic operations is independent of
the input data, making timing attacks more
difficult. Additionally, physical countermeasures
such as shielding or noise generation can be used
to reduce the leakage of electromagnetic
radiation.

In summary, side-channel attacks are a serious
threat to the security of cryptographic systems,
and it is important to use implementations that
are designed to resist these attacks in order to
ensure the confidentiality and integrity of
sensitive data.

10> are there actually ways to measure the EM
radiation and attack AES? is this frequent? are
there notable examples? how is shielding
implemented? what about noise generation?

Yes, electromagnetic radiation analysis (EMA) is
a real attack method that can be used to extract
secret information from cryptographic devices. EMA
works by measuring the electromagnetic radiation
emitted by the device during cryptographic
operations, and using this information to deduce
the value of the secret key. This type of attack
can be especially effective against devices that
lack proper shielding, or that use vulnerable
components such as unhardened microcontrollers or
clock oscillators.

While EMA attacks are not as common as other types
of attacks, they are a serious threat to the
security of cryptographic devices, especially in
high-security environments where attackers have
access to sophisticated equipment and
techniques. There have been several notable
examples of EMA attacks, including the 2013
discovery of a vulnerability in RSA keys generated
by a popular smartcard, which allowed attackers to
extract the private key using EMA techniques.

To protect against EMA attacks, it is important to
use devices that are designed to resist these
attacks. One approach is to use physical
shielding, which involves enclosing the
cryptographic device in a metal case or shield
that blocks or absorbs electromagnetic
radiationk. Another approach is to use noise
generation techniques, which involve adding random
or unpredictable signals to the device's output to
make it more difficult for attackers to extract
useful information from the electromagnetic
radiation.

There are various techniques for implementing
shielding, depending on the specific device and
environment. One common approach is to use
a Faraday cage, which is a conductive enclosure
that blocks electromagnetic radiation. This can be
implemented using metal plates or mesh, and can be
effective at shielding against a wide range of
frequencies. Other techniques for shielding
include using conductive coatings or films, or
adding ferrite beads or other electromagnetic
absorbers to the device's circuitry.

Noise generation techniques can also be effective
at reducing the leakage of electromagnetic
radiation. One approach is to add random values to
the device's output using a technique called
dithering, which can make it more difficult for
attackers to extract useful information from the
electromagnetic radiation. Another approach is to
use hardware-based noise generation techniques,
such as using a random number generator or a noise
source to generate random signals that are added
to the device's output.

In summary, EMA attacks are a real threat to the
security of cryptographic devices, and it is
important to use physical shielding and noise
generation techniques to protect against these
attacks. Implementing these techniques can help
ensure the confidentiality and integrity of
sensitive data, especially in high-security
environments where attackers may have access to
sophisticated equipment and techniques.

11> does this attack require that the attacker has
possession of the device in question?

Yes, most side-channel attacks, including the
electromagnetic (EM) attack on AES, require the
attacker to have physical possession of the device
being attacked. This is because the attacker needs
to be able to directly access the device's
hardware or perform close-range measurements of
its electromagnetic emissions, which is typically
not possible without physical access. However,
there are some attacks that can be carried out
remotely, such as power analysis attacks over
a network connection, although these are typically
more difficult to execute and less effective than
physical attacks.

12> which circuit components need to be faraday
caged?

In general, any circuit component that contains
sensitive or confidential data or processing
should be protected by a Faraday cage or shield to
prevent electromagnetic (EM) leakage and other
forms of side-channel attacks. This may include
components such as:

- Microprocessors and CPUs
- Cryptographic hardware accelerators
- Memory chips (e.g. RAM, ROM, flash)
- Communication interfaces (e.g. Wi-Fi, Bluetooth, cellular)
- Power management circuits
- Sensors (e.g. cameras, microphones)

However, the specific components that need to be
protected will depend on the design and purpose of
the device, as well as the potential attack
vectors and threats that it may face. It is
generally a good practice to conduct a thorough
security analysis and risk assessment of the
device and its components to identify the most
critical areas that require protection.

13> what else should we know about AES? what about
in its relation to bitcoin?

Here are a few additional points about AES and its
relationship to Bitcoin:

- AES is a widely used symmetric-key encryption
  algorithm that is considered secure against most
  known attacks. It is used in a wide variety of
  applications, including data storage,
  communication protocols, and authentication
  mechanisms.

- In the context of Bitcoin, AES is used in
  several key components of the protocol,
  including the generation and protection of
  private keys, the encryption and decryption of
  network traffic, and the storage and retrieval
  of wallet data.

- Specifically, AES-256 is used to encrypt private
  keys stored in Bitcoin wallets, as well as to
  secure communications between Bitcoin nodes over
  the network. AES is also used in conjunction
  with other cryptographic algorithms, such as
  SHA-256 and RIPEMD-160, to generate and verify
  Bitcoin addresses.

- AES is not the only encryption algorithm used in
  Bitcoin. For example, elliptic curve
  cryptography (ECC) is used to generate key pairs
  and sign transactions, and the secp256k1 curve
  is used to implement the Bitcoin-specific
  version of ECC.

- The security of Bitcoin and other
  cryptocurrencies relies heavily on the strength
  of their cryptographic algorithms and protocols,
  including AES. As such, there is ongoing
  research and development in the field of
  cryptography to improve and adapt these
  technologies to new threats and challenges.
