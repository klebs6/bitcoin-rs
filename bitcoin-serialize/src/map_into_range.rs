// ---------------- [ File: bitcoin-serialize/src/map_into_range.rs ]
crate::ix!();

/**
  | Map a value x that is uniformly distributed in
  | the range [0, 2^64) to a value uniformly
  | distributed in [0, n) by returning the upper 64
  | bits of x * n.
  |
  | See:
  | https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
  */
pub fn map_into_range(x: u64, n: u64) -> u64 {
    
    #[cfg(__SIZEOF_INT128__)]
    {
        return (x as u128 * n as u128) >> 64;
    }


    // To perform the calculation on 64-bit
    // numbers without losing the result to
    // overflow, split the numbers into the
    // most significant and least significant
    // 32 bits and perform multiplication
    // piece-wise.
    //
    // See:
    // https://stackoverflow.com/a/26855440
    let x_hi:    u64 = x >> 32;
    let x_lo:    u64 = x & 0xFFFFFFFF;
    let n_hi:    u64 = n >> 32;
    let n_lo:    u64 = n & 0xFFFFFFFF;
    let ac:      u64 = x_hi * n_hi;
    let ad:      u64 = x_hi * n_lo;
    let bc:      u64 = x_lo * n_hi;
    let bd:      u64 = x_lo * n_lo;
    let mid34:   u64 = (bd >> 32) + (bc & 0xFFFFFFFF) + (ad & 0xFFFFFFFF);
    let upper64: u64 = ac + (bc >> 32) + (ad >> 32) + (mid34 >> 32);

    upper64
}

#[cfg(test)]
mod map_into_range_tests {
    use super::*;

    fn reference(x: u64, n: u64) -> u64 {
        (((x as u128) * (n as u128)) >> 64) as u64
    }

    #[traced_test]
    fn matches_reference() {
        const N: u64 = 1_000_003; // arbitrary odd prime
        for x in [
            0u64,
            1,
            2,
            3,
            42,
            1 << 20,
            u64::MAX / 2,
            u64::MAX,
        ] {
            assert_eq!(map_into_range(x, N), reference(x, N));
        }
    }

    #[traced_test]
    fn result_always_less_than_n() {
        const N: u64 = 123_456_789;
        for x in [0, 1, u64::MAX / 3, u64::MAX] {
            let y = map_into_range(x, N);
            assert!(y < N, "map_into_range({x}, {N}) = {y} ≥ {N}");
        }
    }

    #[traced_test]
    fn n_equal_one_maps_to_zero() {
        for x in [0u64, 1, 2, 3, u64::MAX] {
            assert_eq!(map_into_range(x, 1), 0);
        }
    }
}
