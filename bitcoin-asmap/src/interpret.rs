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
