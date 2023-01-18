crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compressor.h]

/**
  | This saves us from making many heap allocations
  | when serializing and deserializing
  | compressed scripts.
  | 
  | This prevector size is determined by
  | the largest .resize() in the
  | 
  | CompressScript function. The largest
  | compressed script format is a compressed
  | public key, which is 33 bytes.
  |
  */
pub type CompressedScript = PreVector<u8,33>;

lazy_static!{
    /*
   FORMATTER_METHODS(CTxOut, obj) { 
       READWRITE(Using<AmountCompression>(obj.nValue), Using<ScriptCompression>(obj.scriptPubKey)); 
   }
    */
}

//-------------------------------------------[.cpp/bitcoin/src/compressor.cpp]

pub fn get_special_script_size(n_size: u32) -> usize {
    
    if n_size == 0 || n_size == 1 {
        return 20;
    }

    if n_size == 2 || n_size == 3 || n_size == 4 || n_size == 5 {
        return 32;
    }

    0
}

//TODO: test this
pub fn u64_plus_i32(u: u64, i: i32) -> u64 {

    match i {
        0          => u,
        i32::MIN   => u.saturating_sub(i32::MAX as u64 + 1),
        i32::MAX   => u.saturating_add(i32::MAX as u64),
        _ => {
            if i > 0 {
                u.saturating_add(i.try_into().unwrap())
            } else {
                u.saturating_sub(i.abs().try_into().unwrap())
            }
        }
    }
}
