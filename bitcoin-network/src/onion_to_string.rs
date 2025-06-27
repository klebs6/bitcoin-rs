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
        encode_base32(
            address.as_slice(),
            None
        )
    )
}
