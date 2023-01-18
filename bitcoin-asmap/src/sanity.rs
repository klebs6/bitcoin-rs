crate::ix!();

pub fn sanity_check_as_map(
    asmap: &Vec<bool>,
    bits:  i32) -> bool 
{
    todo!();
        /*
        const std::vector<bool>::const_iterator begin = asmap.begin(), endpos = asmap.end();
        std::vector<bool>::const_iterator pos = begin;
        std::vector<std::pair<uint32_t, int>> jumps; // All future positions we may jump to (bit offset in asmap -> bits to consume left)
        jumps.reserve(bits);
        Instruction prevopcode = Instruction::JUMP;
        bool had_incomplete_match = false;
        while (pos != endpos) {
            uint32_t offset = pos - begin;
            if (!jumps.empty() && offset >= jumps.back().first) return false; // There was a jump into the middle of the previous instruction
            Instruction opcode = DecodeType(pos, endpos);
            if (opcode == Instruction::RETURN) {
                if (prevopcode == Instruction::DEFAULT) return false; // There should not be any RETURN immediately after a DEFAULT (could be combined into just RETURN)
                uint32_t asn = DecodeASN(pos, endpos);
                if (asn == INVALID) return false; // ASN straddles EOF
                if (jumps.empty()) {
                    // Nothing to execute anymore
                    if (endpos - pos > 7) return false; // Excessive padding
                    while (pos != endpos) {
                        if (*pos) return false; // Nonzero padding bit
                        ++pos;
                    }
                    return true; // Sanely reached EOF
                } else {
                    // Continue by pretending we jumped to the next instruction
                    offset = pos - begin;
                    if (offset != jumps.back().first) return false; // Unreachable code
                    bits = jumps.back().second; // Restore the number of bits we would have had left after this jump
                    jumps.pop_back();
                    prevopcode = Instruction::JUMP;
                }
            } else if (opcode == Instruction::JUMP) {
                uint32_t jump = DecodeJump(pos, endpos);
                if (jump == INVALID) return false; // Jump offset straddles EOF
                if (int64_t{jump} > int64_t{endpos - pos}) return false; // Jump out of range
                if (bits == 0) return false; // Consuming bits past the end of the input
                --bits;
                uint32_t jump_offset = pos - begin + jump;
                if (!jumps.empty() && jump_offset >= jumps.back().first) return false; // Intersecting jumps
                jumps.emplace_back(jump_offset, bits);
                prevopcode = Instruction::JUMP;
            } else if (opcode == Instruction::MATCH) {
                uint32_t match = DecodeMatch(pos, endpos);
                if (match == INVALID) return false; // Match bits straddle EOF
                int matchlen = CountBits(match) - 1;
                if (prevopcode != Instruction::MATCH) had_incomplete_match = false;
                if (matchlen < 8 && had_incomplete_match) return false; // Within a sequence of matches only at most one should be incomplete
                had_incomplete_match = (matchlen < 8);
                if (bits < matchlen) return false; // Consuming bits past the end of the input
                bits -= matchlen;
                prevopcode = Instruction::MATCH;
            } else if (opcode == Instruction::DEFAULT) {
                if (prevopcode == Instruction::DEFAULT) return false; // There should not be two successive DEFAULTs (they could be combined into one)
                uint32_t asn = DecodeASN(pos, endpos);
                if (asn == INVALID) return false; // ASN straddles EOF
                prevopcode = Instruction::DEFAULT;
            } else {
                return false; // Instruction straddles EOF
            }
        }
        return false; // Reached EOF without RETURN instruction
        */
}

