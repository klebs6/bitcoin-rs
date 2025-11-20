// ---------------- [ File: bitcoinleveldb-log/src/init_type_crc.rs ]
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
        let crc = unsafe { crc32c::Value(&t as *const u8, 1) };
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
