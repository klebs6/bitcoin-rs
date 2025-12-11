// ---------------- [ File: bitcoinleveldb-memtable/src/memtable_iterator.rs ]
crate::ix!();

pub struct MemTableIterator {
    table: *mut MemTableTable,
    node:  *mut SkipListNode<*const u8>,

    /**
       For passing to EncodeKey
      */
    tmp: String,
}

impl MemTableIterator {
    pub fn new(table: *mut MemTableTable) -> Self {
        trace!(
            "MemTableIterator::new: table_ptr={:?}",
            table
        );
        MemTableIterator {
            table,
            node: core::ptr::null_mut(),
            tmp:  String::new(),
        }
    }

    #[inline]
    fn list(&self) -> &MemTableTable {
        assert!(
            !self.table.is_null(),
            "MemTableIterator::list: table pointer is null"
        );
        unsafe { &*self.table }
    }

    #[inline]
    fn _list_mut(&mut self) -> &mut MemTableTable {
        assert!(
            !self.table.is_null(),
            "MemTableIterator::list_mut: table pointer is null"
        );
        unsafe { &mut *self.table }
    }

    #[inline]
    fn current_node_key_ptr(&self) -> *const u8 {
        assert!(
            !self.node.is_null(),
            "MemTableIterator::current_node_key_ptr: iterator not valid"
        );
        unsafe {
            let node_ref: &SkipListNode<*const u8> =
                &*self.node;
            let key_ref: &*const u8 =
                node_ref.key_ref();
            let key_ptr = *key_ref;
            trace!(
                "MemTableIterator::current_node_key_ptr: node={:p}, key_ptr={:?}",
                self.node,
                key_ptr
            );
            key_ptr
        }
    }
}

impl LevelDBIteratorInterface for MemTableIterator {}

impl LevelDBIteratorValid for MemTableIterator {
    fn valid(&self) -> bool {
        let v = !self.node.is_null();
        trace!(
            "MemTableIterator::valid -> {} (node={:p})",
            v,
            self.node
        );
        v
    }
}

impl LevelDBIteratorSeek for MemTableIterator {
    fn seek(&mut self, k: &Slice) {
        trace!(
            "MemTableIterator::seek: target_len={}",
            *k.size()
        );

        unsafe {
            let encoded_ptr =
                encode_key(&mut self.tmp, k);
            let encoded_key: *const u8 =
                encoded_ptr;

            self.node = self.list()
                .find_greater_or_equal(
                    &encoded_key,
                    None,
                );

            trace!(
                "MemTableIterator::seek: new node={:p}",
                self.node
            );
        }
    }
}

impl LevelDBIteratorSeekToFirst for MemTableIterator {
    fn seek_to_first(&mut self) {
        trace!("MemTableIterator::seek_to_first");

        let list = self.list();

        unsafe {
            let head_ptr: *mut SkipListNode<
                *const u8,
            > = *list.head();
            let next_ptr = match head_ptr.as_ref() {
                Some(head_node) => {
                    let next = head_node.next(0);
                    trace!(
                        "MemTableIterator::seek_to_first: head={:p}, next={:p}",
                        head_ptr,
                        next
                    );
                    next
                }
                None => {
                    warn!(
                        "MemTableIterator::seek_to_first: head pointer is null"
                    );
                    core::ptr::null_mut()
                }
            };
            self.node = next_ptr;
            trace!(
                "MemTableIterator::seek_to_first: node={:p}",
                self.node
            );
        }
    }
}

impl LevelDBIteratorSeekToLast for MemTableIterator {
    fn seek_to_last(&mut self) {
        trace!("MemTableIterator::seek_to_last");

        let list = self.list();

        unsafe {
            let mut x = list.find_last();
            let head_ptr: *mut SkipListNode<
                *const u8,
            > = *list.head();
            if x == head_ptr {
                x = core::ptr::null_mut();
            }
            self.node = x;
            trace!(
                "MemTableIterator::seek_to_last: node={:p}",
                self.node
            );
        }
    }
}

