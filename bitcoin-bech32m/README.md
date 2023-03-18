# bitcoin-bech32m

The `bitcoin-bech32m` crate is a Rust
implementation of the Bech32m encoding scheme used
in the Bitcoin network. This crate is currently in
the process of being translated from C++ to Rust,
so some of the function bodies may still be in the
process of translation.

The Bech32m encoding scheme is used to represent
Bitcoin addresses and is designed to be
human-readable and error-resistant. The Bech32m
encoding scheme is an extension of the original
Bech32 encoding scheme, which was introduced in
BIP 173 to encode SegWit addresses. The Bech32m
encoding scheme adds support for variable-length
checksums and lower-case characters.

The `bitcoin-bech32m` crate provides functions for
encoding and decoding Bech32m addresses, as well
as functions for creating and verifying
checksums. The `Encoding` struct provides an
interface for encoding and decoding Bech32m
addresses, while the `DecodeResult` struct
represents the result of a decoding operation.

The `cat` function concatenates two byte arrays,
the `encode` function encodes a byte array as
a Bech32m address, and the `create_checksum`
function creates a checksum for a byte array. The
`expand_hrp` function expands a human-readable
part of a Bech32m address, the `lower_case`
function converts a byte array to lower-case, and
the `verify_checksum` function verifies the
checksum of a Bech32m address.

The `poly_mod` function calculates the Bech32m
checksum polynomial for a given byte array, and
the `encoding_constant` represents the constant
used in the Bech32m checksum calculation.

Overall, the `bitcoin-bech32m` crate provides
a convenient and reliable way to encode and decode
Bech32m addresses in the Bitcoin network, using an
error-resistant encoding scheme that is designed
to be human-readable and easy to use.

17> could you please describe bech32 and bech32m
in more detail?

Certainly!

Bech32 and Bech32m are both encoding schemes used
to represent Bitcoin addresses. They were
introduced in BIP 173 and BIP 350, respectively.

Bech32 was introduced as a way to encode SegWit
addresses. It is a human-readable encoding scheme
that uses a combination of lowercase letters and
numbers to represent data. It uses a checksum to
detect errors in the encoding process, and it
supports a variable-length checksum to allow for
future upgrades.

Bech32m is an extension of Bech32 that was
introduced to support lower-case letters in
Bitcoin addresses. It uses a different checksum
algorithm than Bech32, which allows for
variable-length checksums and lower-case letters.

The checksum algorithm used in Bech32m is based on
a generator polynomial of degree 32. The
polynomial is represented by a binary number,
which is then converted into a sequence of 5-bit
values. These values are used to compute
a checksum by processing the Bech32m address in
chunks of 5 bits, with the most significant bit of
each chunk being XORed with the corresponding
value from the polynomial.

The result of this computation is a sequence of 10
5-bit values, which are appended to the original
Bech32m address to form the final encoded
address. The length of the checksum depends on the
length of the original data being encoded, with
longer data resulting in a longer checksum.

Overall, Bech32 and Bech32m are both important
encoding schemes used in the Bitcoin network to
represent addresses. They are designed to be
human-readable and error-resistant, and they
provide a convenient and reliable way to encode
and decode Bitcoin addresses.

18> what is a generator polynomial and how do we
specify them? what are they used for and what are
their strengths? do they have any drawbacks?

A generator polynomial is a polynomial used in
cyclic error-correcting codes, such as those used
in Bech32m, to generate the parity check bits used
in error detection and correction.

In the case of Bech32m, the generator polynomial
is a polynomial of degree 32, represented as
a binary number. This binary number is then
converted into a sequence of 5-bit values using
a specific algorithm. The resulting sequence of
5-bit values is then used to compute the checksum
for the Bech32m address.

The generator polynomial is specified by the
designer of the error-correcting code, and is
chosen to have specific properties that make it
suitable for the application. For example, the
polynomial may be chosen to be irreducible,
meaning that it cannot be factored into the
product of two smaller polynomials. This property
makes the code more robust against errors, as it
ensures that the errors are detected and corrected
properly.

Generator polynomials have several strengths. They
are efficient and simple to implement, and can be
used to generate error-detection and correction
codes with a high degree of accuracy. They can
also be used to generate codes of different
lengths, allowing for flexibility in the design of
error-correcting systems.

However, generator polynomials also have some
drawbacks. They are limited in their ability to
correct errors beyond a certain threshold, and can
become less effective as the number of errors
increases. Additionally, the choice of generator
polynomial can have a significant impact on the
performance of the error-correcting code, and
selecting the appropriate polynomial can be
a challenging task.

19> could you please compare and contrast Bech32
and Bech32m?

Bech32 and Bech32m are two different address
encoding formats used in the Bitcoin network.

Bech32 is a newer address format introduced in
2017 that uses a base32 encoding scheme to
represent Bitcoin addresses. It was proposed as
a replacement for the previous address format,
known as Base58Check, which had some limitations
in terms of error detection and correction. Bech32
addresses start with "bc1" and are
case-insensitive.

Bech32m is a modification of Bech32 that was
introduced in 2021 to address a potential
vulnerability in the original Bech32 address
format. This vulnerability could allow an attacker
to construct a valid Bitcoin address that is
different from the intended address. Bech32m
addresses start with "tb1" for the Bitcoin test
network and "bc1m" for the Bitcoin main network,
and are also case-insensitive.

The main difference between Bech32 and Bech32m is
the way the checksum is computed. Bech32 uses
a generator polynomial of degree 5, while Bech32m
uses a generator polynomial of degree 32. The
higher degree of the generator polynomial used in
Bech32m makes it more resilient to certain types
of errors and also reduces the likelihood of
address collisions.

Another difference between the two formats is that
Bech32m includes an additional guard bit in the
checksum calculation, which provides an additional
layer of error detection and correction. This
guard bit ensures that the checksum is always
non-zero and provides an extra check to detect
errors in the address.

In summary, Bech32 and Bech32m are both address
encoding formats used in the Bitcoin network, with
Bech32m being a modification of Bech32 designed to
address a potential vulnerability in the original
format. The main differences between the two
formats are the degree of the generator polynomial
used in the checksum calculation and the inclusion
of an additional guard bit in Bech32m.

20> do you think there will need to be further
upgrades to the address encoding format?

It's possible that there will be further upgrades
to the address encoding format in the future, as
the Bitcoin network continues to evolve and new
security threats emerge.

While Bech32 and Bech32m are currently the most
widely used address formats in the Bitcoin
network, there is always the possibility that new
vulnerabilities or weaknesses could be discovered
in these formats. In addition, as the Bitcoin
network scales and new use cases emerge, there may
be a need for additional features or capabilities
in the address encoding format.

However, any proposed upgrades to the address
encoding format would need to be carefully
evaluated and tested to ensure that they do not
introduce new vulnerabilities or create
compatibility issues with existing software and
systems. This process can take time and require
significant collaboration and coordination among
developers, miners, and other stakeholders in the
Bitcoin ecosystem.

Overall, while it's possible that there may be
further upgrades to the address encoding format in
the future, any such changes would need to be
carefully considered and thoroughly tested to
ensure their safety and effectiveness in the
Bitcoin network.
