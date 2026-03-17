// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_borrowed_filter_policy_adapter.rs ]
crate::ix!();

pub struct BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter {
    ptr: *const dyn FilterPolicy,
}

impl BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter {
    #[inline]
    pub fn new(ptr: *const dyn FilterPolicy) -> Self {
        trace!(
            "BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter::new: policy_ptr={:p}",
            ptr
        );

        Self { ptr }
    }

    #[inline]
    pub fn policy_ref(&self) -> Option<&dyn FilterPolicy> {
        if self.ptr.is_null() {
            trace!(
                "BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter::policy_ref: null policy pointer"
            );
            None
        } else {
            unsafe { Some(&*self.ptr) }
        }
    }
}

impl FilterPolicy for BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter {}

impl Named for BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter {
    fn name(&self) -> Cow<'_, str> {
        match self.policy_ref() {
            Some(policy) => policy.name(),
            None => Cow::Borrowed("table-builder-null-filter-policy"),
        }
    }
}

impl CreateFilter for BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter {
    fn create_filter(
        &self,
        keys: *const Slice,
        n: i32,
        dst: &mut Vec<u8>,
    ) {
        trace!(
            "BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter::create_filter: n_keys={}, dst_len_before={}",
            n,
            dst.len()
        );

        match self.policy_ref() {
            Some(policy) => {
                policy.create_filter(keys, n, dst);
            }
            None => {
                trace!(
                    "BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter::create_filter: null policy; no-op"
                );
            }
        }

        trace!(
            "BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter::create_filter: dst_len_after={}",
            dst.len()
        );
    }
}

impl KeyMayMatch for BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter {
    fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool {
        trace!(
            "BitcoinLevelDbTableBuilderBorrowedFilterPolicyAdapter::key_may_match: key_len={}, filter_len={}",
            *key.size(),
            *filter.size()
        );

        match self.policy_ref() {
            Some(policy) => policy.key_may_match(key, filter),
            None => true,
        }
    }
}
