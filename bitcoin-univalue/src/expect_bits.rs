// ---------------- [ File: bitcoin-univalue/src/expect_bits.rs ]
crate::ix!();

#[repr(u32)]
pub enum ExpectBits {
    EXP_OBJ_NAME  = 1 << 0,
    EXP_COLON     = 1 << 1,
    EXP_ARR_VALUE = 1 << 2,
    EXP_VALUE     = 1 << 3,
    EXP_NOT_VALUE = 1 << 4,
}

/// C‑style macros from the C++ implementation re‑expressed
/// in Rust.  
///  
/// They expect a mutable `expect_mask: u32` binding to be
/// in scope (exactly matching the C++ variable name
/// `expectMask`).  This keeps the downstream translation of
/// `UniValue::read` almost byte‑for‑byte.
#[macro_export]
macro_rules! expect {
    ($expect_mask:expr,$bit:ident) => {
        ($expect_mask & (ExpectBits::$bit as u32)) != 0
    };
}

#[macro_export]
macro_rules! set_expect {
    ($expect_mask:expr,$bit:ident) => {
        $expect_mask |= ExpectBits::$bit as u32;
    };
}

#[macro_export]
macro_rules! clear_expect {
    ($expect_mask:expr,$bit:ident) => {
        $expect_mask &= !(ExpectBits::$bit as u32);
    };
}

#[cfg(test)]
mod expect_bits_spec {
    use super::*;

    #[traced_test]
    fn macros_manipulate_mask() {
        let mut expect_mask: u32 = 0;
        assert!(!expect!(expect_mask,EXP_VALUE));

        set_expect!(expect_mask,EXP_VALUE);
        assert!(expect!(expect_mask,EXP_VALUE));

        clear_expect!(expect_mask,EXP_VALUE);
        assert!(!expect!(expect_mask,EXP_VALUE));
    }
}
