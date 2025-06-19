// ---------------- [ File: bitcoin-chacha/src/read_write.rs ]
crate::ix!();

#[inline(always)]
pub fn read_le32(src: &[u8]) -> u32 {
    u32::from_le_bytes(src[..4].try_into().unwrap())
}

#[inline(always)]
pub fn write_le32(dst: &mut [u8], v: u32) {
    dst[..4].copy_from_slice(&v.to_le_bytes());
}

#[cfg(test)]
mod read_write_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn le_roundtrip() {
        const VALS: [u32; 4] = [0, 1, 0xdead_beef, u32::MAX];
        for &v in &VALS {
            let mut buf = [0u8; 4];
            write_le32(&mut buf, v);
            let r = read_le32(&buf);
            assert_eq!(r, v, "LE read‑write must round‑trip");
        }
    }
}
