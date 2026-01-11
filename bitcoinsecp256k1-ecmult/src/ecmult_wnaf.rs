// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_wnaf.rs ]
crate::ix!();

/// Convert a number to WNAF notation. The number becomes represented by sum(2^i * wnaf[i],
/// i=0..bits), with the following guarantees:
///
/// - each wnaf[i] is either 0, or an odd integer between -(1<<(w-1) - 1) and (1<<(w-1) - 1)
///
/// - two non-zero entries in wnaf are separated by at least w-1 zeroes.
///
/// - the number of set values in wnaf is returned. This number is at most 256, and at most one
/// more than the number of bits in the (absolute value) of the input.
///
pub fn ecmult_wnaf(
    wnaf: *mut i32,
    len:  i32,
    a:    *const Scalar,
    w:    i32,
) -> i32 {
    tracing::trace!(target: "secp256k1::ecmult", len = len, w = w, "ecmult_wnaf");

    unsafe {
        let mut s: Scalar;
        let mut last_set_bit: i32 = -1;
        let mut bit: i32 = 0;
        let mut sign: i32 = 1;
        let mut carry: i32 = 0;

        verify_check!(!wnaf.is_null());
        verify_check!(0 <= len && len <= 256);
        verify_check!(!a.is_null());
        verify_check!(2 <= w && w <= 31);

        core::ptr::write_bytes(wnaf, 0, len as usize);

        s = core::ptr::read(a);
        if scalar_get_bits(core::ptr::addr_of!(s), 255, 1) != 0 {
            scalar_negate(core::ptr::addr_of_mut!(s), core::ptr::addr_of!(s));
            sign = -1;
        }

        while bit < len {
            let mut now: i32;
            let word: i64;

            if scalar_get_bits(core::ptr::addr_of!(s), bit as u32, 1) == (carry as u32) {
                bit += 1;
                continue;
            }

            now = w;
            if now > len - bit {
                now = len - bit;
            }

            word = (scalar_get_bits_var(core::ptr::addr_of!(s), bit as u32, now as u32) as i64)
                + (carry as i64);

            carry = ((word >> ((w - 1) as i64)) & 1i64) as i32;
            let word = word - ((carry as i64) << (w as i64));

            *wnaf.add(bit as usize) = sign * (word as i32);
            last_set_bit = bit;

            bit += now;
        }

        #[cfg(VERIFY)]
        {
            check!(carry == 0);
            while bit < 256 {
                check!(scalar_get_bits(core::ptr::addr_of!(s), bit as u32, 1) == 0);
                bit += 1;
            }
        }

        last_set_bit + 1
    }
        /*
        scalar s;
        int last_set_bit = -1;
        int bit = 0;
        int sign = 1;
        int carry = 0;

        VERIFY_CHECK(wnaf != NULL);
        VERIFY_CHECK(0 <= len && len <= 256);
        VERIFY_CHECK(a != NULL);
        VERIFY_CHECK(2 <= w && w <= 31);

        memset(wnaf, 0, len * sizeof(wnaf[0]));

        s = *a;
        if (scalar_get_bits(&s, 255, 1)) {
            scalar_negate(&s, &s);
            sign = -1;
        }

        while (bit < len) {
            int now;
            int word;
            if (scalar_get_bits(&s, bit, 1) == (unsigned int)carry) {
                bit++;
                continue;
            }

            now = w;
            if (now > len - bit) {
                now = len - bit;
            }

            word = scalar_get_bits_var(&s, bit, now) + carry;

            carry = (word >> (w-1)) & 1;
            word -= carry << w;

            wnaf[bit] = sign * word;
            last_set_bit = bit;

            bit += now;
        }
    #ifdef VERIFY
        CHECK(carry == 0);
        while (bit < 256) {
            CHECK(scalar_get_bits(&s, bit++, 1) == 0);
        }
    #endif
        return last_set_bit + 1;
        */

}
