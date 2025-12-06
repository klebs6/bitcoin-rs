// ---------------- [ File: bitcoinleveldb-blockiter/src/block_iter_next_entry_offset.rs ]
crate::ix!();

impl BlockIter {

    /// Return the offset in data_ just past
    /// the end of the current entry.
    /// 
    #[inline]
    pub fn next_entry_offset(&self) -> u32 {
        unsafe {
            let base      = self.data_ptr() as usize;
            let value_ref = self.value_slice();

            let end = if (*value_ref.data()).is_null() {
                base + self.current_offset() as usize
            } else {
                (*value_ref.data() as usize).saturating_add(*value_ref.size())
            };

            let offset = end.saturating_sub(base) as u32;
            trace!(
                "BlockIter::next_entry_offset: current={}, computed_offset={}",
                self.current_offset(),
                offset
            );
            offset
        }
    }
}

#[cfg(test)]
mod block_iter_next_entry_offset_tests {
    use super::*;

    #[traced_test]
    fn next_entry_offset_tracks_value_slice_end() {
        let backing = b"\x01\x01\x01kV".to_vec();
        let cmp = BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let mut iter = BlockIter::new(
            cmp_ptr,
            backing.as_ptr(),
            backing.len() as u32,
            1,
        );

        unsafe {
            let value_ptr = backing.as_ptr().add(2);
            let value     = Slice::from_ptr_len(value_ptr, 2);
            *iter.value_slice_mut() = value;
        }

        iter.set_current_offset(0);

        let offset = iter.next_entry_offset();
        trace!(
            "computed next_entry_offset={}, expected={}",
            offset,
            2 + 2
        );
        debug!("backing_len={}", backing.len());

        assert_eq!(offset, 4u32);
    }
}
