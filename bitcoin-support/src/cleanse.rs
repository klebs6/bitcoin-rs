// ---------------- [ File: bitcoin-support/src/cleanse.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/cleanse.h]
//-------------------------------------------[.cpp/bitcoin/src/support/cleanse.cpp]

/// Securely overwrite `len` bytes at `ptr` with zeros.  
/// Uses *volatile* writes plus a compiler fence so the store
/// will **not** be removed by dead‑store or DSE passes.
#[instrument(level = "trace", skip(ptr))]
pub fn memory_cleanse(ptr: *mut c_void, len: usize) {
    if ptr.is_null() || len == 0 {
        trace!("memory_cleanse: nothing to do");
        return;
    }

    unsafe {
        let mut p = ptr as *mut u8;
        for _ in 0..len {
            core::ptr::write_volatile(p, 0);
            p = p.add(1);
        }
        /* Prevent the compiler from re‑ordering or eliding the wipes.   */
        compiler_fence(atomic::Ordering::SeqCst);
    }
}

// -----------------------------------------------------------------------------
// [bitcoin-support/src/cleanse.rs] – comprehensive tests for `memory_cleanse`
// -----------------------------------------------------------------------------
#[cfg(test)]
mod test_memory_cleanse {
    use super::*;
    use core::ffi::c_void;

    /// Helper that fills a buffer with a fixed byte pattern.
    fn patterned_vec(len: usize, pattern: u8) -> Vec<u8> {
        let mut v = Vec::with_capacity(len);
        v.resize(len, pattern);
        v
    }

    /// Verify that a non‑empty buffer is completely overwritten with zeros.
    #[traced_test]
    fn test_memory_cleanse_zeroes_buffer() {
        const LEN: usize = 32;
        const PAT: u8 = 0xAA;

        let mut buf = patterned_vec(LEN, PAT);
        assert!(buf.iter().all(|&b| b == PAT));

        unsafe {
            memory_cleanse(buf.as_mut_ptr() as *mut c_void, LEN);
        }
        assert!(buf.iter().all(|&b| b == 0));
    }

    /// Check that the function is a no‑op for `len == 0`.
    #[traced_test]
    fn test_memory_cleanse_len_zero_no_overwrite() {
        const LEN: usize = 0;
        const PAT: u8 = 0x55;

        let mut buf = patterned_vec(8, PAT);
        unsafe {
            memory_cleanse(buf.as_mut_ptr() as *mut c_void, LEN);
        }
        assert!(buf.iter().all(|&b| b == PAT));
    }

    /// Calling with a null pointer should be harmless even when `len > 0`.
    #[traced_test]
    fn test_memory_cleanse_null_ptr_safe() {
        // SAFETY: we purposely pass null; the function must not deref it.
        unsafe {
            memory_cleanse(core::ptr::null_mut(), 16);
        }
        // Reaching here without panic is success.
        assert!(true);
    }

    /// Exhaustively test a range of lengths to ensure **all** bytes are zeroed.
    #[traced_test]
    fn test_memory_cleanse_various_lengths() {
        const MAX: usize = 65; // covers 0‑to‑64‑byte edge cases

        for len in 1..=MAX {
            let mut buf = (0..len as u8).collect::<Vec<u8>>();
            unsafe {
                memory_cleanse(buf.as_mut_ptr() as *mut c_void, len);
            }
            assert!(
                buf.iter().all(|&b| b == 0),
                "non‑zero byte found after cleanse for len={len}"
            );
        }
    }
}
