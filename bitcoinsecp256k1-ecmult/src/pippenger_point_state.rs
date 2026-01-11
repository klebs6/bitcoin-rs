// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_point_state.rs ]
crate::ix!();

#[derive(Getters)]
#[getset(get="pub")]
pub struct PippengerPointState {
    skew_na:   i32,
    input_pos: usize,
}

impl PippengerPointState {
    #[inline(always)]
    pub const fn new(skew_na: i32, input_pos: usize) -> Self {
        Self { skew_na, input_pos }
    }

    #[inline(always)]
    pub unsafe fn write_skew_na(dst: *mut Self, skew_na: i32) {
        core::ptr::addr_of_mut!((*dst).skew_na).write(skew_na);
    }

    #[inline(always)]
    pub unsafe fn write_input_pos(dst: *mut Self, input_pos: usize) {
        core::ptr::addr_of_mut!((*dst).input_pos).write(input_pos);
    }
}

#[derive(Getters)]
#[getset(get="pub")]
pub struct PippengerState {
    wnaf_na: *mut i32,
    ps:      *mut PippengerPointState,
}

impl PippengerState {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            wnaf_na: core::ptr::null_mut(),
            ps: core::ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub unsafe fn write_wnaf_na(dst: *mut Self, wnaf_na: *mut i32) {
        core::ptr::addr_of_mut!((*dst).wnaf_na).write(wnaf_na);
    }

    #[inline(always)]
    pub unsafe fn write_ps(dst: *mut Self, ps: *mut PippengerPointState) {
        core::ptr::addr_of_mut!((*dst).ps).write(ps);
    }

    #[inline(always)]
    pub unsafe fn wnaf_na_ptr(src: *const Self) -> *mut i32 {
        core::ptr::read(core::ptr::addr_of!((*src).wnaf_na))
    }

    #[inline(always)]
    pub unsafe fn ps_ptr(src: *const Self) -> *mut PippengerPointState {
        core::ptr::read(core::ptr::addr_of!((*src).ps))
    }
}
