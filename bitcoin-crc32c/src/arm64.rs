// ---------------- [ File: bitcoin-crc32c/src/arm64.rs ]
/*!
  | In a separate source file to allow this
  | accelerated CRC32C function to be compiled with
  | the appropriate compiler flags to enable ARM
  | NEON CRC32C instructions.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_arm64.h]
//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_arm64.cc]

/* --------------- * ARM-specific code  --------------- */

/**
   This implementation is based on
   https://github.com/google/leveldb/pull/490.
  */
#[cfg(HAVE_ARM64_CRC32C)]
pub const KBYTES:       usize = 1032;

#[cfg(HAVE_ARM64_CRC32C)]
pub const SEGMENTBYTES: usize = 256;

/**
   compute 8bytes for each segment parallelly
  */
#[cfg(HAVE_ARM64_CRC32C)]
macro_rules! crc32c32bytes {
    ($P:ident, $IND:ident) => {
        /*
        
          do {                                                                    
            crc1 = __crc32cd(                                                     
                crc1, *((const uint64_t *)(P) + (SEGMENTBYTES / 8) * 1 + (IND))); 
            crc2 = __crc32cd(                                                     
                crc2, *((const uint64_t *)(P) + (SEGMENTBYTES / 8) * 2 + (IND))); 
            crc3 = __crc32cd(                                                     
                crc3, *((const uint64_t *)(P) + (SEGMENTBYTES / 8) * 3 + (IND))); 
            crc0 = __crc32cd(                                                     
                crc0, *((const uint64_t *)(P) + (SEGMENTBYTES / 8) * 0 + (IND))); 
          } while (0);
        */
    }
}

/**
   compute 8*8 bytes for each segment parallelly
  */
#[cfg(HAVE_ARM64_CRC32C)]
macro_rules! crc32c256bytes {
    ($P:ident, $IND:ident) => {
        /*
        
          do {                              
            CRC32C32BYTES((P), (IND)*8 + 0) 
            CRC32C32BYTES((P), (IND)*8 + 1) 
            CRC32C32BYTES((P), (IND)*8 + 2) 
            CRC32C32BYTES((P), (IND)*8 + 3) 
            CRC32C32BYTES((P), (IND)*8 + 4) 
            CRC32C32BYTES((P), (IND)*8 + 5) 
            CRC32C32BYTES((P), (IND)*8 + 6) 
            CRC32C32BYTES((P), (IND)*8 + 7) 
          } while (0);
        */
    }
}

/**
   compute 4*8*8 bytes for each segment parallelly
  */
#[cfg(HAVE_ARM64_CRC32C)]
macro_rules! crc32c1024bytes {
    ($P:ident) => {
        /*
        
          do {                       
            CRC32C256BYTES((P), 0)   
            CRC32C256BYTES((P), 1)   
            CRC32C256BYTES((P), 2)   
            CRC32C256BYTES((P), 3)   
            (P) += 4 * SEGMENTBYTES; 
          } while (0)
        */
    }
}

#[cfg(HAVE_ARM64_CRC32C)]
pub mod crc32c {

    pub fn extend_arm64(
            crc:  u32,
            data: *const u8,
            size: usize) -> u32 {
        
        todo!();
            /*
                int64_t length = size;
          uint32_t crc0, crc1, crc2, crc3;
          uint64_t t0, t1, t2;

          // k0=CRC(x^(3*SEGMENTBYTES*8)), k1=CRC(x^(2*SEGMENTBYTES*8)),
          // k2=CRC(x^(SEGMENTBYTES*8))
          const poly64_t k0 = 0x8d96551c, k1 = 0xbd6f81f8, k2 = 0xdcb17aa4;

          crc = crc ^ kCRC32Xor;

          while (length >= KBYTES) {
            crc0 = crc;
            crc1 = 0;
            crc2 = 0;
            crc3 = 0;

            // Process 1024 bytes in parallel.
            CRC32C1024BYTES(data);

            // Merge the 4 partial CRC32C values.
            t2 = (uint64_t)vmull_p64(crc2, k2);
            t1 = (uint64_t)vmull_p64(crc1, k1);
            t0 = (uint64_t)vmull_p64(crc0, k0);
            crc = __crc32cd(crc3, *(uint64_t *)data);
            data += sizeof(uint64_t);
            crc ^= __crc32cd(0, t2);
            crc ^= __crc32cd(0, t1);
            crc ^= __crc32cd(0, t0);

            length -= KBYTES;
          }

          while (length >= 8) {
            crc = __crc32cd(crc, *(uint64_t *)data);
            data += 8;
            length -= 8;
          }

          if (length & 4) {
            crc = __crc32cw(crc, *(uint32_t *)data);
            data += 4;
          }

          if (length & 2) {
            crc = __crc32ch(crc, *(uint16_t *)data);
            data += 2;
          }

          if (length & 1) {
            crc = __crc32cb(crc, *data);
          }

          return crc ^ kCRC32Xor;
            */
    }
}
