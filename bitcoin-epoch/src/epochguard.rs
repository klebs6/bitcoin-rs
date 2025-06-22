// ---------------- [ File: bitcoin-epoch/src/epochguard.rs ]
crate::ix!();

/// RAII guard establishing a fresh epoch.
#[derive(Getters)]
#[SCOPED_LOCKABLE]
pub struct EpochGuard {
    epoch: Rc<RefCell<Epoch>>,
}

impl EpochGuard {

    /// Begin a new epoch and mark it guarded.
    ///
    /// *Panics* if a guard is already active for the
    /// supplied [`Epoch`], preventing accidental
    /// nested scopes.
    #[EXCLUSIVE_LOCK_FUNCTION(epoch)]
    pub fn new(epoch: Rc<RefCell<Epoch>>) -> Self {
        // 1️⃣  Sanity check *before* any state changes or
        //     partially‑constructed values that might
        //     trigger a destructor on unwind.
        {
            let ep_ref = epoch.borrow();
            assert!(
                !ep_ref.guarded(),
                "attempted to create nested EpochGuard"
            );
        }

        // 2️⃣  All clear – start a fresh epoch and mark
        //     it guarded.
        {
            let mut ep_mut = epoch.borrow_mut();
            ep_mut.increment_epoch();
            ep_mut.set_guarded(true);
        }

        trace!(target: "epoch", "EpochGuard created");

        // 3️⃣  Now construct the guard *after* the epoch
        //     state is guaranteed to be valid.
        Self { epoch }
    }
}

impl Drop for EpochGuard {
    #[UNLOCK_FUNCTION]
    fn drop(&mut self) {
        let mut ep = self.epoch.borrow_mut();

        assert!(
            ep.guarded(),
            "EpochGuard dropped while not guarded"
        );

        ep.increment_epoch();     // clear separation
        ep.set_guarded(false);    // leave un‑guarded
        trace!(target: "epoch", "EpochGuard dropped");
    }
}
