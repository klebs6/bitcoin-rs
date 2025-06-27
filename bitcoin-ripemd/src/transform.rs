// ---------------- [ File: bitcoin-ripemd/src/transform.rs ]
crate::ix!();

/**
  | Perform one RIPEMD‑160 compression on
  | a single 64‑byte message chunk.
  */
#[inline]
pub fn ripemd160_transform(s: *mut u32, chunk: *const u8) {
    use crate::{
        ripemd160_r11 as R11, ripemd160_r12 as R12, ripemd160_r21 as R21, ripemd160_r22 as R22,
        ripemd160_r31 as R31, ripemd160_r32 as R32, ripemd160_r41 as R41, ripemd160_r42 as R42,
        ripemd160_r51 as R51, ripemd160_r52 as R52,
    };

    unsafe {
        /* ----- load state words into two parallel lanes ----- */
        let mut a1 = *s.add(0);
        let mut b1 = *s.add(1);
        let mut c1 = *s.add(2);
        let mut d1 = *s.add(3);
        let mut e1 = *s.add(4);

        let mut a2 = a1;
        let mut b2 = b1;
        let mut c2 = c1;
        let mut d2 = d1;
        let mut e2 = e1;

        /* ----- load sixteen little‑endian message words ----- */
        macro_rules! w {
            ($ofs:expr) => {
                readle32(chunk.add($ofs))
            };
        }
        let w0 = w!(0);
        let w1 = w!(4);
        let w2 = w!(8);
        let w3 = w!(12);
        let w4 = w!(16);
        let w5 = w!(20);
        let w6 = w!(24);
        let w7 = w!(28);
        let w8 = w!(32);
        let w9 = w!(36);
        let w10 = w!(40);
        let w11 = w!(44);
        let w12 = w!(48);
        let w13 = w!(52);
        let w14 = w!(56);
        let w15 = w!(60);

        R11(&mut a1, b1, &mut c1, d1, e1, w0, 11);
        R12(&mut a2, b2, &mut c2, d2, e2, w5, 8);
        R11(&mut e1, a1, &mut b1, c1, d1, w1, 14);
        R12(&mut e2, a2, &mut b2, c2, d2, w14, 9);
        R11(&mut d1, e1, &mut a1, b1, c1, w2, 15);
        R12(&mut d2, e2, &mut a2, b2, c2, w7, 9);
        R11(&mut c1, d1, &mut e1, a1, b1, w3, 12);
        R12(&mut c2, d2, &mut e2, a2, b2, w0, 11);
        R11(&mut b1, c1, &mut d1, e1, a1, w4, 5);
        R12(&mut b2, c2, &mut d2, e2, a2, w9, 13);
        R11(&mut a1, b1, &mut c1, d1, e1, w5, 8);
        R12(&mut a2, b2, &mut c2, d2, e2, w2, 15);
        R11(&mut e1, a1, &mut b1, c1, d1, w6, 7);
        R12(&mut e2, a2, &mut b2, c2, d2, w11, 15);
        R11(&mut d1, e1, &mut a1, b1, c1, w7, 9);
        R12(&mut d2, e2, &mut a2, b2, c2, w4, 5);
        R11(&mut c1, d1, &mut e1, a1, b1, w8, 11);
        R12(&mut c2, d2, &mut e2, a2, b2, w13, 7);
        R11(&mut b1, c1, &mut d1, e1, a1, w9, 13);
        R12(&mut b2, c2, &mut d2, e2, a2, w6, 7);
        R11(&mut a1, b1, &mut c1, d1, e1, w10, 14);
        R12(&mut a2, b2, &mut c2, d2, e2, w15, 8);
        R11(&mut e1, a1, &mut b1, c1, d1, w11, 15);
        R12(&mut e2, a2, &mut b2, c2, d2, w8, 11);
        R11(&mut d1, e1, &mut a1, b1, c1, w12, 6);
        R12(&mut d2, e2, &mut a2, b2, c2, w1, 14);
        R11(&mut c1, d1, &mut e1, a1, b1, w13, 7);
        R12(&mut c2, d2, &mut e2, a2, b2, w10, 14);
        R11(&mut b1, c1, &mut d1, e1, a1, w14, 9);
        R12(&mut b2, c2, &mut d2, e2, a2, w3, 12);
        R11(&mut a1, b1, &mut c1, d1, e1, w15, 8);
        R12(&mut a2, b2, &mut c2, d2, e2, w12, 6);

        R21(&mut e1, a1, &mut b1, c1, d1, w7, 7);
        R22(&mut e2, a2, &mut b2, c2, d2, w6, 9);
        R21(&mut d1, e1, &mut a1, b1, c1, w4, 6);
        R22(&mut d2, e2, &mut a2, b2, c2, w11, 13);
        R21(&mut c1, d1, &mut e1, a1, b1, w13, 8);
        R22(&mut c2, d2, &mut e2, a2, b2, w3, 15);
        R21(&mut b1, c1, &mut d1, e1, a1, w1, 13);
        R22(&mut b2, c2, &mut d2, e2, a2, w7, 7);
        R21(&mut a1, b1, &mut c1, d1, e1, w10, 11);
        R22(&mut a2, b2, &mut c2, d2, e2, w0, 12);
        R21(&mut e1, a1, &mut b1, c1, d1, w6, 9);
        R22(&mut e2, a2, &mut b2, c2, d2, w13, 8);
        R21(&mut d1, e1, &mut a1, b1, c1, w15, 7);
        R22(&mut d2, e2, &mut a2, b2, c2, w5, 9);
        R21(&mut c1, d1, &mut e1, a1, b1, w3, 15);
        R22(&mut c2, d2, &mut e2, a2, b2, w10, 11);
        R21(&mut b1, c1, &mut d1, e1, a1, w12, 7);
        R22(&mut b2, c2, &mut d2, e2, a2, w14, 7);
        R21(&mut a1, b1, &mut c1, d1, e1, w0, 12);
        R22(&mut a2, b2, &mut c2, d2, e2, w15, 7);
        R21(&mut e1, a1, &mut b1, c1, d1, w9, 15);
        R22(&mut e2, a2, &mut b2, c2, d2, w8, 12);
        R21(&mut d1, e1, &mut a1, b1, c1, w5, 9);
        R22(&mut d2, e2, &mut a2, b2, c2, w12, 7);
        R21(&mut c1, d1, &mut e1, a1, b1, w2, 11);
        R22(&mut c2, d2, &mut e2, a2, b2, w4, 6);
        R21(&mut b1, c1, &mut d1, e1, a1, w14, 7);
        R22(&mut b2, c2, &mut d2, e2, a2, w9, 15);
        R21(&mut a1, b1, &mut c1, d1, e1, w11, 13);
        R22(&mut a2, b2, &mut c2, d2, e2, w1, 13);
        R21(&mut e1, a1, &mut b1, c1, d1, w8, 12);
        R22(&mut e2, a2, &mut b2, c2, d2, w2, 11);

        R31(&mut d1, e1, &mut a1, b1, c1, w3, 11);
        R32(&mut d2, e2, &mut a2, b2, c2, w15, 9);
        R31(&mut c1, d1, &mut e1, a1, b1, w10, 13);
        R32(&mut c2, d2, &mut e2, a2, b2, w5, 7);
        R31(&mut b1, c1, &mut d1, e1, a1, w14, 6);
        R32(&mut b2, c2, &mut d2, e2, a2, w1, 15);
        R31(&mut a1, b1, &mut c1, d1, e1, w4, 7);
        R32(&mut a2, b2, &mut c2, d2, e2, w3, 11);
        R31(&mut e1, a1, &mut b1, c1, d1, w9, 14);
        R32(&mut e2, a2, &mut b2, c2, d2, w7, 8);
        R31(&mut d1, e1, &mut a1, b1, c1, w15, 9);
        R32(&mut d2, e2, &mut a2, b2, c2, w14, 6);
        R31(&mut c1, d1, &mut e1, a1, b1, w8, 13);
        R32(&mut c2, d2, &mut e2, a2, b2, w6, 6);
        R31(&mut b1, c1, &mut d1, e1, a1, w1, 15);
        R32(&mut b2, c2, &mut d2, e2, a2, w9, 14);
        R31(&mut a1, b1, &mut c1, d1, e1, w2, 14);
        R32(&mut a2, b2, &mut c2, d2, e2, w11, 12);
        R31(&mut e1, a1, &mut b1, c1, d1, w7, 8);
        R32(&mut e2, a2, &mut b2, c2, d2, w8, 13);
        R31(&mut d1, e1, &mut a1, b1, c1, w0, 13);
        R32(&mut d2, e2, &mut a2, b2, c2, w12, 5);
        R31(&mut c1, d1, &mut e1, a1, b1, w6, 6);
        R32(&mut c2, d2, &mut e2, a2, b2, w2, 14);
        R31(&mut b1, c1, &mut d1, e1, a1, w13, 5);
        R32(&mut b2, c2, &mut d2, e2, a2, w10, 13);
        R31(&mut a1, b1, &mut c1, d1, e1, w11, 12);
        R32(&mut a2, b2, &mut c2, d2, e2, w0, 13);
        R31(&mut e1, a1, &mut b1, c1, d1, w5, 7);
        R32(&mut e2, a2, &mut b2, c2, d2, w4, 7);
        R31(&mut d1, e1, &mut a1, b1, c1, w12, 5);
        R32(&mut d2, e2, &mut a2, b2, c2, w13, 5);

        R41(&mut c1, d1, &mut e1, a1, b1, w1, 11);
        R42(&mut c2, d2, &mut e2, a2, b2, w8, 15);
        R41(&mut b1, c1, &mut d1, e1, a1, w9, 12);
        R42(&mut b2, c2, &mut d2, e2, a2, w6, 5);
        R41(&mut a1, b1, &mut c1, d1, e1, w11, 14);
        R42(&mut a2, b2, &mut c2, d2, e2, w4, 8);
        R41(&mut e1, a1, &mut b1, c1, d1, w10, 15);
        R42(&mut e2, a2, &mut b2, c2, d2, w1, 11);
        R41(&mut d1, e1, &mut a1, b1, c1, w0, 14);
        R42(&mut d2, e2, &mut a2, b2, c2, w3, 14);
        R41(&mut c1, d1, &mut e1, a1, b1, w8, 15);
        R42(&mut c2, d2, &mut e2, a2, b2, w11, 14);
        R41(&mut b1, c1, &mut d1, e1, a1, w12, 9);
        R42(&mut b2, c2, &mut d2, e2, a2, w15, 6);
        R41(&mut a1, b1, &mut c1, d1, e1, w4, 8);
        R42(&mut a2, b2, &mut c2, d2, e2, w0, 14);
        R41(&mut e1, a1, &mut b1, c1, d1, w13, 9);
        R42(&mut e2, a2, &mut b2, c2, d2, w5, 6);
        R41(&mut d1, e1, &mut a1, b1, c1, w3, 14);
        R42(&mut d2, e2, &mut a2, b2, c2, w12, 9);
        R41(&mut c1, d1, &mut e1, a1, b1, w7, 5);
        R42(&mut c2, d2, &mut e2, a2, b2, w2, 12);
        R41(&mut b1, c1, &mut d1, e1, a1, w15, 6);
        R42(&mut b2, c2, &mut d2, e2, a2, w13, 9);
        R41(&mut a1, b1, &mut c1, d1, e1, w14, 8);
        R42(&mut a2, b2, &mut c2, d2, e2, w9, 12);
        R41(&mut e1, a1, &mut b1, c1, d1, w5, 6);
        R42(&mut e2, a2, &mut b2, c2, d2, w7, 5);
        R41(&mut d1, e1, &mut a1, b1, c1, w6, 5);
        R42(&mut d2, e2, &mut a2, b2, c2, w10, 15);
        R41(&mut c1, d1, &mut e1, a1, b1, w2, 12);
        R42(&mut c2, d2, &mut e2, a2, b2, w14, 8);

        R51(&mut b1, c1, &mut d1, e1, a1, w4, 9);
        R52(&mut b2, c2, &mut d2, e2, a2, w12, 8);
        R51(&mut a1, b1, &mut c1, d1, e1, w0, 15);
        R52(&mut a2, b2, &mut c2, d2, e2, w15, 5);
        R51(&mut e1, a1, &mut b1, c1, d1, w5, 5);
        R52(&mut e2, a2, &mut b2, c2, d2, w10, 12);
        R51(&mut d1, e1, &mut a1, b1, c1, w9, 11);
        R52(&mut d2, e2, &mut a2, b2, c2, w4, 9);
        R51(&mut c1, d1, &mut e1, a1, b1, w7, 6);
        R52(&mut c2, d2, &mut e2, a2, b2, w1, 12);
        R51(&mut b1, c1, &mut d1, e1, a1, w12, 8);
        R52(&mut b2, c2, &mut d2, e2, a2, w5, 5);
        R51(&mut a1, b1, &mut c1, d1, e1, w2, 13);
        R52(&mut a2, b2, &mut c2, d2, e2, w8, 14);
        R51(&mut e1, a1, &mut b1, c1, d1, w10, 12);
        R52(&mut e2, a2, &mut b2, c2, d2, w7, 6);
        R51(&mut d1, e1, &mut a1, b1, c1, w14, 5);
        R52(&mut d2, e2, &mut a2, b2, c2, w6, 8);
        R51(&mut c1, d1, &mut e1, a1, b1, w1, 12);
        R52(&mut c2, d2, &mut e2, a2, b2, w2, 13);
        R51(&mut b1, c1, &mut d1, e1, a1, w3, 13);
        R52(&mut b2, c2, &mut d2, e2, a2, w13, 6);
        R51(&mut a1, b1, &mut c1, d1, e1, w8, 14);
        R52(&mut a2, b2, &mut c2, d2, e2, w14, 5);
        R51(&mut e1, a1, &mut b1, c1, d1, w11, 11);
        R52(&mut e2, a2, &mut b2, c2, d2, w0, 15);
        R51(&mut d1, e1, &mut a1, b1, c1, w6, 8);
        R52(&mut d2, e2, &mut a2, b2, c2, w3, 13);
        R51(&mut c1, d1, &mut e1, a1, b1, w15, 5);
        R52(&mut c2, d2, &mut e2, a2, b2, w9, 11);
        R51(&mut b1, c1, &mut d1, e1, a1, w13, 6);
        R52(&mut b2, c2, &mut d2, e2, a2, w11, 11);

        /* ----- final feed‑forward ----- */
        let t = *s.add(0);
        *s.add(0) = (*s.add(1)).wrapping_add(c1).wrapping_add(d2);
        *s.add(1) = (*s.add(2)).wrapping_add(d1).wrapping_add(e2);
        *s.add(2) = (*s.add(3)).wrapping_add(e1).wrapping_add(a2);
        *s.add(3) = (*s.add(4)).wrapping_add(a1).wrapping_add(b2);
        *s.add(4) = t.wrapping_add(b1).wrapping_add(c2);
    }

    debug!(target: "ripemd160::transform", "one 64‑byte chunk compressed");
}

#[cfg(test)]
mod spec_transform {
    use super::*;

    /// Verifies that invoking the low‑level `transform`
    /// directly yields the very same state transition
    /// as routing the data through the public `update`
    /// convenience method.
    #[traced_test]
    fn transform_consistency() {
        /* build a deterministic single‑chunk message */
        let mut msg = [0u8; 64];
        for (i, b) in msg.iter_mut().enumerate() {
            *b = i as u8;
        }

        /* reference via public api */
        let mut ref_hasher = Ripemd160::default();
        ref_hasher.update(&msg);

        /* direct compression */
        let mut raw_hasher = Ripemd160::default();
        unsafe {
            ripemd160_transform(
                raw_hasher.s_mut().as_mut_ptr(),
                msg.as_ptr(),
            );
            raw_hasher.set_bytes(64);
        }

        assert_eq!(
            raw_hasher.s(), ref_hasher.s(),
            "state diverged between direct transform and update() path"
        );
        assert_eq!(
            raw_hasher.bytes(), ref_hasher.bytes(),
            "byte counter mismatch"
        );
    }
}
