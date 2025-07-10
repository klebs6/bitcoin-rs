crate::ix!();

/**
  | Initializes a sha256 struct and writes
  | the 64 byte string
  | SHA256(tag)||SHA256(tag) into it.
  |
  */
pub unsafe fn sha256_initialize_tagged(
    hash:   *mut Sha256,
    tag:    *const u8,
    taglen: usize
) {
    let mut buf: [u8; 32] = [0; 32];

    sha256_initialize((*hash).s_mut().as_mut_ptr());
    sha256_write(hash, tag, taglen);
    sha256_finalize(hash, buf.as_mut_ptr());

    sha256_initialize((*hash).s_mut().as_mut_ptr());
    sha256_write(hash, buf.as_mut_ptr(), 32);
    sha256_write(hash, buf.as_mut_ptr(), 32);
}
