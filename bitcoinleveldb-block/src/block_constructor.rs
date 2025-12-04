// ---------------- [ File: bitcoinleveldb-block/src/block_constructor.rs ]
crate::ix!();

pub struct BlockConstructor {
    base:       Constructor,
    comparator: Box<dyn SliceComparator>,
    data:       String,
    block:      *mut Block,
}

impl BlockConstructor {

    pub fn new(cmp: Box<dyn SliceComparator>) -> Self {
        let cmp_raw: *const dyn SliceComparator = &*cmp;

        trace!(
            "BlockConstructor::new: creating with comparator={:p}",
            cmp_raw
        );

        let base = Constructor::with_default();

        BlockConstructor {
            base,
            comparator: cmp,
            data:       String::new(),
            block:      core::ptr::null_mut(),
        }
    }

    #[inline]
    pub fn base(&self) -> &Constructor {
        &self.base
    }

    #[inline]
    pub fn base_mut(&mut self) -> &mut Constructor {
        &mut self.base
    }

    #[inline]
    pub fn comparator_ref(&self) -> &dyn SliceComparator {
        &*self.comparator
    }

    #[inline]
    pub fn data_string(&self) -> &String {
        &self.data
    }

    #[inline]
    pub fn data_string_mut(&mut self) -> &mut String {
        &mut self.data
    }

    #[inline]
    pub fn block_ptr(&self) -> *mut Block {
        self.block
    }

    #[inline]
    pub fn block_ptr_mut(&mut self) -> &mut *mut Block {
        &mut self.block
    }

    pub fn new_iterator(&self) -> *mut LevelDBIterator {
        let block_ptr = self.block_ptr();
        let data_len  = self.data_string().len();
        // Safe as a raw pointer: BlockConstructor owns the Box<dyn SliceComparator>
        let cmp_ptr: *const dyn SliceComparator =
            &*self.comparator as *const dyn SliceComparator;

        trace!(
            "BlockConstructor::new_iterator: block={:?}, data_len={}, comparator_ptr={:p}",
            block_ptr,
            data_len,
            cmp_ptr
        );

        assert!(
            !block_ptr.is_null(),
            "BlockConstructor::new_iterator: block pointer is null (FinishImpl must be called first)"
        );

        unsafe {
            let block_ref: &mut Block = &mut *block_ptr;
            let iter_ptr = block_ref.new_iterator(cmp_ptr);

            trace!(
                "BlockConstructor::new_iterator: created iterator at {:?}",
                iter_ptr
            );

            iter_ptr
        }
    }
}

#[cfg(test)]
mod block_constructor_initialization_tests {
    use super::*;

    #[derive(Clone, Default)]
    struct DummyComparator;

    impl Compare for DummyComparator {
        fn compare(&self, a: &Slice, b: &Slice) -> i32 {
            let a_bytes = unsafe {
                core::slice::from_raw_parts(*a.data(), *a.size())
            };
            let b_bytes = unsafe {
                core::slice::from_raw_parts(*b.data(), *b.size())
            };
            for (aa, bb) in a_bytes.iter().zip(b_bytes.iter()) {
                if aa < bb {
                    return -1;
                }
                if aa > bb {
                    return 1;
                }
            }
            a_bytes.len().cmp(&b_bytes.len()) as i32
        }
    }

    impl Named for DummyComparator {
        fn name(&self) -> &str {
            "dummy-comparator"
        }
    }

    impl FindShortestSeparator for DummyComparator {
        fn find_shortest_separator(&self, _start: &mut String, _limit: &Slice) {
            // Simple no-op for tests; behavior not relied upon.
        }
    }

    impl FindShortSuccessor for DummyComparator {
        fn find_short_successor(&self, _key: &mut String) {
            // Simple no-op for tests; behavior not relied upon.
        }
    }

    impl SliceComparator for DummyComparator {}

    #[traced_test]
    fn block_constructor_new_initializes_fields() {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(bitcoinleveldb_comparator::BytewiseComparatorImpl::default());
        let raw_cmp: *const dyn SliceComparator = &*cmp_box;

        let constructor = BlockConstructor::new(cmp_box);

        trace!(
            "verifying BlockConstructor::new; comparator_ptr={:p}",
            raw_cmp
        );

        assert!(constructor.data_string().is_empty());
        assert!(constructor.block_ptr().is_null());

        let base_data = constructor.base().data();
        debug!("base constructor initial kv size={}", base_data.len());
        assert!(base_data.is_empty());
    }
}
