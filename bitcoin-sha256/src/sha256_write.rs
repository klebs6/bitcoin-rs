crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha256.cpp]
impl Write for Sha256 {

    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_ptr(buf.as_ptr(), buf.len());
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        /* stateless – nothing buffered that needs flushing */
        Ok(())
    }
}

impl Sha256 {

    /// Stream data supplied by any iterator **exactly** `len` bytes long.
    ///
    /// # Panics
    /// * If the iterator yields fewer than `len` bytes.
    pub fn write_from_iterator(
        &mut self,
        mut data: Box<dyn Iterator<Item = u8>>,
        len: usize,
    ) {
        use std::vec::Vec;
        let mut buf = Vec::<u8>::with_capacity(len);
        for _ in 0..len {
            match data.next() {
                Some(b) => buf.push(b),
                None => panic!("write_from_iterator: iterator ended before `len` bytes were read"),
            }
        }
        self.write_ptr(buf.as_ptr(), len);
    }

    /// **Low‑level** pointer‑based writer used by all higher‑level entry points.
    ///
    /// Safety invariants mirror the original C++ logic and are upheld internally.
    pub fn write_ptr(&mut self, mut data: *const u8, mut len: usize) {
        trace!(
            target: "sha256",
            len,
            bytes_before = self.bytes(),
            "Sha256::write_ptr: begin"
        );

        let end = unsafe { data.add(len) };
        let mut bufsize = (self.bytes() % 64) as usize;

        /* ----------------------------------------------------------
         * 1. Top‑up internal buffer to 64 B if it is already partially
         *    filled **and** we have enough incoming data to reach 64 B.
         * -------------------------------------------------------- */
        if bufsize != 0 && bufsize + len >= 64 {
            let fill = 64 - bufsize;
            unsafe {
                copy_nonoverlapping(
                    data,
                    self.buf_mut().as_mut_ptr().add(bufsize),
                    fill,
                );
            }
            *self.bytes_mut() += fill as u64;
            data = unsafe { data.add(fill) };
            len -= fill;
            bufsize = 0;

            /* process the now‑full internal buffer */
            unsafe {
                sha256_transform(self.s_mut().as_mut_ptr(), self.buf().as_ptr(), 1);
            }
        }

        /* ----------------------------------------------------------
         * 2. While we still have full 64‑byte blocks, process them
         *    directly from the caller’s memory without staging.
         * -------------------------------------------------------- */
        if len >= 64 {
            let blocks = len / 64;
            let proc_bytes = blocks * 64;
            unsafe {
                sha256_transform(self.s_mut().as_mut_ptr(), data, blocks);
                data = data.add(proc_bytes);
            }
            *self.bytes_mut() += proc_bytes as u64;
            len -= proc_bytes;
        }

        /* ----------------------------------------------------------
         * 3. Any trailing bytes (< 64) are buffered for later.
         * -------------------------------------------------------- */
        if len > 0 {
            unsafe {
                copy_nonoverlapping(
                    data,
                    self.buf_mut().as_mut_ptr().add(bufsize),
                    len,
                );
            }
            *self.bytes_mut() += len as u64;
        }

        trace!(
            target: "sha256",
            bytes_after = self.bytes(),
            "Sha256::write_ptr: end"
        );
    }
}

#[cfg(test)]
mod sha256_write_equivalence_tests {
    use super::*;
    use hex_literal::hex; // dev‑dependency, small & ubiquitous

    /// Verify that the FFI front‑end [`sha256_write`] produces the same digest
    /// as streaming the same data through the safe Rust interface.
    #[traced_test]
    fn ffi_vs_safe_interface_produce_identical_digests() {
        const INPUT: &[u8] = b"abc";
        const EXPECTED: [u8; 32] = [
            0xBA, 0x78, 0x16, 0xBF, 0x8F, 0x01, 0xCF, 0xEA,
            0x41, 0x41, 0x40, 0xDE, 0x5D, 0xAE, 0x22, 0x23,
            0xB0, 0x03, 0x61, 0xA3, 0x96, 0x17, 0x7A, 0x9C,
            0xB4, 0x10, 0xFF, 0x61, 0xF2, 0x00, 0x15, 0xAD,
        ];

        // --- Path 1: safe Rust API -----------------------------------------
        let mut hasher_safe = Sha256::default();
        hasher_safe.write(INPUT).expect("in‑memory write cannot fail");
        let mut digest_safe = [0u8; 32];
        hasher_safe.finalize(&mut digest_safe);

        // --- Path 2: FFI API ------------------------------------------------
        let mut hasher_ffi = Sha256::default();
        unsafe { sha256_write(&mut hasher_ffi, INPUT.as_ptr(), INPUT.len()) };
        let mut digest_ffi = [0u8; 32];
        hasher_ffi.finalize(&mut digest_ffi);

        // --- Assertions -----------------------------------------------------
        assert_eq!(digest_safe, EXPECTED, "safe path produced wrong digest");
        assert_eq!(digest_ffi, EXPECTED, "ffi path produced wrong digest");
    }

    /// Convenience to compute a digest in one shot.
    fn digest_one_shot(data: &[u8]) -> [u8; SHA256_OUTPUT_SIZE] {
        let mut ctx = Sha256::new();
        ctx.write_all(data).expect("in‑memory write cannot fail");
        let mut out = [0u8; SHA256_OUTPUT_SIZE];
        ctx.finalize(&mut out);
        out
    }

    /// SHA‑256("") from FIPS 180‑4 §7.3 – single empty message.
    const DIGEST_EMPTY: [u8; 32] =
        hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");

    /// SHA‑256("abc") from FIPS 180‑4 test vectors.
    const DIGEST_ABC: [u8; 32] =
        hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");

    #[traced_test]
    fn empty_string_matches_reference() {
        assert_eq!(digest_one_shot(b""), DIGEST_EMPTY);
    }

    #[traced_test]
    fn abc_string_matches_reference() {
        assert_eq!(digest_one_shot(b"abc"), DIGEST_ABC);
    }

    #[traced_test]
    fn chunked_vs_single_write_identical() {
        let data = b"The quick brown fox jumps over the lazy dog";
        /* one‑shot */
        let one_shot = digest_one_shot(data);

        /* chunked 7 + 13 + rest */
        let mut ctx = Sha256::new();
        ctx.write_all(&data[..7]).unwrap();
        ctx.write_all(&data[7..20]).unwrap();
        ctx.write_all(&data[20..]).unwrap();
        let mut chunked = [0u8; SHA256_OUTPUT_SIZE];
        ctx.finalize(&mut chunked);

        assert_eq!(one_shot, chunked, "streaming API must be chunk‑invariant");
    }

    #[traced_test]
    fn write_from_iterator_yields_same_digest() {
        let data = b"iterator feed test";
        let iter = Box::new(data.to_vec().into_iter());
        let mut ctx = Sha256::new();
        ctx.write_from_iterator(iter, data.len());
        let mut out = [0u8; SHA256_OUTPUT_SIZE];
        ctx.finalize(&mut out);

        assert_eq!(out, digest_one_shot(data));
    }
}
