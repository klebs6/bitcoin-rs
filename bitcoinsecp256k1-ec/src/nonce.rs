crate::ix!();

/** 
 | A pointer to a function to deterministically
 | generate a nonce.
 |
 | Returns: 1 if a nonce was successfully
 | generated. 0 will cause signing to fail.
 |
 | Out:     nonce32:   pointer to a 32-byte array
 |                     to be filled by the
 |                     function.
 |
 | In:      msg32:     the 32-byte message hash
 |                     being verified (will not be
 |                     NULL)
 |
 |          key32:     pointer to a 32-byte secret
 |                     key (will not be NULL)
 |
 |          algo16:    pointer to a 16-byte array
 |                     describing the signature
 |                     algorithm (will be NULL for
 |                     ECDSA for compatibility).
 |
 |          data:      Arbitrary data pointer that
 |                     is passed through.
 |
 |          attempt:   how many iterations we have
 |                     tried to find a nonce.
 |                     This will almost always be
 |                     0, but different attempt
 |                     values are required to
 |                     result in a different
 |                     nonce.
 |
 | Except for test cases, this function should
 | compute some cryptographic hash of the message,
 | the algorithm, the key and the attempt.
 */
pub type NonceFunction = fn(
    nonce32: *mut u8,
    msg32:   *const u8,
    key32:   *const u8,
    algo16:  *const u8,
    data:    *mut c_void,
    attempt: u32
) -> i32;


