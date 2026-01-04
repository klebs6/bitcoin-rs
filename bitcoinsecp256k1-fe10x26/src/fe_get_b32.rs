// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_get_b32.rs ]
crate::ix!();

/// Convert a field element to a 32-byte big endian value. 
///
/// Requires the input to be normalized
/// 
pub fn fe_get_b32(
    r: *mut u8,
    a: *const Fe10x26)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!((*a).normalized != 0);
            fe_verify(a);
        }

        *r.add(0)  = (((*a).n[9] >> 14) & 0xff) as u8;
        *r.add(1)  = (((*a).n[9] >>  6) & 0xff) as u8;
        *r.add(2)  = ((((*a).n[9] & 0x3F) << 2) | (((*a).n[8] >> 24) & 0x3)) as u8;
        *r.add(3)  = (((*a).n[8] >> 16) & 0xff) as u8;
        *r.add(4)  = (((*a).n[8] >>  8) & 0xff) as u8;
        *r.add(5)  = ((*a).n[8] & 0xff) as u8;
        *r.add(6)  = (((*a).n[7] >> 18) & 0xff) as u8;
        *r.add(7)  = (((*a).n[7] >> 10) & 0xff) as u8;
        *r.add(8)  = (((*a).n[7] >>  2) & 0xff) as u8;
        *r.add(9)  = ((((*a).n[7] & 0x3) << 6) | (((*a).n[6] >> 20) & 0x3f)) as u8;
        *r.add(10) = (((*a).n[6] >> 12) & 0xff) as u8;
        *r.add(11) = (((*a).n[6] >>  4) & 0xff) as u8;
        *r.add(12) = (((((*a).n[6] & 0xf) << 4) | (((*a).n[5] >> 22) & 0xf))) as u8;
        *r.add(13) = (((*a).n[5] >> 14) & 0xff) as u8;
        *r.add(14) = (((*a).n[5] >>  6) & 0xff) as u8;
        *r.add(15) = (((((*a).n[5] & 0x3f) << 2) | (((*a).n[4] >> 24) & 0x3))) as u8;
        *r.add(16) = (((*a).n[4] >> 16) & 0xff) as u8;
        *r.add(17) = (((*a).n[4] >>  8) & 0xff) as u8;
        *r.add(18) = ((*a).n[4] & 0xff) as u8;
        *r.add(19) = (((*a).n[3] >> 18) & 0xff) as u8;
        *r.add(20) = (((*a).n[3] >> 10) & 0xff) as u8;
        *r.add(21) = (((*a).n[3] >>  2) & 0xff) as u8;
        *r.add(22) = (((((*a).n[3] & 0x3) << 6) | (((*a).n[2] >> 20) & 0x3f))) as u8;
        *r.add(23) = (((*a).n[2] >> 12) & 0xff) as u8;
        *r.add(24) = (((*a).n[2] >>  4) & 0xff) as u8;
        *r.add(25) = (((((*a).n[2] & 0xf) << 4) | (((*a).n[1] >> 22) & 0xf))) as u8;
        *r.add(26) = (((*a).n[1] >> 14) & 0xff) as u8;
        *r.add(27) = (((*a).n[1] >>  6) & 0xff) as u8;
        *r.add(28) = (((((*a).n[1] & 0x3f) << 2) | (((*a).n[0] >> 24) & 0x3))) as u8;
        *r.add(29) = (((*a).n[0] >> 16) & 0xff) as u8;
        *r.add(30) = (((*a).n[0] >>  8) & 0xff) as u8;
        *r.add(31) = ((*a).n[0] & 0xff) as u8;
    }
}

#[cfg(test)]
mod fe_get_b32_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    fn roundtrip_get_b32(bytes: &[u8; 32]) {
        let mut a = fe_from_be_bytes_checked(bytes);
        fe_normalize_in_place(&mut a);

        let mut out = [0u8; 32];
        unsafe { fe_get_b32(out.as_mut_ptr(), &a as *const Fe10x26) };

        debug!(?out, "fe_get_b32 output");
        assert_eq!(&out, bytes);
    }

    #[traced_test]
    fn fe_get_b32_roundtrips_common_vectors() {
        info!("fe_get_b32 should serialize normalized elements losslessly");
        roundtrip_get_b32(&BYTES_ZERO);
        roundtrip_get_b32(&BYTES_ONE);
        roundtrip_get_b32(&BYTES_TWO);
        roundtrip_get_b32(&BYTES_2_POW_255);
        roundtrip_get_b32(&BYTES_PATTERN_A);
        roundtrip_get_b32(&FIELD_PRIME_MINUS_ONE_BYTES_BE);
    }
}
