// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/key.rs ]
crate::ix!();

impl Key for KeyConvertingIterator {

    fn key(&self) -> Slice {
        trace!(
            "KeyConvertingIterator::key: called; iter={:?}",
            self.iter()
        );

        assert!(
            self.valid(),
            "KeyConvertingIterator::key requires the iterator to be valid"
        );

        unsafe {
            assert!(
                !self.iter().is_null(),
                "KeyConvertingIterator::key: underlying iterator pointer is null"
            );

            let internal_key = (*(*self.iter())).key();
            trace!(
                "KeyConvertingIterator::key: underlying internal key slice={:?}",
                internal_key
            );

            let mut parsed = ParsedInternalKey::default();
            let ok = parse_internal_key(
                &internal_key,
                &mut parsed as *mut ParsedInternalKey,
            );

            if !ok {
                trace!(
                    "KeyConvertingIterator::key: ParseInternalKey failed; marking status as corruption"
                );

                let msg_slice = Slice::from("malformed internal key");
                let st = Status::corruption(&msg_slice, None);
                *self.status().borrow_mut() = st;

                let corrupted = Slice::from("corrupted key");
                trace!(
                    "KeyConvertingIterator::key: returning synthetic corrupted key slice={:?}",
                    corrupted
                );
                corrupted
            } else {
                let user_key_ref: &Slice = parsed.user_key();
                let data = *user_key_ref.data();
                let size = *user_key_ref.size();
                let user_key = Slice::from_ptr_len(data, size);

                trace!(
                    "KeyConvertingIterator::key: returning parsed user key slice (data={:?}, size={})",
                    data,
                    size
                );

                user_key
            }
        }
    }
}
