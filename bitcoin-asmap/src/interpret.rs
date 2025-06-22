// ---------------- [ File: bitcoin-asmap/src/interpret.rs ]
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

/// Interpret `asmap` for the given `ip`.  
/// Returns the mapped ASN or 0 on structural failure (which should be ruled
/// out in production by `sanity_check_as_map`).
pub fn interpret(asmap: &[bool], ip: &[bool]) -> u32 {
    let mut pos: usize = 0;
    let mut bits_left: i32 = ip.len() as i32;
    let mut default_asn: u32 = 0;

    while pos < asmap.len() {
        let opcode = decode_type(asmap, &mut pos);
        debug!(?opcode, pos, bits_left, "interpret: opcode");

        match opcode {
            Instruction::RETURN => {
                let asn = decodeasn(asmap, &mut pos);
                if asn == INVALID {
                    error!("interpret: ASN straddles EOF");
                    break;
                }
                return asn;
            }
            Instruction::JUMP => {
                let jump = decode_jump(asmap, &mut pos);
                if jump == INVALID || bits_left == 0 {
                    error!("interpret: invalid jump or no bits left");
                    break;
                }
                if (jump as usize) >= asmap.len() - pos {
                    error!("interpret: jump past EOF");
                    break;
                }
                let ip_bit = ip[ip.len() - bits_left as usize];
                if ip_bit {
                    pos += jump as usize;
                }
                bits_left -= 1;
            }
            Instruction::MATCH => {
                let m = decode_match(asmap, &mut pos);
                if m == INVALID {
                    error!("interpret: match straddles EOF");
                    break;
                }
                let matchlen = count_bits(m) - 1;
                if bits_left < matchlen as i32 {
                    error!("interpret: not enough input bits for match");
                    break;
                }
                for bit in 0..matchlen {
                    let ip_bit = ip[ip.len() - bits_left as usize];
                    let pattern_bit = ((m >> (matchlen - 1 - bit)) & 1) != 0;
                    if ip_bit != pattern_bit {
                        trace!("interpret: match failed, returning default_asn");
                        return default_asn;
                    }
                    bits_left -= 1;
                }
            }
            Instruction::DEFAULT => {
                default_asn = decodeasn(asmap, &mut pos);
                if default_asn == INVALID {
                    error!("interpret: DEFAULT ASN straddles EOF");
                    break;
                }
            }
        }
    }

    error!("interpret: terminated without RETURN – invalid ASMAP");
    0 // 0 is not a valid ASN
}
