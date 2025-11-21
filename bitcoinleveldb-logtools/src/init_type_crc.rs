// ---------------- [ File: bitcoinleveldb-logtools/src/init_type_crc.rs ]
crate::ix!();

pub fn init_type_crc(type_crc: *mut u32) {
    trace!("init_type_crc: initializing type_crc table");

    if type_crc.is_null() {
        error!("init_type_crc: null type_crc pointer");
        return;
    }

    let max_type = LOG_MAX_RECORD_TYPE as usize;

    for i in 0..=max_type {
        let t: u8 = i as u8;
        let crc = unsafe { crc32c_value(&t as *const u8, 1) };
        unsafe {
            *type_crc.add(i) = crc;
        }
        trace!(
            "init_type_crc: precomputed crc for record_type={} crc={:#010x}",
            i,
            crc
        );
    }
}

#[cfg(test)]
mod init_type_crc_spec {
    use super::*;

    #[traced_test]
    fn init_type_crc_populates_table_for_all_record_types() {
        let max_index = LOG_MAX_RECORD_TYPE as usize;
        let table_len = max_index + 1;
        let mut table: Vec<u32> = vec![0u32; table_len];

        info!(
            "init_type_crc_populates_table_for_all_record_types: max_index={} table_len={}",
            max_index, table_len
        );

        init_type_crc(table.as_mut_ptr());

        for i in 0..=max_index {
            let t: u8 = i as u8;
            let expected = unsafe { crc32c_value(&t as *const u8, 1) };
            let actual = table[i];

            debug!(
                "init_type_crc_populates_table_for_all_record_types: index={} expected_crc={:#010x} actual_crc={:#010x}",
                i,
                expected,
                actual
            );

            assert_eq!(actual, expected);
        }
    }

    #[traced_test]
    fn init_type_crc_handles_null_pointer_safely() {
        info!("init_type_crc_handles_null_pointer_safely: invoking with null pointer");
        init_type_crc(core::ptr::null_mut::<u32>());
    }
}
