crate::ix!();

/// Comprehensive behavioural tests for epoch‑based
/// traversal de‑duplication.
#[cfg(test)]
mod epoch_deduplication_behavior {

    use super::*;

    /// The first call to `visited` in a fresh epoch
    /// returns `false`; subsequent calls return `true`.
    #[traced_test]
    fn first_then_second_visit() {
        let epoch = Rc::new(RefCell::new(Epoch::default()));
        let _guard = EpochGuard::new(epoch.clone());

        let mut marker = EpochMarker::default();

        assert!(!epoch.borrow().visited(&mut marker));
        assert!( epoch.borrow().visited(&mut marker));
    }

    /// Dropping an `EpochGuard` cleanly increments the
    /// epoch and clears the guarded flag.
    #[traced_test]
    fn guard_drop_resets_state() {
        let epoch = Rc::new(RefCell::new(Epoch::default()));

        // scope 1
        {
            let _guard = EpochGuard::new(epoch.clone());
            assert!(epoch.borrow().guarded());
        } // drop here

        assert!(!epoch.borrow().guarded());

        // scope 2 – new guard → new epoch
        {
            let _guard = EpochGuard::new(epoch.clone());
            let mut marker = EpochMarker::default();
            assert!(!epoch.borrow().visited(&mut marker));
        }
    }

    /// The helper macro spawns an `EpochGuard`
    /// whose lifetime is the surrounding block.
    #[traced_test]
    fn macro_creates_and_drops_guard() {
        let epoch = Rc::new(RefCell::new(Epoch::default()));

        {
            with_fresh_epoch!(epoch);
            assert!(epoch.borrow().guarded());
        }

        assert!(!epoch.borrow().guarded());
    }

    /// Attempting to nest guards must panic.
    #[test]
    #[should_panic(expected = "nested EpochGuard")]
    fn nested_guard_panics() {
        let epoch = Rc::new(RefCell::new(Epoch::default()));
        let _g1 = EpochGuard::new(epoch.clone());
        let _g2 = EpochGuard::new(epoch); // should panic
    }
}
