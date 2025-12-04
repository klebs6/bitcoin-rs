// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_maybe_check_crc.rs ]
crate::ix!();

pub fn read_block_maybe_check_crc(
    file:    &Rc<RefCell<dyn RandomAccessFile>>,
    options: &ReadOptions,
    data:    &[u8],
    n:       usize,
) -> Option<crate::Status> {
    if !*options.verify_checksums() {
        trace!(
            "read_block_maybe_check_crc: checksum verification disabled; skipping CRC validation"
        );
        return None;
    }

    // Trailer layout: data[0..n] = block, data[n] = type, data[n+1..n+5] = masked CRC.
    let crc_bytes = &data[n + 1..n + 5];

    let stored_crc = {
        let v = bitcoinleveldb_coding::decode_fixed32(
            crc_bytes.as_ptr(),
        );
        crc32c_unmask(v)
    };

    let actual_crc = unsafe { crc32c_value(data.as_ptr(), n + 1) };

    if actual_crc != stored_crc {
        let msg       = b"block checksum mismatch";
        let msg_slice = Slice::from(&msg[..]);

        let status = {
            let file_ref = file.borrow();
            let fname    = file_ref.name();
            let fname_slice = Slice::from(fname.as_bytes());

            error!(
                "read_block: CRC mismatch for file='{}' (stored={:#010x}, actual={:#010x})",
                fname,
                stored_crc,
                actual_crc
            );

            crate::Status::corruption(
                &msg_slice,
                Some(&fname_slice),
            )
        };

        Some(status)
    } else {
        trace!(
            "read_block_maybe_check_crc: checksum verification succeeded (crc={:#010x})",
            actual_crc
        );
        None
    }
}

#[cfg(test)]
mod read_block_checksum_verification_unit_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn stub_file_for_crc_tests(
    ) -> Rc<RefCell<dyn RandomAccessFile>> {
        let slice = Slice::from(b"" as &[u8]);
        let src   = StringSource::new(&slice);
        Rc::new(RefCell::new(src))
    }

    #[traced_test]
    fn checksum_verification_is_skipped_when_disabled() {
        let payload     = b"crc-off";
        let block_bytes =
            build_test_block_bytes(payload, 0u8);

        let file    = stub_file_for_crc_tests();
        let mut opt = ReadOptions::default();
        opt.set_verify_checksums(false);

        let status_opt = read_block_maybe_check_crc(
            &file,
            &opt,
            &block_bytes[..],
            payload.len(),
        );

        trace!(
            "checksum_verification_is_skipped_when_disabled: status_present={}",
            status_opt.is_some()
        );

        assert!(status_opt.is_none());
    }

    #[traced_test]
    fn checksum_verification_succeeds_for_valid_block() {
        let payload     = b"crc-on";
        let block_bytes =
            build_test_block_bytes(payload, 0u8);

        let file    = stub_file_for_crc_tests();
        let mut opt = ReadOptions::default();
        opt.set_verify_checksums(true);

        let status_opt = read_block_maybe_check_crc(
            &file,
            &opt,
            &block_bytes[..],
            payload.len(),
        );

        trace!(
            "checksum_verification_succeeds_for_valid_block: status_present={}",
            status_opt.is_some()
        );

        assert!(
            status_opt.is_none(),
            "expected no error status for valid CRC"
        );
    }

    #[traced_test]
    fn checksum_verification_detects_corrupted_block() {
        let payload = b"crc-bad";
        let mut block_bytes =
            build_test_block_bytes(payload, 0u8);

        if !block_bytes.is_empty() {
            block_bytes[0] ^=
                0xFF;
        }

        let file    = stub_file_for_crc_tests();
        let mut opt = ReadOptions::default();
        opt.set_verify_checksums(true);

        let status_opt = read_block_maybe_check_crc(
            &file,
            &opt,
            &block_bytes[..],
            payload.len(),
        );

        trace!(
            "checksum_verification_detects_corrupted_block: status_present={}",
            status_opt.is_some()
        );

        assert!(status_opt.is_some());
        let status = status_opt.unwrap();
        assert!(status.is_corruption());
    }
}
