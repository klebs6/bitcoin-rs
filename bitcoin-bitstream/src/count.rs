// ---------------- [ File: bitcoin-bitstream/src/count.rs ]
crate::ix!();

#[instrument(level = "trace")]
pub fn count_bits(x: u64) -> u64 {
    info!("count_bits invoked with x={}", x);
    // "Return the smallest number n such that (x >> n) == 0 (or 64 if highest bit is set)."
    // Equivalently, in Rust we can do:
    let count = if x == 0 {
        0
    } else {
        64 - x.leading_zeros() as u64
    };
    debug!("count_bits returning {}", count);
    count
}

#[cfg(test)]
mod test_count_bits {
    use super::*;

    #[traced_test]
    fn test_various_counts() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(1), 1);
        // 0b10 => 2 bits
        assert_eq!(count_bits(2), 2);
        // 0b111 => 3 bits
        assert_eq!(count_bits(7), 3);
        // 0b1000 => 4 bits
        assert_eq!(count_bits(8), 4);
        // 0xFFFFFFFFFFFFFFFF => 64 bits
        assert_eq!(count_bits(u64::MAX), 64);
    }
}
