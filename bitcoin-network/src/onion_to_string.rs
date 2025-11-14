// ---------------- [ File: bitcoin-network/src/onion_to_string.rs ]
crate::ix!();

pub fn onion_to_string(addr: &[u8]) -> String {
    
    let mut checksum = [0_u8; TORV3_CHECKSUM_LEN];

    torv3_checksum(addr, &mut checksum);

    //  TORv3 onion_address = base32(PUBKEY | TORV3_CHECKSUM | TORV3_VERSION) + ".onion"
    let mut address: PreVector::<u8,{TORV3_TOTAL_LEN}> = PreVector::from(addr);

    address.extend(checksum);
    address.extend(TORV3_VERSION.iter().cloned());

    format!(
        "{}.onion", 
        encode_base32_bytes(
            address.as_slice(),
            Some(false)
        )
    )
}

#[cfg(test)]
mod onion_rendering_spec {
    use super::*;

    #[traced_test]
    fn onion_string_roundtrips_pubkey_checksum_and_version() {
        // Deterministic pubkey
        let mut pk = [0u8; ADDR_TORV3_SIZE];
        for (i, b) in pk.iter_mut().enumerate() { *b = (i as u8).wrapping_mul(3).wrapping_add(1); }

        let s = onion_to_string(&pk);
        assert!(s.ends_with(".onion"));

        let b32 = &s[..s.len() - ".onion".len()];
        let mut invalid = false;
        let decoded =
            bitcoin_string::decode_base32_bytes_nopad_lower(b32, Some(&mut invalid as *mut bool));
        assert!(!invalid);
        assert_eq!(decoded.len(), TORV3_TOTAL_LEN);

        let (got_pk, rest) = decoded.split_at(ADDR_TORV3_SIZE);
        let (got_ck, got_ver) = rest.split_at(TORV3_CHECKSUM_LEN);

        assert_eq!(got_pk, &pk);
        assert_eq!(got_ver, TORV3_VERSION);

        let mut expected_ck = [0u8; TORV3_CHECKSUM_LEN];
        torv3_checksum(&pk, &mut expected_ck);
        assert_eq!(got_ck, &expected_ck);
    }
}

#[cfg(test)]
mod onion_rendering_integrity_spec {
    use super::*;

    #[traced_test]
    fn onion_base32_payload_is_pubkey_checksum_version() {
        // Deterministic pubkey
        let mut pk = [0u8; ADDR_TORV3_SIZE];
        for (i, b) in pk.iter_mut().enumerate() { *b = (i as u8).wrapping_mul(3).wrapping_add(1); }

        // Compute expected checksum
        let mut ck = [0u8; TORV3_CHECKSUM_LEN];
        torv3_checksum(&pk, &mut ck);

        // Expected payload: PUBKEY | CHECKSUM | VERSION
        let mut expected_payload = Vec::with_capacity(TORV3_TOTAL_LEN);
        expected_payload.extend_from_slice(&pk);
        expected_payload.extend_from_slice(&ck);
        expected_payload.extend_from_slice(TORV3_VERSION);

        let s = onion_to_string(&pk);
        assert!(s.ends_with(".onion"));
        let b32 = &s[..s.len() - ".onion".len()];

        let expected_b32 = encode_base32_bytes(&expected_payload, Some(false));
        info!(expected_len = expected_b32.len(), "Comparing base32 payload against expected");
        assert_eq!(b32, expected_b32);
    }
}
