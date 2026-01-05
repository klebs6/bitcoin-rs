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

#[cfg(test)]
mod null_filter_policy_contract_suite {
    use super::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn null_filter_policy_name_is_stable() {
        trace!("null_filter_policy_contract_suite: start");

        let p = NullFilterPolicy::default();
        let n = p.name();

        info!(name = %n, "NullFilterPolicy name()");
        assert_eq!(n.as_ref(), "null-filter-policy");

        trace!("null_filter_policy_contract_suite: done");
    }

    #[traced_test]
    fn null_filter_policy_create_filter_is_a_noop_and_preserves_dst() {
        trace!("null_filter_policy_contract_suite: start");

        let p = NullFilterPolicy::default();

        let mut dst: Vec<u8> = vec![0xAA, 0xBB, 0xCC];
        let before = dst.clone();

        // `NullFilterPolicy` must be able to accept any `keys` pointer without dereferencing it.
        let keys: *const Slice = core::ptr::null();

        p.create_filter(keys, 123, &mut dst);

        debug!(before_len = before.len(), after_len = dst.len(), "dst lengths");
        assert_eq!(dst, before);

        trace!("null_filter_policy_contract_suite: done");
    }

    #[traced_test]
    fn null_filter_policy_key_may_match_is_unconditionally_true() {
        trace!("null_filter_policy_contract_suite: start");

        let p = NullFilterPolicy::default();

        let key = Slice::from(&b"any-key"[..]);
        let filter = Slice::from(&b""[..]);

        let ok = p.key_may_match(&key, &filter);
        info!(
            key_len = key.size(),
            filter_len = filter.size(),
            ok,
            "key_may_match result"
        );

        assert!(ok);

        trace!("null_filter_policy_contract_suite: done");
    }
}
