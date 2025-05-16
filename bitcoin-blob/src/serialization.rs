// ---------------- [ File: bitcoin-blob/src/serialization.rs ]
crate::ix!();

impl<const BITS: usize> BaseBlob<BITS>
where
    [u8; (BITS % 8) + usize::MAX]: ,
    [(); base_blob_width::<BITS>()]:
{
    pub fn get_u64(&self, pos: i32) -> u64 {
        trace!(
            "Entering get_u64(pos={}) on BaseBlob<{}>; data={:X?}",
            pos,
            BITS,
            self.data
        );

        // Each position corresponds to an 8-byte chunk in little-endian.
        // We'll do a simple bounds check: pos must be >=0 and within range.
        let width = base_blob_width::<BITS>() as i32;
        let byte_start = pos.checked_mul(8).expect("pos*8 overflow");
        let byte_end = byte_start + 8;
        assert!(
            byte_start >= 0 && byte_end as usize <= width as usize,
            "get_u64 out of range for BITS={}",
            BITS
        );

        let start_usize = byte_start as usize;
        let slice = &self.data[start_usize..start_usize + 8];
        let result = slice[0] as u64
            | ((slice[1] as u64) << 8)
            | ((slice[2] as u64) << 16)
            | ((slice[3] as u64) << 24)
            | ((slice[4] as u64) << 32)
            | ((slice[5] as u64) << 40)
            | ((slice[6] as u64) << 48)
            | ((slice[7] as u64) << 56);

        debug!("get_u64 => 0x{:016X}", result);
        result
    }

    pub fn serialize<Stream>(&self, s: &mut Stream)
    where
        Stream: std::io::Write,
    {
        trace!(
            "serialize => writing {} bytes for BaseBlob<{}>",
            base_blob_width::<BITS>(),
            BITS
        );

        s.write_all(&self.data).expect("Failed to write BaseBlob data");
        debug!("serialize => finished writing.");
    }

    pub fn unserialize<Stream>(&mut self, s: &mut Stream)
    where
        Stream: std::io::Read,
    {
        trace!(
            "unserialize => reading {} bytes for BaseBlob<{}>",
            base_blob_width::<BITS>(),
            BITS
        );

        s.read_exact(&mut self.data).expect("Failed to read BaseBlob data");
        debug!("unserialize => finished reading => data={:X?}", self.data);
    }
}

#[cfg(test)]
mod serialization_exhaustive_tests {
    use super::*;
    use std::io::{Cursor, Write, Read};
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[traced_test]
    fn test_get_u64() {
        info!("Testing get_u64() for BITS=8, BITS=64, BITS=256...");
        test_get_u64_gen::<8>();
        test_get_u64_gen::<64>();
        test_get_u64_gen::<256>();
        info!("get_u64() tests concluded successfully.");
    }

    #[traced_test]
    fn test_serialize_unserialize() {
        info!("Testing serialize() & unserialize() for BITS=8, BITS=64, BITS=256...");
        test_serialize_unserialize_gen::<8>();
        test_serialize_unserialize_gen::<64>();
        test_serialize_unserialize_gen::<256>();
        info!("serialize/unserialize tests concluded successfully.");
    }

    fn test_get_u64_gen<const BITS: usize>()
    where
        [(); base_blob_width::<BITS>()]:,
        [u8; (BITS % 8) + usize::MAX]:,
    {
        let width = base_blob_width::<BITS>();

        // Fill ascending pattern
        let mut blob = BaseBlob::<BITS>::default();
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = i as u8;
        }

        // total_chunks => number of 8-byte chunks
        let total_chunks = width / 8;

        // negative pos => panic
        let caught_neg = catch_unwind(AssertUnwindSafe(|| {
            let _ = blob.get_u64(-1);
        }));
        assert!(caught_neg.is_err(), "get_u64(-1) should panic for BITS={}", BITS);

        // pos >= total_chunks => panic
        if total_chunks > 0 {
            let caught_out = catch_unwind(AssertUnwindSafe(|| {
                let _ = blob.get_u64(total_chunks as i32);
            }));
            assert!(caught_out.is_err(), "pos=total_chunks => out of range => panic, BITS={}", BITS);
        }

