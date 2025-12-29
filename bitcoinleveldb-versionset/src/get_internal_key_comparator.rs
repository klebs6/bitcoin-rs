// ---------------- [ File: bitcoinleveldb-versionset/src/get_internal_key_comparator.rs ]
crate::ix!();

impl GetInternalKeyComparator for VersionSet {
    fn icmp(&self) -> &InternalKeyComparator {
        VersionSet::icmp(self)
    }
}

impl VersionSet {
    pub fn get_internal_key_comparator(&self) -> &InternalKeyComparator {
        let icmp_ref: &InternalKeyComparator = <VersionSet as GetInternalKeyComparator>::icmp(self);

        trace!(
            icmp_ptr = %format!("{:p}", icmp_ref as *const InternalKeyComparator),
            "VersionSet::get_internal_key_comparator"
        );

        icmp_ref
    }
}
