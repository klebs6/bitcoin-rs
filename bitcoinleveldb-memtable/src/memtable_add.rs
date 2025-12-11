// ---------------- [ File: bitcoinleveldb-memtable/src/memtable_add.rs ]
crate::ix!();
    
impl MemTable {

    /**
      | Add an entry into memtable that maps key to
      | value at the specified sequence number and
      | with the specified type.
      |
      | Typically value will be empty if
      | type==kTypeDeletion.
      */
    pub fn add(
        &mut self,
        s:     SequenceNumber,
        ty:    ValueType,
        key_:  &Slice,
        value: &Slice,
    ) {
        // Format of an entry is concatenation of:
        //  key_size     : varint32 of internal_key.size()
        //  key bytes    : char[internal_key.size()]
        //  value_size   : varint32 of value.size()
        //  value bytes  : char[value.size()]
        let key_size  = *key_.size();
        let val_size  = *value.size();
        let internal_key_size: usize = key_size + 8;

        trace!(
            "MemTable::add: seq={}, ty={:?}, key_size={}, val_size={}",
            s,
            ty,
            key_size,
            val_size
        );

        let internal_key_u32: u32 = internal_key_size
            .try_into()
            .expect("MemTable::add: internal_key_size does not fit into u32");
        let val_size_u32: u32 = val_size
            .try_into()
            .expect("MemTable::add: value size does not fit into u32");

        let encoded_len: usize = {
            let header_len: usize = varint_length(internal_key_u32 as u64)
                .try_into()
                .expect("MemTable::add: header varint length does not fit into usize");
            let value_len_prefix: usize = varint_length(val_size_u32 as u64)
                .try_into()
                .expect("MemTable::add: value varint length does not fit into usize");
            header_len
                .checked_add(internal_key_size)
                .and_then(|x| x.checked_add(value_len_prefix))
                .and_then(|x| x.checked_add(val_size))
                .expect("MemTable::add: encoded_len overflow")
        };

        trace!(
            "MemTable::add: internal_key_size={}, encoded_len={}",
            internal_key_size,
            encoded_len
        );

        unsafe {
            let arena_ref: &mut Arena = &mut *self.arena_mut();
            let buf: *mut u8 = arena_ref.allocate(encoded_len);
            let mut p: *mut u8 = encode_varint32(buf, internal_key_u32);

            if key_size > 0 {
                let key_data: *const u8 = *key_.data();
                std::ptr::copy_nonoverlapping(key_data, p, key_size);
                p = p.add(key_size);
            }

            let tag = pack_sequence_and_type(s, ty);
            let tag_bytes = encode_fixed64_le(tag);
            std::ptr::copy_nonoverlapping(tag_bytes.as_ptr(), p, 8);
            p = p.add(8);

            p = encode_varint32(p, val_size_u32);

            if val_size > 0 {
                let val_data: *const u8 = *value.data();
                std::ptr::copy_nonoverlapping(val_data, p, val_size);
            }

            debug_assert_eq!(
                p.add(val_size),
                buf.add(encoded_len),
                "MemTable::add: encoded length mismatch"
            );

            let entry_key: *const u8 = buf as *const u8;
            self.table_mut().insert(entry_key);

            trace!(
                "MemTable::add: inserted entry at {:?}",
                entry_key
            );
        }
    }
}
