// ---------------- [ File: bitcoin-blob/src/serialization.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_base_blob_serialization {
    (
        $blob_ty:ident,
        $bits:expr,
        $bytes:expr
    ) => {

        impl $blob_ty {

            pub fn get_u64(&self, pos: i32) -> u64 {
                trace!(
                    "Entering get_u64(pos={}) on BaseBlob<{}>; data={:X?}",
                    pos,
                    $bits,
                    self.data
                );

                // Each position corresponds to an 8-byte chunk in little-endian.
                // We'll do a simple bounds check: pos must be >=0 and within range.
                let width = $bytes as i32;
                let byte_start = pos.checked_mul(8).expect("pos*8 overflow");
                let byte_end = byte_start + 8;
                assert!(
                    byte_start >= 0 && (byte_end as usize) <= (width as usize),
                    "get_u64 out of range for BITS={}",
                    $bits
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
                    $bytes,
                    $bits
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
                    $bytes,
                    $bits
                );
                s.read_exact(&mut self.data).expect("Failed to read BaseBlob data");
                debug!("unserialize => finished reading => data={:X?}", self.data);
            }
        }
    }
}

#[cfg(test)]
mod serialization_exhaustive_tests {
    use super::*;
    use std::io::{Cursor, Write, Read};
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use crate::simple_rng::SimpleRng;
    use tracing::{info, debug, trace};

    #[traced_test]
    fn test_get_u64() {
        info!("Testing get_u64() for BITS=8, BITS=64, BITS=256...");
        test_get_u64_8();
        test_get_u64_64();
        test_get_u64_256();
        info!("get_u64() tests concluded successfully.");
    }

    fn test_get_u64_8() {
        let width = 1; // BITS=8 => 1 byte
        let mut blob = BaseBlob8::default();
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = i as u8;
        }
        // total_chunks => width/8 => 1/8 => 0 => effectively no valid chunk
        // pos=0 => should panic
        let caught_zero = catch_unwind(AssertUnwindSafe(|| {
            let _ = blob.get_u64(0);
        }));
        assert!(caught_zero.is_err(), "width<8 => pos=0 => out of range => panic, BITS=8");

