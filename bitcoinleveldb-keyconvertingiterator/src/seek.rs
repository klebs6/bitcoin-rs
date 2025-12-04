// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/seek.rs ]
crate::ix!();

impl Seek for KeyConvertingIterator {

    fn seek(&mut self, target: &Slice) {
        trace!(
            "KeyConvertingIterator::seek: target={:?}, iter={:?}",
            target,
            self.iter()
        );

        // This mirrors the original C++:
        //
        //   ParsedInternalKey ikey(target, kMaxSequenceNumber, kTypeValue);
        //   std::string encoded;
        //   AppendInternalKey(&encoded, ikey);
        //   iter_->Seek(encoded);
        //
        let max_sequence: SequenceNumber = ((0x1u64 << 56) - 1);
        let ikey = ParsedInternalKey::new(target, &max_sequence, ValueType::TypeValue);

        let mut encoded = String::new();
        unsafe {
            append_internal_key(&mut encoded as *mut String, &ikey);
        }

        let encoded_slice = Slice::from(&encoded);

        unsafe {
            assert!(
                !self.iter().is_null(),
                "KeyConvertingIterator::seek: underlying iterator pointer is null"
            );
            (*(*self.iter())).seek(&encoded_slice);
        }
    }
}
