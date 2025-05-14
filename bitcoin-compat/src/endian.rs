// ---------------- [ File: bitcoin-compat/src/endian.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/endian.h]

/*
  | While not technically a supported
  | configuration, defaulting to defining these
  | DECLs when we were compiled without autotools
  | makes it easier for other build systems to
  | build things like libbitcoinconsensus for
  | strange targets.
  */

#[cfg(not(HAVE_CONFIG_H))] #[cfg(htobe16)] pub const HAVE_DECL_HTOBE16: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(htole16)] pub const HAVE_DECL_HTOLE16: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(be16toh)] pub const HAVE_DECL_BE16TOH: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(le16toh)] pub const HAVE_DECL_LE16TOH: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(htobe32)] pub const HAVE_DECL_HTOBE32: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(htole32)] pub const HAVE_DECL_HTOLE32: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(be32toh)] pub const HAVE_DECL_BE32TOH: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(le32toh)] pub const HAVE_DECL_LE32TOH: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(htobe64)] pub const HAVE_DECL_HTOBE64: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(htole64)] pub const HAVE_DECL_HTOLE64: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(be64toh)] pub const HAVE_DECL_BE64TOH: usize = 1;
#[cfg(not(HAVE_CONFIG_H))] #[cfg(le64toh)] pub const HAVE_DECL_LE64TOH: usize = 1;

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_HTOBE16_EQ_0)]
#[inline] pub fn htobe16(host_16bits: u16) -> u16 {
    
    todo!();
        /*
            return host_16bits;
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_HTOLE16_EQ_0)]
#[inline] pub fn htole16(host_16bits: u16) -> u16 {
    
    todo!();
        /*
            return bswap_16(host_16bits);
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_BE16TOH_EQ_0)]
#[inline] pub fn be_16toh(big_endian_16bits: u16) -> u16 {
    
    todo!();
        /*
            return big_endian_16bits;
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_LE16TOH_EQ_0)]
#[inline] pub fn le_16toh(little_endian_16bits: u16) -> u16 {
    
    todo!();
        /*
            return bswap_16(little_endian_16bits);
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_HTOBE32_EQ_0)]
#[inline] pub fn htobe32(host_32bits: u32) -> u32 {
    
    todo!();
        /*
            return host_32bits;
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_HTOLE32_EQ_0)]
#[inline] pub fn htole32(host_32bits: u32) -> u32 {
    
    todo!();
        /*
            return bswap_32(host_32bits);
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_BE32TOH_EQ_0)]
#[inline] pub fn be_32toh(big_endian_32bits: u32) -> u32 {
    
    todo!();
        /*
            return big_endian_32bits;
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_LE32TOH_EQ_0)]
#[inline] pub fn le_32toh(little_endian_32bits: u32) -> u32 {
    
    todo!();
        /*
            return bswap_32(little_endian_32bits);
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_HTOBE64_EQ_0)]
#[inline] pub fn htobe64(host_64bits: u64) -> u64 {
    
    todo!();
        /*
            return host_64bits;
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_HTOLE64_EQ_0)]
#[inline] pub fn htole64(host_64bits: u64) -> u64 {
    
    todo!();
        /*
            return bswap_64(host_64bits);
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_BE64TOH_EQ_0)]
#[inline] pub fn be_64toh(big_endian_64bits: u64) -> u64 {
    
    todo!();
        /*
            return big_endian_64bits;
        */
}

#[cfg(WORDS_BIGENDIAN)]
#[cfg(HAVE_DECL_LE64TOH_EQ_0)]
#[inline] pub fn le_64toh(little_endian_64bits: u64) -> u64 {
    
    todo!();
        /*
            return bswap_64(little_endian_64bits);
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_HTOBE16_EQ_0)]
#[inline] pub fn htobe16(host_16bits: u16) -> u16 {
    
    todo!();
        /*
            return bswap_16(host_16bits);
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_HTOLE16_EQ_0)]
#[inline] pub fn htole16(host_16bits: u16) -> u16 {
    
    todo!();
        /*
            return host_16bits;
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_BE16TOH_EQ_0)]
#[inline] pub fn be_16toh(big_endian_16bits: u16) -> u16 {
    
    todo!();
        /*
            return bswap_16(big_endian_16bits);
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_LE16TOH_EQ_0)]
#[inline] pub fn le_16toh(little_endian_16bits: u16) -> u16 {
    
    todo!();
        /*
            return little_endian_16bits;
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_HTOBE32_EQ_0)]
#[inline] pub fn htobe32(host_32bits: u32) -> u32 {
    
    todo!();
        /*
            return bswap_32(host_32bits);
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_HTOLE32_EQ_0)]
#[inline] pub fn htole32(host_32bits: u32) -> u32 {
    
    todo!();
        /*
            return host_32bits;
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_BE32TOH_EQ_0)]
#[inline] pub fn be_32toh(big_endian_32bits: u32) -> u32 {
    
    todo!();
        /*
            return bswap_32(big_endian_32bits);
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_LE32TOH_EQ_0)]
#[inline] pub fn le_32toh(little_endian_32bits: u32) -> u32 {
    
    todo!();
        /*
            return little_endian_32bits;
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_HTOBE64_EQ_0)]
#[inline] pub fn htobe64(host_64bits: u64) -> u64 {
    
    todo!();
        /*
            return bswap_64(host_64bits);
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_HTOLE64_EQ_0)]
#[inline] pub fn htole64(host_64bits: u64) -> u64 {
    
    todo!();
        /*
            return host_64bits;
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_BE64TOH_EQ_0)]
#[inline] pub fn be_64toh(big_endian_64bits: u64) -> u64 {
    
    todo!();
        /*
            return bswap_64(big_endian_64bits);
        */
}

#[cfg(not(WORDS_BIGENDIAN))]
#[cfg(HAVE_DECL_LE64TOH_EQ_0)]
#[inline] pub fn le_64toh(little_endian_64bits: u64) -> u64 {
    
    todo!();
        /*
            return little_endian_64bits;
        */
}
