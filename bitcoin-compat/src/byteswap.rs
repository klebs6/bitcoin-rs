// ---------------- [ File: bitcoin-compat/src/byteswap.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/compat/byteswap.h]

#[cfg(MAC_OSX)] macro_rules! bswap_16 { ($x:ident) => { /* OSSwapInt16(x) */ } }
#[cfg(MAC_OSX)] macro_rules! bswap_32 { ($x:ident) => { /* OSSwapInt32(x) */ } }
#[cfg(MAC_OSX)] macro_rules! bswap_64 { ($x:ident) => { /* OSSwapInt64(x) */ } }

/* ------------ Non-MacOS / non-Darwin  ------------ */
#[cfg(not(MAC_OSX))]
#[cfg(HAVE_DECL_BSWAP_16_EQ_0)]
#[inline] pub fn bswap_16(x: u16) -> u16 {
    
    todo!();
        /*
            return (x >> 8) | (x << 8);
        */
}

#[cfg(not(MAC_OSX))]
#[cfg(HAVE_DECL_BSWAP_32_EQ_0)]
#[inline] pub fn bswap_32(x: u32) -> u32 {
    
    todo!();
        /*
            return (((x & 0xff000000U) >> 24) | ((x & 0x00ff0000U) >>  8) |
                ((x & 0x0000ff00U) <<  8) | ((x & 0x000000ffU) << 24));
        */
}

#[cfg(not(MAC_OSX))]
#[cfg(HAVE_DECL_BSWAP_64_EQ_0)]
#[inline] pub fn bswap_64(x: u64) -> u64 {
    
    todo!();
        /*
            return (((x & 0xff00000000000000ull) >> 56)
              | ((x & 0x00ff000000000000ull) >> 40)
              | ((x & 0x0000ff0000000000ull) >> 24)
              | ((x & 0x000000ff00000000ull) >> 8)
              | ((x & 0x00000000ff000000ull) << 8)
              | ((x & 0x0000000000ff0000ull) << 24)
              | ((x & 0x000000000000ff00ull) << 40)
              | ((x & 0x00000000000000ffull) << 56));
        */
}