        // valid chunk => verify correct
        for pos in 0..(total_chunks as i32) {
            let got = blob.get_u64(pos);
            let start = pos as usize * 8;
            let slice = &blob.data[start..start+8];
            let expected = slice[0] as u64
                | ((slice[1] as u64) << 8)
                | ((slice[2] as u64) << 16)
                | ((slice[3] as u64) << 24)
                | ((slice[4] as u64) << 32)
                | ((slice[5] as u64) << 40)
                | ((slice[6] as u64) << 48)
                | ((slice[7] as u64) << 56);
            assert_eq!(got, expected, "Mismatch at pos={} for BITS={}", pos, BITS);
        }

        // if width < 8 => no valid chunk => pos=0 must panic
        if width < 8 {
            let caught_zero = catch_unwind(AssertUnwindSafe(|| {
                let _ = blob.get_u64(0);
            }));
            assert!(
                caught_zero.is_err(),
                "width<8 => pos=0 => out of range => panic, BITS={}",
                BITS
            );
        }
    }

    fn test_serialize_unserialize_gen<const BITS: usize>()
    where
        [(); base_blob_width::<BITS>()]:,
        [u8; (BITS % 8) + usize::MAX]:,
    {
        let width = base_blob_width::<BITS>();

        // 1) Fill random data
        let mut blob_src = BaseBlob::<BITS>::default();
        let mut rng = SimpleRng::new(0x9999_8888_7777_6666);
        let mut temp_buf = vec![0u8; width];
        rng.fill_bytes(&mut temp_buf);
        blob_src.data.copy_from_slice(&temp_buf);

        // 2) serialize => Vec<u8>
        let mut out_vec = Vec::new();
        blob_src.serialize(&mut out_vec);
        assert_eq!(out_vec.len(), width, "Must write exactly width bytes, BITS={}", BITS);
        assert_eq!(out_vec, blob_src.data, "serialized != .data for BITS={}", BITS);

        // 3) unserialize => second blob
        let mut blob_dst = BaseBlob::<BITS>::default();
        let mut cursor = Cursor::new(&out_vec);
        blob_dst.unserialize(&mut cursor);
        assert_eq!(blob_src.data, blob_dst.data, "Mismatch after unserialize, BITS={}", BITS);

        // short read => partial => must panic
        if width > 0 {
            let short_vec = &out_vec[..(width - 1)];
            let mut blob_short = BaseBlob::<BITS>::default();
            let mut short_cursor = Cursor::new(short_vec);

            let caught_short = catch_unwind(AssertUnwindSafe(|| {
                blob_short.unserialize(&mut short_cursor);
            }));
            assert!(caught_short.is_err(), "unserialize => partial read => panic, BITS={}", BITS);
        }

        // short write => partial => must panic
        if width > 0 {
            let mut short_sink = ShortWriter { max_len: width - 1, written: 0 };
            let caught_write = catch_unwind(AssertUnwindSafe(|| {
                blob_src.serialize(&mut short_sink);
            }));
            assert!(caught_write.is_err(), "serialize => partial write => panic, BITS={}", BITS);
        }
    }

    // ------------------------------------------------------------------------
    // A writer that allows only `max_len` bytes, then fails
    struct ShortWriter {
        max_len: usize,
        written: usize,
    }
    impl Write for ShortWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let remaining = self.max_len.saturating_sub(self.written);
            if remaining == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "ShortWriter => no capacity left",
                ));
            }
            let to_write = remaining.min(buf.len());
            self.written += to_write;
            Ok(to_write)
        }
        fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
    }

    // A simple PRNG
    struct SimpleRng(u64);
    impl SimpleRng {
        fn new(seed: u64) -> Self { Self(seed) }
        fn next_u64(&mut self) -> u64 {
            self.0 = self.0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1);
            self.0
        }
        fn fill_bytes(&mut self, buf: &mut [u8]) {
            for chunk in buf.chunks_mut(8) {
                let rnd = self.next_u64().to_le_bytes();
                let n = chunk.len();
                chunk.copy_from_slice(&rnd[..n]);
            }
        }
    }
}