        // negative pos => panic
        let caught_neg = catch_unwind(AssertUnwindSafe(|| {
            let _ = blob.get_u64(-1);
        }));
        assert!(caught_neg.is_err(), "get_u64(-1) => panic, BITS=8");
    }

    fn test_get_u64_64() {
        let width = 8; // BITS=64 => 8 bytes => exactly 1 chunk of 8
        let mut blob = BaseBlob64::default();
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = i as u8;
        }

        // pos=0 => valid => expect the 8 bytes
        let got = blob.get_u64(0);
        let slice = &blob.data[0..8];
        let expected = slice[0] as u64
            | ((slice[1] as u64) << 8)
            | ((slice[2] as u64) << 16)
            | ((slice[3] as u64) << 24)
            | ((slice[4] as u64) << 32)
            | ((slice[5] as u64) << 40)
            | ((slice[6] as u64) << 48)
            | ((slice[7] as u64) << 56);
        assert_eq!(got, expected, "get_u64(0) mismatch, BITS=64");

        // pos=1 => out of range => panic
        let caught_out = catch_unwind(AssertUnwindSafe(|| {
            let _ = blob.get_u64(1);
        }));
        assert!(caught_out.is_err(), "pos=1 => out of range => panic, BITS=64");
    }

    fn test_get_u64_256() {
        let width = 32; // BITS=256 => 32 bytes => 4 chunks
        let mut blob = BaseBlob256::default();
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = i as u8;
        }

        // negative pos => panic
        let caught_neg = catch_unwind(AssertUnwindSafe(|| {
            let _ = blob.get_u64(-1);
        }));
        assert!(caught_neg.is_err(), "get_u64(-1) => panic, BITS=256");

        // pos=4 => out of range => panic
        let caught_out = catch_unwind(AssertUnwindSafe(|| {
            let _ = blob.get_u64(4);
        }));
        assert!(caught_out.is_err(), "pos=4 => out of range => panic, BITS=256");

        // valid chunk => test all 0..3
        for pos in 0..4 {
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
            assert_eq!(got, expected, "get_u64(pos={}) mismatch, BITS=256", pos);
        }
    }

    #[traced_test]
    fn test_serialize_unserialize() {
        info!("Testing serialize() & unserialize() for BITS=8, BITS=64, BITS=256...");
        test_serialize_unserialize_8();
        test_serialize_unserialize_64();
        test_serialize_unserialize_256();
        info!("serialize/unserialize tests concluded successfully.");
    }

    fn test_serialize_unserialize_8() {
        let width = 1;
        let mut blob_src = BaseBlob8::default();
        let mut rng = SimpleRng::new(0x9999_8888_7777_6666);
        let mut temp_buf = vec![0u8; width];
        rng.fill_bytes(&mut temp_buf);
        blob_src.data.copy_from_slice(&temp_buf);

        let mut out_vec = Vec::new();
        blob_src.serialize(&mut out_vec);
        assert_eq!(out_vec.len(), width, "Must write exactly width bytes, BITS=8");
        assert_eq!(out_vec, blob_src.data, "serialized != .data, BITS=8");

        let mut blob_dst = BaseBlob8::default();
        let mut cursor = Cursor::new(&out_vec);
        blob_dst.unserialize(&mut cursor);
        assert_eq!(blob_src.data, blob_dst.data, "Mismatch after unserialize, BITS=8");

        // short read => panic
        if width > 0 {
            let short_vec = &out_vec[..(width - 1)];
            let mut blob_short = BaseBlob8::default();
            let mut short_cursor = Cursor::new(short_vec);
            let caught_short = catch_unwind(AssertUnwindSafe(|| {
                blob_short.unserialize(&mut short_cursor);
            }));
            assert!(caught_short.is_err(), "unserialize => partial read => panic, BITS=8");
        }

        // short write => panic
        if width > 0 {
            let mut short_sink = ShortWriter { max_len: width - 1, written: 0 };
            let caught_write = catch_unwind(AssertUnwindSafe(|| {
                blob_src.serialize(&mut short_sink);
            }));
            assert!(caught_write.is_err(), "serialize => partial write => panic, BITS=8");
        }
    }

    fn test_serialize_unserialize_64() {
        let width = 8;
        let mut blob_src = BaseBlob64::default();
        let mut rng = SimpleRng::new(0x9999_8888_7777_6666);
        let mut temp_buf = vec![0u8; width];
        rng.fill_bytes(&mut temp_buf);
        blob_src.data.copy_from_slice(&temp_buf);

        let mut out_vec = Vec::new();
        blob_src.serialize(&mut out_vec);
        assert_eq!(out_vec.len(), width, "Must write exactly width bytes, BITS=64");
        assert_eq!(out_vec, blob_src.data, "serialized != .data, BITS=64");

        let mut blob_dst = BaseBlob64::default();
        let mut cursor = Cursor::new(&out_vec);
        blob_dst.unserialize(&mut cursor);
        assert_eq!(blob_src.data, blob_dst.data, "Mismatch after unserialize, BITS=64");

        // short read => panic
        if width > 0 {
            let short_vec = &out_vec[..(width - 1)];
            let mut blob_short = BaseBlob64::default();
            let mut short_cursor = Cursor::new(short_vec);
            let caught_short = catch_unwind(AssertUnwindSafe(|| {
                blob_short.unserialize(&mut short_cursor);
            }));
            assert!(caught_short.is_err(), "unserialize => partial read => panic, BITS=64");
        }

        // short write => panic
        if width > 0 {
            let mut short_sink = ShortWriter { max_len: width - 1, written: 0 };
            let caught_write = catch_unwind(AssertUnwindSafe(|| {
                blob_src.serialize(&mut short_sink);
            }));
            assert!(caught_write.is_err(), "serialize => partial write => panic, BITS=64");
        }
    }

    fn test_serialize_unserialize_256() {
        let width = 32;
        let mut blob_src = BaseBlob256::default();
        let mut rng = SimpleRng::new(0x9999_8888_7777_6666);
        let mut temp_buf = vec![0u8; width];
        rng.fill_bytes(&mut temp_buf);
        blob_src.data.copy_from_slice(&temp_buf);

        let mut out_vec = Vec::new();
        blob_src.serialize(&mut out_vec);
        assert_eq!(out_vec.len(), width, "Must write exactly width bytes, BITS=256");
        assert_eq!(out_vec, blob_src.data, "serialized != .data, BITS=256");

        let mut blob_dst = BaseBlob256::default();
        let mut cursor = Cursor::new(&out_vec);
        blob_dst.unserialize(&mut cursor);
        assert_eq!(blob_src.data, blob_dst.data, "Mismatch after unserialize, BITS=256");

        // short read => panic
        if width > 0 {
            let short_vec = &out_vec[..(width - 1)];
            let mut blob_short = BaseBlob256::default();
            let mut short_cursor = Cursor::new(short_vec);
            let caught_short = catch_unwind(AssertUnwindSafe(|| {
                blob_short.unserialize(&mut short_cursor);
            }));
            assert!(caught_short.is_err(), "unserialize => partial read => panic, BITS=256");
        }

        // short write => panic
        if width > 0 {
            let mut short_sink = ShortWriter { max_len: width - 1, written: 0 };
            let caught_write = catch_unwind(AssertUnwindSafe(|| {
                blob_src.serialize(&mut short_sink);
            }));
            assert!(caught_write.is_err(), "serialize => partial write => panic, BITS=256");
        }
    }

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
}
