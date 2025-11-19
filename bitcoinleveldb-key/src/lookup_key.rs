// ---------------- [ File: bitcoinleveldb-key/src/lookup_key.rs ]
crate::ix!();

/**
  | A helper class useful for DBImpl::Get()
  |
  */
pub struct LookupKey {
    /**
      | We construct a char array of the form:
      |
      |    klength  varint32               <-- start_
      |    userkey  char[klength]          <-- kstart_
      |    tag      uint64
      |                                    <-- end_
      | The array is a suitable MemTable key.
      |
      | The suffix starting with "userkey" can be
      | used as an InternalKey.
      */
    start:  *const u8,
    kstart: *const u8,
    end:    *const u8,

    /**
      | Avoid allocation for short keys
      |
      */
    space:  [u8; 200],
    buf:    Vec<u8>,
}

impl Drop for LookupKey {
    fn drop(&mut self) {
        trace!("LookupKey::drop");
        // `buf` and `space` clean themselves up; `start`, `kstart`, and
        // `end` are raw pointers into that storage and require no action.
    }
}

impl LookupKey {
    /**
      | Initialize *this for looking up user_key
      | at a snapshot with the specified sequence
      | number.
      |
      */
    pub fn new(user_key_: &Slice, sequence: SequenceNumber) -> Self {
        let usize = *user_key_.size();
        trace!(
            "LookupKey::new: user_key_len={}, seq={}",
            usize,
            sequence
        );

        // A conservative estimate: user key + varint32(len+8) + 8 tag bytes
        let needed = usize + 13;
        let mut buf = Vec::with_capacity(needed);

        // Encode length of internal key as varint32
        let key_len = (usize + 8) as u32;
        put_varint32(&mut buf, key_len);
        let kstart_index = buf.len();

        unsafe {
            if usize > 0 {
                let data = *user_key_.data();
                let bytes = std::slice::from_raw_parts(data, usize);
                buf.extend_from_slice(bytes);
            }
        }

        let packed = pack_sequence_and_type(sequence, VALUE_TYPE_FOR_SEEK);
        let tag = encode_fixed64_le(packed);
        buf.extend_from_slice(&tag);

        let start_ptr = buf.as_ptr();
        let kstart_ptr = unsafe { start_ptr.add(kstart_index) };
        let end_ptr = unsafe { start_ptr.add(buf.len()) };

        LookupKey {
            start:  start_ptr,
            kstart: kstart_ptr,
            end:    end_ptr,
            space:  [0u8; 200],
            buf,
        }
    }

    #[inline]
    fn end_offset(&self) -> usize {
        (self.end as usize) - (self.start as usize)
    }

    /**
      | Return a key suitable for lookup in a
      | MemTable.
      |
      */
    pub fn memtable_key(&self) -> Slice {
        trace!("LookupKey::memtable_key");
        let len = self.end_offset();
        unsafe { Slice::from_ptr_len(self.start, len) }
    }

    /**
      | Return an internal key (suitable for
      | passing to an internal iterator)
      |
      */
    pub fn internal_key(&self) -> Slice {
        trace!("LookupKey::internal_key");
        let len = (self.end as usize) - (self.kstart as usize);
        unsafe { Slice::from_ptr_len(self.kstart, len) }
    }

    /**
      | Return the user key
      |
      */
    pub fn user_key(&self) -> Slice {
        trace!("LookupKey::user_key");
        let internal_len = (self.end as usize) - (self.kstart as usize);
        assert!(
            internal_len >= 8,
            "LookupKey::user_key: internal_len too small: {}",
            internal_len
        );
        let user_len = internal_len - 8;
        unsafe { Slice::from_ptr_len(self.kstart, user_len) }
    }
}

#[cfg(test)]
mod lookup_key_tests {
    use super::*;

    fn build_lookup(user: &[u8], seq: SequenceNumber) -> LookupKey {
        unsafe {
            let user_slice = Slice::from_ptr_len(user.as_ptr(), user.len());
            LookupKey::new(&user_slice, seq)
        }
    }

    #[traced_test]
    fn lookup_key_layout_and_accessors_non_empty_user_key() {
        let user = b"hello";
        let seq: SequenceNumber = 100;

        let lk = build_lookup(user, seq);

        let memtable_key_slice = lk.memtable_key();
        let internal_key_slice = lk.internal_key();
        let user_key_slice = lk.user_key();

        unsafe {
            let mk_len = *memtable_key_slice.size();
            let mk_ptr = *memtable_key_slice.data();
            let mem_bytes = std::slice::from_raw_parts(mk_ptr, mk_len);

            // Decode varint length prefix.
            let (internal_len, varint_len) = decode_varint32(mem_bytes);
            assert_eq!(
                internal_len as usize,
                user.len() + 8,
                "internal length must be user length + 8"
            );
            assert_eq!(
                mk_len,
                varint_len + internal_len as usize,
                "memtable key length must match prefix+internal"
            );

            // Internal key should be suffix after varint32.
            let internal_bytes = &mem_bytes[varint_len..];
            let ik_len = *internal_key_slice.size();
            let ik_ptr = *internal_key_slice.data();
            let ik_bytes = std::slice::from_raw_parts(ik_ptr, ik_len);
            assert_eq!(internal_bytes, ik_bytes);

            // User key should be prefix of internal key.
            let uk_len = *user_key_slice.size();
            let uk_ptr = *user_key_slice.data();
            let uk_bytes = std::slice::from_raw_parts(uk_ptr, uk_len);
            assert_eq!(uk_bytes, user);

            // Last 8 bytes of internal key must be the packed tag.
            let tag_expected =
                encode_fixed64_le(pack_sequence_and_type(seq, VALUE_TYPE_FOR_SEEK));
            let tag_actual = &internal_bytes[internal_bytes.len() - 8..];
            assert_eq!(
                tag_expected, tag_actual,
                "internal key tag mismatch"
            );
        }
    }

    #[traced_test]
    fn lookup_key_handles_empty_user_key() {
        let user: &[u8] = &[];
        let seq: SequenceNumber = 999;

        let lk = build_lookup(user, seq);

        let memtable_key_slice = lk.memtable_key();
        let internal_key_slice = lk.internal_key();
        let user_key_slice = lk.user_key();

        unsafe {
            let mk_len = *memtable_key_slice.size();
            let mk_ptr = *memtable_key_slice.data();
            let mem_bytes = std::slice::from_raw_parts(mk_ptr, mk_len);

            let (internal_len, varint_len) = decode_varint32(mem_bytes);
            assert_eq!(internal_len as usize, 8);
            assert_eq!(
                mk_len,
                varint_len + 8,
                "memtable key should contain only length prefix + tag for empty user key"
            );

            let ik_len = *internal_key_slice.size();
            let ik_ptr = *internal_key_slice.data();
            let ik_bytes = std::slice::from_raw_parts(ik_ptr, ik_len);
            assert_eq!(
                ik_bytes.len(),
                8,
                "internal key must be exactly 8 bytes for empty user key"
            );

            let uk_len = *user_key_slice.size();
            assert_eq!(
                uk_len, 0,
                "user_key() must report zero length for empty user"
            );
        }
    }
}
