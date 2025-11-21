// ---------------- [ File: bitcoinleveldb-logwriter/src/build_record_header.rs ]
crate::ix!();

impl LogWriter {

    /// Build the physical record header for the given type, length and CRC.
    pub fn build_record_header(
        &self,
        t: LogRecordType,
        length: usize,
        crc: u32,
    ) -> [u8; LOG_HEADER_SIZE as usize] {
        debug!(
            "LogWriter::build_record_header: type={:?} length={} crc={:#010x}",
            t,
            length,
            crc
        );

        let mut buf = [0u8; LOG_HEADER_SIZE as usize];

        // Length (little-endian) and type.
        buf[4] = (length & 0xff) as u8;
        buf[5] = ((length >> 8) & 0xff) as u8;
        buf[6] = t as u8;

        unsafe {
            encode_fixed32(buf.as_mut_ptr(), crc);
        }

        trace!(
            "LogWriter::build_record_header: header_bytes={:02x?}",
            &buf[..]
        );

        buf
    }
}

#[cfg(test)]
mod log_writer_build_record_header_tests {
    use super::*;

    #[traced_test]
    fn build_record_header_encodes_length_type_and_crc() {
        let file = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let writer = LogWriter::new(file.clone(), 0);

        let length: usize = 0x3412;
        let record_type = LogRecordType::Full;
        let crc: u32 = 0x89abcdef;

        let header = writer.build_record_header(record_type, length, crc);

        assert_eq!(header.len(), LOG_HEADER_SIZE as usize);

        let length_lo = header[4] as usize;
        let length_hi = header[5] as usize;
        let decoded_length = length_lo | (length_hi << 8);
        assert_eq!(decoded_length, length);

        assert_eq!(header[6], record_type as u8);

        let crc_bytes = [header[0], header[1], header[2], header[3]];
        let decoded_crc = u32::from_le_bytes(crc_bytes);
        assert_eq!(decoded_crc, crc);
    }
}
