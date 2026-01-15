// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_max_next_level_overlapping_bytes.rs ]
crate::ix!();

impl DBImpl {
    
    /// Return the maximum overlapping data (in bytes) at next level for any file at a level >= 1.
    pub fn test_max_next_level_overlapping_bytes(&mut self) -> i64 { 
        self.mutex.lock();
        let v = unsafe { (*self.versions).max_next_level_overlapping_bytes() };
        unsafe { self.mutex.unlock() };
        v
    }
}

#[cfg(test)]
mod dbimpl_max_next_level_overlap_contract_suite {
    use super::*;

    #[traced_test]
    fn dbimpl_test_max_next_level_overlapping_bytes_signature_is_stable() {
        tracing::info!("Asserting DBImpl::test_max_next_level_overlapping_bytes signature is stable");

        type Sig = fn(&mut DBImpl) -> i64;
        let _sig: Sig = DBImpl::test_max_next_level_overlapping_bytes;

        tracing::debug!("DBImpl::test_max_next_level_overlapping_bytes signature check compiled");
    }

    #[traced_test]
    fn versionset_max_next_level_overlapping_bytes_requires_mut_self_by_contract() {
        tracing::info!("Asserting VersionSet::max_next_level_overlapping_bytes requires &mut self");

        type Sig = fn(&mut VersionSet) -> i64;
        let _sig: Sig = VersionSet::max_next_level_overlapping_bytes;

        tracing::debug!("VersionSet::max_next_level_overlapping_bytes signature check compiled");
    }

    #[traced_test]
    fn max_next_level_overlapping_bytes_trait_is_implemented_for_versionset() {
        tracing::info!("Asserting MaxNextLevelOverlappingBytes is implemented for VersionSet");

        fn _assert_impl<T: MaxNextLevelOverlappingBytes>() {}
        _assert_impl::<VersionSet>();

        tracing::debug!("Trait implementation constraint compiled");
    }

    #[traced_test]
    fn raw_mutex_unlock_is_unsafe_and_must_be_called_under_unsafe_block() {
        tracing::info!("Validating RawMutex unlock requires unsafe and is usable in a lock/unlock cycle");

        let mut mu: RawMutex = RawMutex::INIT;

        tracing::trace!("Locking RawMutex");
        mu.lock();

        tracing::trace!("Unlocking RawMutex (unsafe)");
        unsafe {
            mu.unlock();
        }

        tracing::trace!("Re-locking RawMutex after unsafe unlock");
        mu.lock();

        tracing::trace!("Final unlock (unsafe)");
        unsafe {
            mu.unlock();
        }

        tracing::debug!("RawMutex lock/unlock cycle completed");
    }

    #[traced_test]
    fn const_versionset_pointer_can_be_cast_to_mut_pointer_for_mut_method_dispatch() {
        tracing::info!("Validating *const VersionSet -> *mut VersionSet cast compiles for call sites");

        let p_const: *const VersionSet = core::ptr::null::<VersionSet>();
        let p_mut: *mut VersionSet = p_const as *mut VersionSet;

        tracing::trace!(p_const = ?p_const, p_mut = ?p_mut, "Pointer cast results");
        assert!(p_mut.is_null());
    }
}
