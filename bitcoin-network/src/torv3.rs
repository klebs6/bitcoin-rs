crate::ix!();

/**
   https://gitweb.torproject.org/torspec.git/tree/rend-spec-v3.txt#n2135
  */
pub const TORV3_CHECKSUM_LEN: usize = 2;
pub const TORV3_VERSION:      &[u8] = &[3];
pub const TORV3_TOTAL_LEN:    usize = ADDR_TORV3_SIZE + TORV3_CHECKSUM_LEN + size_of_val(TORV3_VERSION);

pub fn torv3_checksum(
    addr_pubkey: &[u8],
    checksum:    &mut [u8; TORV3_CHECKSUM_LEN])  {

    // TORv3 CHECKSUM = H(".onion checksum" | PUBKEY | VERSION)[:2]
    pub const PREFIX: &'static str = ".onion checksum";

    let mut hasher: SHA3_256 = SHA3_256::default();

    hasher.write(PREFIX.as_bytes());

    hasher.write(addr_pubkey);

    hasher.write(TORV3_VERSION);

    let mut checksum_full = [0_u8; SHA3_256_OUTPUT_SIZE];

    hasher.finalize(checksum_full.as_slice());

    checksum[0..TORV3_CHECKSUM_LEN].copy_from_slice(&checksum_full[0..TORV3_CHECKSUM_LEN]);
}
