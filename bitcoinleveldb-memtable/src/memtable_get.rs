// ---------------- [ File: bitcoinleveldb-memtable/src/memtable_get.rs ]
crate::ix!();
    
impl MemTable {

    /// If memtable contains a value for key, store it in *value and return
    /// true.
    /// 
    /// If memtable contains a deletion for key, store a NotFound() error in
    /// *status and return true.
    /// 
    /// Both a deletion and an absent key leave the byte output unchanged.
    /// Else, return false and preserve the supplied status.
    ///
    pub fn get_bytes(
        &mut self,
        key_:   &LookupKey,
        value: *mut Vec<u8>,
        s:     *mut Status,
    ) -> bool {
        trace!("MemTable::get: begin lookup");

        assert!(
            !value.is_null(),
            "MemTable::get: value pointer must not be null"
        );
        assert!(
            !s.is_null(),
            "MemTable::get: Status pointer must not be null"
        );

        let memkey = key_.memtable_key();
        trace!(
            "MemTable::get: memtable_key_len={}",
            *memkey.size()
        );

        unsafe {
            let table_ref: &MemTableTable = self.table();
            let mut iter =
                SkipListIterator::new(table_ref);

            let memkey_ptr: *const u8 = *memkey.data();
            iter.seek(&memkey_ptr);

            if iter.valid() {
                // entry format is:
                //    klength  varint32
                //    userkey  char[klength]
                //    tag      uint64
                //    vlength  varint32
                //    value    char[vlength]
                let entry: *const u8 = iter.key();
                trace!(
                    "MemTable::get: found candidate entry @ {:?}",
                    entry
                );

                // Decode varint32 key length from the entry prefix (up to 5 bytes).
                let header_bytes =
                    core::slice::from_raw_parts(entry, 5);
                let (key_length32, key_varint_len) =
                    decode_varint32(header_bytes);
                let key_length = key_length32 as usize;
                let key_ptr = entry.add(key_varint_len);

                trace!(
                    "MemTable::get: key_length={} (varint_len={})",
                    key_length,
                    key_varint_len
                );

                assert!(
                    key_length >= 8,
                    "MemTable::get: key_length too small ({})",
                    key_length
                );

                let internal_cmp =
                    self.comparator().internal_comparator();
                let user_cmp_ptr =
                    internal_cmp.user_comparator();

                let mem_user_key =
                    Slice::from_ptr_len(
                        key_ptr,
                        key_length - 8,
                    );
                let lookup_user_key =
                    key_.user_key();

                let same_user_key =
                    if !user_cmp_ptr.is_null() {
                        let uc = &*user_cmp_ptr;
                        let cmp_res =
                            uc.compare(
                                &mem_user_key,
                                &lookup_user_key,
                            );
                        cmp_res == 0
                    } else {
                        bytewise_compare(
                            slice_as_bytes(&mem_user_key),
                            slice_as_bytes(
                                &lookup_user_key,
                            ),
                        ) == 0
                    };

                if same_user_key {
                    let tag_ptr =
                        key_ptr.add(key_length - 8);
                    let tag =
                        decode_fixed64_le(tag_ptr);
                    let value_type_byte =
                        (tag & 0xff) as u8;

                    trace!(
                        "MemTable::get: matched user key, tag=0x{:x}",
                        tag
                    );

                    match ValueType::from_tag(
                        value_type_byte,
                    ) {
                        Some(ValueType::TypeValue) => {
                            // Pointer to value length varint prefix
                            // immediately follows the internal key.
                            let value_header_ptr =
                                key_ptr.add(key_length);

                            // Decode value length varint32 directly
                            // from raw bytes (at most 5 bytes).
                            let mut value_len_u32: u32 = 0;
                            let mut shift: u32 = 0;
                            let mut consumed: usize = 0;

                            loop {
                                let byte = *value_header_ptr
                                    .add(consumed)
                                    as u32;
                                value_len_u32 |=
                                    (byte & 0x7f) << shift;
                                consumed += 1;

                                if (byte & 0x80) == 0 {
                                    break;
                                }

                                shift += 7;

                                assert!(
                                    shift < 32,
                                    "MemTable::get: value length varint32 shift overflow"
                                );
                                assert!(
                                    consumed < 5,
                                    "MemTable::get: value length varint32 exceeds 5 bytes"
                                );
                            }

                            let value_len =
                                value_len_u32 as usize;
                            let value_ptr =
                                value_header_ptr.add(consumed);

                            trace!(
                                "MemTable::get: decoded value_len={} (varint_len={})",
                                value_len,
                                consumed
                            );

                            let value_slice =
                                Slice::from_ptr_len(
                                    value_ptr,
                                    value_len,
                                );

                            trace!(
                                "MemTable::get: found value_len={}",
                                *value_slice.size()
                            );

                            // Materialize before assigning the destination; no UTF-8 invariant
                            // applies to a LevelDB value.
                            *value = value_slice.as_bytes().to_vec();
                            return true;
                        }
                        Some(ValueType::TypeDeletion) => {
                            trace!(
                                "MemTable::get: tombstone found; returning NotFound"
                            );
                            let empty =
                                Slice::default();
                            let st =
                                Status::not_found(
                                    &empty,
                                    None,
                                );
                            *s = st;
                            return true;
                        }
                        None => {
                            debug!(
                                "MemTable::get: unknown value type tag {}; treating as missing entry",
                                value_type_byte
                            );
                            return false;
                        }
                    }
                }
            }
        }

        trace!("MemTable::get: key not found");
        false
    }


    /// Checked legacy text adapter. A direct memtable tombstone still clears
    /// the text output as before; get_bytes preserves its output on tombstones.
    /// Invalid UTF-8 returns true with Corruption and preserves the text output.
    pub fn get(&mut self, key_: &LookupKey, value: *mut String, s: *mut Status) -> bool {
        assert!(!value.is_null(), "MemTable::get: value pointer must not be null");
        assert!(!s.is_null(), "MemTable::get: Status pointer must not be null");
        let mut bytes = Vec::new();
        let mut status = Status::ok();
        if !self.get_bytes(key_, &mut bytes, &mut status) {
            return false;
        }
        unsafe {
            if status.is_ok() {
                match String::from_utf8(bytes) {
                    Ok(text) => { *value = text; return true; }
                    Err(_) => status = Status::corruption(&Slice::from("memtable value is not valid UTF-8"), None),
                }
            } else if status.is_not_found() {
                (*value).clear();
            }
            *s = status;
        }
        true
    }
}
