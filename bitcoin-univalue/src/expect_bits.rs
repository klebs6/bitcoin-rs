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

macro_rules! expect {
    ($bit:ident) => {
        /*
                (expectMask & (EXP_##bit))
        */
    }
}

macro_rules! set_expect {
    ($bit:ident) => {
        /*
                (expectMask |= EXP_##bit)
        */
    }
}

macro_rules! clear_expect {
    ($bit:ident) => {
        /*
                (expectMask &= ~EXP_##bit)
        */
    }
}
