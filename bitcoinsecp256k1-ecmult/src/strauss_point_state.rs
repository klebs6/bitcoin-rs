// ---------------- [ File: bitcoinsecp256k1-ecmult/src/strauss_point_state.rs ]
crate::ix!();

pub struct StraussPointState {
    na_1:        Scalar,
    na_lam:      Scalar,
    wnaf_na_1:   [i32; 129],
    wnaf_na_lam: [i32; 129],
    bits_na_1:   i32,
    bits_na_lam: i32,
    input_pos:   usize,
}

impl StraussPointState {
    #[inline(always)]
    pub unsafe fn na_1_ptr(this: *const Self) -> *const Scalar {
        core::ptr::addr_of!((*this).na_1)
    }

    #[inline(always)]
    pub unsafe fn na_1_mut_ptr(this: *mut Self) -> *mut Scalar {
        core::ptr::addr_of_mut!((*this).na_1)
    }

    #[inline(always)]
    pub unsafe fn na_lam_ptr(this: *const Self) -> *const Scalar {
        core::ptr::addr_of!((*this).na_lam)
    }

    #[inline(always)]
    pub unsafe fn na_lam_mut_ptr(this: *mut Self) -> *mut Scalar {
        core::ptr::addr_of_mut!((*this).na_lam)
    }

    #[inline(always)]
    pub unsafe fn wnaf_na_1_ptr(this: *const Self) -> *const i32 {
        core::ptr::addr_of!((*this).wnaf_na_1) as *const i32
    }

    #[inline(always)]
    pub unsafe fn wnaf_na_1_mut_ptr(this: *mut Self) -> *mut i32 {
        core::ptr::addr_of_mut!((*this).wnaf_na_1) as *mut i32
    }

    #[inline(always)]
    pub unsafe fn wnaf_na_lam_ptr(this: *const Self) -> *const i32 {
        core::ptr::addr_of!((*this).wnaf_na_lam) as *const i32
    }

    #[inline(always)]
    pub unsafe fn wnaf_na_lam_mut_ptr(this: *mut Self) -> *mut i32 {
        core::ptr::addr_of_mut!((*this).wnaf_na_lam) as *mut i32
    }

    #[inline(always)]
    pub unsafe fn write_bits_na_1(this: *mut Self, bits: i32) {
        core::ptr::addr_of_mut!((*this).bits_na_1).write(bits);
    }

    #[inline(always)]
    pub unsafe fn write_bits_na_lam(this: *mut Self, bits: i32) {
        core::ptr::addr_of_mut!((*this).bits_na_lam).write(bits);
    }

    #[inline(always)]
    pub unsafe fn bits_na_1(this: *const Self) -> i32 {
        core::ptr::read(core::ptr::addr_of!((*this).bits_na_1))
    }

    #[inline(always)]
    pub unsafe fn bits_na_lam(this: *const Self) -> i32 {
        core::ptr::read(core::ptr::addr_of!((*this).bits_na_lam))
    }

    #[inline(always)]
    pub unsafe fn write_input_pos(this: *mut Self, input_pos: usize) {
        core::ptr::addr_of_mut!((*this).input_pos).write(input_pos);
    }

    #[inline(always)]
    pub unsafe fn input_pos(this: *const Self) -> usize {
        core::ptr::read(core::ptr::addr_of!((*this).input_pos))
    }
}
