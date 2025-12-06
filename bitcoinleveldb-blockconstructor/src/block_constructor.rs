// ---------------- [ File: bitcoinleveldb-blockconstructor/src/block_constructor.rs ]
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

    #[traced_test]
    fn block_constructor_new_initializes_fields() {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());
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

    fn new_block_constructor_for_tests() -> (BlockConstructor, *const dyn SliceComparator) {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());
        let raw_cmp: *const dyn SliceComparator = &*cmp_box;

        trace!(
            "new_block_constructor_for_tests: creating BlockConstructor with comparator={:p}",
            raw_cmp
        );

        let constructor = BlockConstructor::new(cmp_box);
        (constructor, raw_cmp)
    }

    #[traced_test]
    fn block_constructor_new_initializes_all_fields_to_empty_state() {
        let (constructor, raw_cmp) = new_block_constructor_for_tests();

        trace!(
            "block_constructor_new_initializes_all_fields_to_empty_state: comparator_ptr={:p}",
            raw_cmp
        );

        assert!(constructor.data_string().is_empty());
        assert!(constructor.block_ptr().is_null());

        let base_data = constructor.base().data();
        debug!(
            "block_constructor_new_initializes_all_fields_to_empty_state: base constructor initial kv size={}",
            base_data.len()
        );
        assert!(base_data.is_empty());
    }

    #[traced_test]
    fn block_constructor_retains_comparator_box_ownership() {
        let (constructor, original_cmp_ptr) = new_block_constructor_for_tests();

        let cmp_ref: &dyn SliceComparator = constructor.comparator_ref();
        let cmp_ptr_from_constructor: *const dyn SliceComparator =
            cmp_ref as *const dyn SliceComparator;

        trace!(
            "block_constructor_retains_comparator_box_ownership: original_ptr={:p}, ctor_ptr={:p}",
            original_cmp_ptr,
            cmp_ptr_from_constructor
        );

        assert_eq!(cmp_ptr_from_constructor, original_cmp_ptr);

        // Also ensure base_mut can be borrowed mutably without panicking.
        let mut ctor = constructor;
        let base_mut_ref: &mut Constructor = ctor.base_mut();
        debug!(
            "block_constructor_retains_comparator_box_ownership: base_mut_ref.data_len={}",
            base_mut_ref.data().len()
        );
        assert!(base_mut_ref.data().is_empty());
    }
}
