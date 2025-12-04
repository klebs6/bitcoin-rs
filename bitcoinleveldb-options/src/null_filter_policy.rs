// ---------------- [ File: bitcoinleveldb-options/src/null_filter_policy.rs ]
crate::ix!();

#[derive(Debug, Default, Clone, Copy)]
pub struct NullFilterPolicy;

impl FilterPolicy for NullFilterPolicy {}

impl CreateFilter for NullFilterPolicy {

    fn create_filter(
        &self,
        keys:  *const Slice,
        n:     i32,
        dst:   &mut Vec<u8>,
    ) {
        trace!(
            "NullFilterPolicy::create_filter: ignoring {} keys; current dst_len={}",
            n,
            dst.len(),
        );

        // This is intentionally a no-op filter implementation.
        //
        // We still touch `keys` via a no-op read to avoid "unused" warnings
        // and to make it clear that the pointer is expected to be valid,
        // even though we do not inspect any of the key material.
        let _ = keys;
    }
}

impl Named for NullFilterPolicy {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("null-filter-policy".to_string())
    }
}

impl KeyMayMatch for NullFilterPolicy {

    fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool {
        trace!(
            "NullFilterPolicy::key_may_match: unconditionally true \
             (key_len={}, filter_len={})",
            key.size(),
            filter.size(),
        );

        // A null / pass-through policy always reports a possible match.
        true
    }
}
