// ---------------- [ File: bitcoinleveldbt-util/src/random_key.rs ]
crate::ix!();

/// Generate a reference-test key as literal bytes, including fd/fe/ff.
/// Conditional length selection and per-byte draws match the original workload.
/// A null PRNG returns an empty key without dereferencing it.
pub fn dbtest_random_key_bytes(rnd: *mut Random) -> Vec<u8> {
    if rnd.is_null() { return Vec::new(); }
    let len = unsafe {
        if (*rnd).one_in(3) { 1 }
        else if (*rnd).one_in(100) { (*rnd).skewed(10) as i32 }
        else { (*rnd).uniform(10) as i32 }
    };
    random_key_bytes(rnd, len)
}

/// Exactly len raw bytes from the LevelDB boundary-key alphabet.
/// Nonpositive lengths and a null PRNG return empty without consuming draws.
pub fn random_key_bytes(rnd: *mut Random, len: i32) -> Vec<u8> {
    const ALPHABET: [u8; 10] = [0, 1, b'a', b'b', b'c', b'd', b'e', 0xfd, 0xfe, 0xff];
    if rnd.is_null() || len <= 0 { return Vec::new(); }
    let mut bytes = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let index = unsafe { (*rnd).uniform(ALPHABET.len() as i32) } as usize;
        bytes.push(ALPHABET[index]);
    }
    bytes
}

/// Legacy text workload: each selected byte is mapped to a Unicode scalar.
/// High bytes therefore occupy two UTF-8 bytes. This intentionally preserves
/// existing operation tapes; callers wanting raw keys must opt into the byte API.
pub fn dbtest_random_key(rnd: *mut Random) -> String {
    dbtest_random_key_bytes(rnd).into_iter().map(char::from).collect()
}

/// Legacy text counterpart of random_key_bytes; len counts selected scalars,
/// not UTF-8 bytes. PRNG draw order is identical to the byte counterpart.
pub fn random_key(rnd: *mut Random, len: i32) -> String {
    random_key_bytes(rnd, len).into_iter().map(char::from).collect()
}
