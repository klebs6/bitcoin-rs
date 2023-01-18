crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/epochguard.h]

//-------------------------
#[SCOPED_LOCKABLE]
pub struct EpochGuard {
    pub(crate) epoch: Rc<RefCell<Epoch>>,
}

impl Drop for EpochGuard {

    #[UNLOCK_FUNCTION]
    fn drop(&mut self) {

        assert!(self.epoch.borrow().guarded);

        // ensure clear separation between epochs
        self.epoch.borrow_mut().raw_epoch += 1; 

        self.epoch.borrow_mut().guarded = false;
    }
}

impl EpochGuard {

    #[EXCLUSIVE_LOCK_FUNCTION(epoch)]
    pub fn new(epoch: Rc<RefCell<Epoch>>) -> Self {

        let x = Self {
            epoch: epoch.clone(),
        };

        assert!(!x.epoch.borrow().guarded);
        x.epoch.borrow_mut().raw_epoch += 1;
        x.epoch.borrow_mut().guarded = true;

        x
    }
}

macro_rules! with_fresh_epoch {
    ($epoch:ident) => {
        /*
                const Epoch::Guard PASTE2(epoch_guard_, __COUNTER__)(epoch)
        */
    }
}
