// ---------------- [ File: bitcoin-chacha/src/chacha20_keystream.rs ]
crate::ix!();

impl ChaCha20 {

    /**
      | outputs the keystream of size `bytes` into `c`
      */
    pub fn keystream(&mut self, mut c: *mut u8, mut bytes: usize) {

        // copy the current input (j*) so that we can keep track of the counter
        let mut state = self.input().clone();
        let mut tmp   = [0u8; 64];

        unsafe {
            while bytes != 0 {
                // decide where this 64‑byte chunk will be written
                let (out_ptr, ctarget) = if bytes < 64 {
                    (tmp.as_mut_ptr(), Some(c))
                } else {
                    (c, None)
                };

                // working variables
                let mut x = state;

                // 20 ChaCha rounds (10 × double‑round)
                for _ in 0..10 {
                    quarterround!(x[0], x[4],  x[8],  x[12]);
                    quarterround!(x[1], x[5],  x[9],  x[13]);
                    quarterround!(x[2], x[6],  x[10], x[14]);
                    quarterround!(x[3], x[7],  x[11], x[15]);

                    quarterround!(x[0], x[5],  x[10], x[15]);
                    quarterround!(x[1], x[6],  x[11], x[12]);
                    quarterround!(x[2], x[7],  x[8],  x[13]);
                    quarterround!(x[3], x[4],  x[9],  x[14]);
                }

                // add the original state
                for i in 0..16 {
                    x[i] = x[i].wrapping_add(state[i]);
                }

                // serialize little‑endian words
                for (i, chunk) in (0..64).step_by(4).enumerate() {
                    write_le32(
                        core::slice::from_raw_parts_mut(out_ptr.add(chunk), 4),
                        x[i].into(),
                    );
                }

                // increment 64‑byte counter
                state[12] = state[12].wrapping_add(1);
                if state[12] == 0 {
                    state[13] = state[13].wrapping_add(1);
                }

                // copy partial block if necessary
                if let Some(dst) = ctarget {
                    core::ptr::copy_nonoverlapping(tmp.as_ptr(), dst, bytes);
                    self.input_mut()[12] = state[12];
                    self.input_mut()[13] = state[13];
                    return;
                }

                bytes -= 64;
                c = c.add(64);
            }
        }

        self.input_mut()[12] = state[12];
        self.input_mut()[13] = state[13];
    }
}

#[cfg(test)]
mod keystream_exhaustive_small_block {
    use super::*;

    #[traced_test]
    fn generates_partial_block() {
        let mut c = ChaCha20::new([0u8; 32].as_ptr(), 32);
        c.setiv(0);
        let mut buf = [0u8; 7]; // less than 64
        c.keystream(buf.as_mut_ptr(), buf.len());

        // second call should continue (not repeat) stream
        let mut next7 = [0u8; 7];
        c.keystream(next7.as_mut_ptr(), next7.len());
        assert_ne!(buf, next7, "stream must advance across calls");
    }
}
