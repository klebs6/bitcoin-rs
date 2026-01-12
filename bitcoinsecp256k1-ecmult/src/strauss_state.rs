// ---------------- [ File: bitcoinsecp256k1-ecmult/src/strauss_state.rs ]
crate::ix!();

#[derive(Getters)]
#[getset(get="pub")]
pub struct StraussState {
    prej:      *mut Gej,
    zr:        *mut Fe,
    pre_a:     *mut Ge,
    pre_a_lam: *mut Ge,
    ps:        *mut StraussPointState,
}

impl StraussState {
    pub const fn new() -> Self {
        Self {
            prej: core::ptr::null_mut(),
            zr: core::ptr::null_mut(),
            pre_a: core::ptr::null_mut(),
            pre_a_lam: core::ptr::null_mut(),
            ps: core::ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub fn set_prej(&mut self, prej: *mut Gej) {
        self.prej = prej;
    }

    #[inline(always)]
    pub fn set_zr(&mut self, zr: *mut Fe) {
        self.zr = zr;
    }

    #[inline(always)]
    pub fn set_pre_a(&mut self, pre_a: *mut Ge) {
        self.pre_a = pre_a;
    }

    #[inline(always)]
    pub fn set_pre_a_lam(&mut self, pre_a_lam: *mut Ge) {
        self.pre_a_lam = pre_a_lam;
    }

    #[inline(always)]
    pub fn set_ps(&mut self, ps: *mut StraussPointState) {
        self.ps = ps;
    }
}

#[cfg(test)]
mod strauss_state_pointer_contract_suite {
    use super::*;

    #[traced_test]
    fn strauss_state_new_starts_null_and_setters_round_trip() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "strauss_state_new_starts_null_and_setters_round_trip"
        );

        let mut st = StraussState::new();

        assert!(st.prej().is_null());
        assert!(st.zr().is_null());
        assert!(st.pre_a().is_null());
        assert!(st.pre_a_lam().is_null());
        assert!(st.ps().is_null());

        let prej = 0x1usize as *mut Gej;
        let zr = 0x2usize as *mut Fe;
        let pre_a = 0x3usize as *mut Ge;
        let pre_a_lam = 0x4usize as *mut Ge;
        let ps = 0x5usize as *mut StraussPointState;

        st.set_prej(prej);
        st.set_zr(zr);
        st.set_pre_a(pre_a);
        st.set_pre_a_lam(pre_a_lam);
        st.set_ps(ps);

        assert_eq!(*st.prej(), prej);
        assert_eq!(*st.zr(), zr);
        assert_eq!(*st.pre_a(), pre_a);
        assert_eq!(*st.pre_a_lam(), pre_a_lam);
        assert_eq!(*st.ps(), ps);
    }

    #[traced_test]
    fn strauss_point_state_accessors_are_stable_for_mutation_via_pointers() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "strauss_point_state_accessors_are_stable_for_mutation_via_pointers"
        );

        unsafe {
            let mut ps: StraussPointState = core::mem::MaybeUninit::<StraussPointState>::zeroed().assume_init();

            StraussPointState::write_input_pos(core::ptr::addr_of_mut!(ps), 7);
            StraussPointState::write_bits_na_1(core::ptr::addr_of_mut!(ps), 11);
            StraussPointState::write_bits_na_lam(core::ptr::addr_of_mut!(ps), 13);

            assert_eq!(StraussPointState::input_pos(core::ptr::addr_of!(ps)), 7);
            assert_eq!(StraussPointState::bits_na_1(core::ptr::addr_of!(ps)), 11);
            assert_eq!(StraussPointState::bits_na_lam(core::ptr::addr_of!(ps)), 13);
        }
    }
}
