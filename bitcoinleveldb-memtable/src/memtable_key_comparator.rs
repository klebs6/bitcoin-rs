// ---------------- [ File: bitcoinleveldb-memtable/src/memtable_key_comparator.rs ]
crate::ix!();

pub struct MemTableKeyComparator {
    comparator: InternalKeyComparator,
}

impl MemTableKeyComparator {
    pub fn new(c: &InternalKeyComparator) -> Self {
        let user_cmp_ptr = c.user_comparator();
        trace!(
            "MemTableKeyComparator::new: user_comparator_ptr={:p}",
            user_cmp_ptr
        );

        let internal_copy = InternalKeyComparator::new(user_cmp_ptr);

        MemTableKeyComparator {
            comparator: internal_copy,
        }
    }

    #[inline]
    pub fn internal_comparator(&self) -> &InternalKeyComparator {
        &self.comparator
    }

    pub fn invoke(
        &self,
        aptr: *const u8,
        bptr: *const u8,
    ) -> i32 {
        trace!(
            "MemTableKeyComparator::invoke: aptr={:?}, bptr={:?}",
            aptr,
            bptr
        );

        unsafe {
            // Decode internal key for `aptr`.
            let header_a =
                core::slice::from_raw_parts(aptr, 5);
            let (a_len32, a_varint_len) =
                decode_varint32(header_a);
            let a_len = a_len32 as usize;
            let a_key_ptr =
                aptr.add(a_varint_len);

            // Decode internal key for `bptr`.
            let header_b =
                core::slice::from_raw_parts(bptr, 5);
            let (b_len32, b_varint_len) =
                decode_varint32(header_b);
            let b_len = b_len32 as usize;
            let b_key_ptr =
                bptr.add(b_varint_len);

            let a_slice =
                Slice::from_ptr_len(
                    a_key_ptr,
                    a_len,
                );
            let b_slice =
                Slice::from_ptr_len(
                    b_key_ptr,
                    b_len,
                );

            trace!(
                "MemTableKeyComparator::invoke: a_len={} (varint_len={}), b_len={} (varint_len={})",
                a_len,
                a_varint_len,
                b_len,
                b_varint_len
            );

            let result =
                self.comparator.compare(
                    &a_slice,
                    &b_slice,
                );

            trace!(
                "MemTableKeyComparator::invoke: result={}",
                result
            );

            result
        }
    }
}

unsafe impl Send for MemTableKeyComparator {}
unsafe impl Sync for MemTableKeyComparator {}

impl SkipListComparator<*const u8> for MemTableKeyComparator {
    fn compare(
        &self,
        a: &*const u8,
        b: &*const u8,
    ) -> i32 {
        trace!(
            "MemTableKeyComparator::compare (SkipListComparator): a={:?}, b={:?}",
            a,
            b
        );
        self.invoke(*a, *b)
    }
}
