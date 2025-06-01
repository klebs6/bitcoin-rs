// ---------------- [ File: bitcoin-bigint/src/simple_lcg.rs ]
crate::ix!();

pub struct SimpleLCG {
    state: u64,
}

impl SimpleLCG {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        self.state
    }
}

pub fn random_u256(rng: &mut SimpleLCG) -> BaseUInt256 {
    let mut out = BaseUInt256::default();
    let r0 = rng.next_u64();
    let r1 = rng.next_u64();
    let r2 = rng.next_u64();
    let r3 = rng.next_u64();
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
/// for convenience. 
pub fn random_hex_string(rng: &mut SimpleLCG, max_digits: usize) -> String {
    use std::fmt::Write as FmtWrite;
    let digit_count = (rng.next_u64() as usize % max_digits).max(1);
    let mut s = String::new();
    if (rng.next_u64() & 1) == 0 {
        s.push_str("0x");
    }
    for i in 0..digit_count {
        let nibble = (rng.next_u64() & 0xF) as u8;
        let c = std::char::from_digit(nibble as u32, 16).unwrap();
        if i > 0 && (rng.next_u64() & 7) == 0 {
            s.push('_');
        }
        if (rng.next_u64() & 1) == 0 {
            s.push(c.to_ascii_lowercase());
        } else {
            s.push(c.to_ascii_uppercase());
        }
    }
    s
}

#[cfg(test)]
mod test_simple_lcg {
    use super::*;
    use tracing::{info, debug};

    #[traced_test]
    fn test_lcg_new_seed_zero() {
        info!("Testing SimpleLCG::new(0) does not panic or misbehave.");
        let mut lcg = SimpleLCG::new(0);
        let first = lcg.next_u64();
        let second = lcg.next_u64();
        assert_ne!(first, second);
    }

    #[traced_test]
    fn test_lcg_new_seed_max() {
        info!("Testing SimpleLCG::new(u64::MAX).");
        let mut lcg = SimpleLCG::new(u64::MAX);
        let first = lcg.next_u64();
        let second = lcg.next_u64();
        assert_ne!(first, second);
    }

    #[traced_test]
    fn test_lcg_reproducibility() {
        info!("Verifying SimpleLCG yields a consistent sequence for the same seed.");
        let seed = 0xDEAD_BEEF_1234_5678;
        let mut lcg1 = SimpleLCG::new(seed);
        let seq1 = [
            lcg1.next_u64(),
            lcg1.next_u64(),
            lcg1.next_u64(),
            lcg1.next_u64(),
        ];
        let mut lcg2 = SimpleLCG::new(seed);
        let seq2 = [
            lcg2.next_u64(),
            lcg2.next_u64(),
            lcg2.next_u64(),
            lcg2.next_u64(),
        ];
        assert_eq!(seq1, seq2);
    }
}

#[cfg(test)]
mod test_random_u256 {
    use super::*;
    use tracing::{info, debug};

    #[traced_test]
    fn test_random_u256_basics() {
        info!("Testing random_u256 basic usage.");
        let mut rng = SimpleLCG::new(12345);
        let val1 = random_u256(&mut rng);
        let val2 = random_u256(&mut rng);
        assert_ne!(val1, val2);
    }

    #[traced_test]
    fn test_random_u256_consistency() {
        info!("Verifying random_u256 is reproducible for a fixed seed.");
        let seed = 0xABCDEF01_23456789;
        let mut rng1 = SimpleLCG::new(seed);
        let results1 = [
            random_u256(&mut rng1),
            random_u256(&mut rng1),
            random_u256(&mut rng1),
        ];
        let mut rng2 = SimpleLCG::new(seed);
        let results2 = [
            random_u256(&mut rng2),
            random_u256(&mut rng2),
            random_u256(&mut rng2),
        ];
        assert_eq!(results1, results2);
    }

    #[traced_test]
    fn test_random_u256_variety() {
        info!("Checking random_u256 returns different bits in multiple calls.");
        let mut rng = SimpleLCG::new(0x1111_2222_3333_4444);
        let sample_count = 8;
        let mut distinct_count = 0;
        let mut last_val = random_u256(&mut rng);
        for _ in 1..sample_count {
            let curr_val = random_u256(&mut rng);
            if curr_val != last_val {
                distinct_count += 1;
            }
            last_val = curr_val;
        }
        assert!(distinct_count >= 1, "No variety in random_u256 output!");
    }
}

#[cfg(test)]
mod test_random_hex_string {
    use super::*;
    use tracing::{info, debug};

    #[traced_test]
    fn test_hex_string_min_length() {
        info!("Testing random_hex_string with max_digits=1.");
        let mut rng = SimpleLCG::new(9999);
        let s = random_hex_string(&mut rng, 1);
        assert!(s.len() == 1 || s.len() == 3);
    }

    #[traced_test]
    fn test_hex_string_varying_lengths() {
        info!("Testing random_hex_string with multiple max_digits values.");
        let mut rng = SimpleLCG::new(0x1111_1111_1111_1111);
        for max_dig in [1, 2, 5, 10, 20, 40].iter() {
            let s = random_hex_string(&mut rng, *max_dig);
            assert!(s.len() >= 1);
            assert!(s.len() <= 2 + *max_dig * 2);
        }
    }

    #[traced_test]
    fn test_hex_string_underscores_and_prefix() {
        info!("Verifying random_hex_string can produce underscores and '0x'.");
        let mut rng = SimpleLCG::new(0xABCDEF0123456789);
        let tries = 50;
        let mut saw_underscore = false;
        let mut saw_prefix = false;
        for _ in 0..tries {
            let s = random_hex_string(&mut rng, 10);
            if s.contains('_') {
                saw_underscore = true;
            }
            if s.starts_with("0x") || s.starts_with("0X") {
                saw_prefix = true;
            }
            if saw_underscore && saw_prefix {
                break;
            }
        }
        assert!(saw_underscore, "No underscore found in many tries!");
        assert!(saw_prefix, "No '0x' prefix found in many tries!");
    }

    #[traced_test]
    fn test_hex_string_reproducibility() {
        info!("Verifying random_hex_string is reproducible for same seed.");
        let mut rng1 = SimpleLCG::new(0xDEAD_BEEF);
        let out1_1 = random_hex_string(&mut rng1, 10);
        let out1_2 = random_hex_string(&mut rng1, 10);
        let mut rng2 = SimpleLCG::new(0xDEAD_BEEF);
        let out2_1 = random_hex_string(&mut rng2, 10);
        let out2_2 = random_hex_string(&mut rng2, 10);
        assert_eq!(out1_1, out2_1);
        assert_eq!(out1_2, out2_2);
    }
}