impl LevelDBIteratorNext for MemTableIterator {
    fn next(&mut self) {
        trace!(
            "MemTableIterator::next: current node={:p}",
            self.node
        );
        assert!(
            !self.node.is_null(),
            "MemTableIterator::next called on invalid iterator"
        );
        unsafe {
            let node_ref: &SkipListNode<*const u8> =
                &*self.node;
            self.node = node_ref.next(0);
            trace!(
                "MemTableIterator::next: new node={:p}",
                self.node
            );
        }
    }
}

impl LevelDBIteratorPrev for MemTableIterator {
    fn prev(&mut self) {
        trace!(
            "MemTableIterator::prev: current node={:p}",
            self.node
        );
        assert!(
            !self.node.is_null(),
            "MemTableIterator::prev called on invalid iterator"
        );

        let list = self.list();

        unsafe {
            let cur_key_ptr =
                self.current_node_key_ptr();
            let cur_key: *const u8 =
                cur_key_ptr;

            let mut x =
                list.find_less_than(&cur_key);
            let head_ptr: *mut SkipListNode<
                *const u8,
            > = *list.head();
            if x == head_ptr {
                x = core::ptr::null_mut();
            }
            self.node = x;
            trace!(
                "MemTableIterator::prev: new node={:p}",
                self.node
            );
        }
    }
}

impl LevelDBIteratorKey for MemTableIterator {

    fn key(&self) -> Slice {
        trace!(
            "MemTableIterator::key: node={:p}",
            self.node
        );
        assert!(
            !self.node.is_null(),
            "MemTableIterator::key called on invalid iterator"
        );
        unsafe {
            let entry_ptr =
                self.current_node_key_ptr();

            // Decode the internal key length prefix (varint32, up to 5 bytes).
            let header_bytes =
                core::slice::from_raw_parts(entry_ptr, 5);
            let (key_len32, key_varint_len) =
                decode_varint32(header_bytes);
            let key_len = key_len32 as usize;
            let key_ptr =
                entry_ptr.add(key_varint_len);

            let key_slice =
                Slice::from_ptr_len(
                    key_ptr,
                    key_len,
                );

            trace!(
                "MemTableIterator::key: key_len={} (varint_len={})",
                key_len,
                key_varint_len
            );

            key_slice
        }
    }

}

impl LevelDBIteratorValue for MemTableIterator {

    fn value(&self) -> Slice {
        trace!(
            "MemTableIterator::value: node={:p}",
            self.node
        );
        assert!(
            !self.node.is_null(),
            "MemTableIterator::value called on invalid iterator"
        );
        unsafe {
            let entry_ptr =
                self.current_node_key_ptr();

            // Decode the internal key length prefix at the entry start.
            let header_bytes =
                core::slice::from_raw_parts(entry_ptr, 5);
            let (key_len32, key_varint_len) =
                decode_varint32(header_bytes);
            let key_len = key_len32 as usize;
            let key_ptr =
                entry_ptr.add(key_varint_len);

            trace!(
                "MemTableIterator::value: internal_key_len={} (varint_len={})",
                key_len,
                key_varint_len
            );

            // Pointer to the value length varint prefix.
            let value_header_ptr =
                key_ptr.add(key_len);

            // Decode value length varint32 directly from raw bytes
            // without constructing an oversized Slice.
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
                    "MemTableIterator::value: value length varint32 shift overflow"
                );
                assert!(
                    consumed < 5,
                    "MemTableIterator::value: value length varint32 exceeds 5 bytes"
                );
            }

            let value_len = value_len_u32 as usize;
            let value_ptr =
                value_header_ptr.add(consumed);

            trace!(
                "MemTableIterator::value: decoded value_len={} (varint_len={})",
                value_len,
                consumed
            );

            let value_slice =
                Slice::from_ptr_len(
                    value_ptr,
                    value_len,
                );

            trace!(
                "MemTableIterator::value: returning value slice len={}",
                *value_slice.size()
            );

            value_slice
        }
    }

}

impl LevelDBIteratorStatus for MemTableIterator {
    fn status(&self) -> Status {
        trace!(
            "MemTableIterator::status: always OK"
        );
        Status::ok()
    }
}
