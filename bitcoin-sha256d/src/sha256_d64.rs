crate::ix!();

/**
  | Compute multiple double-Sha256's
  | of 64-byte blobs.
  | 
  | Output: pointer to a blocks*32 byte
  | output buffer
  | 
  | Input: pointer to a blocks*64 byte input
  | buffer
  | 
  | Blocks: the number of hashes to compute.
  |
  */
pub fn sha256d64(
        out:    *mut u8,
        in_:    *const u8,
        blocks: usize)  {
    
    todo!();
        /*
            if (TransformD64_8way) {
            while (blocks >= 8) {
                TransformD64_8way(out, in);
                out += 256;
                in += 512;
                blocks -= 8;
            }
        }
        if (TransformD64_4way) {
            while (blocks >= 4) {
                TransformD64_4way(out, in);
                out += 128;
                in += 256;
                blocks -= 4;
            }
        }
        if (TransformD64_2way) {
            while (blocks >= 2) {
                TransformD64_2way(out, in);
                out += 64;
                in += 128;
                blocks -= 2;
            }
        }
        while (blocks) {
            TransformD64(out, in);
            out += 32;
            in += 64;
            --blocks;
        }
        */
}
