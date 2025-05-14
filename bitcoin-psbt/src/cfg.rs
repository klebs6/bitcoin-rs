// ---------------- [ File: bitcoin-psbt/src/cfg.rs ]
crate::ix!();

/* ----------------- Global types  ----------------- */
pub const PSBT_GLOBAL_UNSIGNED_TX: u8 = 0x00;

/* ------------------ Input types  ------------------ */
pub const PSBT_IN_NON_WITNESS_UTXO: u8 = 0x00;
pub const PSBT_IN_WITNESS_UTXO:     u8 = 0x01;
pub const PSBT_IN_PARTIAL_SIG:      u8 = 0x02;
pub const PSBT_IN_SIGHASH:          u8 = 0x03;
pub const PSBT_IN_REDEEMSCRIPT:     u8 = 0x04;
pub const PSBT_IN_WITNESSSCRIPT:    u8 = 0x05;
pub const PSBT_IN_BIP32_DERIVATION: u8 = 0x06;
pub const PSBT_IN_SCRIPTSIG:        u8 = 0x07;
pub const PSBT_IN_SCRIPTWITNESS:    u8 = 0x08;

/* ----------------- Output types  ----------------- */
pub const PSBT_OUT_REDEEMSCRIPT:     u8 = 0x00;
pub const PSBT_OUT_WITNESSSCRIPT:    u8 = 0x01;
pub const PSBT_OUT_BIP32_DERIVATION: u8 = 0x02;

/**
  | The separator is 0x00. Reading this in means
  | that the unserializer can interpret it as
  | a 0 length key which indicates that this is the
  | separator. The separator has no value.
  */
pub const PSBT_SEPARATOR: u8 = 0x00;

/**
  | BIP 174 does not specify a maximum file size,
  | but we set a limit anyway to prevent reading
  | a stream indefinitely and running out of
  | memory.
  */
pub const MAX_FILE_SIZE_PSBT: usize = 100000000; // 100 MiB

/* ------------------ Magic bytes  ------------------ */
lazy_static!{
    pub static ref PSBT_MAGIC_BYTES: [u8; 5] = 
    [
        b'p', 
        b's', 
        b'b', 
        b't', 
        0xff
    ];
}
