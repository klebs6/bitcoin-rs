// ---------------- [ File: bitcoin-base58/src/decode_check.rs ]
crate::ix!();

pub fn decode_base_58check_raw(
        psz:         *const u8,
        vch_ret:     &mut Vec<u8>,
        max_ret_len: i32) -> bool {
    
    let b = match max_ret_len > i32::MAX - 4
    {
        true  => i32::MAX,
        false => max_ret_len + 4,
    };

    if !unsafe { decode_base58_raw(psz, vch_ret, b) } 
        || vch_ret.len() < 4
    {
        vch_ret.clear();
        return false;
    }

    // re-calculate the checksum, ensure it
    // matches the included 4-byte checksum
    let hash: u256 
        = hash1(&vch_ret[0..vch_ret.len() - 4]);

    if unsafe { 
        libc::memcmp(
            &hash as *const _ as *const libc::c_void, 
            &vch_ret[vch_ret.len() - 4] as *const _ as *const libc::c_void, 
            4) 
    } != 0 
    {
        vch_ret.clear();
        return false;
    }


    vch_ret.resize(vch_ret.len() - 4, 0);
    return true
}

/// Decode a base58-encoded string (str) that includes a checksum into a byte
/// vector (vchRet), return true if decoding is successful
/// 
pub fn decode_base_58check(input: &str, vch_ret: &mut Vec<u8>, max_ret_len: usize) -> bool {
    if max_ret_len > usize::MAX - 4 {
        return false;
    }

    if !decode_base58(input, vch_ret, max_ret_len + 4) || vch_ret.len() < 4 {
        vch_ret.clear();
        return false;
    }

    let payload_len = vch_ret.len() - 4;
    let hash = hash1(&vch_ret[0..payload_len]);

    if &hash.as_ref()[..4] != &vch_ret[payload_len..] {
        vch_ret.clear();
        return false;
    }

    vch_ret.resize(payload_len, 0);
    true
}

#[cfg(test)]
mod decode_check_spec {
    use super::*;

    /// Round‑trip using **Base‑58‑check** with random payloads.
    #[traced_test]
    fn base58check_roundtrip_randomised() {
        // Simple LCG for deterministic pseudo‑random bytes
        let mut state = 0x1234_5678_9ABC_DEF0u64;
        for case in 0..128 {
            state = state.wrapping_mul(6364136223846793005u64).wrapping_add(1);
            let len = (state & 0x1F) as usize + 1; // 1‑32 bytes
            let mut data = (0..len).map(|i| (state >> ((i * 7) % 56)) as u8).collect::<Vec<_>>();

            let encoded = encode_base_58check(&data);
            trace!(case, len, ?encoded, "encoded random payload");

            let mut decoded = Vec::new();
            assert!(
                decode_base_58check(&encoded, &mut decoded, ((len + 4) as i32).try_into().unwrap()),
                "case {case}: decode failed"
            );
            assert_eq!(decoded, data, "case {case}: round‑trip mismatch");
        }
    }

    /// Corruption in **checksum** must be detected.
    #[traced_test]
    fn detects_bad_checksum() {
        let payload = b"bitcoin";
        let mut encoded = encode_base_58check(payload);
        // Flip one character that is guaranteed to stay in the alphabet.
        encoded.pop();
        encoded.push('2'); // replace final char
        let mut sink = Vec::new();
        assert!(
            !decode_base_58check(&encoded, &mut sink, 128),
            "corrupted checksum was not detected"
        );
    }
}
