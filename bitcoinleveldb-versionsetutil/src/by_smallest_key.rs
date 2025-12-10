// ---------------- [ File: bitcoinleveldb-versionsetutil/src/by_smallest_key.rs ]
crate::ix!();

/**
  | Helper to sort by
  | v->files_[file_number].smallest
  |
  */
pub struct BySmallestKeyComparator {
    internal_comparator: *const InternalKeyComparator,
}

impl BySmallestKeyComparator {

    pub fn invoke(
        &self,
        f1: *mut FileMetaData,
        f2: *mut FileMetaData,
    ) -> bool {
        unsafe {
            debug_assert!(
                !self.internal_comparator.is_null(),
                "BySmallestKeyComparator::invoke: internal_comparator is null"
            );
            debug_assert!(
                !f1.is_null() && !f2.is_null(),
                "BySmallestKeyComparator::invoke: file pointers must not be null"
            );

            let icmp  = &*self.internal_comparator;
            let file1 = &*f1;
            let file2 = &*f2;

            let r = icmp.compare_internal_key(file1.smallest(), file2.smallest());

            trace!(
                file1_number = *file1.number(),
                file2_number = *file2.number(),
                cmp_smallest = r,
                "BySmallestKeyComparator::invoke"
            );

            if r != 0 {
                r < 0
            } else {
                // Break ties by file number
                *file1.number() < *file2.number()
            }
        }
    }
}

#[cfg(test)]
mod by_smallest_key_spec {
    use super::*;

    fn ikey_from_str(user_key_str: &str, seq: u64) -> InternalKey {
        let user_slice = Slice::from(user_key_str);
        InternalKey::new(&user_slice, seq, ValueType::TypeValue)
    }

    #[traced_test]
    fn verify_by_smallest_key_orders_by_smallest_internal_key() {
        let icmp = InternalKeyComparator::new(bytewise_comparator());
        let cmp = BySmallestKeyComparator {
            internal_comparator: &icmp as *const InternalKeyComparator,
        };

        let mut f1 = FileMetaData::default();
        f1.set_number(1);
        f1.set_smallest(ikey_from_str("100", 10));
        f1.set_largest(ikey_from_str("199", 10));

        let mut f2 = FileMetaData::default();
        f2.set_number(2);
        f2.set_smallest(ikey_from_str("200", 10));
        f2.set_largest(ikey_from_str("299", 10));

        let p1: *mut FileMetaData = &mut f1;
        let p2: *mut FileMetaData = &mut f2;

        let r12 = cmp.invoke(p1, p2);
        let r21 = cmp.invoke(p2, p1);

        debug!(
            "verify_by_smallest_key_orders_by_smallest_internal_key: r12={}, r21={}",
            r12,
            r21
        );

        assert!(r12, "file with smallest='100' must come before '200'");
        assert!(
            !r21,
            "file with smallest='200' must not come before '100'"
        );
    }

    #[traced_test]
    fn verify_by_smallest_key_breaks_ties_by_file_number() {
        let icmp = InternalKeyComparator::new(bytewise_comparator());
        let cmp = BySmallestKeyComparator {
            internal_comparator: &icmp as *const InternalKeyComparator,
        };

        let mut f1 = FileMetaData::default();
        f1.set_number(1);
        f1.set_smallest(ikey_from_str("100", 10));
        f1.set_largest(ikey_from_str("199", 10));

        let mut f2 = FileMetaData::default();
        f2.set_number(2);
        f2.set_smallest(ikey_from_str("100", 9)); // same user key, different sequence
        f2.set_largest(ikey_from_str("199", 9));

        let p1: *mut FileMetaData = &mut f1;
        let p2: *mut FileMetaData = &mut f2;

        let r12 = cmp.invoke(p1, p2);
        let r21 = cmp.invoke(p2, p1);

        debug!(
            "verify_by_smallest_key_breaks_ties_by_file_number: r12={}, r21={}",
            r12,
            r21
        );

        assert!(
            r12,
            "When smallest keys compare equal, lower-numbered file should come first"
        );
        assert!(
            !r21,
            "Higher-numbered file must not be ordered before lower-numbered file when keys tie"
        );
    }
}
