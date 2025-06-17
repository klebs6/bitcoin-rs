// ---------------- [ File: bitcoin-bitstream/src/write.rs ]
crate::ix!();

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn writebe16(ptr: *mut u8, x: u16) {
    info!("writebe16 called with 0x{:04X}", x);
    let bytes = x.to_be_bytes();
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, 2);
    }
    debug!("writebe16 completed");
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn writele16(ptr: *mut u8, x: u16) {
    info!("writele16 called with 0x{:04X}", x);
    let bytes = x.to_le_bytes();
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, 2);
    }
    debug!("writele16 completed");
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn writele32(ptr: *mut u8, x: u32) {
    info!("writele32 called with 0x{:08X}", x);
    let bytes = x.to_le_bytes();
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, 4);
    }
    debug!("writele32 completed");
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn writele64(ptr: *mut u8, x: u64) {
    info!("writele64 called with 0x{:016X}", x);
    let bytes = x.to_le_bytes();
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, 8);
    }
    debug!("writele64 completed");
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn writebe32(ptr: *mut u8, x: u32) {
    info!("writebe32 called with 0x{:08X}", x);
    let bytes = x.to_be_bytes();
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, 4);
    }
    debug!("writebe32 completed");
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn writebe64(ptr: *mut u8, x: u64) {
    info!("writebe64 called with 0x{:016X}", x);
    let bytes = x.to_be_bytes();
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, 8);
    }
    debug!("writebe64 completed");
}

#[cfg(test)]
mod test_endian_read_write {
    use super::*;
    use traced_test::traced_test;

    #[traced_test]
    fn test_read_write_le16() {
        let mut arr = [0u8; 2];
        writele16(arr.as_mut_ptr(), 0x1234);
        assert_eq!(arr, [0x34, 0x12]);
        let val = readle16(arr.as_ptr());
        assert_eq!(val, 0x1234);
    }

    #[traced_test]
    fn test_read_write_le32() {
        let mut arr = [0u8; 4];
        writele32(arr.as_mut_ptr(), 0x12345678);
        assert_eq!(arr, [0x78, 0x56, 0x34, 0x12]);
        let val = readle32(arr.as_ptr());
        assert_eq!(val, 0x12345678);
    }

    #[traced_test]
    fn test_read_write_le64() {
        let mut arr = [0u8; 8];
        writele64(arr.as_mut_ptr(), 0x1122334455667788);
        assert_eq!(arr, [0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]);
        let val = readle64(arr.as_ptr());
        assert_eq!(val, 0x1122334455667788);
    }

    #[traced_test]
    fn test_read_write_be16() {
        let mut arr = [0u8; 2];
        writebe16(arr.as_mut_ptr(), 0x1234);
        assert_eq!(arr, [0x12, 0x34]);
        let val = readbe16(arr.as_ptr());
        assert_eq!(val, 0x1234);
    }

    #[traced_test]
    fn test_read_write_be32() {
        let mut arr = [0u8; 4];
        writebe32(arr.as_mut_ptr(), 0x12345678);
        assert_eq!(arr, [0x12, 0x34, 0x56, 0x78]);
        let val = readbe32(arr.as_ptr());
        assert_eq!(val, 0x12345678);
    }

    #[traced_test]
    fn test_read_write_be64() {
        let mut arr = [0u8; 8];
        writebe64(arr.as_mut_ptr(), 0x1122334455667788);
        assert_eq!(arr, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
        let val = readbe64(arr.as_ptr());
        assert_eq!(val, 0x1122334455667788);
    }
}
