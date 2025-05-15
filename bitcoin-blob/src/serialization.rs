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
