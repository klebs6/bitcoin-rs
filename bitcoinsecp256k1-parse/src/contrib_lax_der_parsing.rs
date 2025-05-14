// ---------------- [ File: bitcoinsecp256k1-parse/src/contrib_lax_der_parsing.rs ]
/*!
  | Please do not link this file directly.
  | It is not part of the libsecp256k1 project
  | and does not promise any stability in
  | its API, functionality or presence.
  | Projects which use this code should
  | instead copy this header and its accompanying
  | .c file directly into their codebase.
  | **
  |
  -------------------------
  | This file defines a function that parses
  | DER with various errors and violations.
  | This is not a part of the library itself,
  | because the allowed violations are
  | chosen arbitrarily and do not follow
  | or establish any standard.
  | 
  | In many places it matters that different
  | implementations do not only accept
  | the same set of valid signatures, but
  | also reject the same set of signatures.
  | The only means to accomplish that is
  | by strictly obeying a standard, and
  | not accepting anything else.
  | 
  | Nonetheless, sometimes there is a need
  | for compatibility with systems that
  | use signatures which do not strictly
  | obey DER. The snippet below shows how
  | certain violations are easily supported.
  | You may need to adapt it.
  | 
  | Do not use this for new systems. Use well-defined
  | DER or compact signatures instead if
  | you have the choice (see ecdsa_signature_parse_der
  | and ecdsa_signature_parse_compact).
  | 
  | The supported violations are:
  | 
  | - All numbers are parsed as nonnegative
  | integers, even though X.609-0207 section
  | 8.3.3 specifies that integers are always
  | encoded as two's complement.
  | 
  | - Integers can have length 0, even though
  | section 8.3.1 says they can't.
  | 
  | - Integers with overly long padding
  | are accepted, violation section 8.3.2.
  | 
  | - 127-byte long length descriptors
  | are accepted, even though section 8.1.3.5.c
  | says that they are not.
  | 
  | - Trailing garbage data inside or after
  | the signature is ignored.
  | 
  | - The length descriptor of the sequence
  | is ignored.
  | 
  | Compared to for example OpenSSL, many
  | violations are NOT supported:
  | 
  | - Using overly long tag descriptors
  | for the sequence or integers inside,
  | violating section 8.1.2.2.
  | 
  | - Encoding primitive integers as constructed
  | values, violating section 8.3.1.
  |
  */

crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/contrib/lax_der_parsing.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/contrib/lax_der_parsing.c]

/** 
 | Parse a signature in "lax DER" format
 |
 |  Returns: 1 when the signature could be parsed,
 |  0 otherwise.
 |
 |  Args: ctx:      a secp256k1 context object
 |
 |  Out:  sig:      a pointer to a signature
 |                  object
 |
 |  In:   input:    a pointer to the signature to
 |                  be parsed
 |
 |        inputlen: the length of the array
 |                  pointed to be input
 |
 |  This function will accept any valid DER
 |  encoded signature, even if the encoded numbers
 |  are out of range. In addition, it will accept
 |  signatures which violate the DER spec in
 |  various ways. Its purpose is to allow
 |  validation of the Bitcoin blockchain, which
 |  includes non-DER signatures from before the
 |  network rules were updated to enforce
 |  DER. Note that the set of supported violations
 |  is a strict subset of what OpenSSL will
 |  accept.
 |
 |  After the call, sig will always be
 |  initialized. If parsing failed or the encoded
 |  numbers are out of range, signature validation
 |  with it is guaranteed to fail for every
 |  message and public key.
 */
pub fn ecdsa_signature_parse_der_lax(
        ctx:      *const Secp256k1Context,
        sig:      *mut Secp256k1EcdsaSignature,
        input:    *const u8,
        inputlen: usize) -> i32 {
    
    todo!();
        /*
            size_t rpos, rlen, spos, slen;
        size_t pos = 0;
        size_t lenbyte;
        unsigned char tmpsig[64] = {0};
        int overflow = 0;

        /* Hack to initialize sig with a correctly-parsed but invalid signature. */
        ecdsa_signature_parse_compact(ctx, sig, tmpsig);

        /* Sequence tag byte */
        if (pos == inputlen || input[pos] != 0x30) {
            return 0;
        }
        pos++;

        /* Sequence length bytes */
        if (pos == inputlen) {
            return 0;
        }
        lenbyte = input[pos++];
        if (lenbyte & 0x80) {
            lenbyte -= 0x80;
            if (lenbyte > inputlen - pos) {
                return 0;
            }
            pos += lenbyte;
        }

        /* Integer tag byte for R */
        if (pos == inputlen || input[pos] != 0x02) {
            return 0;
        }
        pos++;

        /* Integer length for R */
        if (pos == inputlen) {
            return 0;
        }
        lenbyte = input[pos++];
        if (lenbyte & 0x80) {
            lenbyte -= 0x80;
            if (lenbyte > inputlen - pos) {
                return 0;
            }
            while (lenbyte > 0 && input[pos] == 0) {
                pos++;
                lenbyte--;
            }
            if (lenbyte >= sizeof(size_t)) {
                return 0;
            }
            rlen = 0;
            while (lenbyte > 0) {
                rlen = (rlen << 8) + input[pos];
                pos++;
                lenbyte--;
            }
        } else {
            rlen = lenbyte;
        }
        if (rlen > inputlen - pos) {
            return 0;
        }
        rpos = pos;
        pos += rlen;

        /* Integer tag byte for S */
        if (pos == inputlen || input[pos] != 0x02) {
            return 0;
        }
        pos++;

        /* Integer length for S */
        if (pos == inputlen) {
            return 0;
        }
        lenbyte = input[pos++];
        if (lenbyte & 0x80) {
            lenbyte -= 0x80;
            if (lenbyte > inputlen - pos) {
                return 0;
            }
            while (lenbyte > 0 && input[pos] == 0) {
                pos++;
                lenbyte--;
            }
            if (lenbyte >= sizeof(size_t)) {
                return 0;
            }
            slen = 0;
            while (lenbyte > 0) {
                slen = (slen << 8) + input[pos];
                pos++;
                lenbyte--;
            }
        } else {
            slen = lenbyte;
        }
        if (slen > inputlen - pos) {
            return 0;
        }
        spos = pos;

        /* Ignore leading zeroes in R */
        while (rlen > 0 && input[rpos] == 0) {
            rlen--;
            rpos++;
        }
        /* Copy R value */
        if (rlen > 32) {
            overflow = 1;
        } else if (rlen) {
            memcpy(tmpsig + 32 - rlen, input + rpos, rlen);
        }

        /* Ignore leading zeroes in S */
        while (slen > 0 && input[spos] == 0) {
            slen--;
            spos++;
        }
        /* Copy S value */
        if (slen > 32) {
            overflow = 1;
        } else if (slen) {
            memcpy(tmpsig + 64 - slen, input + spos, slen);
        }

        if (!overflow) {
            overflow = !ecdsa_signature_parse_compact(ctx, sig, tmpsig);
        }
        if (overflow) {
            memset(tmpsig, 0, 64);
            ecdsa_signature_parse_compact(ctx, sig, tmpsig);
        }
        return 1;
        */
}
