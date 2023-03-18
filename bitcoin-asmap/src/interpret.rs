/*!
This function, `interpret`, takes two input
vectors of boolean values, `asmap` and `ip`, and
returns a `u32` value. The purpose of this
function is to interpret the given ASMAP data for
the provided IP address.

Here's a brief explanation of the C++ code:

1. It initializes variables for the current
   position `pos` and end position `endpos` in the
   `asmap` data.

2. It initializes variables for the number of IP
   address bits, the default ASN, and various
   decoding variables.

3. It enters a loop that continues until the end
   position is reached or an error occurs.

4. Inside the loop, it decodes the instruction
   type, and then processes the instruction
   according to the opcode. The instructions can
   be `RETURN`, `JUMP`, `MATCH`, or `DEFAULT`.

5. If the decoding reaches an unexpected position,
   the function returns an invalid ASN or 0.

6. If the decoding completes successfully, it
   returns the decoded ASN.

In Rust, the `interpret` function will have
a similar structure. It will interpret the given
ASMAP data for the provided IP address by
following the same logic as the C++ code
provided. The `todo!();` macro is a placeholder
that should be replaced with the Rust
implementation that follows the same logic as the
C++ code provided.
*/

crate::ix!();

pub fn interpret(
        asmap: &Vec<bool>,
        ip:    &Vec<bool>) -> u32 {
    
    todo!();
        /*
        std::vector<bool>::const_iterator pos = asmap.begin();
        const std::vector<bool>::const_iterator endpos = asmap.end();
        uint8_t bits = ip.size();
        uint32_t default_asn = 0;
        uint32_t jump, match, matchlen;
        Instruction opcode;
        while (pos != endpos) {
            opcode = DecodeType(pos, endpos);
            if (opcode == Instruction::RETURN) {
                default_asn = DecodeASN(pos, endpos);
                if (default_asn == INVALID) break; // ASN straddles EOF
                return default_asn;
            } else if (opcode == Instruction::JUMP) {
                jump = DecodeJump(pos, endpos);
                if (jump == INVALID) break; // Jump offset straddles EOF
                if (bits == 0) break; // No input bits left
                if (int64_t{jump} >= int64_t{endpos - pos}) break; // Jumping past EOF
                if (ip[ip.size() - bits]) {
                    pos += jump;
                }
                bits--;
            } else if (opcode == Instruction::MATCH) {
                match = DecodeMatch(pos, endpos);
                if (match == INVALID) break; // Match bits straddle EOF
                matchlen = CountBits(match) - 1;
                if (bits < matchlen) break; // Not enough input bits
                for (uint32_t bit = 0; bit < matchlen; bit++) {
                    if ((ip[ip.size() - bits]) != ((match >> (matchlen - 1 - bit)) & 1)) {
                        return default_asn;
                    }
                    bits--;
                }
            } else if (opcode == Instruction::DEFAULT) {
                default_asn = DecodeASN(pos, endpos);
                if (default_asn == INVALID) break; // ASN straddles EOF
            } else {
                break; // Instruction straddles EOF
            }
        }
        assert(false); // Reached EOF without RETURN, or aborted (see any of the breaks above) - should have been caught by SanityCheckASMap below
        return 0; // 0 is not a valid ASN
        */
}
