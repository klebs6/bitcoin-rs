crate::ix!();

// Simple linear congruential generator from earlier
pub struct SimpleLCG {
    state: u64,
}

impl SimpleLCG {

    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        self.state
    }
}

// Instead of capturing `rng` in a closure, define a standalone function:
pub fn random_u256(rng: &mut SimpleLCG) -> BaseUInt<256> {
    let mut out = BaseUInt::<256>::default();
    let r0 = rng.next_u64();
    let r1 = rng.next_u64();
    let r2 = rng.next_u64();
    let r3 = rng.next_u64();
    // Store in little-endian limbs
    out.pn[0] = (r0 & 0xffff_ffff) as u32;
    out.pn[1] = ((r0 >> 32) & 0xffff_ffff) as u32;
    out.pn[2] = (r1 & 0xffff_ffff) as u32;
    out.pn[3] = ((r1 >> 32) & 0xffff_ffff) as u32;
    out.pn[4] = (r2 & 0xffff_ffff) as u32;
    out.pn[5] = ((r2 >> 32) & 0xffff_ffff) as u32;
    out.pn[6] = (r3 & 0xffff_ffff) as u32;
    out.pn[7] = ((r3 >> 32) & 0xffff_ffff) as u32;
    out
}

/// Helper to produce a random hex string. We'll keep it to at most 40 hex digits
/// for convenience, but it can still exceed some smaller bit widths.
pub fn random_hex_string(rng: &mut SimpleLCG, max_digits: usize) -> String {
    // We'll produce [1..max_digits] nibbles of random hex.
    // Then possibly prefix with "0x" half the time, and possibly underscores occasionally.
    use std::fmt::Write;

    let digit_count = (rng.next_u64() as usize % max_digits).max(1);
    let mut s = String::new();

    // 50% chance to prepend "0x"
    if (rng.next_u64() & 1) == 0 {
        s.push_str("0x");
    }

    for i in 0..digit_count {
        let nibble = (rng.next_u64() & 0xF) as u8;
        let c = std::char::from_digit(nibble as u32, 16).unwrap();
        // Maybe sometimes insert an underscore
        if i > 0 && (rng.next_u64() & 7) == 0 {
            s.push('_');
        }
        // 50% chance to uppercase
        if (rng.next_u64() & 1) == 0 {
            s.push(c.to_ascii_lowercase());
        } else {
            s.push(c.to_ascii_uppercase());
        }
    }
    s
}
