// ---------------- [ File: bitcoin-ripemd/src/ripemd160.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ripemd160.h]

/**
  | A hasher class for RIPEMD-160.
  |
  */
#[derive(Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct Ripemd160 {
    s:     [u32; 5],
    buf:   [u8; 64],
    bytes: u64,
}

pub const RIPEMD160_OUTPUT_SIZE: usize = 20;

impl Default for Ripemd160 {
    fn default() -> Self {
        let mut s = [0u32; 5];
        ripemd160_initialize(s.as_mut_ptr());

        tracing::info!(target: "ripemd160::core", "Ripemd160::default");
        Self {
            s,
            buf: [0u8; 64],
            bytes: 0,
        }
    }
}

impl Ripemd160 {
    pub fn reset(&mut self) -> &mut Self {
        self.bytes = 0;
        ripemd160_initialize(self.s.as_mut_ptr());
        tracing::info!(target: "ripemd160::core", "state reset");
        self
    }
}


impl Ripemd160 {

    pub fn new() -> Self { 
        Self::default() 
    }
    
    /// Feed an arbitrary byte‑slice into the running RIPEMD‑160
    /// computation (thin convenience wrapper around `write`).
    #[inline]
    pub fn update(&mut self, data: &[u8]) -> &mut Self {
        // SAFETY: pointer/length pair is exactly what the low‑level
        // API expects, and both remain valid for the duration of
        // this call.
        self.write(data.as_ptr(), data.len())
    }
}

//-------------------------------------------[.cpp/bitcoin/src/crypto/ripemd160.cpp]

/**
  | Initialize RIPEMD‑160 state.
  |
  */
#[inline]
pub fn ripemd160_initialize(s: *mut u32) {
    // # Safety
    // Caller guarantees that `s` points to **at least five**
    // writable `u32` words.
    unsafe {
        *s.add(0) = 0x6745_2301;
        *s.add(1) = 0xEFCD_AB89;
        *s.add(2) = 0x98BA_DCFE;
        *s.add(3) = 0x1032_5476;
        *s.add(4) = 0xC3D2_E1F0;
    }
    debug!(target: "ripemd160::core", "state initialised");
}

#[cfg(test)]
mod spec_core_hasher {
    use super::*;

    #[traced_test]
    fn initialise_sets_expected_constants() {
        let mut state = [0u32; 5];
        ripemd160_initialize(state.as_mut_ptr());

        // Golden constants as per the RIPEMD‑160 specification.
        assert_eq!(state, [
            0x6745_2301,
            0xEFCD_AB89,
            0x98BA_DCFE,
            0x1032_5476,
            0xC3D2_E1F0
        ]);
    }

    #[traced_test]
    fn default_yields_clean_hasher() {
        let h = Ripemd160::default();
        assert_eq!(h.bytes, 0);
    }

    #[traced_test]
    fn reset_restores_clean_state() {
        let mut h = Ripemd160::default();
        // Artificially perturb internal counters.
        h.bytes = 1234;
        h.s[0] ^= 0xdead_beef;

        h.reset();
        assert_eq!(h.bytes, 0);

        let mut ref_state = [0u32; 5];
        ripemd160_initialize(ref_state.as_mut_ptr());
        assert_eq!(h.s, ref_state);
    }
}
