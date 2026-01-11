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
