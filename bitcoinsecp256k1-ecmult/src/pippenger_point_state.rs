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

#[cfg(test)]
mod pippenger_state_pointer_contract_suite {
    use super::*;

    #[traced_test]
    fn pippenger_point_state_new_and_mutators_round_trip() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "pippenger_point_state_new_and_mutators_round_trip"
        );

        let s = PippengerPointState::new(1, 42);
        assert_eq!(*s.skew_na(), 1);
        assert_eq!(*s.input_pos(), 42);

        unsafe {
            let mut s2 = PippengerPointState::new(0, 0);
            PippengerPointState::write_skew_na(core::ptr::addr_of_mut!(s2), 7);
            PippengerPointState::write_input_pos(core::ptr::addr_of_mut!(s2), 99);

            assert_eq!(*s2.skew_na(), 7);
            assert_eq!(*s2.input_pos(), 99);
        }
    }

    #[traced_test]
    fn pippenger_state_new_starts_null_and_raw_pointer_accessors_reflect_writes() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "pippenger_state_new_starts_null_and_raw_pointer_accessors_reflect_writes"
        );

        unsafe {
            let mut st = PippengerState::new();

            assert!(PippengerState::ps_ptr(core::ptr::addr_of!(st)).is_null());
            assert!(PippengerState::wnaf_na_ptr(core::ptr::addr_of!(st)).is_null());

            let dummy_ps = 0x1usize as *mut PippengerPointState;
            let dummy_wnaf = 0x2usize as *mut i32;

            PippengerState::write_ps(core::ptr::addr_of_mut!(st), dummy_ps);
            PippengerState::write_wnaf_na(core::ptr::addr_of_mut!(st), dummy_wnaf);

            assert_eq!(PippengerState::ps_ptr(core::ptr::addr_of!(st)), dummy_ps);
            assert_eq!(PippengerState::wnaf_na_ptr(core::ptr::addr_of!(st)), dummy_wnaf);
        }
    }
}
