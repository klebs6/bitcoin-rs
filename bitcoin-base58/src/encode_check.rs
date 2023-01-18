crate::ix!();

/**
  | Encode a byte span into a base58-encoded
  | string, including checksum
  |
  */
pub fn encode_base_58check(input: &[u8]) -> String {
    
    // add 4-byte hash check to the end
    let mut vch: Vec::<u8> = input.to_vec();

    let hash: u256 = hash1(&vch);

    for byte in &hash.blob.data[0..4] {
        vch.push(*byte);
    }

    encode_base58(&vch)
}
