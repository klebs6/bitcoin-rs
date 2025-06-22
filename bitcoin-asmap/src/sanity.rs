// ---------------- [ File: bitcoin-asmap/src/sanity.rs ]
/*
This function, `sanity_check_as_map`, checks if
the given ASMAP (`asmap`) is well-formed and does
not have any inconsistencies. It takes
a `Vec<bool>` as input, which represents the ASMAP
data, and an `i32` called `bits`, which represents
the number of bits available in the address. The
function returns a boolean indicating whether the
ASMAP is well-formed and consistent.

The C++ code provided performs a series of checks
on the ASMAP data while iterating through it. It
checks for issues like instructions straddling the
end of file (EOF), unreachable code, incorrect
jump offsets, and more. The function returns
`true` if the ASMAP is well-formed and consistent,
and `false` otherwise.

In Rust, the `sanity_check_as_map` function will
follow a similar structure. The `todo!();` macro
is a placeholder that should be replaced with
a Rust implementation that follows the same logic
as the C++ code provided. The Rust implementation
will iterate through the ASMAP data and perform
the necessary checks, returning a boolean
indicating whether the ASMAP is well-formed and
consistent.
*/

crate::ix!();


// ---------------- [ File: bitcoin-asmap/src/sanity.rs ]

/// Exhaustive structural validation, exactly mirroring the original C++ logic.
pub fn sanity_check_as_map(asmap: &[bool], mut bits: i32) -> bool {
    let mut pos: usize = 0;
    let mut jumps: Vec<(u32, i32)> = Vec::with_capacity(bits as usize); // (offset, bits_left)
    let mut prev_opcode = Instruction::JUMP;
    let mut had_incomplete_match = false;

    while pos < asmap.len() {
        let offset = pos as u32;
        if jumps.last().map_or(false, |jump| offset >= jump.0) {
            error!("sanity: jump into middle of previous instruction");
            return false;
        }

        let opcode = decode_type(asmap, &mut pos);
        trace!(?opcode, pos, bits, "sanity: opcode");

        match opcode {
            Instruction::RETURN => {
                if prev_opcode == Instruction::DEFAULT {
                    error!("sanity: RETURN immediately after DEFAULT");
                    return false;
                }
                let asn = decodeasn(asmap, &mut pos);
                if asn == INVALID {
                    error!("sanity: ASN straddles EOF");
                    return false;
                }
                if jumps.is_empty() {
                    // verify padding
                    if asmap.len() - pos > 7 {
                        error!("sanity: excessive padding");
                        return false;
                    }
                    if asmap[pos..].iter().any(|&b| b) {
                        error!("sanity: non‑zero padding bits");
                        return false;
                    }
                    return true;
                } else {
                    let (jump_offset, restored_bits) = jumps.pop().unwrap();
                    if offset != jump_offset {
                        error!("sanity: unreachable code after RETURN");
                        return false;
                    }
                    bits = restored_bits;
                    prev_opcode = Instruction::JUMP;
                }
            }
            Instruction::JUMP => {
                let jump = decode_jump(asmap, &mut pos);
                if jump == INVALID || (jump as usize) > asmap.len() - pos {
                    error!("sanity: invalid jump");
                    return false;
                }
                if bits == 0 {
                    error!("sanity: consuming bits past end of input");
                    return false;
                }
                bits -= 1;
                let jump_offset = pos as u32 + jump;
                if jumps.last().map_or(false, |j| jump_offset >= j.0) {
                    error!("sanity: intersecting jumps");
                    return false;
                }
                jumps.push((jump_offset, bits));
                prev_opcode = Instruction::JUMP;
            }
            Instruction::MATCH => {
                let m = decode_match(asmap, &mut pos);
                if m == INVALID {
                    error!("sanity: MATCH straddles EOF");
                    return false;
                }
                let matchlen = count_bits(m) as i32 - 1;
                if prev_opcode != Instruction::MATCH {
                    had_incomplete_match = false;
                }
                if matchlen < 8 && had_incomplete_match {
                    error!("sanity: multiple short MATCHes in a row");
                    return false;
                }
                had_incomplete_match = matchlen < 8;
                if bits < matchlen {
                    error!("sanity: MATCH consumes past end of input");
                    return false;
                }
                bits -= matchlen;
                prev_opcode = Instruction::MATCH;
            }
            Instruction::DEFAULT => {
                if prev_opcode == Instruction::DEFAULT {
                    error!("sanity: consecutive DEFAULTs");
                    return false;
                }
                let asn = decodeasn(asmap, &mut pos);
                if asn == INVALID {
                    error!("sanity: DEFAULT ASN straddles EOF");
                    return false;
                }
                prev_opcode = Instruction::DEFAULT;
            }
        }
    }

    error!("sanity: reached EOF without RETURN");
    false
}
