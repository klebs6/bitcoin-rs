// ---------------- [ File: bitcoin-ripemd/src/round.rs ]
crate::ix!();

#[inline]
pub fn ripemd160_rol(x: u32, i: i32) -> u32 {
    // Cast once to avoid repeating conversions.
    let r = x.rotate_left(i as u32);
    trace!(
        target: "ripemd160::round",
        "rol({:#010x}, {}) -> {:#010x}",
        x,
        i,
        r
    );
    r
}

#[inline]
pub fn ripemd160_round(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    f: u32,
    x: u32,
    k: u32,
    r: i32,
) {
    trace!(
        target: "ripemd160::round",
        "round‑in  a={:#010x}, c={:#010x}, f={:#010x}, x={:#010x}, k={:#010x}, r={}",
        *a,
        *c,
        f,
        x,
        k,
        r
    );

    *a = ripemd160_rol(
        a.wrapping_add(f).wrapping_add(x).wrapping_add(k),
        r,
    )
    .wrapping_add(e);
    *c = ripemd160_rol(*c, 10);

    trace!(
        target: "ripemd160::round",
        "round‑out a={:#010x}, c={:#010x}",
        *a,
        *c
    );
}

#[cfg(test)]
mod spec_round {
    use super::*;

    #[traced_test]
    fn rol_matches_rotate_left() {
        let x = 0x1234_5678;
        for i in 0..32 {
            assert_eq!(ripemd160_rol(x, i), x.rotate_left(i as u32));
        }
    }

    #[traced_test]
    fn round_known_state_transition() {
        // Initial state values mirror canonical RIPEMD‑160 constants
        let mut a = 0x6745_2301;
        let b = 0xefcd_ab89;
        let mut c = 0x98ba_dcfe;
        let d = 0x1032_5476;
        let e = 0xc3d2_e1f0;

        // Arbitrary but fixed inputs
        let f = 0x1111_1111;
        let x = 0x2222_2222;
        let k = 0x3333_3333;
        let r = 13;

        ripemd160_round(&mut a, b, &mut c, d, e, f, x, k, r);

        assert_eq!(a, 0x34ff_dba5);
        assert_eq!(c, 0xeb73_fa62);
    }
}
